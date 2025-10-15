/*
Module: rssify_core::lib
Purpose: Crate root; exposes core domain types/traits without any I/O
Public API surface: pub use ids::*, model::*, error::*, repo::*, sched::*
Invariants: Core remains pure; all side effects live in adapters/repos
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod error;
pub mod ids;
pub mod model;
pub mod repo;
pub mod sched;

pub use error::*;
pub use ids::*;
pub use model::*;
pub use repo::*;
pub use sched::*;

