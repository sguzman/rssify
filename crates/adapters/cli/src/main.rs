/*
Module: rssify_cli::main
Purpose: Minimal CLI skeleton for subcommand parsing; no business logic or I/O
Public API surface: binary entrypoint only
Invariants: Adapters must not contain business logic; they call into core traits
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

// ... header and attrs unchanged ...

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "rssify", version, about = "RSS toolkit CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Fetch {
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        store: Option<String>,
        #[arg(long)]
        json: bool,
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
    },
    Stats {
        #[arg(long)]
        store: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Import {
        #[arg(long)]
        file: Option<String>,
        #[arg(long)]
        out: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Add {
        url: String,
        #[arg(long)]
        out: Option<String>,
        #[arg(long)]
        json: bool,
    },
}

// Public helper so tests do not need `clap::Parser` import.
pub fn parse_from<I, S>(iter: I) -> Cli
where
    I: IntoIterator<Item = S>,
    S: Into<std::ffi::OsString> + Clone,
{
    Cli::parse_from(iter)
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(_cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // TODO(phase1): wire Command::Fetch to:
    // - parse feeds.json into Vec<FeedSeed>
    // - for each seed: fetch -> parse -> persist_fs
    // - aggregate PersistStats; if --json, print a JSON summary
    // TODO(phase1): wire Command::Stats to read fs:<root> and count entries
    // TODO(phase1): wire Command::Import and Command::Add to update feeds.json
    Ok(())
}

pub mod pipeline;
pub mod repo_fs;
pub mod spec;
