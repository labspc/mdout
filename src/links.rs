use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail};
use chrono::{DateTime, Utc};
use futures::{StreamExt, stream};
use regex::Regex;
use reqwest::{Client, Method, StatusCode};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct LinkRecord {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    final_url: Option<String>,
    response_ms: u128,
    sources: Vec<String>,
    checked_at: String,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Report {
    generated_at: String,
    links: BTreeMap<String, LinkRecord>,
}

pub async fn run(directory: &Path, force: bool, strict: bool, concurrency: usize) -> Result<()> {
    let source_links = collect(directory)?;
    let cache_path = Path::new(".mdout-cache/links.json");
    let report_path = Path::new("reports/links.json");
    let cache = read_report(cache_path);
    let now = Utc::now();
    let mut results = BTreeMap::new();
    let mut pending = Vec::new();

    for (url, sources) in source_links {
        let cached = cache.links.get(&url).filter(|record| {
            !force
                && DateTime::parse_from_rfc3339(&record.checked_at)
                    .map(|checked| {
                        now.signed_duration_since(checked.with_timezone(&Utc))
                            .num_days()
                            < 7
                    })
                    .unwrap_or(false)
        });
        if let Some(cached) = cached {
            let mut record = cached.clone();
            record.sources = sources;
            results.insert(url, record);
        } else {
            pending.push((url, sources));
        }
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(8))
        .redirect(reqwest::redirect::Policy::limited(10))
        .user_agent("mdout/0.1 (+https://github.com/labspc/mdout)")
        .build()?;
    let checked = stream::iter(pending)
        .map(|(url, sources)| {
            let client = client.clone();
            async move {
                let record = check_url(&client, &url, sources).await;
                (url, record)
            }
        })
        .buffer_unordered(concurrency.max(1))
        .collect::<Vec<_>>()
        .await;
    results.extend(checked);

    let report = Report {
        generated_at: now.to_rfc3339(),
        links: results,
    };
    write_report(cache_path, &report)?;
    write_report(report_path, &report)?;

    for (url, record) in &report.links {
        println!(
            "{:<12} {:<3} {url}",
            record.status,
            record
                .http_status
                .map(|value| value.to_string())
                .unwrap_or_else(|| "-".into())
        );
    }
    println!("Checked {} links", report.links.len());

    if strict
        && report
            .links
            .values()
            .any(|record| record.status == "broken")
    {
        bail!("broken external links were found");
    }
    Ok(())
}

async fn check_url(client: &Client, url: &str, sources: Vec<String>) -> LinkRecord {
    let started = Instant::now();
    let mut response = client.request(Method::HEAD, url).send().await;
    if matches!(
        response.as_ref().map(|value| value.status()),
        Ok(StatusCode::METHOD_NOT_ALLOWED | StatusCode::NOT_IMPLEMENTED)
    ) {
        response = client
            .request(Method::GET, url)
            .header("range", "bytes=0-0")
            .send()
            .await;
    }

    match response {
        Ok(response) => LinkRecord {
            status: classify(response.status()).into(),
            http_status: Some(response.status().as_u16()),
            final_url: Some(response.url().to_string()),
            response_ms: started.elapsed().as_millis(),
            sources,
            checked_at: Utc::now().to_rfc3339(),
        },
        Err(_) => LinkRecord {
            status: "unreachable".into(),
            http_status: None,
            final_url: None,
            response_ms: started.elapsed().as_millis(),
            sources,
            checked_at: Utc::now().to_rfc3339(),
        },
    }
}

fn classify(status: StatusCode) -> &'static str {
    match status.as_u16() {
        200..=399 => "ok",
        401 | 403 => "restricted",
        429 => "rate-limited",
        404 | 410 => "broken",
        _ => "error",
    }
}

fn collect(directory: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let link = Regex::new(r#"\[[^\]]*\]\((https?://[^\s)]+)"#)?;
    let mut links: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        let source = fs::read_to_string(path)?;
        let relative = path
            .strip_prefix(directory)
            .unwrap_or(path)
            .display()
            .to_string();
        for capture in link.captures_iter(&source) {
            links
                .entry(capture[1].to_string())
                .or_default()
                .insert(relative.clone());
        }
    }
    Ok(links
        .into_iter()
        .map(|(url, sources)| (url, sources.into_iter().collect()))
        .collect())
}

fn read_report(path: &Path) -> Report {
    fs::read_to_string(path)
        .ok()
        .and_then(|source| serde_json::from_str(&source).ok())
        .unwrap_or_default()
}

fn write_report(path: &Path, report: &Report) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut source = serde_json::to_string_pretty(report)?;
    source.push('\n');
    fs::write(path, source).with_context(|| format!("failed to write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn classifies_http_statuses() {
        assert_eq!(classify(StatusCode::OK), "ok");
        assert_eq!(classify(StatusCode::FORBIDDEN), "restricted");
        assert_eq!(classify(StatusCode::TOO_MANY_REQUESTS), "rate-limited");
        assert_eq!(classify(StatusCode::NOT_FOUND), "broken");
        assert_eq!(classify(StatusCode::INTERNAL_SERVER_ERROR), "error");
    }

    #[test]
    fn collects_markdown_links() {
        let root = tempdir().unwrap();
        fs::write(
            root.path().join("post.md"),
            "[Zola](https://www.getzola.org/)\n",
        )
        .unwrap();
        let links = collect(root.path()).unwrap();
        assert_eq!(links["https://www.getzola.org/"], ["post.md"]);
    }
}
