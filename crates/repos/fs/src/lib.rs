// File: crates/repos/fs/src/lib.rs
// Purpose: Filesystem-backed implementation of rssify_core repository traits.
// Inputs: rssify_core::{Tx, FeedRepo, EntryRepo, ScheduleRepo, Feed, Entry, FeedId, EntryId, RepoError}.
// Outputs: RepoError on failures; JSON-serialized records on disk.
// Side effects: Filesystem I/O with atomic renames; structured comments for logging.
// Invariants:
//  - Public API stays within core traits; no business logic here.
//  - Writes are atomic per file via tmp+rename.
// Tests: crates/repos/fs/test/roundtrip.rs provides end-to-end coverage.

use rssify_core::{Entry, EntryId, EntryRepo, Feed, FeedId, FeedRepo, RepoError, ScheduleRepo, Tx};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Trivial transaction handle for FS backend (no real transactions).
#[derive(Clone, Copy, Debug, Default)]
pub struct FsTx {
    active: bool,
}

impl Tx for FsTx {
    fn is_active(&self) -> bool {
        self.active
    }
}

/// Filesystem repository rooted at `root`.
#[derive(Clone, Debug)]
pub struct FsRepo {
    root: PathBuf,
}

impl FsRepo {
    /// Create a new FS repo rooted at `root`. The path is not created eagerly.
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        Self { root: root.into() }
    }

    /// Convenience: start an "active" no-op transaction.
    pub fn begin_tx(&self) -> FsTx {
        FsTx { active: true }
    }

    // ----- Path helpers -----------------------------------------------------

    fn feeds_dir(&self) -> PathBuf {
        self.root.join("feeds")
    }
    fn entries_by_id_dir(&self) -> PathBuf {
        self.root.join("entries").join("by_id")
    }
    fn entries_by_feed_dir(&self, feed: &FeedId) -> PathBuf {
        self.root
            .join("entries")
            .join("by_feed")
            .join(escape_id(feed.as_str()))
    }
    fn schedule_dir(&self, feed: &FeedId) -> PathBuf {
        self.root.join("schedule").join(escape_id(feed.as_str()))
    }

    fn feed_path(&self, id: &FeedId) -> PathBuf {
        self.feeds_dir()
            .join(escape_id(id.as_str()))
            .join("feed.json")
    }
    fn entry_by_id_path(&self, id: &EntryId) -> PathBuf {
        self.entries_by_id_dir().join(format!("{}.json", escape_id(id.as_str())))
    }
    fn entry_by_feed_path(&self, feed: &FeedId, id: &EntryId) -> PathBuf {
        self.entries_by_feed_dir(feed)
            .join(format!("{}.json", escape_id(id.as_str())))
    }
    fn last_ok_path(&self, feed: &FeedId) -> PathBuf {
        self.schedule_dir(feed).join("last_ok.txt")
    }

    // ----- FS utils ---------------------------------------------------------

    fn ensure_parent(path: &Path) -> Result<(), RepoError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| RepoError::Backend(e.to_string()))?;
        }
        Ok(())
    }

    fn write_atomic_json<T: Serialize>(path: &Path, value: &T) -> Result<(), RepoError> {
        Self::ensure_parent(path)?;
        let tmp = path.with_extension("json.tmp");
        {
            let mut f = fs::File::create(&tmp).map_err(|e| RepoError::Backend(e.to_string()))?;
            let data =
                serde_json::to_vec_pretty(value).map_err(|e| RepoError::Ser(e.to_string()))?;
            f.write_all(&data)
                .map_err(|e| RepoError::Backend(e.to_string()))?;
            f.sync_all().ok(); // best-effort
        }
        fs::rename(&tmp, path).map_err(|e| RepoError::Backend(e.to_string()))?;
        Ok(())
    }

    fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, RepoError> {
        let mut f = fs::File::open(path).map_err(|_| RepoError::NotFound)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)
            .map_err(|e| RepoError::Backend(e.to_string()))?;
        serde_json::from_slice(&buf).map_err(|e| RepoError::Ser(e.to_string()))
    }

    fn write_atomic_text(path: &Path, text: &str) -> Result<(), RepoError> {
        Self::ensure_parent(path)?;
        let tmp = path.with_extension("tmp");
        {
            let mut f = fs::File::create(&tmp).map_err(|e| RepoError::Backend(e.to_string()))?;
            f.write_all(text.as_bytes())
                .map_err(|e| RepoError::Backend(e.to_string()))?;
            f.sync_all().ok();
        }
        fs::rename(&tmp, path).map_err(|e| RepoError::Backend(e.to_string()))?;
        Ok(())
    }
}

// -------------------- FeedRepo ----------------------------------------------

