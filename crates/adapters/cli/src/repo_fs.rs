/*
Module: rssify_cli::repo_fs
Purpose: Filesystem repository layout helpers (pure; no I/O)
Public API surface: FsPaths::{feed_dir, feed_json, last_blob, entry_json}
Invariants:
- Filenames are derived only from canonical ID strings (FeedId/EntryId::as_str).
- Path components are URL-safe via a local percent-encoder.
- No filesystem access; string building only.
Notes: Keep file <= 200 LOC; refactor if approaching the soft cap.
*/

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use rssify_core::{EntryId, FeedId};

/// Percent-encode a string so it is safe as a single filesystem component.
/// Allowed unescaped bytes: ASCII letters, digits, '-', '_', and '.'.
/// Everything else becomes "%HH" (uppercase hex).
fn urlsafe_component(s: &str) -> String {
    fn is_safe(b: u8) -> bool {
        matches!(b,
            b'0'..=b'9' |
            b'a'..=b'z' |
            b'A'..=b'Z' |
            b'-' | b'_' | b'.'
        )
    }
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        if is_safe(b) {
            out.push(b as char);
        } else {
            out.push('%');
            out.push_str(&format!("{:02X}", b));
        }
    }
    out
}

/// Helper to join path segments and stringify using lossless UTF-8 fallback.
fn join(root: &str, segments: &[&str]) -> String {
    let mut pb = PathBuf::from(root);
    for seg in segments {
        pb.push(seg);
    }
    pb.to_string_lossy().into_owned()
}

/// Proposed layout under fs:<root>
/// root/
///   feeds/
///     <feed_id_urlsafe>/
///       feed.json
///       last_blob.bin
///       entries/
///         <entry_id_urlsafe>.json
pub struct FsPaths;

impl FsPaths {
    /// Directory for a given feed.
    pub fn feed_dir(root: &str, feed: &FeedId) -> String {
        let id = urlsafe_component(feed.as_str());
        join(root, &["feeds", &id])
    }

    /// Path to the canonical feed JSON record.
    pub fn feed_json(root: &str, feed: &FeedId) -> String {
        let dir = Self::feed_dir(root, feed);
        Path::new(&dir).join("feed.json").to_string_lossy().into_owned()
    }

    /// Path to the last raw blob bytes for the feed.
    pub fn last_blob(root: &str, feed: &FeedId) -> String {
        let dir = Self::feed_dir(root, feed);
        Path::new(&dir).join("last_blob.bin").to_string_lossy().into_owned()
    }

    /// Path to a specific entry JSON under the feed.
    pub fn entry_json(root: &str, feed: &FeedId, entry: &EntryId) -> String {
        let dir = Self::feed_dir(root, feed);
        let entry_id = urlsafe_component(entry.as_str());
        Path::new(&dir)
            .join("entries")
            .join(format!("{}.json", entry_id))
            .to_string_lossy()
            .into_owned()
    }
}

