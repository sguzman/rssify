//// Contract:
//// Purpose: Define pure types and minimal trait seams for the RSS pipeline.
//// Inputs/Outputs: Pure data structures; no I/O. Traits return Results with domain Error.
//// Invariants: Timestamps are RFC3339 UTC strings; URLs are absolute; no panics.
//// Examples: See doctest in lib.rs.
//// Task: Keep under 300 LOC; split if traits grow.
// (No co-located tests; integration tests live in crates/core/test/)

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Invalid(String),
    NotFound(String),
    Io(String),
    Other(String),
}

impl<E: std::error::Error> From<E> for Error {
    fn from(e: E) -> Self {
        Error::Other(e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntryMeta {
    pub url: String,
    pub title: Option<String>,
    pub published_rfc3339: Option<String>,
    pub source_label: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Item {
    pub url: String,
    pub readable_text: Option<String>,
    pub fingerprint: String,
    pub meta: EntryMeta,
}

pub trait Repository: Send + Sync {
    fn save_item(&self, item: Item) -> Result<(), Error>;
    fn exists(&self, fingerprint: &str) -> Result<bool, Error>;
}

pub trait Fetcher: Send + Sync {
    fn fetch(&self, url: &str) -> Result<Vec<u8>, Error>;
}

pub trait Parser: Send + Sync {
    fn parse_readable(&self, html_bytes: &[u8]) -> Result<String, Error>;
}

pub trait Scheduler: Send + Sync {
    fn next_interval_secs(&self, last_http_status: Option<u16>, saw_new: bool) -> u64;
}

impl fmt::Display for EntryMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title.as_deref().unwrap_or("<untitled>"))
    }
}

