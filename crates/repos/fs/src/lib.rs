//// File: crates/repos/fs/src/lib.rs
//// Purpose: Filesystem-backed minimal repo utilities for Phase 2 (no DB).
//// Notes:
//// - Safe ID encoding via escape_id.
//// - Atomic JSON/text writers.
//// - Feed and Entry helpers: put/get/list.
//// - Layout (created on demand):
////     <root>/feeds/<feed_id>/feed.json
////     <root>/entries/by_id/<entry_id>.json
////     <root>/entries/by_feed/<feed_id>/<entry_id>.json
////     <root>/schedule/<feed_id>/last_ok.txt

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Minimal filesystem repository.
#[derive(Clone, Debug)]
pub struct FsRepo {
    root: PathBuf,
}

impl FsRepo {
    /// Open (and create) the repository at `root`.
    pub fn open<P: AsRef<Path>>(root: P) -> Self {
        let pb = root.as_ref().to_path_buf();
        // Only ensure top-level exists; subdirs are created on demand.
        let _ = fs::create_dir_all(&pb);
        Self { root: pb }
    }

    /// Return absolute path to feeds dir.
    fn feeds_dir(&self) -> PathBuf {
        self.root.join("feeds")
    }
    /// Return absolute path to entries/by_id dir.
    fn entries_by_id_dir(&self) -> PathBuf {
        self.root.join("entries").join("by_id")
    }
    /// Return absolute path to entries/by_feed/<feed_id> dir.
    fn entries_by_feed_dir(&self, feed_id: &str) -> PathBuf {
        self.root
            .join("entries")
            .join("by_feed")
            .join(escape_id(feed_id))
    }
    /// Return absolute path to feed.json for a given feed id.
    fn feed_json_path(&self, feed_id: &str) -> PathBuf {
        self.feeds_dir()
            .join(escape_id(feed_id))
            .join("feed.json")
    }
    /// Return absolute path to entry file by id.
    fn entry_by_id_path(&self, entry_id: &str) -> PathBuf {
        self.entries_by_id_dir()
            .join(format!("{}.json", escape_id(entry_id)))
    }
    /// Return absolute path to entry file under by_feed/<feed>/.
    fn entry_under_feed_path(&self, feed_id: &str, entry_id: &str) -> PathBuf {
        self.entries_by_feed_dir(feed_id)
            .join(format!("{}.json", escape_id(entry_id)))
    }
    /// Return absolute path to schedule marker.
    fn last_ok_path(&self, feed_id: &str) -> PathBuf {
        self.root
            .join("schedule")
            .join(escape_id(feed_id))
            .join("last_ok.txt")
    }

    /// Put a feed document atomically: <root>/feeds/<id>/feed.json
    pub fn put_feed_json(
        &self,
        feed_id: &str,
        feed_obj: &serde_json::Value,
    ) -> std::io::Result<()> {
        let path = self.feed_json_path(feed_id);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        write_json_atomic(&path, feed_obj)
    }

    /// Get a feed document if it exists.
    pub fn get_feed_json(&self, feed_id: &str) -> std::io::Result<Option<serde_json::Value>> {
        let path = self.feed_json_path(feed_id);
        if !path.exists() {
            return Ok(None);
        }
        let mut file = fs::File::open(&path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let val: serde_json::Value = serde_json::from_str(&buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(Some(val))
    }

    /// List all feed ids that currently have a feed.json.
    pub fn list_feed_ids(&self) -> std::io::Result<Vec<String>> {
        let mut ids = Vec::new();
        let root = self.feeds_dir();
        if !root.exists() {
            return Ok(ids);
        }
        for ent in fs::read_dir(&root)? {
            let ent = ent?;
            if ent.file_type()?.is_dir() {
                let id = ent.file_name().to_string_lossy().to_string();
                // Reverse the escape to present a friendlier id if it was hex-escaped.
                ids.push(unescape_id(&id));
            }
        }
        ids.sort();
        Ok(ids)
    }

    /// Put an entry atomically under both by_id and by_feed trees.
    pub fn put_entry_json(
        &self,
        feed_id: &str,
        entry_id: &str,
        entry_obj: &serde_json::Value,
    ) -> std::io::Result<()> {
        let by_id = self.entry_by_id_path(entry_id);
        let by_feed = self.entry_under_feed_path(feed_id, entry_id);
        if let Some(d) = by_id.parent() {
            fs::create_dir_all(d)?;
        }
        if let Some(d) = by_feed.parent() {
            fs::create_dir_all(d)?;
        }
        write_json_atomic(&by_id, entry_obj)?;
        write_json_atomic(&by_feed, entry_obj)
    }

    /// Get entry JSON by entry id.
    pub fn get_entry_json(
        &self,
        entry_id: &str,
    ) -> std::io::Result<Option<serde_json::Value>> {
        let path = self.entry_by_id_path(entry_id);
        if !path.exists() {
            return Ok(None);
        }
        let mut file = fs::File::open(&path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let val: serde_json::Value = serde_json::from_str(&buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(Some(val))
    }

    /// List all entry ids stored under a feed directory.
    pub fn list_entries_by_feed(&self, feed_id: &str) -> std::io::Result<Vec<String>> {
        let mut ids = Vec::new();
        let dir = self.entries_by_feed_dir(feed_id);
        if !dir.exists() {
            return Ok(ids);
        }
        for ent in fs::read_dir(&dir)? {
            let ent = ent?;
            if ent.file_type()?.is_file() {
                let name = ent.file_name().to_string_lossy().to_string();
                if let Some(stripped) = name.strip_suffix(".json") {
                    ids.push(unescape_id(stripped));
                }
            }
        }
        ids.sort();
        Ok(ids)
    }

    /// Mark last-ok timestamp for a feed (simple text).
    pub fn mark_last_ok(&self, feed_id: &str, stamp: &str) -> std::io::Result<()> {
        let p = self.last_ok_path(feed_id);
        if let Some(d) = p.parent() {
            fs::create_dir_all(d)?;
        }
        write_text_atomic(&p, stamp)
    }
}

/// Escape an id into a filesystem-safe string (alnum stays, others -> _XX hex).
pub fn escape_id(id: &str) -> String {
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

/// Best-effort reverse of escape_id (only for presentation).
pub fn unescape_id(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'_' && i + 2 < bytes.len() {
            if let (Some(h), Some(l)) = (unhex(bytes[i + 1]), unhex(bytes[i + 2])) {
                out.

