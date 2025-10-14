/*
Module: rssify_cli::tests::spec
Purpose: Validate repo spec parsing (no I/O, no backend behavior)
Public API surface: tests only
Invariants: Spec is stable and rejects malformed inputs
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Test files may exceed header rules; scripts skip /tests/.
*/

#[path = "../src/spec.rs"]
mod spec;

use spec::{RepoKind, RepoSpec};
use std::str::FromStr;

#[test]
fn parses_fs_and_sqlite_specs() {
    let fs = RepoSpec::from_str("fs:/var/lib/rssify").unwrap();
    assert_eq!(fs.kind, RepoKind::Fs);
    assert_eq!(fs.target, "/var/lib/rssify");

    let fs_rel = RepoSpec::from_str("Fs:./data").unwrap(); // case-insensitive
    assert_eq!(fs_rel.kind, RepoKind::Fs);
    assert_eq!(fs_rel.target, "./data");

    let db = RepoSpec::from_str("sqlite:/var/lib/rssify/db.sqlite").unwrap();
    assert_eq!(db.kind, RepoKind::Sqlite);
    assert_eq!(db.target, "/var/lib/rssify/db.sqlite");
}

#[test]
fn rejects_bad_specs() {
    assert!(RepoSpec::from_str("unknown:/x").is_err());
    assert!(RepoSpec::from_str("fs:").is_err());
    assert!(RepoSpec::from_str("sqlite").is_err());
    assert!(RepoSpec::from_str("nocolon").is_err());
}
