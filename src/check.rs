use std::{fs, path::Path};

use anyhow::{Result, bail};
use regex::Regex;
use serde_yaml::{Mapping, Value};
use walkdir::WalkDir;

const IMAGE_EXTENSIONS: &[&str] = &["avif", "gif", "jpeg", "jpg", "png", "svg", "webp"];

struct Rules {
    date: Regex,
    markdown_image: Regex,
    html_image: Regex,
    mermaid: Regex,
    math: Regex,
    tex_punctuation: Regex,
    mermaid_type: Regex,
}

impl Rules {
    fn new() -> Result<Self> {
        Ok(Self {
            date: Regex::new(r"^\d{4}-\d{2}-\d{2}$")?,
            markdown_image: Regex::new(r"!\[[^\]]*\]\([^)]+\)")?,
            html_image: Regex::new(r"(?i)<\s*img\b")?,
            mermaid: Regex::new(r"(?s)```mermaid\s*\n(.*?)\n```")?,
            math: Regex::new(r"(?s)(\$\$.*?\$\$|\$[^$\n]+?\$)")?,
            tex_punctuation: Regex::new(r"(^|[^\\])\\([,;:! ])")?,
            mermaid_type: Regex::new(
                r"^(flowchart|graph|sequenceDiagram|classDiagram|stateDiagram(?:-v2)?|erDiagram|journey|gantt|pie|gitGraph|mindmap|timeline|quadrantChart|xychart-beta|requirementDiagram|C4\w*|block-beta|packet-beta|architecture-beta|kanban|sankey-beta|radar-beta|treemap-beta)\b",
            )?,
        })
    }
}

pub fn run(directory: &Path) -> Result<()> {
    let diagnostics = validate(directory)?;
    if diagnostics.is_empty() {
        println!("Content check passed: {}", directory.display());
        return Ok(());
    }

    for diagnostic in &diagnostics {
        eprintln!("{diagnostic}");
    }
    bail!("content check failed with {} error(s)", diagnostics.len())
}

fn validate(directory: &Path) -> Result<Vec<String>> {
    let mut diagnostics = Vec::new();
    let rules = Rules::new()?;

    if !directory.is_dir() {
        return Ok(vec![format!(
            "{}: content directory does not exist",
            directory.display()
        )]);
    }

    for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let relative = path.strip_prefix(directory).unwrap_or(path);
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        if IMAGE_EXTENSIONS.contains(&extension.to_ascii_lowercase().as_str()) {
            diagnostics.push(format!(
                "{}: article images are not supported",
                relative.display()
            ));
            continue;
        }
        if extension != "md" {
            continue;
        }
        validate_markdown(path, relative, &rules, &mut diagnostics)?;
    }

    Ok(diagnostics)
}

fn validate_markdown(
    path: &Path,
    relative: &Path,
    rules: &Rules,
    diagnostics: &mut Vec<String>,
) -> Result<()> {
    let source = fs::read_to_string(path)?;
    let (frontmatter, body) = match split_frontmatter(&source) {
        Ok(parts) => parts,
        Err(message) => {
            diagnostics.push(format!("{}: {message}", relative.display()));
            return Ok(());
        }
    };
    let data: Value = match serde_yaml::from_str(frontmatter) {
        Ok(value) => value,
        Err(error) => {
            diagnostics.push(format!(
                "{}: invalid frontmatter: {error}",
                relative.display()
            ));
            return Ok(());
        }
    };
    let mapping = data.as_mapping().cloned().unwrap_or_default();
    validate_frontmatter(&mapping, path, relative, &rules.date, diagnostics);
    validate_body(body, relative, rules, diagnostics);
    Ok(())
}

fn validate_frontmatter(
    mapping: &Mapping,
    path: &Path,
    relative: &Path,
    date: &Regex,
    diagnostics: &mut Vec<String>,
) {
    require_string(mapping, "title", relative, diagnostics);
    reject_fields(mapping, relative, diagnostics);

    let is_post = relative.starts_with("posts") && !is_section_index(path);
    if !is_post {
        return;
    }
    match string(mapping, "date") {
        Some(value) if date.is_match(value) => {}
        _ => diagnostics.push(format!(
            "{}: frontmatter.date must use YYYY-MM-DD",
            relative.display()
        )),
    }
    if let Some(draft) = mapping.get(Value::String("draft".into()))
        && !draft.is_bool()
    {
        diagnostics.push(format!(
            "{}: frontmatter.draft must be true or false",
            relative.display()
        ));
    }
    validate_tags(mapping, relative, diagnostics);
}

