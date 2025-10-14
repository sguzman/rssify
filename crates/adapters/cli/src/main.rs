// Contract:
// Purpose: Provide the rssify CLI with subcommands that call core trait seams.
// Inputs/Outputs: Parse CLI args; delegate to implementations (stubbed for now).
// Invariants: CLI returns non-zero exit on error; logs are concise.
// Examples: `rssify fetch --feeds feeds.json --out data/`
// Task: Keep under 300 LOC; split subcommands if they grow.
// Tests: CLI parsing tests included; behavior tests to follow when impls land.

use clap::{Parser, Subcommand};
use rssify_core::*;

#[derive(Parser)]
#[command(name = "rssify", version, about = "RSS toolkit CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read feeds.json, fetch and extract items, persist artifacts and records.
    Fetch {
        #[arg(long, default_value = "feeds.json")]
        feeds: String,
        #[arg(long, default_value = "data")]
        out: String,
        #[arg(long)]
        verbose: bool,
    },
    /// Import newline-delimited URLs and append to the fat JSON without network I/O.
    Import {
        #[arg(long)]
        input: String,
        #[arg(long, default_value = "data")]
        out: String,
    },
    /// Add a single URL (optionally fetch immediately).
    Add {
        url: String,
        #[arg(long)]
        fetch: bool,
        #[arg(long, default_value = "data")]
        out: String,
    },
    /// Compute and print basic stats.
    Stats {
        #[arg(long, default_value = "data")]
        data: String,
    },
    /// Serve an HTTP API (and optional in-process scheduler).
    Serve {
        #[arg(long, default_value = "127.0.0.1:8080")]
        addr: String,
        #[arg(long)]
        scheduler: bool,
        #[arg(long, default_value = "feeds.json")]
        feeds: String,
    },
    /// Export derived stats.
    Export {
        #[arg(long, default_value = "data")]
        data: String,
        #[arg(long, default_value = "out.json")]
        out: String,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!(
            "error: {}",
            match e {
                Error::Invalid(s) | Error::NotFound(s) | Error::Io(s) | Error::Other(s) => s,
            }
        );
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Error> {
    match cli.cmd {
        Commands::Fetch {
            feeds,
            out,
            verbose,
        } => {
            if verbose {
                eprintln!("fetching feeds from {} -> {}", feeds, out);
            }
            // TODO: wire Fetcher + Parser + Repository impls
            todo!("implement fetch pipeline");
        }
        Commands::Import { input, out } => {
            let _ = (input, out);
            todo!("implement import pipeline");
        }
        Commands::Add { url, fetch, out } => {
            let _ = (url, fetch, out);
            todo!("implement add");
        }
        Commands::Stats { data } => {
            let _ = data;
            todo!("implement stats");
        }
        Commands::Serve {
            addr,
            scheduler,
            feeds,
        } => {
            let _ = (addr, scheduler, feeds);
            todo!("implement serve");
        }
        Commands::Export { data, out } => {
            let _ = (data, out);
            todo!("implement export");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_parses() {
        Cli::command().debug_assert();
        // smoke parse
        let _ = Cli::parse_from(["rssify", "stats", "--data", "data"]);
    }
}
