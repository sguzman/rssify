/*
Module: rssify_cli::test::repo_fs
Purpose: Ensure FsPaths placeholder functions link
*/

#[path = "../src/repo_fs.rs"]
mod repo_fs;

use repo_fs::FsPaths;

#[test]
fn placeholder_paths_link() {
    let _ = FsPaths::feed_dir("/root", &rssify_core::FeedId::from_url("https://ex.com/feed"));
}

