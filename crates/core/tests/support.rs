/*
Module: rssify_core::tests::support
Purpose: Tiny helpers for reading fixtures (no extra deps)
Public API surface: tests only
Invariants: Only used within tests
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep tiny and dependency-free.
*/

use std::fs;

pub fn read(path: &std::path::Path) -> String {
    fs::read_to_string(path).expect("read fixture")
}
