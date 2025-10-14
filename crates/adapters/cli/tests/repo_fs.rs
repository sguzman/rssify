/*
Module: rssify_cli::tests::repo_fs
Purpose: Ensure FsPaths placeholder functions link
Public API surface: tests only
Invariants: No filesystem I/O executed
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Test files may exceed header rules; scripts skip /tests/.
*/

#[path = "../src/repo_fs.rs"]
mod repo_fs;

use repo_fs::FsPaths;

#[test]
fn placeholder_paths_link() {
    let _ = FsPaths::feed_dir(
        "/root",
        &rssify_core::FeedId::from_url("https://ex.com/feed"),
    );
}
