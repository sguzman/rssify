 // File: crates/repos/fs/src/repo.rs
 // Purpose: FsRepo struct and path helpers.
 // Inputs: root path injected at construction.
 // Outputs: FsRepo methods for path building and tx creation.
 // Side effects: None, except path string work.

 use crate::tx::FsTx;
 use crate::util::escape_id;
 use rssify_core::FeedId;
 use std::path::{Path, PathBuf};

 #[derive(Clone, Debug)]
 pub struct FsRepo {
     pub(crate) root: PathBuf,
 }

 impl FsRepo {
     /// Open or create a filesystem-backed repository rooted at `root`.
     /// Phase 2 keeps this minimal and only ensures the root exists; subdirs are created on demand.
     pub fn open<P: AsRef<Path>>(root: P) -> Self {
         let pb = root.as_ref().to_path_buf();
         let _ = std::fs::create_dir_all(&pb);
         Self { root: pb }
     }

     pub fn new<P: Into<PathBuf>>(root: P) -> Self {
         Self { root: root.into() }
     }

     pub fn begin_tx(&self) -> FsTx {
         FsTx { active: true }
     }

     // Directory roots
     pub(crate) fn feeds_dir(&self) -> PathBuf {
         self.root.join("feeds")
     }
     pub(crate) fn entries_by_id_dir(&self) -> PathBuf {
         self.root.join("entries").join("by_id")
     }
     pub(crate) fn entries_by_feed_dir(&self, feed: &FeedId) -> PathBuf {
         self.root
             .join("entries")
             .join("by_feed")
             .join(escape_id(feed.as_str()))
     }
     pub(crate) fn schedule_dir(&self, feed: &FeedId) -> PathBuf {
         self.root.join("schedule").join(escape_id(feed.as_str()))
     }

     // Concrete file paths
     pub(crate) fn feed_path(&self, id: &FeedId) -> PathBuf {
         self.feeds_dir()
             .join(escape_id(id.as_str()))
             .join("feed.json")
     }
     pub(crate) fn entry_by_id_path(&self, id: &rssify_core::EntryId) -> PathBuf {
         self.entries_by_id_dir()
             .join(format!("{}.json", escape_id(id.as_str())))
     }
     pub(crate) fn entry_by_feed_path(
         self: &Self,
         feed: &FeedId,
         id: &rssify_core::EntryId,
     ) -> PathBuf {
         self.entries_by_feed_dir(feed)
             .join(format!("{}.json", escape_id(id.as_str())))
     }
     pub(crate) fn last_ok_path(&self, feed: &FeedId) -> PathBuf {
         self.schedule_dir(feed).join("last_ok.txt")
     }

     // Helper to check existence in tests or future code
     #[allow(dead_code)]
     pub(crate) fn exists(&self, p: &Path) -> bool {
         p.exists()
     }
 }

