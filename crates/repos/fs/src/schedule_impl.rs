// File: crates/repos/fs/src/schedule_impl.rs
// Purpose: ScheduleRepo impl for FsRepo.
// Inputs: rssify_core ScheduleRepo trait.
// Outputs: last_ok fetch timestamps per feed.
// Side effects: Filesystem I/O.

use crate::repo::FsRepo;
use crate::util::{read_json, write_atomic_text};
use rssify_core::{FeedId, RepoError, ScheduleRepo};
use std::fs::File;
use std::io::Read;

impl ScheduleRepo for FsRepo {
    type Tx<'a> = crate::tx::FsTx where Self: 'a;

    fn last_ok_fetch_ts<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
    ) -> Result<Option<i64>, RepoError> {
        let p = self.last_ok_path(feed);
        if !p.exists() {
            return Ok(None);
        }
        let mut s = String::new();
        File::open(&p)
            .map_err(|e| RepoError::Backend(e.to_string()))?
            .read_to_string(&mut s)
            .map_err(|e| RepoError::Backend(e.to_string()))?;
        let ts = s.trim().parse::<i64>().map_err(|e| RepoError::Ser(e.to_string()))?;
        Ok(Some(ts))
    }

    fn record_fetch_ts<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
        ts: i64,
    ) -> Result<(), RepoError> {
        let p = self.last_ok_path(feed);
        write_atomic_text(&p, &format!("{ts}"))
    }
}

