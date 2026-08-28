use std::{fs, path::Path};

use anyhow::{Context, Result, bail, ensure};
use include_dir::{Dir, include_dir};
use toml_edit::{DocumentMut, value};

static CONTENT: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content");
static REPORTS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/reports");
static SASS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/sass");
static STATIC: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");
static TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");
static WORKFLOWS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/scaffold/.github/workflows");

const ZOLA_CONFIG: &str = include_str!("../zola.toml");
const GITIGNORE: &str = include_str!("../.gitignore");
const SCAFFOLD_README: &str = include_str!("../scaffold/README.md");
const MDOUT_MANIFEST: &str = include_str!("../mdout.toml");

pub fn run(path: &Path, title: &str, base_url: &str, author: &str) -> Result<()> {
    validate_input(path, title, base_url)?;

    fs::create_dir_all(path).with_context(|| format!("could not create {}", path.display()))?;
    extract(&CONTENT, path.join("content"))?;
    extract(&REPORTS, path.join("reports"))?;
    extract(&SASS, path.join("sass"))?;
    extract(&STATIC, path.join("static"))?;
    extract(&TEMPLATES, path.join("templates"))?;
    extract(&WORKFLOWS, path.join(".github/workflows"))?;

    let mut config = ZOLA_CONFIG
        .parse::<DocumentMut>()
        .context("embedded zola.toml is invalid")?;
    config["base_url"] = value(base_url);
    config["title"] = value(title);
    config["author"] = value(author);
    config["languages"]["en"]["title"] = value(title);
    fs::write(path.join("zola.toml"), config.to_string())?;
    fs::write(path.join("mdout.toml"), MDOUT_MANIFEST)?;
    fs::write(path.join(".gitignore"), GITIGNORE)?;
    fs::write(path.join("README.md"), SCAFFOLD_README)?;

    println!(
        "Created mdout {} site at {}",
        env!("CARGO_PKG_VERSION"),
        path.display()
    );
    println!("Next: cd {} && mdout doctor && mdout serve", path.display());
    Ok(())
}

fn validate_input(path: &Path, title: &str, base_url: &str) -> Result<()> {
    ensure!(!title.trim().is_empty(), "--title must not be empty");
    ensure!(
        base_url.starts_with("https://") || base_url.starts_with("http://"),
        "--base-url must start with https:// or http://"
    );
    ensure!(base_url.ends_with('/'), "--base-url must end with /");

    if path.exists() {
        ensure!(path.is_dir(), "{} is not a directory", path.display());
        let mut entries =
            fs::read_dir(path).with_context(|| format!("could not read {}", path.display()))?;
        if entries.next().transpose()?.is_some() {
            bail!(
                "{} is not empty; choose a new or empty directory",
                path.display()
            );
        }
    }
    Ok(())
}

fn extract(dir: &Dir<'_>, destination: impl AsRef<Path>) -> Result<()> {
    let destination = destination.as_ref();
    fs::create_dir_all(destination)?;
    dir.extract(destination)
        .with_context(|| format!("could not write {}", destination.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zola::REQUIRED_ZOLA_VERSION;
    use tempfile::tempdir;

    #[test]
    fn creates_a_standalone_blog_without_rust_sources() {
        let root = tempdir().unwrap();
        let site = root.path().join("blog");
        run(&site, "Notes", "https://example.com/notes/", "Writer").unwrap();

        assert!(site.join("content/posts/_index.md").is_file());
        assert!(site.join("templates/base.html").is_file());
        assert!(site.join("static/js/search.js").is_file());
        assert!(site.join(".github/workflows/pages.yml").is_file());
        assert!(!site.join("Cargo.toml").exists());
        assert!(!site.join("src").exists());

        let config = fs::read_to_string(site.join("zola.toml")).unwrap();
        assert!(config.contains("base_url = \"https://example.com/notes/\""));
        assert!(config.contains("title = \"Notes\""));
        assert!(config.contains("author = \"Writer\""));
        assert!(config.contains("[languages.en]"));
        assert!(config.contains("title = \"Notes\""));
    }

    #[test]
    fn refuses_to_overwrite_a_nonempty_directory() {
        let root = tempdir().unwrap();
        fs::write(root.path().join("keep.txt"), "keep").unwrap();
        let error = run(root.path(), "Notes", "https://example.com/", "").unwrap_err();
        assert!(error.to_string().contains("not empty"));
        assert_eq!(
            fs::read_to_string(root.path().join("keep.txt")).unwrap(),
            "keep"
        );
    }

    #[test]
    fn validates_the_base_url_before_writing() {
        let root = tempdir().unwrap();
        let site = root.path().join("blog");
        let error = run(&site, "Notes", "example.com", "").unwrap_err();
        assert!(error.to_string().contains("must start"));
        assert!(!site.exists());
    }

    #[test]
    fn embedded_manifest_matches_the_binary() {
        assert!(MDOUT_MANIFEST.contains(&format!(
            "mdout_version = \"{}\"",
            env!("CARGO_PKG_VERSION")
        )));
        assert!(MDOUT_MANIFEST.contains(&format!("zola_version = \"{REQUIRED_ZOLA_VERSION}\"")));
    }
}
