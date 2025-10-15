/* 
File: crates/repos/fs/src/lib.rs
Purpose: Module glue and public re-exports for the filesystem repository adapter.
Inputs: rssify_core traits and types.
Outputs: Public FsRepo and FsTx types; trait impls are in submodules.
Side effects: None here.
Invariants:
 - Keep this file minimal and under 200 LOC.
 - All heavy logic lives in dedicated modules.
*/

mod tx;
mod util;
mod repo;
mod feed_impl;
mod entry_impl;
mod schedule_impl;

pub use repo::FsRepo;
pub use tx::FsTx;

