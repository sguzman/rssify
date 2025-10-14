/*
Module: rssify_core::ids
Purpose: Opaque identifiers for feeds and entries, stable across backends
Public API surface: FeedId, EntryId
Invariants: IDs are opaque; construction/formatting rules are centralized
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

use serde::{Deserialize, Serialize};

/// Opaque identifier for a feed (string form for portability).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FeedId(String);

/// Opaque identifier for a feed entry (string form for portability).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntryId(String);

impl FeedId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

impl EntryId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}
