#![forbid(unsafe_code)]

use rssify_core::RepoError;
use rusqlite::Connection;

pub(crate) fn init_schema(conn: &Connection) -> Result<(), RepoError> {
    conn.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS feeds(
          id     TEXT PRIMARY KEY,
          json   TEXT NOT NULL,
          active INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS entries(
          id      TEXT PRIMARY KEY,
          feed_id TEXT NOT NULL,
          json    TEXT NOT NULL,
          FOREIGN KEY(feed_id) REFERENCES feeds(id) ON DELETE CASCADE
        );
        CREATE INDEX IF NOT EXISTS idx_entries_feed ON entries(feed_id);

        CREATE TABLE IF NOT EXISTS schedule(
          feed_id TEXT PRIMARY KEY,
          last_ok_fetch_ts INTEGER
        );
        "#,
    )
    .map_err(|e| RepoError::Backend(e.to_string()))?;
    Ok(())
}
