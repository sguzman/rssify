/*
Module: tests::id_policy_golden
Purpose: Golden tests for entry ID precedence (guid > link > content_hash).
Public API under test: rssify_core::id_policy::choose_id()
Notes: Reads JSONL fixtures; table-driven; no I/O beyond file read.
*/
#![forbid(unsafe_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

use rssify_core::id_policy::choose_id;
use serde::Deserialize;

#[derive(Deserialize)]
struct Case {
    name: String,
    guid: String,
    link: String,
    #[serde(rename = "hash")]
    content_hash: String,
    expect: String,
}

#[test]
fn id_policy_golden() {
    let path = "crates/core/tests/fixtures/id_policy/golden.jsonl";
    let f = File::open(path).expect("open golden.jsonl");
    let br = BufReader::new(f);

    for (i, line) in br.lines().enumerate() {
        let line = line.expect("read line");
        if line.trim().is_empty() {
            continue;
        }
        let case: Case = serde_json::from_str(&line)
            .unwrap_or_else(|e| panic!("bad json on line {}: {e}\n{line}", i + 1));

        let got = choose_id(n2o(&case.guid), n2o(&case.link), &case.content_hash);
        assert_eq!(
            got, case.expect,
            "case '{}': expected '{}', got '{}'",
            case.name, case.expect, got
        );
    }
}

fn n2o(s: &str) -> Option<&str> {
    let t = s.trim();
    if t.is_empty() { None } else { Some(t) }
}
