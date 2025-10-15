//// File: crates/adapters/cli/src/repo_fs.rs
//// Role: Filesystem path helpers referenced by tests (no I/O here).

use rssify_core::{EntryId, FeedId};
use std::path::{Path, PathBuf};

pub struct FsPaths;

impl FsPaths {
    /// <root>/feeds/<feed_id>
    pub fn feed_dir(root: &str, feed: &FeedId) -> String {
        Path::new(root)
            .join("feeds")
            .join(escape_id(feed.as_str()))
            .to_string_lossy()
            .into_owned()
    }

    /// <root>/feeds/<feed_id>/feed.json
    pub fn feed_json(root: &str, feed: &FeedId) -> String {
        Path::new(&Self::feed_dir(root, feed))
            .join("feed.json")
            .to_string_lossy()
            .into_owned()
    }

    /// <root>/entries/by_id/<entry_id>.json
    pub fn entry_id_file(root: &str, entry: &EntryId) -> String {
        Path::new(root)
            .join("entries")
            .join("by_id")
            .join(format!("{}.json", escape_id(entry.as_str())))
            .to_string_lossy()
            .into_owned()
    }

    /// <root>/entries/by_feed/<feed_id>
    pub fn entry_by_feed_dir(root: &str, feed: &FeedId) -> String {
        Path::new(root)
            .join("entries")
            .join("by_feed")
            .join(escape_id(feed.as_str()))
            .to_string_lossy()
            .into_owned()
    }

    /// <root>/entries/by_feed/<feed_id>/<entry_id>.json
    pub fn entry_by_feed_file(root: &str, feed: &FeedId, entry: &EntryId) -> String {
        Path::new(&Self::entry_by_feed_dir(root, feed))
            .join(format!("{}.json", escape_id(entry.as_str())))
            .to_string_lossy()
            .into_owned()
    }

    /// Alias expected by older tests for the schedule path:
    /// <root>/schedule/<feed_id>/last_ok.txt
    pub fn last_blob(root: &str, feed: &FeedId) -> String {
        Self::schedule_last_ok(root, feed)
    }

    /// <root>/schedule/<feed_id>/last_ok.txt
    pub fn schedule_last_ok(root: &str, feed: &FeedId) -> String {
        Path::new(root)
            .join("schedule")
            .join(escape_id(feed.as_str()))
            .join("last_ok.txt")
            .to_string_lossy()
            .into_owned()
    }

    /// Alias expected by older tests for by-feed entry JSON path.
    pub fn entry_json(root: &str, feed: &FeedId, entry: &EntryId) -> String {
        Self::entry_by_feed_file(root, feed, entry)
    }
}

fn escape_id(id: &str) -> String {
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

fn hex(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'a' + (n - 10)) as char,
        _ => '?',
    }
}

