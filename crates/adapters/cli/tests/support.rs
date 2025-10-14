/*
Module: rssify_cli::tests::support
Purpose: Small helpers for tests (path building, temp dirs later)
Public API surface: tests only
Invariants: No external dependencies
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep tiny and dependency-free.
*/

pub fn td(p: &str) -> std::path::PathBuf {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    here.join("tests").join("testdata").join(p)
}
