/*
Module: rssify_cli::test::repo_fs_path
Purpose: Validate FsPaths builders and urlsafe encoding shapes (no I/O)
*/

#[path = "../src/repo_fs.rs"]
mod repo_fs;

use repo_fs::FsPaths;
use std::path::Path;

#[test]
fn feed_dir_and_files_layout() {
    let root = "./data";
    let feed = rssify_core::FeedId::from_url("https://example.com/feed");
    let dir = FsPaths::feed_dir(root, &feed);
    let json = FsPaths::feed_json(root, &feed);
    let blob = FsPaths::last_blob(root, &feed);

    assert!(dir.ends_with("feeds/url%3Ahttps%3A%2F%2Fexample.com%2Ffeed"));
    assert!(json.ends_with("feeds/url%3Ahttps%3A%2F%2Fexample.com%2Ffeed/feed.json"));
    assert!(blob.ends_with("feeds/url%3Ahttps%3A%2F%2Fexample.com%2Ffeed/last_blob.bin"));

    assert_eq!(Path::new(&json).parent().unwrap().to_string_lossy(), dir);
    assert_eq!(Path::new(&blob).parent().unwrap().to_string_lossy(), dir);
}

#[test]
fn entry_json_layout() {
    let root = "/var/lib/rssify";
    let feed = rssify_core::FeedId::new("guid:FEED 01");
    let entry = rssify_core::EntryId::new("guid:ABC 123");

    let p = FsPaths::entry_json(root, &feed, &entry);

    assert!(p.ends_with("feeds/guid%3AFEED%2001/entries/guid%3AABC%20123.json"));
    assert!(p.contains("/feeds/"));
    assert!(p.contains("/entries/"));
}

