/*
Module: rssify_cli::tests::pipeline_fetch
Purpose: Validate load_feed_seeds() and fetch_from_file() without network or writes.
*/

#[path = "../src/pipeline.rs"]
mod pipeline;

use pipeline::{fetch_from_file, load_feed_seeds, FetchSummary};
use std::fs;
use std::path::PathBuf;

fn write_temp(contents: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    // Use PID-based filename to avoid collisions in parallel runs.
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    p.push(format!("rssify-seeds-{}-{}.json", pid, nanos));
    fs::write(&p, contents).expect("write seeds json");
    p
}

#[test]
fn load_simple_string_array() {
    let p = write_temp(r#"[
        "https://example.com/a.xml",
        "guid:FEED 01"
    ]"#);

    let seeds = load_feed_seeds(&p).expect("load seeds");
    assert_eq!(seeds.len(), 2);
    // URL gets normalized via FeedId::from_url; literal stays as-is
    assert!(seeds[0].as_str().contains("example.com"));
    assert_eq!(seeds[1].as_str(), "guid:FEED 01");
}

#[test]
fn load_object_array_with_url_and_id() {
    let p = write_temp(r#"[
        {"url": "https://example.org/x"},
        {"id": "custom-id"},
        {"url": "https://site/y", "id": "override-id"}
    ]"#);

    let seeds = load_feed_seeds(&p).expect("load seeds");
    assert_eq!(seeds.len(), 3);
    assert!(seeds[0].as_str().contains("example.org"));
    assert_eq!(seeds[1].as_str(), "custom-id");
    assert_eq!(seeds[2].as_str(), "override-id");
}

#[test]
fn fetch_summary_matches_seed_count() {
    let p = write_temp(r#"[
        "https://a.test/feed",
        "https://b.test/feed",
        "guid:LOCAL"
    ]"#);

    let summary: FetchSummary = fetch_from_file(&p).expect("fetch");
    assert_eq!(summary.feeds_total, 3);
    assert_eq!(summary.feeds_processed, 3);
    assert_eq!(summary.items_parsed, 3);
    assert_eq!(summary.items_written, 3);
}

