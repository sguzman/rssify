//// File: crates/adapters/cli/src/pipeline.rs
//// Purpose: Seed parsing and Phase-2/3 test-facing pipeline skeleton.
//// Public API (stable for tests):
////   - Types: FetchSummary, FeedSeed, FeedMetaDelta, PersistStats
////   - Functions: load_feed_seeds(path), fetch_from_file(path)
//// Accepted seed JSON formats for load_feed_seeds:
////   1) ["https://a", "guid:FEED 01"]
////   2) [{"id":"X","url":"..."}, {"url":"..."}]  (prefers id, else url, else guid)
////   3) {"seeds": [ ... either 1 or 2 ... ]}
//// Notes:
////   - No network or repo writes here; this is a pure adapter helper used by tests.
////   - Keep this file <= 300 LOC; split when adding real fetching in later phases.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io;
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

/// Errors from the seed/pipeline helpers (kept minimal and explicit).
#[derive(Debug)]
pub enum PipelineError {
    Io(io::Error),
    Json(String),
    Structure(String),
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineError::Io(e) => write!(f, "io error: {}", e),
            PipelineError::Json(e) => write!(f, "json parse error: {}", e),
            PipelineError::Structure(e) => write!(f, "invalid seeds structure: {}", e),
        }
    }
}
impl std::error::Error for PipelineError {}
impl From<io::Error> for PipelineError {
    fn from(e: io::Error) -> Self {
        PipelineError::Io(e)
    }
}

/// Load feed seeds from a JSON file. See header for accepted formats.
pub fn load_feed_seeds<P: AsRef<Path>>(path: P) -> Result<Vec<String>, PipelineError> {
    let data = fs::read_to_string(&path)?;
    let v: Value =
        serde_json::from_str(&data).map_err(|e| PipelineError::Json(e.to_string()))?;
    extract_seeds_from_value(v)
}

/// Phase-2 stub: parse seeds and return a count-only summary.
/// Tests expect items_* fields to equal the number of seeds.
pub fn fetch_from_file<P: AsRef<Path>>(path: P) -> Result<FetchSummary, PipelineError> {
    let seeds = load_feed_seeds(path)?;
    let n = seeds.len() as u32;
    Ok(FetchSummary {
        feeds_total: n,
        feeds_processed: n,
        items_parsed: n,
        items_written: n,
    })
}

// -------------------- internals --------------------

fn extract_seeds_from_value(v: Value) -> Result<Vec<String>, PipelineError> {
    let seeds_val = match &v {
        Value::Array(_) => v,
        Value::Object(map) => {
            if let Some(seeds) = map.get("seeds") {
                seeds.clone()
            } else {
                return Err(PipelineError::Structure(
                    "expected an array or an object with 'seeds'".to_string(),
                ));
            }
        }
        _ => {
            return Err(PipelineError::Structure(
                "expected a JSON array or object".to_string(),
            ))
        }
    };

    let arr = seeds_val.as_array().ok_or_else(|| {
        PipelineError::Structure("the 'seeds' value must be an array".to_string())
    })?;

    if arr.is_empty() {
        return Err(PipelineError::Structure("seeds array is empty".to_string()));
    }

    let mut out = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        match item {
            Value::String(s) => {
                let s = s.trim();
                if s.is_empty() {
                    return Err(PipelineError::Structure(format!(
                        "item {} is an empty string",
                        i
                    )));
                }
                out.push(s.to_string());
            }
            Value::Object(obj) => {
                // Prefer id, else url, else guid.
                let id = obj
                    .get("id")
                    .and_then(|v| v.as_str())
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string());
                let url = obj
                    .get("url")
                    .and_then(|v| v.as_str())
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string());
                let guid = obj
                    .get("guid")
                    .and_then(|v| v.as_str())
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string());
                if let Some(chosen) = id.or(url).or(guid) {
                    out.push(chosen);
                } else {
                    return Err(PipelineError::Structure(format!(
                        "item {} object missing 'id' and 'url' and 'guid'",
                        i
                    )));
                }
            }
            other => {
                return Err(PipelineError::Structure(format!(
                    "item {} must be string or object, got {}",
                    i,
                    type_name_of_json(other)
                )));
            }
        }
    }

    Ok(out)
}

fn type_name_of_json(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
