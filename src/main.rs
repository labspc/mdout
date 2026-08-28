mod check;
mod links;
mod zola;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "mdout",
    version,
    about = "Markdown in, HTML out. Built on Zola."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Diagnose the local mdout and Zola environment.
    Doctor,
    /// Validate Markdown content without building the site.
    Check {
        #[arg(short, long, default_value = "content")]
        directory: PathBuf,
    },
    /// Validate content and build the static site with Zola.
    Build {
        #[arg(long)]
        base_url: Option<String>,
        #[arg(long)]
        drafts: bool,
        #[arg(short, long)]
        output_dir: Option<PathBuf>,
    },
    /// Preview the site locally, including drafts.
    Serve {
        #[arg(long, default_value = "127.0.0.1")]
        interface: String,
        #[arg(short, long, default_value_t = 1111)]
        port: u16,
        #[arg(long)]
        base_url: Option<String>,
    },
    /// Check external links and update reports/links.json.
    Links {
        #[arg(short, long, default_value = "content")]
        directory: PathBuf,
        #[arg(long)]
        force: bool,
        #[arg(long)]
        strict: bool,
        #[arg(short, long, default_value_t = 4)]
        concurrency: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Doctor => zola::doctor(),
        Command::Check { directory } => check::run(&directory),
        Command::Build {
            base_url,
            drafts,
            output_dir,
        } => {
            check::run(&PathBuf::from("content"))?;
            zola::build(base_url.as_deref(), drafts, output_dir.as_deref())
        }
        Command::Serve {
            interface,
            port,
            base_url,
        } => {
            check::run(&PathBuf::from("content"))?;
            zola::serve(&interface, port, base_url.as_deref())
        }
        Command::Links {
            directory,
            force,
            strict,
            concurrency,
        } => links::run(&directory, force, strict, concurrency).await,
    }
}
