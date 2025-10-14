/*
Module: rssify_core::repo
Purpose: Repository boundary contracts (traits) for feeds, entries, schedule
Public API surface: Tx, FeedRepo, EntryRepo, ScheduleRepo
Invariants: No I/O in core; implement in adapters; results return RepoError
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

use crate::{Entry, EntryId, Feed, FeedId, RepoError};

/// Opaque transaction/context handle exposed by backends.
/// Adapters decide whether this is real transactional state or a no-op.
pub trait Tx {
    /// Return true if this handle represents an active transactional scope.
    fn is_active(&self) -> bool;
}

/// CRUD surface for feeds.
pub trait FeedRepo {
    type Tx<'a>: Tx
    where
        Self: 'a;

    fn get<'a>(&'a self, tx: Option<&'a Self::Tx<'a>>, id: &FeedId) -> Result<Feed, RepoError>;
    fn put<'a>(&'a self, tx: Option<&'a Self::Tx<'a>>, feed: &Feed) -> Result<(), RepoError>;
    fn list<'a>(&'a self, tx: Option<&'a Self::Tx<'a>>) -> Result<Vec<Feed>, RepoError>;
}

/// CRUD surface for entries.
pub trait EntryRepo {
    type Tx<'a>: Tx
    where
        Self: 'a;

    fn get<'a>(&'a self, tx: Option<&'a Self::Tx<'a>>, id: &EntryId) -> Result<Entry, RepoError>;
    fn upsert<'a>(&'a self, tx: Option<&'a Self::Tx<'a>>, entry: &Entry) -> Result<(), RepoError>;
    fn list_by_feed<'a>(
        &'a self,
        tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
    ) -> Result<Vec<Entry>, RepoError>;
}

/// Minimal scheduling persistence (future phases can expand).
pub trait ScheduleRepo {
    type Tx<'a>: Tx
    where
        Self: 'a;

    /// Last successful fetch time (unix seconds) for a feed.
    fn last_ok_fetch_ts<'a>(
        &'a self,
        tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
    ) -> Result<Option<i64>, RepoError>;

    /// Record an outcome time to inform future scheduling.
    fn record_fetch_ts<'a>(
        &'a self,
        tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
        ts: i64,
    ) -> Result<(), RepoError>;
}
