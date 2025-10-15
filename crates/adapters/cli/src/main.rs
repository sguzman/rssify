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

use clap::{Parser, Subcommand};
use rssify_core::{Feed, FeedRepo};
use rssify_repo_fs;

#[derive(Debug, Parser)]
#[command(name = "rssify", version, about = "RSS toolkit CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
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
}

/// Public helper so tests can parse without pulling in clap::Parser traits directly.
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
        Command::Fetch {
            from,
            store,
            json,
            verbose,
        } => {
            // Parse repo spec if provided and prepare optional FS repo
            let repo_spec = match store.as_deref() {
                Some(s) => Some(spec::RepoSpec::parse(s).map_err(|why| {
                    format!("invalid --store {:?}: {}", s, why)
                })?),
                None => None,
            };

            let seed_path = from.unwrap_or_else(|| "feeds.json".to_string());

            if verbose > 0 && !json {
                eprintln!(
                    "[rssify] op=fetch from={} store={} verbosity={}",
                    seed_path,
                    store.as_deref().unwrap_or("-"),
                    verbose
                );
            }

            // Load seeds as normalized IDs
            let ids = pipeline::load_feed_seeds(&seed_path).map_err(|e| format!("{}", e))?;

            // If fs: store is provided, persist minimal Feed records
            let mut feeds_written = 0u32;
            if let Some(spec) = &repo_spec {
                match spec.kind {
                    spec::RepoKind::Fs => {
                        // Open filesystem repo rooted at spec.target
                        let repo = rssify_repo_fs::FsRepo::open(&spec.target);
                        for id in &ids {
                            let feed = rssify_core::Feed {
                                id: id.clone(),
                                // In Phase 2 we do not resolve network URLs; use the id string as the url field.
                                url: id.as_str().to_string(),
                                title: None,
                                site_url: None,
                                etag: None,
                                last_modified: None,
                                active: true,
                            };
                            // Ignore individual write errors but count only successes
                            if rssify_core::FeedRepo::put(&repo, None, &feed).is_ok() {
                                feeds_written += 1;
                            }
                        }
                    }
                    _ => {
                        // Other backends not wired in Phase 2
                    }
                }
            }

            let summary = pipeline::FetchSummary {
                feeds_total: ids.len() as u32,
                feeds_processed: ids.len() as u32,
                items_parsed: 0,
                items_written: feeds_written,
            };

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
                println!("stats: not implemented yet (Phase 2)");
            }
        }
        Command::Import { json, .. } => {
            if json {
                println!("{}", serde_json::json!({"status": "not_implemented", "op": "import"}));
            } else {
                println!("import: not implemented yet (Phase 2)");
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

