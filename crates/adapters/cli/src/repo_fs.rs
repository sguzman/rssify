/*
Module: rssify_cli::repo_fs
Purpose: Filesystem repository planning for Phase 1 (no I/O yet)
Public API surface: File layout notes and fn names to implement later
Invariants: Idempotent writes; filenames derived from canonical IDs
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

#![allow(dead_code)]

use rssify_core::{Entry, EntryId, Feed, FeedId};

/// Proposed layout under fs:<root>
/// root/
///   feeds/
///     <feed_id_urlsafe>/
///       feed.json          (Feed record incl. etag/last_modified)
///       last_blob.bin      (raw ContentBlob bytes)
///       entries/
///         <entry_id_urlsafe>.json
///
/// - entry JSON contains canonical fields and optional inlined ContentBlob metadata
/// - urlsafe = percent-encode or base64url without padding
///
/// TODO(phase1):
/// - urlsafe encoders (pure helpers)
/// - deterministic path builders from FeedId/EntryId
/// - atomic write strategy (tmp file + rename)
/// - idempotent upsert semantics
///
/// NOTE: No filesystem calls yet; only function names and doc comments.

pub struct FsPaths;
impl FsPaths {
    pub fn feed_dir(_root: &str, _feed: &FeedId) -> String {
        String::new()
    }
    pub fn feed_json(_root: &str, _feed: &FeedId) -> String {
        String::new()
    }
    pub fn last_blob(_root: &str, _feed: &FeedId) -> String {
        String::new()
    }
    pub fn entry_json(_root: &str, _feed: &FeedId, _entry: &EntryId) -> String {
        String::new()
    }
}
