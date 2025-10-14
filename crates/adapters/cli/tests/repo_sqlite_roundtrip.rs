/*
Module: tests::repo_sqlite_roundtrip
Purpose: Integration tests for SQLite adapter public surface.
Scope: feed roundtrip, entry upsert, schedule ts, tx visibility.
Notes: Uses :memory: DSN; no fixtures on disk.
*/

#![forbid(unsafe_code)]

use rssify_cli::repo_sqlite::SqliteRepo;
use rssify_core::{
    ContentBlob, ContentKind, Entry, EntryId, EntryRepo, Feed, FeedId, FeedRepo, ScheduleRepo, Tx,
}; // re-export mod path as public in your crate root

#[test]
fn feed_roundtrip_put_get_list() {
    let mut r = SqliteRepo::open(":memory:").unwrap();
    let mut tx = r.begin_tx().unwrap();

    let f = Feed {
        id: FeedId::from_url("https://ex.com/feed"),
        url: "https://ex.com/feed".into(),
        title: Some("Example".into()),
        site_url: Some("https://ex.com".into()),
        etag: Some("W/123".into()),
        last_modified: None,
        active: true,
    };
    r.put(None, &f).unwrap();
    let out = r.get(None, &f.id).unwrap();
    assert_eq!(out.id.as_str(), f.id.as_str());
    assert_eq!(out.title, Some("Example".into()));
    assert!(!r.list(None).unwrap().is_empty());
}

#[test]
fn entry_roundtrip_upsert_get_list() {
    let r = SqliteRepo::open(":memory:").unwrap();
    let feed = FeedId::from_url("https://ex.com/feed");
    r.put(
        None,
        &Feed {
            id: feed.clone(),
            url: "https://ex.com/feed".into(),
            title: None,
            site_url: None,
            etag: None,
            last_modified: None,
            active: true,
        },
    )
    .unwrap();

    let e = Entry {
        id: EntryId::new("guid:abc"),
        feed: feed.clone(),
        url: Some("https://ex.com/1".into()),
        title: Some("hello".into()),
        published_ts: Some(1_700_000_000),
        updated_ts: None,
        summary: Some("s".into()),
        content: Some(ContentBlob {
            kind: ContentKind::Xml,
            bytes: b"<x/>".to_vec(),
        }),
    };
    r.upsert(None, &e).unwrap();

    let out = r.get(None, &e.id).unwrap();
    assert_eq!(out.id.as_str(), e.id.as_str());
    assert_eq!(out.feed.as_str(), feed.as_str());
    assert_eq!(r.list_by_feed(None, &feed).unwrap().len(), 1);
}

#[test]
fn schedule_roundtrip_and_tx() {
    let r = SqliteRepo::open(":memory:").unwrap();
    let f = Feed {
        id: FeedId::from_url("https://tx.test"),
        url: "https://tx.test".into(),
        title: None,
        site_url: None,
        etag: None,
        last_modified: None,
        active: true,
    };
    let mut tx = r.begin_tx().unwrap();
    r.put(Some(&tx), &f).unwrap();
    tx.commit().unwrap();
    assert!(r.get(None, &f.id).is_ok());

    let feed = FeedId::from_url("https://ex.com/feed");
    assert_eq!(r.last_ok_fetch_ts(None, &feed).unwrap(), None);
    r.record_fetch_ts(None, &feed, 1_700_000_123).unwrap();
    assert_eq!(
        r.last_ok_fetch_ts(None, &feed).unwrap(),
        Some(1_700_000_123)
    );
}
