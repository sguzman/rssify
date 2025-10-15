//// File: crates/adapters/cli/src/main.rs
//// Purpose: CLI entrypoint with Phase 2 wiring for fetch and stats (filesystem-only).
use clap::{Parser, Subcommand};
use serde_json::json;

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
        #[arg(long)]
        store: String,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Fetch {
            from,
            store,
            json,
            verbose,
        } => {
            let seed_path = from.unwrap_or_else(|| "feeds.json".to_string());
            let ids = load_feed_seeds(&seed_path)
                .map_err(|e| format!("failed to parse seeds: {}", e))?;

            let mut written = 0usize;
            if let Some(store) = store {
                // Expect fs:<root>
                if let Some(root) = store.strip_prefix("fs:") {
                    let repo = rssify_repo_fs::FsRepo::open(root);
                    // For Phase 2, persist minimal feed records; no network, no entries.
                    for id in &ids {
                        let feed_obj = json!({
                            "id": id,
                            "url": id,
                            "title": null,
                            "site_url": null,
                            "etag": null,
                            "last_modified": null,
                            "active": true
                        });
                        if rssify_repo_fs::FsRepo::put_feed_json(&repo, id, &feed_obj).is_ok() {
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

// -------------------------
// Seed loading (Phase 2)
// -------------------------
use std::fs::File;
use std::io::Read as _;

fn load_feed_seeds(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let val: serde_json::Value = serde_json::from_str(&buf)?;
    let mut out = Vec::new();

    match val {
        serde_json::Value::Array(arr) => {
            for item in arr {
                match item {
                    serde_json::Value::String(s) => out.push(s),
                    serde_json::Value::Object(mut m) => {
                        // Prefer explicit id when present; else fall back to url.
                        if let Some(idv) = m.remove("id") {
                            if let Some(id) = idv.as_str() {
                                out.push(id.to_string());
                                continue;
                            }
                        }
                        if let Some(urlv) = m.remove("url") {
                            if let Some(url) = urlv.as_str() {
                                out.push(url.to_string());
                                continue;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => return Err("expected a JSON array at top-level".into()),
    }

    Ok(out)
}

// Re-export for tests or other crates if needed.
pub mod reexports {
    pub use super::load_feed_seeds;
}

