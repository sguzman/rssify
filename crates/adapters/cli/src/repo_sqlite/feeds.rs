/*
Module: repo_sqlite::feeds
Purpose: FeedRepo impl on SQLite (get, put, list).
Public API: FeedRepo for SqliteRepo
Invariants: id is TEXT key; payload is serde JSON.
Notes: Use OptionalExtension; keep queries explicit.
*/

#![forbid(unsafe_code)]

use crate::repo_sqlite::{Ctx, SqliteRepo, SqliteTx};
use rssify_core::{Feed, FeedId, FeedRepo, RepoError};
use rusqlite::OptionalExtension;
use rusqlite::params;
use serde_json::{from_str as json_from, to_string as json_to};

impl FeedRepo for SqliteRepo {
    type Tx<'a>
        = SqliteTx<'a>
    where
        Self: 'a;

    fn get<'a>(&'a self, tx: Option<&'a Self::Tx<'a>>, id: &FeedId) -> Result<Feed, RepoError> {
        let sql = "SELECT json FROM feeds WHERE id = ?";
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

    fn put<'a>(&'a self, tx: Option<&'a Self::Tx<'a>>, feed: &Feed) -> Result<(), RepoError> {
        let key = feed.id.as_str();
        let json = json_to(feed).map_err(|e| RepoError::Ser(e.to_string()))?;
        let active = if feed.active { 1 } else { 0 };
        let sql = r#"
            INSERT INTO feeds(id, json, active) VALUES(?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET json = excluded.json, active = excluded.active
        "#;
        match self.ctx(tx) {
            Ctx::Tx(t) => t.execute(sql, params![key, json, active]),
            Ctx::Conn(c) => c.execute(sql, params![key, json, active]),
        }
        .map_err(|e| RepoError::Backend(e.to_string()))?;
        Ok(())
    }

    fn list<'a>(&'a self, tx: Option<&'a Self::Tx<'a>>) -> Result<Vec<Feed>, RepoError> {
        let sql = "SELECT json FROM feeds ORDER BY id";
        let mut out = Vec::new();
        match self.ctx(tx) {
            Ctx::Tx(t) => {
                let mut s = t
                    .prepare(sql)
                    .map_err(|e| RepoError::Backend(e.to_string()))?;
                let rows = s
                    .query_map([], |r| r.get::<_, String>(0))
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
                    .query_map([], |r| r.get::<_, String>(0))
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
