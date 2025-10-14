/*
Module: rssify_core::ids
Purpose: Opaque identifiers for feeds and entries, stable across backends
Public API surface: FeedId, EntryId; constructors new, from_url, from_parts
Invariants: IDs are opaque; construction/formatting rules are centralized
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Opaque identifier for a feed (string form for portability).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FeedId(String);

/// Opaque identifier for a feed entry (string form for portability).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntryId(String);

impl FeedId {
    /// Access the underlying stable string form.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Construct from an arbitrary string (caller ensures stability).
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }

    /// Canonical constructor from a source URL.
    /// Policy (docs/ID_POLICY.md): "url:<original_url_trimmed>"
    pub fn from_url(url: &str) -> Self {
        let trimmed = url.trim();
        Self(format!("url:{trimmed}"))
    }
}

impl EntryId {
    /// Access the underlying stable string form.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Construct from an arbitrary string (caller ensures stability).
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }

    /// Canonical constructor from available entry parts.
    /// Precedence (docs/ID_POLICY.md):
    /// 1) GUID -> "guid:<guid>"
    /// 2) Link -> "link:<link>"
    /// 3) Hash(feed_id, title, published_ts) -> "hash:<u64_hex>"
    pub fn from_parts(
        feed: &FeedId,
        guid: Option<&str>,
        link: Option<&str>,
        title: Option<&str>,
        published_ts: Option<i64>,
    ) -> Self {
        if let Some(g) = guid.filter(|s| !s.trim().is_empty()) {
            return Self(format!("guid:{}", g.trim()));
        }
        if let Some(l) = link.filter(|s| !s.trim().is_empty()) {
            return Self(format!("link:{}", l.trim()));
        }
        let mut hasher = DefaultHasher::new();
        feed.as_str().hash(&mut hasher);
        if let Some(t) = title {
            t.hash(&mut hasher);
        }
        if let Some(ts) = published_ts {
            ts.hash(&mut hasher);
        }
        let h = hasher.finish();
        Self(format!("hash:{:016x}", h))
    }
}
