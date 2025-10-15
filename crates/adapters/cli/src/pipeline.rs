/*
Module: rssify_cli::pipeline
Purpose: Pure, no-network pipeline utilities for Phase 2
Public API:
  - Types: FeedSeed, FeedMetaDelta, PersistStats, FetchSummary
  - fns: load_feed_seeds(path), fetch_from_file(path)
Notes:
  - No I/O writes; only reads the given JSON file.
  - Types here are intentionally minimal placeholders to satisfy CLI/tests.
  - Accepts flexible seed formats to reduce friction.
*/

use rssify_core::FeedId;
use std::fs;
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum PipelineError {
    #[error("input file not found: {0}")]
    NotFound(String),
    #[error("failed to read file {path}: {source}")]
    Io { path: String, source: std::io::Error },
    #[error("invalid JSON in {path}: {source}")]
    Json { path: String, source: serde_json::Error },
    #[error("no seeds found in {0}")]
    Empty(String),
}

/// A normalized seed for processing; wraps a canonical FeedId.
/// Intentionally tiny for now; more fields can be added later if needed.
#[derive(Debug, Clone)]
pub struct FeedSeed {
    pub id: FeedId,
}

impl From<FeedId> for FeedSeed {
    fn from(id: FeedId) -> Self { Self { id } }
}

/// Minimal meta delta carrier produced by parsers.
/// For Phase 2 it only tracks whether anything changed.
#[derive(Debug, Clone, Default)]
pub struct FeedMetaDelta {
    pub changed: bool,
}

/// Minimal persist stats returned by the persist stage.
/// For Phase 2 we expose a single items_written counter.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct PersistStats {
    pub items_written: u32,
}

/// Rollup summary suitable for `--json` CLI output.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct FetchSummary {
    pub feeds_total: u32,
    pub feeds_processed: u32,
    pub items_parsed: u32,
    pub items_written: u32,
}

/// Supported seed shapes (flexible):
/// 1) ["https://site/a.xml", "https://site/b.xml"]
/// 2) [{"url":"https://..."}, {"id":"custom"}]
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum Seed {
    Str(String),
    Obj { url: Option<String>, id: Option<String> },
}

/// Load feed seeds from a JSON file and normalize to FeedId values.
/// Returns Err(Empty) if the file contains zero usable seeds.
pub fn load_feed_seeds<P: AsRef<Path>>(path: P) -> Result<Vec<FeedId>, PipelineError> {
    let p = path.as_ref();
    if !p.exists() {
        return Err(PipelineError::NotFound(p.display().to_string()));
    }
    let raw = fs::read_to_string(p).map_err(|e| PipelineError::Io {
        path: p.display().to_string(),
        source: e,
    })?;
    let seeds: Vec<Seed> =
        serde_json::from_str(&raw).map_err(|e| PipelineError::Json {
            path: p.display().to_string(),
            source: e,
        })?;

    let mut out = Vec::with_capacity(seeds.len());
    for s in seeds {
        match s {
            Seed::Str(s) => out.push(normalize_id(&s)),
            Seed::Obj { url: Some(u), id } => {
                // Prefer explicit id if present; otherwise use URL-derived ID.
                out.push(match id {
                    Some(id) => FeedId::new(&id),
                    None => FeedId::from_url(&u),
                });
            }
            Seed::Obj { url: None, id: Some(id) } => out.push(FeedId::new(&id)),
            Seed::Obj { url: None, id: None } => { /* skip unusable */ }
        }
    }

    if out.is_empty() {
        return Err(PipelineError::Empty(p.display().to_string()));
    }
    Ok(out)
}

/// Run a stubbed fetch-parse-persist loop using seeds from a JSON file.
/// This does not perform network or disk writes. It simulates:
/// - 1 item parsed per feed
/// - 1 item written per feed
pub fn fetch_from_file<P: AsRef<Path>>(path: P) -> Result<FetchSummary, PipelineError> {
    let seeds = load_feed_seeds(path)?;
    let feeds_total = seeds.len() as u32;

    // Stubbed loop: pretend each feed yields 1 parsed and 1 written entry.
    let feeds_processed = feeds_total;
    let items_parsed = feeds_total;

    // Phase-2 placeholder persist stats.
    let persist = PersistStats { items_written: feeds_total };

    Ok(FetchSummary {
        feeds_total,
        feeds_processed,
        items_parsed,
        items_written: persist.items_written,
    })
}

/// Best-effort ID normalization:
/// - If it looks like a URL (starts with "http"), use URL-based constructor.
/// - Otherwise, treat as a literal ID.
fn normalize_id(s: &str) -> FeedId {
    if s.starts_with("http://") || s.starts_with("https://") {
        FeedId::from_url(s)
    } else {
        FeedId::new(s)
    }
}

