/*
Module: rssify_cli::tests::integration_fetch
Purpose: End-to-end shape test for `rssify fetch` with fixtures (ignored)
Public API surface: tests only
Invariants: Uses testdata; no network and no filesystem writes yet
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: #[ignore] until Phase 1 implementation.
*/

#[path = "../src/main.rs"]
mod bin_main; // exposes Cli, Command, parse_from

use bin_main::{Command, parse_from};

fn td(p: &str) -> std::path::PathBuf {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    here.join("tests").join("testdata").join(p)
}

#[test]
#[ignore] // enable after Phase 1 is implemented
fn fetch_invocation_parses_and_will_wire_pipeline() {
    let feeds = td("feeds_min.json");
    let cli = parse_from([
        "rssify",
        "fetch",
        "--from",
        feeds.to_string_lossy().as_ref(),
        "--store",
        "fs:./.tmp-test",
        "--json",
        "-v",
    ]);
    match cli.command {
        Command::Fetch {
            from,
            store,
            json,
            verbose,
        } => {
            assert!(from.is_some());
            assert!(store.is_some());
            assert!(json);
            assert!(verbose >= 1);
        }
        _ => panic!("expected fetch"),
    }
}
