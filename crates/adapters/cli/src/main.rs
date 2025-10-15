//// File: crates/adapters/cli/src/main.rs
//// Role: CLI entrypoint; uses pipeline, repo_fs, stats, spec, store, and a minimal logger.
// Contract:
// Purpose: Parse CLI args and dispatch to simple adapter functions; no business logic.
// Inputs/Outputs: Reads flags/subcommands via clap; prints user-facing output (text or JSON) to stdout.
// Invariants: Repo selection is parsed via spec::RepoSpec; logs go to stderr.
// Examples: `rssify fetch --from feeds.json --store fs:./data --json`
// Task: Keep under 300 LOC; split if orchestration grows. No tests in this file (tests live under tests/).

use clap::{Parser, Subcommand};
use serde_json::json;
use std::str::FromStr;

pub mod pipeline;
pub mod repo_fs;
pub mod stats;
pub mod spec;
pub mod log;
pub mod store;

use log::{LogLevel, Logger};
use store::{resolve_store_spec, ENV_REPO};

#[derive(Debug, Parser)]
#[command(name = "rssify", version, about = "RSS toolkit CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Fetch feeds from a seed source (Phase 2/3: file-only, no network).
    Fetch {
        /// Seed file to read (JSON). Defaults to "feeds.json" if omitted.
        #[arg(long)]
        from: Option<String>,
        /// Repository target (e.g., fs:<root>).
        ///
        /// Precedence: --store > env RSSIFY_REPO > fs:.
        #[arg(long)]
        store: Option<String>,
        /// Emit machine-readable JSON.
        #[arg(long)]
        json: bool,
        /// Increase verbosity (-v, -vv).
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
    },
    /// Show repository stats (filesystem only in this phase).
    Stats {
        /// Repository target (e.g., fs:<root>).
        ///
        /// Precedence: --store > env RSSIFY_REPO > fs:.
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
            let log = Logger::new(LogLevel::from_verbosity(verbose));
            let seed_path = from.unwrap_or_else(|| "feeds.json".to_string());

            // Resolve store spec with precedence: flag > env > default.
            let resolved = resolve_store_spec(store.clone());
            log.info("fetch_start", &[
                ("from", seed_path.as_str()),
                ("store", resolved.as_str()),
                ("env", ENV_REPO),
            ]);

            let ids = match pipeline::load_feed_seeds(&seed_path) {
                Ok(v) => v,
                Err(e) => {
                    Logger::new(LogLevel::Error).error("fetch_parse_error", &[("error", format!("{}", e))]);
                    return Err(format!("failed to parse seeds: {}", e).into());
                }
            };

            // Open repo via RepoSpec.
            let spec = spec::RepoSpec::from_str(&resolved)
                .map_err(|e| format!("invalid --store: {} ({})", resolved, e))?;

            let mut written = 0usize;
            match spec.kind {
                spec::RepoKind::Fs => {
                    let repo = rssify_repo_fs::FsRepo::open(&spec.target);
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
                    log.debug("fetch_persist_done", &[("written", written), ("feeds", ids.len())]);
                }
                other => {
                    Logger::new(LogLevel::Error).error("fetch_repo_unsupported", &[("kind", other.as_str())]);
                    return Err(format!("repo kind '{}' is not supported in this phase", other.as_str()).into());
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
            }

            log.info("fetch_done", &[("feeds", ids.len()), ("written", written)]);
        }
        Command::Stats { store, json } => {
            let log = Logger::new(LogLevel::Warn);

            // Resolve store spec with precedence: flag > env > default.
            let resolved = resolve_store_spec(store);
            let spec = spec::RepoSpec::from_str(&resolved)
                .map_err(|e| format!("invalid --store: {} ({})", resolved, e))?;

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
                    log.info("stats_done", &[("feeds", s.feeds), ("entries", s.entries)]);
                }
                other => {
                    Logger::new(LogLevel::Error).error("stats_repo_unsupported", &[("kind", other.as_str())]);
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
            Logger::new(LogLevel::Warn).warn("import_stub", &[("status", "not_implemented")]);
        }
        Command::Add { json, .. } => {
            if json {
                println!("{}", json!({"status": "not_implemented", "op": "add"}));
            } else {
                println!("add: not implemented yet (later phase)");
            }
            Logger::new(LogLevel::Warn).warn("add_stub", &[("status", "not_implemented")]);
        }
    }

    Ok(())
}
