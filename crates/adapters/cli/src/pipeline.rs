/*
Module: rssify_cli::pipeline
Purpose: Document the Phase 1 fetch+persist pipeline surfaces (no logic)
Public API surface: type aliases and TODOs for step-by-step implementation
Invariants: Adapters remain thin; core types drive IDs and records
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

#![allow(dead_code)]

use rssify_core::{ContentBlob, ContentKind, Entry, EntryId, Feed, FeedId, FetchOutcome};

/// Phase 1 pipeline plan (no code yet):
/// 1) load_feeds(from) -> Vec<FeedSeed>
/// 2) fetch_feed(seed) -> FetchOutcome
/// 3) parse_blob(seed.feed_id, blob) -> (FeedMetaDelta, Vec<Entry>)
/// 4) persist_fs(root, feed_delta, entries) -> PersistStats
/// 5) aggregate metrics and emit CLI-friendly summary

/// A minimal seed parsed from feeds.json.
pub struct FeedSeed {
    pub url: String,
    pub title_hint: Option<String>,
}

/// Minimal feed metadata delta detected after a successful fetch/parse.
pub struct FeedMetaDelta {
    pub title: Option<String>,
    pub site_url: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

/// Persist summary for metrics and CLI reporting.
pub struct PersistStats {
    pub feed: FeedId,
    pub items_written: usize,
    pub elapsed_ms: u64,
    pub not_modified: bool,
    pub failure_hint: Option<String>,
}

// TODO(phase1):
// - Implement `load_feeds` to read feeds.json (pure file read in CLI adapter).
// - Implement `fetch_feed` using an HTTP client with conditional headers.
// - Implement `parse_blob` for minimal RSS/Atom fields -> entries + meta delta.
// - Implement `persist_fs` with idempotent writes under fs:<root>.
// - Wire `rssify fetch` to iterate seeds and gather stats for --json output.

// NOTE: No business logic in this file yet. Only shapes and plan.
