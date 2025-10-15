//// File: crates/adapters/cli/src/main.rs
//// Role: CLI entrypoint; uses pipeline, repo_fs, stats, and spec (pure parsing).

// Contract:
// Purpose: Parse CLI args and dispatch to simple adapter functions; no business logic.
// Inputs/Outputs: Reads flags/subcommands via clap; prints user-facing output (text or JSON).
// Invariants: Repo selection is parsed via spec::RepoSpec; only fs backend is supported in this phase.
// Examples: `rssify fetch --from feeds.json --store fs:./data --json`
// Task: Keep under 300 LOC; split if orchestration grows. No tests in this file (tests live under test/).

use clap::{Parser, Subcommand};
use serde_json::json;
use std::str::FromStr;

pub mod pipeline;
pub mod repo_fs;
pub mod stats;
pub mod spec;

#[derive(Debug, Parser)]
#[command(name = "rssify", version, about = "RSS toolkit CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Fetch feeds from a seed source (Phase 2: file-only, no network).
    Fetch {
        /// Seed file to read (JSON). Defaults to "feeds.json" if omitted.
        #[arg(long)]
        from: Option<String>,
        /// Repository target (e.g., fs:<root>).
        #[arg(long)]
        store: Option<String>,
        /// Emit machine-readable JSON.
        #[arg(long)]
        json: bool,
        /// Increase verbosity (-v, -vv).
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
    },
    /// Show repository stats (Phase 2: filesystem only).
    Stats {
        /// Repository target (defaults to fs:. if omitted).
        #[arg(long)]
        store: Option<String>,
        /// Emit machine-readable JSON.
        #[arg(long)]
        json: bool,
    },
    /// Stubs kept for later phases.
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

/// Public helper used by tests to exercise clap parsing without exec.
pub fn parse_from<I, S>(iter: I) -> Cli
where
    I: IntoIterator<Item = S>,
    S: Into<std::ffi::OsString> + Clone,
{
    Cli::parse_from(iter)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Fetch { from, store, json, verbose } => {
            let seed_path = from.unwrap_or_else(|| "feeds.json".to_string());
            let ids = pipeline::load_feed_seeds(&seed_path)
                .map_err(|e| format!("failed to parse seeds: {}", e))?;

            // Open repo when provided and supported; use RepoSpec parsing.
            let mut written = 0usize;
            if let Some(spec_str) = store {
                let spec = spec::RepoSpec::from_str(&spec_str)
                    .map_err(|e| format!("invalid --store: {} ({})", spec_str, e))?;

                match spec.kind {
                    spec::RepoKind::Fs => {
                        // For fs, target is the repo root path.
                        let repo = rssify_repo_fs::FsRepo::open(&spec.target);
                        // Use rssify_core types and traits
                        use rssify_core::{Feed, FeedId, FeedRepo};
                        for id in &ids {
                            let fid = FeedId::new(id.clone());
                            let feed = Feed {
                                id: fid,
                                url: id.clone(),
                                title: None,
                                site_url: None,
                                etag: None,
                                last_modified: None,
                                active: true,
                            };
                            if FeedRepo::put(&repo, None, &feed).is_ok() {
                                written += 1;
                            }
                        }
                    }
                    // Future adapters (e.g., sqlite) are parsed but not supported yet.
                    other => {
                        return Err(format!("repo kind '{}' is not supported in this phase", other.as_str()).into());
                    }
                }
            }

            let summary = json!({
                "feeds_total": ids.len(),
                "feeds_processed": ids.len(),
                "items_parsed": 0,
                "items_written": written
            });

            if json {
                println!("{}", serde_json::to_string_pretty(&summary)?);
            } else {
                println!(
                    "Processed {}/{} feeds; items parsed=0, written={}",
                    ids.len(),
                    ids.len(),
                    written
                );
                if verbose > 1 {
                    eprintln!("[rssify] component=adapter.cli op=fetch status=ok items={} ", written);
                }
            }
        }
        Command::Stats { store, json } => {
            // Default to filesystem repo at current directory when not provided.
            let spec_str = store.unwrap_or_else(|| "fs:.".to_string());
            let spec = spec::RepoSpec::from_str(&spec_str)
                .map_err(|e| format!("invalid --store: {} ({})", spec_str, e))?;

            match spec.kind {
                spec::RepoKind::Fs => {
                    let s = stats::stats_fs(&spec.target)?;
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&json!({
                                "feeds": s.feeds,
                                "entries": s.entries
                            }))?
                        );
                    } else {
                        println!("feeds={} entries={}", s.feeds, s.entries);
                    }
                }
                other => {
                    return Err(format!("repo kind '{}' is not supported in this phase", other.as_str()).into());
                }
            }
        }
        Command::Import { json, .. } => {
            if json {
                println!("{}", json!({"status": "not_implemented", "op": "import"}));
            } else {
                println!("import: not implemented yet (later phase)");
            }
        }
        Command::Add { json, .. } => {
            if json {
                println!("{}", json!({"status": "not_implemented", "op": "add"}));
            } else {
                println!("add: not implemented yet (later phase)");
            }
        }
    }

    Ok(())
}

