//// File: crates/repos/fs/src/repo.rs
//// Role: FsRepo struct and all repo operations (paths, put/get/list, schedule).

use crate::util::{escape_id, unescape_id, write_json_atomic, write_text_atomic};
use serde_json::Value as Json;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct FsRepo {
    root: PathBuf,
}

impl FsRepo {
    /// Open (create if missing) a filesystem repo rooted at `root`.
    /// Subdirectories are created on demand.
    pub fn open<P: AsRef<Path>>(root: P) -> Self {
        let pb = root.as_ref().to_path_buf();
        let _ = fs::create_dir_all(&pb);
        Self { root: pb }
    }

    /// Alias used by some tests.
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self::open(root)
    }

    // ---------- path helpers ----------
    fn feeds_dir(&self) -> PathBuf {
        self.root.join("feeds")
    }

    fn entries_by_id_dir(&self) -> PathBuf {
        self.root.join("entries").join("by_id")
    }

    fn entries_by_feed_dir(&self, feed_id: &str) -> PathBuf {
        self.root
            .join("entries")
            .join("by_feed")
            .join(escape_id(feed_id))
    }

    fn feed_json_path(&self, feed_id: &str) -> PathBuf {
        self.feeds_dir()
            .join(escape_id(feed_id))
            .join("feed.json")
    }

    fn entry_by_id_path(&self, entry_id: &str) -> PathBuf {
        self.entries_by_id_dir()
            .join(format!("{}.json", escape_id(entry_id)))
    }

    fn entry_under_feed_path(&self, feed_id: &str, entry_id: &str) -> PathBuf {
        self.entries_by_feed_dir(feed_id)
            .join(format!("{}.json", escape_id(entry_id)))
    }

    fn last_ok_path(&self, feed_id: &str) -> PathBuf {
        self.root
            .join("schedule")
            .join(escape_id(feed_id))
            .join("last_ok.txt")
    }

    // ---------- feed ops ----------
    /// Atomically write <root>/feeds/<id>/feed.json
    pub fn put_feed_json(&self, feed_id: &str, feed_obj: &Json) -> std::io::Result<()> {
        let path = self.feed_json_path(feed_id);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        write_json_atomic(&path, feed_obj)
    }

    /// Read feed.json if present.
    pub fn get_feed_json(&self, feed_id: &str) -> std::io::Result<Option<Json>> {
        let path = self.feed_json_path(feed_id);
        if !path.exists() {
            return Ok(None);
        }
        let mut f = fs::File::open(&path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        let val: Json =
            serde_json::from_str(&buf).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(Some(val))
    }

    /// List known feed ids (only those with a feed.json).
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
                ids.push(unescape_id(&id));
            }
        }
        ids.sort();
        Ok(ids)
    }

    // ---------- entry ops ----------
    /// Atomically write entry under both by_id and by_feed.
    pub fn put_entry_json(
        &self,
        feed_id: &str,
        entry_id: &str,
        entry_obj: &Json,
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

    /// Read entry JSON by id (from entries/by_id).
    pub fn get_entry_json(&self, entry_id: &str) -> std::io::Result<Option<Json>> {
        let path = self.entry_by_id_path(entry_id);
        if !path.exists() {
            return Ok(None);
        }
        let mut f = fs::File::open(&path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        let val: Json =
            serde_json::from_str(&buf).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(Some(val))
    }

    /// List entry ids stored under entries/by_feed/<feed_id>.
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

    // ---------- schedule marker ----------
    /// Write a simple last-ok marker for a feed (UTF-8 text).
    pub fn mark_last_ok(&self, feed_id: &str, stamp: &str) -> std::io::Result<()> {
        let p = self.last_ok_path(feed_id);
        if let Some(d) = p.parent() {
            fs::create_dir_all(d)?;
        }
        write_text_atomic(&p, stamp)
    }
}

