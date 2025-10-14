/*
Module: rssify_cli::tests::pipeline
Purpose: Ensure the pipeline skeleton types exist and are public
Public API surface: tests only
Invariants: Shapes compile; no logic is executed
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Test files may exceed header rules; scripts skip /tests/.
*/

#[path = "../src/pipeline.rs"]
mod pipeline;

use pipeline::{FeedMetaDelta, FeedSeed, PersistStats};

#[test]
fn pipeline_types_are_constructible() {
    let _a = FeedSeed {
        url: "https://ex.com/feed".into(),
        title_hint: None,
    };
    let _b = FeedMetaDelta {
        title: None,
        site_url: None,
        etag: None,
        last_modified: None,
    };
    let _c = PersistStats {
        feed: rssify_core::FeedId::from_url("https://ex.com/feed"),
        items_written: 0,
        elapsed_ms: 0,
        not_modified: false,
        failure_hint: None,
    };
}
