//// File: crates/adapters/cli/src/pipeline.rs
//// Role: Phase 2 pipeline helpers and error types used by tests.

use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct FetchSummary {
    pub feeds_total: u32,
    pub feeds_processed: u32,
    pub items_parsed: u32,
    pub items_written: u32,
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
pub fn load_feed_seeds(path: &str) -> Result<Vec<String>, PipelineError> {
    let mut f = File::open(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            PipelineError::NotFound(path.to_string())
        } else {
            PipelineError::Read {
                path: path.to_string(),
                source: e,
            }
        }
    })?;

    let mut buf = String::new();
    f.read_to_string(&mut buf).map_err(|e| PipelineError::Read {
        path: path.to_string(),
        source: e,
    })?;

    let val: Json =
        serde_json::from_str(&buf).map_err(|e| PipelineError::InvalidJson { path: path.to_string(), source: e })?;

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
        return Err(PipelineError::NoSeeds(path.to_string()));
    }
    Ok(out)
}

