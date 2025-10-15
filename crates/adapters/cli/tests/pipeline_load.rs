/*
Module: rssify_cli::tests::pipeline_load
Purpose: Validate pipeline::load_feed_seeds supports arrays, objects, and {seeds: [...]}
*/

#[path = "../src/pipeline.rs"]
mod pipeline;

use std::fs;
use std::path::PathBuf;

/// Create a unique temp file path.
fn tf(ext: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    p.push(format!("rssify-pipeline-{}-{}.{}", pid, nanos, ext));
    p
}

#[test]
fn loads_array_of_strings() {
    let file = tf("json");
    fs::write(&file, r#"["u1","u2"]"#).unwrap();
    let ids = pipeline::load_feed_seeds(&file).expect("parse");
    assert_eq!(ids, vec!["u1".to_string(), "u2".to_string()]);
    let _ = fs::remove_file(&file);
}

#[test]
fn loads_array_of_objects_prefers_id_then_url_then_guid() {
    let file = tf("json");
    fs::write(&file, r#"[{"id":"X","url":"u1"},{"guid":"G2"},{"url":"u3"}]"#).unwrap();
    let ids = pipeline::load_feed_seeds(&file).expect("parse");
    assert_eq!(ids, vec!["X".to_string(), "G2".to_string(), "u3".to_string()]);
    let _ = fs::remove_file(&file);
}

#[test]
fn loads_object_with_seeds_array() {
    let file = tf("json");
    fs::write(&file, r#"{ "seeds": ["u1", {"url":"u2"}] }"#).unwrap();
    let ids = pipeline::load_feed_seeds(&file).expect("parse");
    assert_eq!(ids, vec!["u1".to_string(), "u2".to_string()]);
    let _ = fs::remove_file(&file);
}

#[test]
fn rejects_empty_array() {
    let file = tf("json");
    fs::write(&file, r#"[]"#).unwrap();
    let err = pipeline::load_feed_seeds(&file).unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("seeds array is empty"));
    let _ = fs::remove_file(&file);
}
