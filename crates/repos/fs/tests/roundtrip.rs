// File: crates/repos/fs/test/roundtrip.rs
// Purpose: Round-trip tests for FsRepo implementing core repo traits.
// Inputs: FsRepo; rssify_core domain types and traits.
// Outputs: Asserts on RepoError results; creates temp dirs.
// Side effects: Filesystem I/O in a tempdir.
// Invariants:
//  - Tests do not depend on external network.
//  - Each test uses a fresh temp directory.
// Tests: This file.

use rssify_core::{ContentBlob, ContentKind, Entry, EntryId, EntryRepo, Feed, FeedId, FeedRepo, ScheduleRepo, Tx};
use rssify_repo_fs::FsRepo;

fn temp_root() -> tempfile::TempDir {
    tempfile::tempdir().expect("tempdir")
}

#[test]
fn feed_roundtrip_and_list() {
    let root = temp_root();
    let repo = FsRepo::new(root.path());
    let f1 = Feed {
        id: FeedId::from_url("https://example.com/feed"),
        url: "https://example.com/feed".into(),
        title: Some("Example".into()),
        site_url: Some("https://example.com".into()),
        etag: None,
        last_modified: None,
        active: true,
    };
    let f2 = Feed {
        id: FeedId::from_url("https://blog.test/rss"),
        url: "https://blog.test/rss".into(),
        title: None,
        site_url: None,
        etag: Some("W/123".into()),
        last_modified: None,
        active: true,
    };
    let tx = repo.begin_tx();
    repo.put(Some(&tx), &f1).unwrap();
    repo.put(Some(&tx), &f2).unwrap();

    let got = repo.get(None, &f1.id).unwrap();
    assert_eq!(got.url, f1.url);

    let list = repo.list(None).unwrap();
    assert_eq!(list.len(), 2);
}

#[test]
fn entry_roundtrip_and_scan() {
    let root = temp_root();
    let repo = FsRepo::new(root.path());
    let feed = Feed {
        id: FeedId::from_url("https://ex.com/rss"),
        url: "https://ex.com/rss".into(),
        title: None,
        site_url: None,
        etag: None,
        last_modified: None,
        active: true,
    };
    repo.put(None, &feed).unwrap();

    let e1 = Entry {
        id: EntryId::from_parts(&feed.id, Some("guid-1"), None, Some("A"), Some(10)),
        feed: feed.id.clone(),
        url: Some("https://ex.com/a".into()),
        title: Some("A".into()),
        published_ts: Some(10),
        updated_ts: None,
        summary: None,
        content: Some(ContentBlob { kind: ContentKind::Xml, bytes: b"<xml/>".to_vec() }),
    };
    let e2 = Entry {
        id: EntryId::from_parts(&feed.id, None, Some("https://ex.com/b"), Some("B"), Some(20)),
        feed: feed.id.clone(),
        url: Some("https://ex.com/b".into()),
        title: Some("B".into()),
        published_ts: Some(20),
        updated_ts: None,
        summary: None,
        content: None,
    };

    repo.upsert(None, &e1).unwrap();
    repo.upsert(None, &e1).unwrap(); // idempotent
    repo.upsert(None, &e2).unwrap();

    let got = repo.get(None, &e1.id).unwrap();
    assert_eq!(got.title.as_deref(), Some("A"));

    let list = repo.list_by_feed(None, &feed.id).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].id, e1.id);
    assert_eq!(list[1].id, e2.id);
}

#[test]
fn schedule_record_and_read() {
    let root = temp_root();
    let repo = FsRepo::new(root.path());
    let feed = FeedId::from_url("https://ex.com/rss");
    assert_eq!(repo.last_ok_fetch_ts(None, &feed).unwrap(), None);
    repo.record_fetch_ts(None, &feed, 12345).unwrap();
    assert_eq!(repo.last_ok_fetch_ts(None, &feed).unwrap(), Some(12345));
}

