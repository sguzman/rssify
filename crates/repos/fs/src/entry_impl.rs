// File: crates/repos/fs/src/entry_impl.rs
// Purpose: EntryRepo impl for FsRepo.
// Inputs: rssify_core EntryRepo trait and model types.
// Outputs: JSON per entry with by-id and by-feed indexing.
// Side effects: Filesystem I/O.

use crate::repo::FsRepo;
use crate::util::{read_json, write_atomic_json};
use rssify_core::{Entry, EntryId, EntryRepo, FeedId, RepoError};

impl EntryRepo for FsRepo {
    type Tx<'a> = crate::tx::FsTx where Self: 'a;

    fn get<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        id: &EntryId,
    ) -> Result<Entry, RepoError> {
        let p = self.entry_by_id_path(id);
        read_json::<Entry>(&p)
    }

    fn upsert<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        entry: &Entry,
    ) -> Result<(), RepoError> {
        let by_id = self.entry_by_id_path(&entry.id);
        let by_feed = self.entry_by_feed_path(&entry.feed, &entry.id);
        write_atomic_json(&by_id, entry)?;
        write_atomic_json(&by_feed, entry)?;
        Ok(())
    }

    fn list_by_feed<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
    ) -> Result<Vec<Entry>, RepoError> {
        let dir = self.entries_by_feed_dir(feed);
        let mut out = Vec::new();
        let rd = match std::fs::read_dir(&dir) {
            Ok(rd) => rd,
            Err(_) => return Ok(out),
        };
        for entry in rd {
            let entry = entry.map_err(|e| RepoError::Backend(e.to_string()))?;
            let p = entry.path();
            if p.extension().and_then(|s| s.to_str()) == Some("json") && p.is_file() {
                if let Ok(e) = read_json::<Entry>(&p) {
                    out.push(e);
                }
            }
        }
        out.sort_by(|a, b| {
            a.published_ts
                .cmp(&b.published_ts)
                .then(a.updated_ts.cmp(&b.updated_ts))
                .then(a.id.as_str().cmp(b.id.as_str()))
        });
        Ok(out)
    }
}

