/*
Module: rssify_cli::pipeline
Purpose: Pure, no-network pipeline utilities for Phase 2
Public API:
  - Types:
      FeedSeed { url, title_hint }
      FeedMetaDelta { title, site_url, etag, last_modified }
      PersistStats { feed, items_written, elapsed_ms, not_modified, failure_hint }
      FetchSummary { feeds_total, feeds_processed, items_parsed, items_written }
  - fns:
      load_feed_seeds(path) -> Result<Vec<FeedId>, PipelineError>
      fetch_from_file(path) -> Result<FetchSummary, PipelineError>
Notes:
  - No I/O writes; only reads the given JSON file.
  - Accepts flexible seed formats to reduce friction.
  - Keep structs minimal but aligned with tests.
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

/// A normalized seed for processing as expected by tests.
/// - url: original URL string (or identifier-like string)
/// - title_hint: optional human label
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FeedSeed {
    pub url: String,
    pub title_hint: Option<String>,
}

/// Minimal meta delta carrier produced by parsers.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct FeedMetaDelta {
    pub title: Option<String>,
    pub site_url: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

/// Minimal persist stats returned by the persist stage.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PersistStats {
    pub feed: FeedId,
    pub items_written: u32,
    pub elapsed_ms: u64,
    pub not_modified: bool,
    pub failure_hint: Option<String>,
}

/// Rollup summary suitable for --json CLI output.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct FetchSummary {
    pub feeds_total: u32,
    pub feeds_processed: u32,
    pub items_parsed: u32,
    pub items_written: u32,
}

/// Supported seed shapes (flexible):
/// 1) ["https://site/a.xml", "https://site/b.xml"]
/// 2) [{"url":"https://..."}, {"id":"custom"}]  // "id" accepted as alternate to "url"
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum Seed {
    Str(String),
    Obj { url: Option<String>, id: Option<String>, title_hint: Option<String> },
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
            // IMPORTANT: Prefer explicit id when both url and id are present.
            Seed::Obj { url: Some(u), id: Some(id), .. } => {
                out.push(FeedId::new(&id));
            }
            Seed::Obj { url: Some(u), id: None, .. } => {
                out.push(FeedId::from_url(&u));
            }
            Seed::Obj { url: None, id: Some(id), .. } => out.push(FeedId::new(&id)),
            Seed::Obj { url: None, id: None, .. } => { /* skip unusable */ }
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

    let feeds_processed = feeds_total;
    let items_parsed = feeds_total;

    Ok(FetchSummary {
        feeds_total,
        feeds_processed,
        items_parsed,
        items_written: feeds_total,
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

