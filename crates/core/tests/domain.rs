/*
Module: rssify_core::test::domain
Purpose: Sanity-check core public model types (no I/O).
Notes: Tests live under test/ per AI-FRIENDLY.
*/

use rssify_core::{Feed, FeedId};

#[test]
fn feed_is_constructible_with_expected_fields() {
    let id = FeedId::from_url("https://example.com/feed");
    let f = Feed {
        id: id.clone(),
        url: "https://example.com/feed".into(),
        title: Some("Example".into()),
        site_url: Some("https://example.com".into()),
        etag: None,
        last_modified: None,
        active: true,
    };

    assert_eq!(f.id.as_str(), id.as_str());
    assert_eq!(f.title.as_deref(), Some("Example"));
    assert!(f.active);
}