fn validate_body(body: &str, relative: &Path, rules: &Rules, diagnostics: &mut Vec<String>) {
    if rules.markdown_image.is_match(body) {
        diagnostics.push(format!(
            "{}: Markdown images are not supported",
            relative.display()
        ));
    }
    if rules.html_image.is_match(body) {
        diagnostics.push(format!(
            "{}: HTML images are not supported",
            relative.display()
        ));
    }
    for capture in rules.mermaid.captures_iter(body) {
        let diagram = capture
            .get(1)
            .map(|value| value.as_str().trim())
            .unwrap_or("");
        if diagram.is_empty() {
            diagnostics.push(format!("{}: Mermaid block is empty", relative.display()));
        } else if !rules.mermaid_type.is_match(diagram) {
            diagnostics.push(format!(
                "{}: Mermaid block has an unknown diagram type",
                relative.display()
            ));
        }
    }
    for formula in rules.math.find_iter(body) {
        if rules.tex_punctuation.is_match(formula.as_str()) {
            diagnostics.push(format!(
                "{}: TeX punctuation commands need a doubled backslash in Markdown (for example `\\\\,`)",
                relative.display()
            ));
        }
    }
}

fn is_section_index(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "_index.md" || name.starts_with("_index."))
}

fn split_frontmatter(source: &str) -> std::result::Result<(&str, &str), &'static str> {
    let source = source
        .strip_prefix("---\n")
        .ok_or("missing YAML frontmatter")?;
    let end = source.find("\n---\n").ok_or("frontmatter is not closed")?;
    Ok((&source[..end], &source[end + 5..]))
}

fn string<'a>(mapping: &'a Mapping, key: &str) -> Option<&'a str> {
    mapping
        .get(Value::String(key.into()))
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
}

fn require_string(mapping: &Mapping, key: &str, file: &Path, diagnostics: &mut Vec<String>) {
    if string(mapping, key).is_none() {
        diagnostics.push(format!(
            "{}: frontmatter.{key} must be a non-empty string",
            file.display()
        ));
    }
}

fn reject_fields(mapping: &Mapping, file: &Path, diagnostics: &mut Vec<String>) {
    for field in ["image", "cover", "socialImage"] {
        if mapping.contains_key(Value::String(field.into())) {
            diagnostics.push(format!(
                "{}: frontmatter.{field} is not supported",
                file.display()
            ));
        }
    }
}

fn validate_tags(mapping: &Mapping, file: &Path, diagnostics: &mut Vec<String>) {
    let tags = mapping
        .get(Value::String("taxonomies".into()))
        .and_then(Value::as_mapping)
        .and_then(|taxonomies| taxonomies.get(Value::String("tags".into())));
    if let Some(tags) = tags
        && tags
            .as_sequence()
            .is_none_or(|items| items.iter().any(|item| item.as_str().is_none()))
    {
        diagnostics.push(format!(
            "{}: frontmatter.taxonomies.tags must be a list of strings",
            file.display()
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn accepts_a_zola_site_and_rejects_images() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join("posts")).unwrap();
        fs::write(root.path().join("_index.md"), "---\ntitle: Home\n---\n").unwrap();
        fs::write(
            root.path().join("posts/post.md"),
            "---\ntitle: Post\ndate: 2026-08-28\n---\nText.",
        )
        .unwrap();
        fs::write(
            root.path().join("posts/_index.en.md"),
            "---\ntitle: Articles\n---\n",
        )
        .unwrap();
        fs::write(
            root.path().join("posts/post.en.md"),
            "---\ntitle: Post\ndate: 2026-08-28\n---\nText.",
        )
        .unwrap();
        assert!(validate(root.path()).unwrap().is_empty());

        fs::write(root.path().join("posts/image.png"), "image").unwrap();
        assert!(validate(root.path()).unwrap()[0].contains("images are not supported"));
    }

    #[test]
    fn rejects_invalid_mermaid() {
        let root = tempdir().unwrap();
        fs::write(
            root.path().join("page.md"),
            "---\ntitle: Page\n---\n```mermaid\nunknown\n```",
        )
        .unwrap();
        assert!(validate(root.path()).unwrap()[0].contains("Mermaid"));
    }

    #[test]
    fn explains_tex_punctuation_escaping() {
        let root = tempdir().unwrap();
        fs::write(
            root.path().join("page.md"),
            "---\ntitle: Page\n---\n$$x\\,dx$$",
        )
        .unwrap();
        assert!(validate(root.path()).unwrap()[0].contains("doubled backslash"));

        fs::write(
            root.path().join("page.md"),
            "---\ntitle: Page\n---\n$$x\\\\,dx$$",
        )
        .unwrap();
        assert!(validate(root.path()).unwrap().is_empty());
    }
}
