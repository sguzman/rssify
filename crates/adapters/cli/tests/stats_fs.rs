/*
Module: rssify_cli::test::stats_fs
Purpose: Validate stats_fs counts feeds and per-feed entries (and stays compatible with legacy layout)
*/

#[path = "../src/stats.rs"]
mod stats;

use std::fs;
use std::path::PathBuf;

fn td() -> PathBuf {
    let mut p = std::env::temp_dir();
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    p.push(format!("rssify-stats-{}-{}", pid, nanos));
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn counts_per_feed_entries_and_feeds() {
    let root = td();

    // feed A with 2 entries
    let a = root.join("feeds").join("url%3Ahttps%3A%2F%2Fsite%2Fa");
    fs::create_dir_all(a.join("entries")).unwrap();
    fs::write(a.join("feed.json"), "{}").unwrap();
    fs::write(a.join("entries").join("e1.json"), "{}").unwrap();
    fs::write(a.join("entries").join("e2.json"), "{}").unwrap();

    // feed B with 1 entry
    let b = root.join("feeds").join("guid%3AFEED%2002");
    fs::create_dir_all(b.join("entries")).unwrap();
    fs::write(b.join("feed.json"), "{}").unwrap();
    fs::write(b.join("entries").join("x.json"), "{}").unwrap();

    // legacy flat entries directory with 1 file (should also be counted)
    let legacy = root.join("entries").join("by_id");
    fs::create_dir_all(&legacy).unwrap();
    fs::write(legacy.join("legacy.json"), "{}").unwrap();

    let s = stats::stats_fs(root.to_string_lossy().as_ref()).expect("stats");
    assert_eq!(s.feeds, 2, "should count feeds that have feed.json");
    assert_eq!(s.entries, 4, "3 per-feed + 1 legacy");
}
