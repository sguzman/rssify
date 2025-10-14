/*
Module: core::scheduler
Purpose: Public surface (re-exports) for scheduling.
Public API: decide(now_sec, &Stats) -> Decision
Notes: Keep logic in submodules; tests live in /tests.
*/
#![forbid(unsafe_code)]

mod decide;
mod types;

pub use decide::decide;
pub use types::{Decision, Stats};
