/*
Module: rssify_core::test::domain
Purpose: Integration test for small behaviors extracted from core domain types.
Notes: Tests are placed under test/ per AI-FRIENDLY.
*/

use rssify_core::EntryMeta;

#[test]
fn entry_meta_display_prefers_title() {
    let m = EntryMeta {
        url: "https://example.com/x".into(),
        title: Some("Hello".into()),
        published_rfc3339: None,
        source_label: None,
    };
    assert_eq!(m.to_string(), "Hello");
}

