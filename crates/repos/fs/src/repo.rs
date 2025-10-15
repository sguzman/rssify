/*
File: crates/repos/fs/src/repo.rs
Purpose: FsRepo struct and path helpers.
Inputs: root path injected at construction.
Outputs: FsRepo methods for path building and tx creation.
Side effects: None, except path string work.
*/

use crate::tx::FsTx;
use crate::util::escape_id;
use rssify_core::{EntryId, FeedId};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct FsRepo {
    pub(crate) root: PathBuf,
}

impl FsRepo {
    /// Create a repo rooted at `root` (directories created lazily by writers).
    pub fn open<P: AsRef<Path>>(root: P) -> Self {
        Self { root: root.as_ref().to_path_buf() }
    }

    /// Alias used by some tests.
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self::open(root)
    }

    /// Begin a no-op transactional scope (for API symmetry).
    pub fn begin_tx(&self) -> FsTx {
        FsTx { active: true }
    }

    // --------- path builders ---------
    pub(crate) fn feeds_dir(&self) -> PathBuf {
        self.root.join("feeds")
    }

    pub(crate) fn entries_by_id_dir(&self) -> PathBuf {
        self.root.join("entries").join("by_id")
    }

    pub(crate) fn entries_by_feed_dir(&self, feed: &FeedId) -> PathBuf {
        self.root.join("entries").join("by_feed").join(escape_id(feed.as_str()))
    }

    pub(crate) fn feed_path(&self, id: &FeedId) -> PathBuf {
        self.feeds_dir().join(escape_id(id.as_str())).join("feed.json")
    }

    pub(crate) fn entry_by_id_path(&self, id: &EntryId) -> PathBuf {
        self.entries_by_id_dir().join(format!("{}.json", escape_id(id.as_str())))
    }

    pub(crate) fn entry_by_feed_path(&self, feed: &FeedId, id: &EntryId) -> PathBuf {
        self.entries_by_feed_dir(feed).join(format!("{}.json", escape_id(id.as_str())))
    }

    pub(crate) fn schedule_last_ok_path(&self, feed: &FeedId) -> PathBuf {
        self.root.join("schedule").join(escape_id(feed.as_str())).join("last_ok.txt")
    }
}

