/*
Module: rssify_core::tests::golden_ids
Purpose: Golden test scaffold for canonical ID policy using fixtures
Public API surface: tests only
Invariants: Golden outputs must be stable across versions
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Marked #[ignore] until fixtures are added.
*/

use rssify_core::{EntryId, FeedId};

fn td(p: &str) -> std::path::PathBuf {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    here.join("tests").join("testdata").join(p)
}

#[test]
#[ignore] // Add fixtures, then remove this ignore
fn golden_entry_id_examples() {
    // Example: read a tiny CSV/JSON fixture with fields (guid, link, title, ts)
    // and assert the computed EntryId strings match a saved snapshot.
    let _fixture = td("entries_min.csv");
    // TODO: parse fixture, compute ids with EntryId::from_parts, compare to snapshot.
    assert!(true);
}
