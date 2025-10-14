#![forbid(unsafe_code)]

use crate::repo_sqlite::{Ctx, SqliteRepo, SqliteTx};
use rssify_core::{Entry, EntryId, EntryRepo, Feed, FeedId, RepoError};
use rusqlite::params;
use rusqlite::OptionalExtension;
use serde_json::{from_str as json_from, to_string as json_to};

impl EntryRepo for SqliteRepo {
    type Tx<'a>
        = SqliteTx<'a>
    where
        Self: 'a;

    fn get<'a>(&'a self, tx: Option<&'a Self::Tx<'a>>, id: &EntryId) -> Result<Entry, RepoError> {
        let sql = "SELECT json FROM entries WHERE id = ?";
        let key = id.as_str();
        let res = match self.ctx(tx) {
            Ctx::Tx(t) => t
                .query_row(sql, params![key], |r| r.get::<_, String>(0))
                .optional(),
            Ctx::Conn(c) => c
                .query_row(sql, params![key], |r| r.get::<_, String>(0))
                .optional(),
        }
        .map_err(|e| RepoError::Backend(e.to_string()))?;
        let Some(json) = res else {
            return Err(RepoError::NotFound);
        };
        json_from(&json).map_err(|e| RepoError::Ser(e.to_string()))
    }

    fn upsert<'a>(&'a self, tx: Option<&'a Self::Tx<'a>>, entry: &Entry) -> Result<(), RepoError> {
        // Ensure feed exists minimally (FK will enforce as well).
        let minimal_feed = Feed {
            id: entry.feed.clone(),
            url: String::new(),
            title: None,
            site_url: None,
            etag: None,
            last_modified: None,
            active: true,
        };
        let feed_json = json_to(&minimal_feed).map_err(|e| RepoError::Ser(e.to_string()))?;
        let ensure_feed_sql = r#"
            INSERT INTO feeds(id, json, active) VALUES(?, ?, 1)
            ON CONFLICT(id) DO NOTHING
        "#;
        match self.ctx(tx) {
            Ctx::Tx(t) => t.execute(ensure_feed_sql, params![entry.feed.as_str(), feed_json]),
            Ctx::Conn(c) => c.execute(ensure_feed_sql, params![entry.feed.as_str(), feed_json]),
        }
        .map_err(|e| RepoError::Backend(e.to_string()))?;

        let key = entry.id.as_str();
        let json = json_to(entry).map_err(|e| RepoError::Ser(e.to_string()))?;
        let sql = r#"
            INSERT INTO entries(id, feed_id, json) VALUES(?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET json = excluded.json, feed_id = excluded.feed_id
        "#;
        match self.ctx(tx) {
            Ctx::Tx(t) => t.execute(sql, params![key, entry.feed.as_str(), json]),
            Ctx::Conn(c) => c.execute(sql, params![key, entry.feed.as_str(), json]),
        }
        .map_err(|e| RepoError::Backend(e.to_string()))?;
        Ok(())
    }

    fn list_by_feed<'a>(
        &'a self,
        tx: Option<&'a Self::Tx<'a>>,
        feed: &FeedId,
    ) -> Result<Vec<Entry>, RepoError> {
        let sql = "SELECT json FROM entries WHERE feed_id = ? ORDER BY id";
        let mut out = Vec::new();
        match self.ctx(tx) {
            Ctx::Tx(t) => {
                let mut s = t
                    .prepare(sql)
                    .map_err(|e| RepoError::Backend(e.to_string()))?;
                let rows = s
                    .query_map(params![feed.as_str()], |r| r.get::<_, String>(0))
                    .map_err(|e| RepoError::Backend(e.to_string()))?;
                for r in rows {
                    let s = r.map_err(|e| RepoError::Backend(e.to_string()))?;
                    out.push(json_from(&s).map_err(|e| RepoError::Ser(e.to_string()))?);
                }
            }
            Ctx::Conn(c) => {
                let mut s = c
                    .prepare(sql)
                    .map_err(|e| RepoError::Backend(e.to_string()))?;
                let rows = s
                    .query_map(params![feed.as_str()], |r| r.get::<_, String>(0))
                    .map_err(|e| RepoError::Backend(e.to_string()))?;
                for r in rows {
                    let s = r.map_err(|e| RepoError::Backend(e.to_string()))?;
                    out.push(json_from(&s).map_err(|e| RepoError::Ser(e.to_string()))?);
                }
            }
        }
        Ok(out)
    }
}
