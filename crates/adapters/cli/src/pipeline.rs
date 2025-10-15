//// File: crates/adapters/cli/src/pipeline.rs
//// Role: Phase 2 pipeline helpers and error types used by tests.

use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// Core types referenced by tests
use rssify_core::FeedId;

/// Summary returned by fetch routines (Phase 2: no network).
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct FetchSummary {
    pub feeds_total: u32,
    pub feeds_processed: u32,
    pub items_parsed: u32,
    pub items_written: u32,
}

/// Test-facing: seed item (string or object in JSON fixtures).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FeedSeed {
    pub url: String,
    pub title_hint: Option<String>,
}

/// Test-facing: metadata changes detected for a feed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FeedMetaDelta {
    pub title: Option<String>,
    pub site_url: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

/// Test-facing: persistence counters for a single feed operation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PersistStats {
    pub feed: FeedId,
    pub items_written: u32,
    pub elapsed_ms: u64,
    pub not_modified: bool,
    pub failure_hint: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum PipelineError {
    #[error("input file not found: {0}")]
    NotFound(String),
    #[error("failed to read file {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid JSON in {path}: {source}")]
    InvalidJson {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("no seeds found in {0}")]
    NoSeeds(String),
}

/// Load seeds from a JSON file.
/// Accepts an array of strings or objects with "id" or "url".
pub fn load_feed_seeds<P: AsRef<Path>>(path: P) -> Result<Vec<String>, PipelineError> {
    let path_ref = path.as_ref();
    let mut f = File::open(path_ref).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            PipelineError::NotFound(path_ref.display().to_string())
        } else {
            PipelineError::Read {
                path: path_ref.display().to_string(),
                source: e,
            }
        }
    })?;

    let mut buf = String::new();
    f.read_to_string(&mut buf).map_err(|e| PipelineError::Read {
        path: path_ref.display().to_string(),
        source: e,
    })?;

    let val: Json = serde_json::from_str(&buf).map_err(|e| PipelineError::InvalidJson {
        path: path_ref.display().to_string(),
        source: e,
    })?;

    let mut out = Vec::new();
    match val {
        Json::Array(arr) => {
            for item in arr {
                match item {
                    Json::String(s) => out.push(s),
                    Json::Object(mut m) => {
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
        _ => {}
    }

    if out.is_empty() {
        return Err(PipelineError::NoSeeds(path_ref.display().to_string()));
    }
    Ok(out)
}

/// Phase-2 stub: "fetch from file" parses seeds and returns a summary.
/// Tests expect items_written to equal the number of seeds.
pub fn fetch_from_file<P: AsRef<Path>>(path: P) -> Result<FetchSummary, PipelineError> {
    let seeds = load_feed_seeds(path)?;
    let count = seeds.len() as u32;
    Ok(FetchSummary {
        feeds_total: count,
        feeds_processed: count,
        items_parsed: count,
        items_written: count,
    })
}

