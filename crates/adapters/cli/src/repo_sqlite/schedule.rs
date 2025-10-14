#![forbid(unsafe_code)]

use crate::repo_sqlite::{Ctx, SqliteRepo, SqliteTx};
use rssify_core::{FeedId, RepoError, ScheduleRepo};
use rusqlite::params;

impl ScheduleRepo for SqliteRepo {
    type Tx<'a>
        = SqliteTx<'a>
    where
        Self: 'a;

    fn last_ok_fetch_ts<'a>(
        &'a self,
        tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
    ) -> Result<Option<i64>, RepoError> {
        let sql = "SELECT last_ok_fetch_ts FROM schedule WHERE feed_id = ?";
        let res: Option<Option<i64>> = match self.ctx(tx) {
            Ctx::Tx(t) => t
                .query_row(sql, params![feed.as_str()], |r| r.get::<_, Option<i64>>(0))
                .optional(),
            Ctx::Conn(c) => c
                .query_row(sql, params![feed.as_str()], |r| r.get::<_, Option<i64>>(0))
                .optional(),
        }
        .map_err(|e| RepoError::Backend(e.to_string()))?;
        Ok(res.flatten())
    }

    fn record_fetch_ts<'a>(
        &'a self,
        tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
        ts: i64,
    ) -> Result<(), RepoError> {
        let sql = r#"
            INSERT INTO schedule(feed_id, last_ok_fetch_ts) VALUES(?, ?)
            ON CONFLICT(feed_id) DO UPDATE SET last_ok_fetch_ts = excluded.last_ok_fetch_ts
        "#;
        match self.ctx(tx) {
            Ctx::Tx(t) => t.execute(sql, params![feed.as_str(), ts]),
            Ctx::Conn(c) => c.execute(sql, params![feed.as_str(), ts]),
        }
        .map_err(|e| RepoError::Backend(e.to_string()))?;
        Ok(())
    }
}
