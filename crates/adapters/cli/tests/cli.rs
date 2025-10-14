/*
Module: rssify_cli::tests::cli
Purpose: Validate CLI parsing for subcommands and flags (no business logic)
Public API surface: tests only
Invariants: Adapter parses args; core handles logic in later steps
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Test files may exceed header rules; scripts skip /tests/.
*/

#[path = "../src/main.rs"]
mod bin_main;

use bin_main::{Command, parse_from};

#[test]
fn parses_fetch_with_flags() {
    let cli = parse_from([
        "rssify",
        "fetch",
        "--from",
        "feeds.json",
        "--store",
        "fs:/tmp",
        "--json",
        "-vv",
    ]);
    match cli.command {
        Command::Fetch {
            from,
            store,
            json,
            verbose,
        } => {
            assert_eq!(from.as_deref(), Some("feeds.json"));
            assert_eq!(store.as_deref(), Some("fs:/tmp"));
            assert!(json);
            assert!(verbose >= 2);
        }
        _ => panic!("expected fetch"),
    }
}

#[test]
fn parses_stats_minimal() {
    let cli = parse_from(["rssify", "stats"]);
    match cli.command {
        Command::Stats { json, .. } => assert!(!json),
        _ => panic!("expected stats"),
    }
}

#[test]
fn parses_import_and_add() {
    let cli = parse_from([
        "rssify",
        "import",
        "--file",
        "urls.txt",
        "--out",
        "feeds.json",
        "--json",
    ]);
    match cli.command {
        Command::Import { file, out, json } => {
            assert_eq!(file.as_deref(), Some("urls.txt"));
            assert_eq!(out.as_deref(), Some("feeds.json"));
            assert!(json);
        }
        _ => panic!("expected import"),
    }

    let cli = parse_from([
        "rssify",
        "add",
        "https://ex.com/feed",
        "--out",
        "feeds.json",
    ]);
    match cli.command {
        Command::Add { url, out, json } => {
            assert_eq!(url, "https://ex.com/feed");
            assert_eq!(out.as_deref(), Some("feeds.json"));
            assert!(!json);
        }
        _ => panic!("expected add"),
    }
}
