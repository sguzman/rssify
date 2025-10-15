//// File: crates/adapters/cli/src/main.rs
//// Role: CLI entrypoint; uses pipeline and repo_fs modules exposed for tests.

use clap::{Parser, Subcommand};
use serde_json::json;

pub mod pipeline;
pub mod repo_fs;
pub mod stats;

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
        /// Repository target (fs:<root>).
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

            let mut written = 0usize;
            if let Some(store) = store {
                if let Some(root) = store.strip_prefix("fs:") {
                    let repo = rssify_repo_fs::FsRepo::open(root);
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
                } else {
                    return Err(format!("unsupported --store: {}", store).into());
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
                    eprintln!("[rssify] done op=fetch status=ok");
                }
            }
        }
        Command::Stats { store, json } => {
            // Default to current directory repo when not provided.
            let store = store.unwrap_or_else(|| "fs:.".to_string());
            let root = store
                .strip_prefix("fs:")
                .ok_or("stats only supports fs:<root> in Phase 2")?;
            let s = stats::stats_fs(root)?;
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

