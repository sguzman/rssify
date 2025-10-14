/*
Module: rssify_cli::main
Purpose: Minimal CLI skeleton for subcommand parsing; no business logic or I/O
Public API surface: binary entrypoint only
Invariants: Adapters must not contain business logic; they call into core traits
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use clap::{Parser, Subcommand};

/// rssify - RSS toolkit (CLI adapter)
#[derive(Debug, Parser)]
#[command(name = "rssify", version, about = "RSS toolkit CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Fetch feeds and persist results (scope defined in core/contracts)
    Fetch {
        /// Path to feeds.json (source of truth for Phase 1)
        #[arg(long)]
        from: Option<String>,
        /// Destination repo spec, e.g. "fs:/path"
        #[arg(long)]
        store: Option<String>,
        /// Emit JSON to stdout
        #[arg(long)]
        json: bool,
        /// Increase verbosity
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
    },

    /// Compute and print simple statistics (placeholder)
    Stats {
        /// Repo spec to read from
        #[arg(long)]
        store: Option<String>,
        /// Emit JSON to stdout
        #[arg(long)]
        json: bool,
    },

    /// Import URLs from a list into feeds.json (placeholder)
    Import {
        /// Path to a newline-delimited URLs file
        #[arg(long)]
        file: Option<String>,
        /// Output feeds.json path
        #[arg(long)]
        out: Option<String>,
        /// Emit JSON to stdout
        #[arg(long)]
        json: bool,
    },

    /// Add a single URL to feeds.json (placeholder)
    Add {
        /// Feed URL to add
        url: String,
        /// Output feeds.json path
        #[arg(long)]
        out: Option<String>,
        /// Emit JSON to stdout
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(_cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // Step 2: no business logic yet.
    // This function will delegate to core traits in later steps.
    Ok(())
}