impl FeedRepo for FsRepo {
    type Tx<'a> = FsTx where Self: 'a;

    fn get<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        id: &FeedId,
    ) -> Result<Feed, RepoError> {
        let path = self.feed_path(id);
        Self::read_json::<Feed>(&path)
    }

    fn put<'a>(&'a self, _tx: Option<&'a Self::Tx<'a>>, feed: &Feed) -> Result<(), RepoError> {
        let path = self.feed_path(&feed.id);
        Self::write_atomic_json(&path, feed)
    }

    fn list<'a>(&'a self, _tx: Option<&'a Self::Tx<'a>>) -> Result<Vec<Feed>, RepoError> {
        let mut out = Vec::new();
        let feeds_dir = self.feeds_dir();
        let rd = match fs::read_dir(&feeds_dir) {
            Ok(rd) => rd,
            Err(_) => return Ok(out), // empty if dir absent
        };
        for entry in rd {
            let entry = entry.map_err(|e| RepoError::Backend(e.to_string()))?;
            let p = entry.path().join("feed.json");
            if p.is_file() {
                if let Ok(feed) = Self::read_json::<Feed>(&p) {
                    out.push(feed);
                }
            }
        }
        Ok(out)
    }
}

// -------------------- EntryRepo ---------------------------------------------

impl EntryRepo for FsRepo {
    type Tx<'a> = FsTx where Self: 'a;

    fn get<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        id: &EntryId,
    ) -> Result<Entry, RepoError> {
        let p = self.entry_by_id_path(id);
        Self::read_json::<Entry>(&p)
    }

    fn upsert<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        entry: &Entry,
    ) -> Result<(), RepoError> {
        // Write both the by-id index and the by-feed shard.
        let by_id = self.entry_by_id_path(&entry.id);
        let by_feed = self.entry_by_feed_path(&entry.feed, &entry.id);
        Self::write_atomic_json(&by_id, entry)?;
        Self::write_atomic_json(&by_feed, entry)?;
        Ok(())
    }

    fn list_by_feed<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
    ) -> Result<Vec<Entry>, RepoError> {
        let dir = self.entries_by_feed_dir(feed);
        let mut out = Vec::new();
        let rd = match fs::read_dir(&dir) {
            Ok(rd) => rd,
            Err(_) => return Ok(out),
        };
        for entry in rd {
            let entry = entry.map_err(|e| RepoError::Backend(e.to_string()))?;
            let p = entry.path();
            if p.extension().and_then(|s| s.to_str()) == Some("json") && p.is_file() {
                if let Ok(e) = Self::read_json::<Entry>(&p) {
                    out.push(e);
                }
            }
        }
        // Stable-ish order: published_ts then updated_ts, then id.
        out.sort_by(|a, b| {
            a.published_ts
                .cmp(&b.published_ts)
                .then(a.updated_ts.cmp(&b.updated_ts))
                .then(a.id.as_str().cmp(b.id.as_str()))
        });
        Ok(out)
    }
}

// -------------------- ScheduleRepo ------------------------------------------

impl ScheduleRepo for FsRepo {
    type Tx<'a> = FsTx where Self: 'a;

    fn last_ok_fetch_ts<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
    ) -> Result<Option<i64>, RepoError> {
        let p = self.last_ok_path(feed);
        if !p.exists() {
            return Ok(None);
        }
        let mut s = String::new();
        fs::File::open(&p)
            .map_err(|e| RepoError::Backend(e.to_string()))?
            .read_to_string(&mut s)
            .map_err(|e| RepoError::Backend(e.to_string()))?;
        let ts = s.trim().parse::<i64>().map_err(|e| RepoError::Ser(e.to_string()))?;
        Ok(Some(ts))
    }

    fn record_fetch_ts<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
        ts: i64,
    ) -> Result<(), RepoError> {
        let p = self.last_ok_path(feed);
        Self::write_atomic_text(&p, &format!("{ts}"))
    }
}

// -------------------- Utilities ---------------------------------------------

/// Escape an id string to be safe as a directory/file name without extra deps.
/// Rules: '/' and '\\' -> '_', ':' -> '-', rest unchanged; trim spaces.
fn escape_id(s: &str) -> String {
    s.trim()
        .chars()
        .map(|c| match c {
            '/' | '\\' => '_',
            ':' => '-',
            _ => c,
        })
        .collect()
}

/// Helper: current unix seconds (used in tests or future metrics).
#[allow(dead_code)]
fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escaping_is_stable() {
        assert_eq!(escape_id("url:https://ex/a/b"), "url-https___ex_a_b");
        assert_eq!(escape_id("hash:deadbeef"), "hash-deadbeef");
    }
}

