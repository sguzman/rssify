/*
Module: rssify_core::tests::ids
Purpose: Minimal tests to lock ID policy; golden fixtures to be added later
Public API surface: tests only
Invariants: Deterministic and precedence-respecting identifiers
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Test files may exceed header rules; scripts skip /tests/.
*/

use rssify_core::{EntryId, FeedId};

#[test]
fn feed_id_from_url_is_prefixed_and_trimmed() {
    let f = FeedId::from_url("  https://example.com/feed  ");
    assert_eq!(f.as_str(), "url:https://example.com/feed");
}

#[test]
fn entry_id_prefers_guid() {
    let feed = FeedId::from_url("https://ex.com/feed");
    let e = EntryId::from_parts(
        &feed,
        Some("G-123"),
        Some("https://ex.com/p"),
        Some("t"),
        Some(1),
    );
    assert_eq!(e.as_str(), "guid:G-123");
}

#[test]
fn entry_id_falls_back_to_link() {
    let feed = FeedId::from_url("https://ex.com/feed");
    let e = EntryId::from_parts(&feed, None, Some("https://ex.com/p"), Some("t"), Some(1));
    assert_eq!(e.as_str(), "link:https://ex.com/p");
}

#[test]
fn entry_id_hash_is_deterministic_without_guid_or_link() {
    let feed = FeedId::from_url("https://ex.com/feed");
    let e1 = EntryId::from_parts(&feed, None, None, Some("Title"), Some(1_700_000_000));
    let e2 = EntryId::from_parts(&feed, None, None, Some("Title"), Some(1_700_000_000));
    assert_eq!(e1, e2);
}

#[test]
fn entry_id_hash_changes_when_inputs_change() {
    let feed = FeedId::from_url("https://ex.com/feed");
    let a = EntryId::from_parts(&feed, None, None, Some("Title A"), Some(1));
    let b = EntryId::from_parts(&feed, None, None, Some("Title B"), Some(1));
    assert_ne!(a, b);
}

// Placeholder for future golden tests using testdata fixtures.
// #[test]
// #[ignore]
// fn golden_examples_match_expected_snapshots() {
//     // Read fixtures from tests/testdata/ and compare against saved snapshots.
// }
