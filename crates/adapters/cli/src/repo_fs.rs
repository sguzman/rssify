/*
Module: rssify_cli::repo_fs
Purpose: Filesystem repository layout helpers (pure; no I/O)
Public API surface: FsPaths::{feed_dir, feed_json, last_blob, entry_json}
Invariants:
- Filenames are derived only from canonical ID strings (FeedId/EntryId::as_str).
- Path components are URL-safe via a local percent-encoder.
- No filesystem access; string building only.

Layout expected by tests:
  <root>/feeds/<feed>/feed.json
  <root>/feeds/<feed>/last_blob.bin
  <root>/feeds/<feed>/entries/<entry>.json
*/

#![allow(dead_code)]

use std::path::Path;
use rssify_core::{EntryId, FeedId};

fn urlsafe_component(s: &str) -> String {
    fn is_safe(b: u8) -> bool {
        matches!(
            b,
            b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'-' | b'_' | b'.'
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

fn join_to_string(parts: &[&str]) -> String {
    let mut p = Path::new(parts[0]).to_path_buf();
    for seg in &parts[1..] {
        p = p.join(seg);
    }
    p.to_string_lossy().into_owned()
}

pub struct FsPaths;

impl FsPaths {
    /// <root>/feeds/<feed_id>
    pub fn feed_dir(root: &str, feed: &FeedId) -> String {
        join_to_string(&[root, "feeds", &urlsafe_component(feed.as_str())])
    }

    /// <root>/feeds/<feed_id>/feed.json
    pub fn feed_json(root: &str, feed: &FeedId) -> String {
        join_to_string(&[&Self::feed_dir(root, feed), "feed.json"])
    }

    /// Deprecated in tests (kept for completeness): <root>/entries/by_id/<entry>.json
    pub fn entry_id_file(root: &str, entry: &EntryId) -> String {
        join_to_string(&[
            root,
            "entries",
            "by_id",
            &format!("{}.json", urlsafe_component(entry.as_str())),
        ])
    }

    /// <root>/feeds/<feed_id>/entries
    pub fn entry_by_feed_dir(root: &str, feed: &FeedId) -> String {
        join_to_string(&[
            &Self::feed_dir(root, feed),
            "entries",
        ])
    }

    /// <root>/feeds/<feed_id>/entries/<entry_id>.json
    pub fn entry_by_feed_file(root: &str, feed: &FeedId, entry: &EntryId) -> String {
        join_to_string(&[
            &Self::entry_by_feed_dir(root, feed),
            &format!("{}.json", urlsafe_component(entry.as_str())),
        ])
    }

    /// <root>/feeds/<feed_id>/last_blob.bin
    pub fn last_blob(root: &str, feed: &FeedId) -> String {
        join_to_string(&[
            &Self::feed_dir(root, feed),
            "last_blob.bin",
        ])
    }

    /// Alias used by tests for the entry path.
    /// <root>/feeds/<feed_id>/entries/<entry_id>.json
    pub fn entry_json(root: &str, feed: &FeedId, entry: &EntryId) -> String {
        Self::entry_by_feed_file(root, feed, entry)
    }
}

