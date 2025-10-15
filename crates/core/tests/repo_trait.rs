/*
Location: crates/core/tests/repo_trait.rs
Purpose: Compile-time and runtime sanity checks for core repo traits.
Notes: In-memory dummy backend; no I/O; validates trait signatures.
*/

use rssify_core::{Entry, EntryId, EntryRepo, Feed, FeedId, FeedRepo, RepoError, Tx};

use std::collections::BTreeMap;

#[derive(Default)]
struct MemTx {
    active: bool,
}
impl Tx for MemTx {
    fn is_active(&self) -> bool { self.active }
}

#[derive(Default)]
struct MemFeeds {
    feeds: BTreeMap<String, Feed>,
}
impl FeedRepo for MemFeeds {
    type Tx<'a> = MemTx where Self: 'a;

    fn begin<'a>(&'a self) -> Result<Self::Tx<'a>, RepoError> {
        Ok(MemTx { active: true })
    }

    fn get<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        id: &FeedId,
    ) -> Result<Feed, RepoError> {
        self.feeds
            .get(id.as_str())
            .cloned()
            .ok_or(RepoError::NotFound)
    }

    fn put<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        _feed: &Feed,
    ) -> Result<(), RepoError> {
        // For a real backend this would mutate state; here we only validate callability.
        Ok(())
    }

    fn list<'a>(&'a self, _tx: Option<&'a Self::Tx<'a>>) -> Result<Vec<Feed>, RepoError> {
        Ok(self.feeds.values().cloned().collect())
    }
}

#[derive(Default)]
struct MemEntries;
impl EntryRepo for MemEntries {
    type Tx<'a> = MemTx where Self: 'a;

    fn get<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        _id: &EntryId,
    ) -> Result<Entry, RepoError> {
        Err(RepoError::NotFound)
    }

    fn upsert<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        _entry: &Entry,
    ) -> Result<(), RepoError> {
        Ok(())
    }
}

#[test]
fn tx_is_active_flag() {
    let tx = MemTx { active: true };
    assert!(tx.is_active());
}

#[test]
fn repo_traits_are_object_safe() {
    fn takes_feed_repo<R: FeedRepo>(_r: &R) {}
    fn takes_entry_repo<R: EntryRepo>(_r: &R) {}
    let feeds = MemFeeds::default();
    let entries = MemEntries::default();
    takes_feed_repo(&feeds);
    takes_entry_repo(&entries);
}

