/*
File: crates/repos/fs/src/schedule_impl.rs
Purpose: ScheduleRepo impl for FsRepo (tiny "last ok" timestamp file).
Inputs: rssify_core::{FeedId, RepoError, ScheduleRepo}; std::fs for I/O.
Outputs: last_ok.txt per feed; i64 unix seconds.
Side effects: Filesystem I/O.
*/

use crate::repo::FsRepo;
use rssify_core::{FeedId, RepoError, ScheduleRepo};
use std::fs;
use std::io::Write;

impl ScheduleRepo for FsRepo {
    type Tx<'a> = crate::tx::FsTx where Self: 'a;

    fn last_ok_fetch_ts<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
    ) -> Result<Option<i64>, RepoError> {
        let p = self.schedule_last_ok_path(feed);
        if !p.exists() {
            return Ok(None);
        }
        let s = fs::read_to_string(&p).map_err(|e| RepoError::Backend(e.to_string()))?;
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }
        let ts: i64 = trimmed.parse().map_err(|e| RepoError::Backend(e.to_string()))?;
        Ok(Some(ts))
    }

    fn record_fetch_ts<'a>(
        &'a self,
        _tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
        ts: i64,
    ) -> Result<(), RepoError> {
        let p = self.schedule_last_ok_path(feed);
        if let Some(dir) = p.parent() {
            fs::create_dir_all(dir).map_err(|e| RepoError::Backend(e.to_string()))?;
        }
        let mut f = fs::File::create(&p).map_err(|e| RepoError::Backend(e.to_string()))?;
        write!(f, "{ts}\n").map_err(|e| RepoError::Backend(e.to_string()))?;
        f.sync_all().map_err(|e| RepoError::Backend(e.to_string()))?;
        Ok(())
    }
}

