#![forbid(unsafe_code)]
use rssify_cli::repo_selector::{RepoKind, open_from_spec};

#[test]
fn selects_fs_by_default() {
    let repo = open_from_spec("fs:./tmp-data").unwrap();
    match repo {
        RepoKind::Fs(_) => {}
        _ => panic!("expected fs"),
    }
}

#[test]
fn selects_sqlite_memory() {
    let repo = open_from_spec("sqlite::memory:").unwrap();
    match repo {
        RepoKind::Sqlite(_) => {}
        _ => panic!("expected sqlite"),
    }
}
