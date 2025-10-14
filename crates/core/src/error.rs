/*
Module: rssify_core::error
Purpose: Boundary error enums for core and repositories
Public API surface: CoreError, RepoError
Invariants: Errors are small, typed, and thiserror-based for adapters
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

use thiserror::Error;

/// Errors that can be raised by pure core logic (no I/O).
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("invalid argument: {0}")]
    InvalidArgument(&'static str),

    #[error("invariant violation: {0}")]
    Invariant(&'static str),

    #[error("not found")]
    NotFound,
}

/// Errors that occur at repository boundaries/adapters.
#[derive(Debug, Error)]
pub enum RepoError {
    #[error("conflict")]
    Conflict,

    #[error("not found")]
    NotFound,

    #[error("serialization failure: {0}")]
    Ser(String),

    #[error("backend failure: {0}")]
    Backend(String),
}
