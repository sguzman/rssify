#![forbid(unsafe_code)]

use crate::repo_sqlite::schema::init_schema;
use rssify_core::{RepoError, Tx};
use rusqlite::{Connection, Transaction};

pub mod entries;
pub mod feeds;
pub mod schedule;
pub mod schema;

/// Public entry for the SQLite-backed repository adapter.
/// Keep this as the tiny orchestration layer.
pub struct SqliteRepo {
    pub(crate) conn: Connection,
}

impl SqliteRepo {
    /// Open a repository using a DSN/path (":memory:", "file::memory:?cache=shared", "/path.db").
    pub fn open(dsn: &str) -> Result<Self, RepoError> {
        let conn = Connection::open(dsn).map_err(|e| RepoError::Backend(e.to_string()))?;
        init_schema(&conn)?;
        Ok(Self { conn })
    }

    /// Begin a transaction.
    pub fn begin_tx(&mut self) -> Result<SqliteTx<'_>, RepoError> {
        let tx = self
            .conn
            .transaction()
            .map_err(|e| RepoError::Backend(e.to_string()))?;
        Ok(SqliteTx { tx, active: true })
    }
}

/// A thin Tx wrapper so we do not leak rusqlite types through the trait boundary.
pub struct SqliteTx<'a> {
    pub(crate) tx: Transaction<'a>,
    pub(crate) active: bool,
}

impl Tx for SqliteTx<'_> {
    fn is_active(&self) -> bool {
        self.active
    }
}

impl<'a> SqliteTx<'a> {
    pub fn commit(mut self) -> Result<(), RepoError> {
        self.active = false;
        self.tx
            .commit()
            .map_err(|e| RepoError::Backend(e.to_string()))
    }
}

/// Internal context that abstracts over Connection vs Transaction.
pub(crate) enum Ctx<'a> {
    Tx(&'a Transaction<'a>),
    Conn(&'a Connection),
}

impl SqliteRepo {
    pub(crate) fn ctx<'a>(&'a self, tx: Option<&'a SqliteTx<'a>>) -> Ctx<'a> {
        match tx {
            Some(t) => Ctx::Tx(&t.tx),
            None => Ctx::Conn(&self.conn),
        }
    }
}
