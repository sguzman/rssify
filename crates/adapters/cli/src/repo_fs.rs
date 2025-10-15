//// File: crates/adapters/cli/src/repo_fs.rs
//// Role: Filesystem path helpers referenced by tests (no I/O here).

use rssify_core::{EntryId, FeedId};
use std::path::{Path, PathBuf};

pub struct FsPaths;

impl FsPaths {
    pub fn feed_dir(root: &str, feed: &FeedId) -> PathBuf {
        Path::new(root).join("feeds").join(escape_id(feed.as_str()))
    }

    pub fn feed_json(root: &str, feed: &FeedId) -> PathBuf {
        Self::feed_dir(root, feed).join("feed.json")
    }

    pub fn entry_id_file(root: &str, entry: &EntryId) -> PathBuf {
        Path::new(root)
            .join("entries")
            .join("by_id")
            .join(format!("{}.json", escape_id(entry.as_str())))
    }

    pub fn entry_by_feed_dir(root: &str, feed: &FeedId) -> PathBuf {
        Path::new(root)
            .join("entries")
            .join("by_feed")
            .join(escape_id(feed.as_str()))
    }

    pub fn entry_by_feed_file(root: &str, feed: &FeedId, entry: &EntryId) -> PathBuf {
        Self::entry_by_feed_dir(root, feed).join(format!("{}.json", escape_id(entry.as_str())))
    }

    pub fn schedule_last_ok(root: &str, feed: &FeedId) -> PathBuf {
        Path::new(root)
            .join("schedule")
            .join(escape_id(feed.as_str()))
            .join("last_ok.txt")
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

