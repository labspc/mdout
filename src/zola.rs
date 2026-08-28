use std::{env, fs, path::Path, process::Command};

use anyhow::{Context, Result, bail, ensure};

pub const REQUIRED_ZOLA_VERSION: &str = "0.23.4";

fn command() -> Command {
    if let Ok(binary) = env::var("MDOUT_ZOLA") {
        return Command::new(binary);
    }
    if Path::new(".tools/zola").is_file() {
        return Command::new(".tools/zola");
    }
    Command::new("zola")
}

fn run(mut command: Command) -> Result<()> {
    let status = command.status().with_context(missing_zola_message)?;
    if !status.success() {
        bail!("Zola exited with status {status}");
    }
    Ok(())
}

fn missing_zola_message() -> String {
    format!(
        "Zola was not found. Install Zola {REQUIRED_ZOLA_VERSION} or set MDOUT_ZOLA to its executable path."
    )
}

pub fn doctor() -> Result<()> {
    ensure!(
        Path::new("zola.toml").is_file(),
        "zola.toml was not found. Run mdout from the site root."
    );
    ensure!(
        Path::new("content").is_dir(),
        "content/ was not found. Run mdout from the site root."
    );
    validate_manifest()?;

    let output = command()
        .arg("--version")
        .output()
        .with_context(missing_zola_message)?;
    ensure!(output.status.success(), "Zola could not report its version");
    let version_output = String::from_utf8_lossy(&output.stdout);
    let version = parse_zola_version(&version_output)
        .context("Could not parse the installed Zola version")?;
    ensure!(
        version == REQUIRED_ZOLA_VERSION,
        "Zola {REQUIRED_ZOLA_VERSION} is required, but {version} is installed"
    );

    fs::read_dir("content").context("content/ is not readable")?;
    println!("mdout {}", env!("CARGO_PKG_VERSION"));
    println!("Zola {version}");
    println!("zola.toml ok");
    println!("content/ ok");
    Ok(())
}

fn validate_manifest() -> Result<()> {
    let source = fs::read_to_string("mdout.toml")
        .context("mdout.toml was not found. Run mdout from a v0.2+ site root")?;
    let document = source
        .parse::<toml_edit::DocumentMut>()
        .context("mdout.toml is not valid TOML")?;
    let format = document["format_version"]
        .as_integer()
        .context("mdout.toml must define format_version")?;
    ensure!(
        format == 1,
        "unsupported mdout.toml format_version {format}"
    );
    let mdout = document["mdout_version"]
        .as_str()
        .context("mdout.toml must define mdout_version")?;
    ensure!(
        mdout == env!("CARGO_PKG_VERSION"),
        "this site requires mdout {mdout}, but the installed CLI is {}",
        env!("CARGO_PKG_VERSION")
    );
    let zola = document["zola_version"]
        .as_str()
        .context("mdout.toml must define zola_version")?;
    ensure!(
        zola == REQUIRED_ZOLA_VERSION,
        "mdout.toml requires Zola {zola}, but this CLI requires {REQUIRED_ZOLA_VERSION}"
    );
    Ok(())
}

fn parse_zola_version(output: &str) -> Option<&str> {
    output
        .split_whitespace()
        .find(|part| part.chars().next().is_some_and(|c| c.is_ascii_digit()))
}

pub fn build(base_url: Option<&str>, drafts: bool, output_dir: Option<&Path>) -> Result<()> {
    let mut check = command();
    check.arg("check");
    run(check)?;

    let mut build = command();
    build.arg("build");
    if let Some(base_url) = base_url {
        build.args(["--base-url", base_url]);
    }
    if drafts {
        build.arg("--drafts");
    }
    if let Some(output_dir) = output_dir {
        build.arg("--output-dir").arg(output_dir);
    }
    run(build)
}

pub fn serve(interface: &str, port: u16, base_url: Option<&str>) -> Result<()> {
    let mut serve = command();
    serve
        .arg("serve")
        .arg("--drafts")
        .args(["--interface", interface])
        .args(["--port", &port.to_string()]);
    if let Some(base_url) = base_url {
        serve.args(["--base-url", base_url]);
    }
    run(serve)
}

#[cfg(test)]
mod tests {
    use super::{REQUIRED_ZOLA_VERSION, parse_zola_version};

    #[test]
    fn parses_zola_version_output() {
        let output = format!("zola {REQUIRED_ZOLA_VERSION}\n");
        assert_eq!(parse_zola_version(&output), Some(REQUIRED_ZOLA_VERSION));
    }
}
