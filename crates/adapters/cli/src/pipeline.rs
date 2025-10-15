//// File: crates/adapters/cli/src/pipeline.rs
//// Purpose: Parse seed inputs for the fetch path.
//// Inputs: JSON file path. Accepted formats:
////   1) Array of strings: ["https://a", "https://b"]
////   2) Array of objects: [{"id":"foo","url":"https://a"}, {"url":"https://b"}]
////   3) Object with "seeds": { "seeds": [ ... formats 1 or 2 ... ] }
//// Output: Vec<String> of seed identifiers. If an object has "id", prefer it;
////   otherwise fall back to "url", then "guid".
//// Errors:
////   - File or IO errors bubble up.
////   - JSON parse errors are wrapped with context.
////   - Structural errors (no seeds, empty array, bad element) give clear messages.
//// Notes:
////   - Keep dependency surface minimal: uses serde_json only.
////   - Function is pure w.r.t. effect beyond reading the file path.

use std::fs;
use std::io;
use std::path::Path;

use serde_json::Value;

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

/// Load feed seeds from a JSON file. See module header for accepted formats.
pub fn load_feed_seeds<P: AsRef<Path>>(path: P) -> Result<Vec<String>, PipelineError> {
    let data = fs::read_to_string(&path)?;
    let v: Value = serde_json::from_str(&data)
        .map_err(|e| PipelineError::Json(e.to_string()))?;

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
        return Err(PipelineError::Structure(
            "seeds array is empty".to_string(),
        ));
    }

    let mut out = Vec::with_capacity(arr.len());

    for (i, item) in arr.iter().enumerate() {
        match item {
            Value::String(s) => {
                let s = s.trim();
                if s.is_empty() {
                    return Err(PipelineError::Structure(format!(
                        "item {} is an empty string", i
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
                        "item {} object missing 'id' and 'url' and 'guid'", i
                    )));
                }
            }
            other => {
                return Err(PipelineError::Structure(format!(
                    "item {} must be string or object, got {}", i, type_name_of_json(other)
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

#[cfg(test)]
mod tests_inline {
    use super::*;

    #[test]
    fn parse_array_of_strings() {
        let v: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
        let out = super::extract_from_value_for_test(v).unwrap();
        assert_eq!(out, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn parse_array_of_objects_prefers_id() {
        let v: Value = serde_json::from_str(r#"[{"id":"X","url":"u1"},{"url":"u2"}]"#).unwrap();
        let out = super::extract_from_value_for_test(v).unwrap();
        assert_eq!(out, vec!["X".to_string(), "u2".to_string()]);
    }

    #[test]
    fn parse_object_with_seeds() {
        let v: Value = serde_json::from_str(r#"{ "seeds": ["u1", {"url":"u2"}] }"#).unwrap();
        let out = super::extract_from_value_for_test(v).unwrap();
        assert_eq!(out, vec!["u1".to_string(), "u2".to_string()]);
    }

    // Helper for inline tests to bypass fs.
    impl super::PipelineError {
        fn structure_msg(&self) -> Option<&str> {
            match self {
                super::PipelineError::Structure(s) => Some(s.as_str()),
                _ => None,
            }
        }
    }
}

// Internal helper exclusively for tests in this module or external test file.
// Not exposed in API to keep surface minimal.
#[doc(hidden)]
pub fn extract_from_value_for_test(v: Value) -> Result<Vec<String>, PipelineError> {
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
        return Err(PipelineError::Structure(
            "seeds array is empty".to_string(),
        ));
    }

    let mut out = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        match item {
            Value::String(s) => {
                let s = s.trim();
                if s.is_empty() {
                    return Err(PipelineError::Structure(format!(
                        "item {} is an empty string", i
                    )));
                }
                out.push(s.to_string());
            }
            Value::Object(obj) => {
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
                        "item {} object missing 'id' and 'url' and 'guid'", i
                    )));
                }
            }
            other => {
                return Err(PipelineError::Structure(format!(
                    "item {} must be string or object, got {}", i, type_name_of_json(other)
                )));
            }
        }
    }
    Ok(out)
}
