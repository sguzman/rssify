// File: crates/repos/fs/src/feed_impl.rs
// Purpose: FeedRepo impl for FsRepo.
// Inputs: rssify_core FeedRepo trait and model types.
// Outputs: JSON files per feed.
// Side effects: Filesystem I/O.

use crate::repo::FsRepo;
use crate::util::{read_json, write_atomic_json};
use rssify_core::{Feed, FeedId, FeedRepo, RepoError};

impl FeedRepo for FsRepo {
    type Tx<'a> = crate::tx::FsTx where Self: 'a;

    fn get<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        id: &FeedId,
    ) -> Result<Feed, RepoError> {
        let path = self.feed_path(id);
        read_json::<Feed>(&path)
    }

    fn put<'a>(&'a self, _tx: Option<&'a Self::Tx<'a>>, feed: &Feed) -> Result<(), RepoError> {
        let path = self.feed_path(&feed.id);
        write_atomic_json(&path, feed)
    }

    fn list<'a>(&'a self, _tx: Option<&'a Self::Tx<'a>>) -> Result<Vec<Feed>, RepoError> {
        let mut out = Vec::new();
        let feeds_dir = self.feeds_dir();
        let rd = match std::fs::read_dir(&feeds_dir) {
            Ok(rd) => rd,
            Err(_) => return Ok(out),
        };
        for entry in rd {
            let entry = entry.map_err(|e| RepoError::Backend(e.to_string()))?;
            let p = entry.path().join("feed.json");
            if p.is_file() {
                if let Ok(feed) = read_json::<Feed>(&p) {
                    out.push(feed);
                }
            }
        }
        Ok(out)
    }
}

