/*
File: crates/repos/fs/src/util.rs
Purpose: Small utilities: JSON read/write and ID escaping.
Inputs: serde/serde_json.
Outputs: helpers used by repo/trait impls.
Side effects: Filesystem I/O in read/write helpers.
*/

use serde::de::DeserializeOwned;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn write_atomic_json<T: ?Sized + serde::Serialize>(path: &Path, value: &T) -> Result<(), rssify_core::RepoError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| rssify_core::RepoError::Backend(e.to_string()))?;
    }
    let tmp = tmp_path(path, "tmp");
    {
        let mut f = fs::File::create(&tmp).map_err(|e| rssify_core::RepoError::Backend(e.to_string()))?;
        let s = serde_json::to_string_pretty(value)
            .map_err(|e| rssify_core::RepoError::Backend(e.to_string()))?;
        f.write_all(s.as_bytes())
            .map_err(|e| rssify_core::RepoError::Backend(e.to_string()))?;
        f.sync_all()
            .map_err(|e| rssify_core::RepoError::Backend(e.to_string()))?;
    }
    fs::rename(&tmp, path).map_err(|e| rssify_core::RepoError::Backend(e.to_string()))
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, rssify_core::RepoError> {
    let s = fs::read_to_string(path).map_err(|e| rssify_core::RepoError::Backend(e.to_string()))?;
    serde_json::from_str(&s).map_err(|e| rssify_core::RepoError::Backend(e.to_string()))
}

fn tmp_path(p: &Path, ext: &str) -> PathBuf {
    let mut tmp = p.as_os_str().to_owned();
    tmp.push(format!(".{}", ext));
    PathBuf::from(tmp)
}

/// Escape an ID into a filesystem-safe component:
/// - ASCII alnum, '-' and '_' pass through
/// - everything else becomes _XX (lower-hex)
pub fn escape_id(id: &str) -> String {
    fn hex(n: u8) -> char {
        match n {
            0..=9 => (b'0' + n) as char,
            10..=15 => (b'a' + (n - 10)) as char,
            _ => '?',
        }
    }
    let mut out = String::with_capacity(id.len());
    for b in id.bytes() {
        let c = b as char;
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
            out.push(c);
        } else {
            out.push('_');
            out.push(hex(b >> 4));
            out.push(hex(b & 0x0F));
        }
    }
    out
}

