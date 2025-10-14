//! Core domain: pure types and trait seams for adapters.
//!
//! Doctest shows how an adapter would implement `Repository` in-memory.
//!
//! ```
//! use rssify_core::{Item, Repository};
//! struct MemRepo { items: std::sync::Mutex<Vec<Item>> }
//! impl Repository for MemRepo {
//!     fn save_item(&self, item: Item) -> Result<(), rssify_core::Error> {
//!         self.items.lock().unwrap().push(item);
//!         Ok(())
//!     }
//! }
//! ```

pub mod domain;

pub use domain::{EntryMeta, Error, Fetcher, Item, Parser, Repository, Scheduler};
