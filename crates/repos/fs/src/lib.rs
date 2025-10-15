//// File: crates/repos/fs/src/lib.rs
//// Role: Module hub and public surface for the filesystem repo adapter.

pub mod repo;
pub mod util;

pub use crate::repo::FsRepo;
pub use crate::util::{escape_id, unescape_id, write_json_atomic, write_text_atomic};

