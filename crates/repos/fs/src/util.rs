// File: crates/repos/fs/src/util.rs
// Purpose: Shared FS helpers and simple escaping.
// Inputs: serde for JSON, std::fs and std::io.
// Outputs: small utility functions used by repo impls.
// Side effects: Filesystem I/O.

use rssify_core::RepoError;
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

pub fn ensure_parent(path: &Path) -> Result<(), RepoError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| RepoError::Backend(e.to_string()))?;
    }
    Ok(())
}

pub fn write_atomic_json<T: Serialize>(path: &Path, value: &T) -> Result<(), RepoError> {
    ensure_parent(path)?;
    let tmp = path.with_extension("json.tmp");
    {
        let mut f = fs::File::create(&tmp).map_err(|e| RepoError::Backend(e.to_string()))?;
        let data = serde_json::to_vec_pretty(value).map_err(|e| RepoError::Ser(e.to_string()))?;
        f.write_all(&data)
            .map_err(|e| RepoError::Backend(e.to_string()))?;
        let _ = f.sync_all();
    }
    fs::rename(&tmp, path).map_err(|e| RepoError::Backend(e.to_string()))?;
    Ok(())
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, RepoError> {
    let mut f = fs::File::open(path).map_err(|_| RepoError::NotFound)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)
        .map_err(|e| RepoError::Backend(e.to_string()))?;
    serde_json::from_slice(&buf).map_err(|e| RepoError::Ser(e.to_string()))
}

pub fn write_atomic_text(path: &Path, text: &str) -> Result<(), RepoError> {
    ensure_parent(path)?;
    let tmp = path.with_extension("tmp");
    {
        let mut f = fs::File::create(&tmp).map_err(|e| RepoError::Backend(e.to_string()))?;
        f.write_all(text.as_bytes())
            .map_err(|e| RepoError::Backend(e.to_string()))?;
        let _ = f.sync_all();
    }
    fs::rename(&tmp, path).map_err(|e| RepoError::Backend(e.to_string()))?;
    Ok(())
}

// Escape an id string to be safe as a directory or file name without extra deps.
// Rules: '/' and '\' -> '_', ':' -> '-', rest unchanged; trim spaces.
pub fn escape_id(s: &str) -> String {
    s.trim()
        .chars()
        .map(|c| match c {
            '/' | '\\' => '_',
            ':' => '-',
            _ => c,
        })
        .collect()
}

