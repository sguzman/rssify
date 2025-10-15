/*
File: crates/repos/fs/test/roundtrip.rs
Purpose: Round-trip tests for FsRepo implementing core repo traits.
Inputs: FsRepo; rssify_core domain types and traits.
Outputs: Asserts on RepoError results; creates temp dirs.
Side effects: Filesystem I/O in a tempdir.
Invariants:
 - Tests do not depend on external network.
 - Each test uses a fresh temp directory.
*/

use rssify_core::{
    ContentBlob, ContentKind, Entry, EntryId, EntryRepo, Feed, FeedId, FeedRepo, ScheduleRepo,
};
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
    FeedRepo::put(&repo, Some(&tx), &f1).expect("put f1");
    FeedRepo::put(&repo, Some(&tx), &f2).expect("put f2");

    let got = FeedRepo::get(&repo, None, &f1.id).expect("get f1");
    assert_eq!(got.url, f1.url);

    let list = FeedRepo::list(&repo, None).expect("list feeds");
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
    FeedRepo::put(&repo, None, &feed).expect("put feed");

    let e1 = Entry {
        id: EntryId::from_parts(&feed.id, Some("guid-1"), None, Some("A"), Some(10)),
        feed: feed.id.clone(),
        url: Some("https://ex.com/a".into()),
        title: Some("A".into()),
        published_ts: Some(10),
        updated_ts: None,
        summary: None,
        content: Some(ContentBlob {
            kind: ContentKind::Xml,
            bytes: b"<xml/>".to_vec(),
        }),
    };
    let e2 = Entry {
        id: EntryId::from_parts(
            &feed.id,
            None,
            Some("https://ex.com/b"),
            Some("B"),
            Some(20),
        ),
        feed: feed.id.clone(),
        url: Some("https://ex.com/b".into()),
        title: Some("B".into()),
        published_ts: Some(20),
        updated_ts: None,
        summary: None,
        content: None,
    };

    EntryRepo::upsert(&repo, None, &e1).expect("upsert e1");
    EntryRepo::upsert(&repo, None, &e1).expect("upsert e1 again (idempotent)");
    EntryRepo::upsert(&repo, None, &e2).expect("upsert e2");

    let got = EntryRepo::get(&repo, None, &e1.id).expect("get e1");
    assert_eq!(got.title.as_deref(), Some("A"));

    let list = EntryRepo::list_by_feed(&repo, None, &feed.id).expect("list entries by feed");
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].id, e1.id);
    assert_eq!(list[1].id, e2.id);
}

#[test]
fn schedule_record_and_read() {
    let root = temp_root();
    let repo = FsRepo::new(root.path());
    let feed = FeedId::from_url("https://ex.com/rss");
    assert_eq!(
        ScheduleRepo::last_ok_fetch_ts(&repo, None, &feed).expect("none yet"),
        None
    );
    ScheduleRepo::record_fetch_ts(&repo, None, &feed, 12345).expect("record ts");
    assert_eq!(
        ScheduleRepo::last_ok_fetch_ts(&repo, None, &feed).expect("read ts"),
        Some(12345)
    );
}

