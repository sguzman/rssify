//// File: crates/adapters/cli/src/main.rs
//// Purpose: CLI entrypoint - parse args and dispatch to pipeline/core with simple output.
//// Inputs: OS args via clap; optional seed file (--from) and store spec (--store).
//// Outputs: Human summary to stdout by default or JSON when --json; non-zero exit on error.
//// Side effects: Reads files when executing pipeline helpers; prints to stdout/stderr.
//// Invariants:
////  - No business logic: call pipeline/core only.
////  - JSON mode must be machine-friendly and stable.
////  - Keep file under 400 LOC; split subcommands later if needed.
//// Tests: crates/adapters/cli/tests/*.rs cover parsing and, later, integration.

use std::process::ExitCode;

use clap::Parser;
use rssify_cli::repo_selector::{RepoSpec, parse_repo_spec};
use rssify_cli::repo_sqlite::SqliteRepo;
use rssify_core::RepoError;

#[derive(Parser, Debug)]
#[command(name = "rssify", version, disable_help_subcommand = true)]
struct Cli {
    // requires Cargo.toml clap feature: "env"
    #[arg(long, env = "RSSIFY_REPO", default_value = "fs:./data")]
    repo: String,

    #[command(subcommand)]
    cmd: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Fetch feeds from a seed source (Phase 2: file-only pipeline)
    Fetch {
        /// Seed file to read (JSON). Defaults to "feeds.json" if omitted.
        #[arg(long)]
        from: Option<String>,
        /// Repository target (e.g., fs:/var/lib/rssify, sqlite:/path/db.sqlite)
        #[arg(long)]
        store: Option<String>,
        /// Emit machine-readable JSON (quiet otherwise).
        #[arg(long)]
        json: bool,
        /// Increase verbosity (-v, -vv).
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
    },
    /// Show repository stats (stub in Phase 2)
    Stats {
        #[arg(long)]
        store: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// Import URLs and write a canonical feeds.json (stub in Phase 2)
    Import {
        #[arg(long)]
        file: Option<String>,
        #[arg(long)]
        out: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// Add a single feed URL to feeds.json (stub in Phase 2)
    Add {
        url: String,
        #[arg(long)]
        out: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Stats {},
    Import {},
    Add {},
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

/// Public helper so tests can parse without pulling in clap::Parser traits directly.
pub fn parse_from<I, S>(iter: I) -> Cli
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    Cli::try_parse_from(iter)
}

fn run(cli: Cli) -> Result<(), RepoError> {
    let spec = parse_repo_spec(&cli.repo)?;
    match (spec, &cli.cmd) {
        (RepoSpec::Sqlite(dsn), Command::Fetch { .. }) => {
            let repo = SqliteRepo::open(dsn)?;
            fetch_impl(&repo)
        }
        (RepoSpec::Fs(_path), Command::Fetch { .. }) => {
            // TODO: wire FS backend; returning clear placeholder for now
            Err(RepoError::Backend("fs repo not wired yet".into()))
        }
        // Stubs for other subcommands to keep compile green
        (_spec, Command::Stats { .. }) => Ok(()),
        (_spec, Command::Import { .. }) => Ok(()),
        (_spec, Command::Add { .. }) => Ok(()),
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Command::Fetch {
            from,
            store,
            json,
            verbose,
        } => {
            // Validate repository seam early (even if unused in Phase 2).
            if let Some(s) = store.as_deref() {
                let _spec = spec::RepoSpec::parse(s).map_err(|why| {
                    format!("invalid --store {:?}: {}", s, why)
                })?;
            }

            let seed_path = from.unwrap_or_else(|| "feeds.json".to_string());

            if verbose > 0 && !json {
                eprintln!(
                    "[rssify] op=fetch from={} store={} verbosity={}",
                    seed_path,
                    store.as_deref().unwrap_or("-"),
                    verbose
                );
            }

            let summary = pipeline::fetch_from_file(&seed_path).map_err(|e| {
                // Map structured error to readable message; preserve path context.
                format!("{}", e)
            })?;

            if json {
                // Stable JSON schema mirrors FetchSummary.
                let out = serde_json::to_string_pretty(&summary)?;
                println!("{}", out);
            } else {
                println!(
                    "Processed {}/{} feeds; items parsed={}, written={}",
                    summary.feeds_processed,
                    summary.feeds_total,
                    summary.items_parsed,
                    summary.items_written
                );
                if verbose > 1 {
                    eprintln!("[rssify] done op=fetch status=ok");
                }
            }
        }
        Command::Stats { json, .. } => {
            if json {
                println!("{}", serde_json::json!({"status": "not_implemented", "op": "stats"}));
            } else {
                println!("stats: not implemented yet (Phase 3)");
            }
        }
        Command::Import { json, .. } => {
            if json {
                println!("{}", serde_json::json!({"status": "not_implemented", "op": "import"}));
            } else {
                println!("import: not implemented yet (Phase 3)");
            }
        }
        Command::Add { json, .. } => {
            if json {
                println!("{}", serde_json::json!({"status": "not_implemented", "op": "add"}));
            } else {
                println!("add: not implemented yet (Phase 3)");
            }
        }
    }
    Ok(())
}

pub mod pipeline;
pub mod repo_fs;
pub mod spec;

