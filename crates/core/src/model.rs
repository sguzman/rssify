/*
Module: rssify_core::model
Purpose: Pure domain records for feeds, entries, and fetch results
Public API surface: Feed, Entry, ContentBlob, ContentKind, FetchOutcome
Invariants: Records are serde-serializable and transport-friendly
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

use crate::{EntryId, FeedId};
use serde::{Deserialize, Serialize};

/// Raw content captured from a source (kept as bytes; encoding may vary).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentBlob {
    pub kind: ContentKind,
    pub bytes: Vec<u8>,
}

/// Lightly-typed blob kinds we care about at the boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentKind {
    Xml,
    Json,
    Html,
    Text,
    Binary,
}

/// Canonical feed metadata known to the system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Feed {
    pub id: FeedId,
    pub url: String,
    pub title: Option<String>,
    pub site_url: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub active: bool,
}

/// Canonical entry representation post-parse/normalize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub id: EntryId,
    pub feed: FeedId,
    pub url: Option<String>,
    pub title: Option<String>,
    pub published_ts: Option<i64>, // unix seconds
    pub updated_ts: Option<i64>,   // unix seconds
    pub summary: Option<String>,
    pub content: Option<ContentBlob>,
}

/// Result of a fetch attempt at the boundary (no network in core).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FetchOutcome {
    NotModified, // 304-equivalent
    NewContent { blob: ContentBlob, elapsed_ms: u64 },
    TransientFailure { hint: Option<String> },
    PermanentFailure { hint: Option<String> },
}
