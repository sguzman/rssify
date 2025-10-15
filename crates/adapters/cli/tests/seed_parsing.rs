// File: crates/adapters/cli/tests/seed_parsing.rs
// Purpose: Integration tests for seed parsing and fetch summary behavior via the CLI binary.
// Notes:
// - We avoid external crates; use std and serde_json (already a dependency).
// - We invoke the compiled binary through the Cargo-provided env var CARGO_BIN_EXE_rssify.
// - Each case creates a temporary working directory and a JSON seed file, then runs:
//     rssify fetch --from <file> --store fs:<dir> --json
//   and asserts that the JSON summary counts match the number of seeds.
//
// Accepted formats under test:
// 1) Array of strings: ["https://a", "https://b"]
// 2) Object with seeds array: {"seeds": ["https://a", "https://b"]}
// 3) Array of objects picking id/url/guid: [{"id":"A"}, {"url":"B"}, {"guid":"C"}]

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn bin_path() -> PathBuf {
    // Cargo sets this for each [[bin]] as CARGO_BIN_EXE_<name>
    let key = "CARGO_BIN_EXE_rssify";
    let val = env::var_os(key)
        .unwrap_or_else(|| panic!("env {} not set; ensure [[bin]] name is 'rssify'", key));
    PathBuf::from(val)
}

fn mktemp_dir(prefix: &str) -> io::Result<PathBuf> {
    let mut dir = env::temp_dir();
    // Make a reasonably unique directory name without extra deps.
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    dir.push(format!("rssify_test_{}_{}_{}", prefix, pid, nanos));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn write_file(dir: &Path, name: &str, contents: &str) -> io::Result<PathBuf> {
    let path = dir.join(name);
    fs::write(&path, contents)?;
    Ok(path)
}

fn run_fetch(from: &Path, store_root: &Path) -> io::Result<Output> {
    let exe = bin_path();
    let out = Command::new(exe)
        .arg("fetch")
        .arg("--from")
        .arg(from.as_os_str())
        .arg("--store")
        .arg(format!("fs:{}", store_root.display()))
        .arg("--json")
        .output()?;
    Ok(out)
}

fn assert_success_json(output: Output) -> serde_json::Value {
    if !output.status.success() {
        panic!(
            "binary exited with status {:?}\nstdout:\n{}\nstderr:\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let stdout = String::from_utf8(output.stdout).expect("stdout must be utf8");
    serde_json::from_str::<serde_json::Value>(&stdout)
        .unwrap_or_else(|e| panic!("stdout was not valid JSON: {}\n{}", e, stdout))
}

fn summary_counts(v: &serde_json::Value) -> (u32, u32) {
    let feeds_total = v
        .get("feeds_total")
        .and_then(|x| x.as_u64())
        .expect("feeds_total missing") as u32;
    let items_written = v
        .get("items_written")
        .and_then(|x| x.as_u64())
        .expect("items_written missing") as u32;
    (feeds_total, items_written)
}

#[test]
fn seeds_array_of_strings() {
    let work = mktemp_dir("array_strings").unwrap();
    let repo = mktemp_dir("repo_array_strings").unwrap();
    let seeds = r#"["https://a.example/rss","https://b.example/rss","https://c.example/rss"]"#;
    let file = write_file(&work, "seeds.json", seeds).unwrap();

    let out = run_fetch(&file, &repo).unwrap();
    let json = assert_success_json(out);
    let (total, written) = summary_counts(&json);
    assert_eq!(total, 3, "feeds_total must equal seed count");
    assert_eq!(written, 3, "items_written must equal seed count");
}

#[test]
fn seeds_object_with_array() {
    let work = mktemp_dir("object_seeds").unwrap();
    let repo = mktemp_dir("repo_object_seeds").unwrap();
    let seeds = r#"{"seeds": ["https://x.example/rss","https://y.example/rss"]}"#;
    let file = write_file(&work, "seeds.json", seeds).unwrap();

    let out = run_fetch(&file, &repo).unwrap();
    let json = assert_success_json(out);
    let (total, written) = summary_counts(&json);
    assert_eq!(total, 2);
    assert_eq!(written, 2);
}

#[test]
fn seeds_array_of_objects_id_url_guid() {
    let work = mktemp_dir("array_objects").unwrap();
    let repo = mktemp_dir("repo_array_objects").unwrap();
    // Three objects; the code prefers id, else url, else guid.
    let seeds = r#"
      [
        {"id":"id-A","url":"ignored-if-id-present"},
        {"url":"url-B"},
        {"guid":"guid-C"}
      ]
    "#;
    let file = write_file(&work, "seeds.json", seeds).unwrap();

    let out = run_fetch(&file, &repo).unwrap();
    let json = assert_success_json(out);
    let (total, written) = summary_counts(&json);
    assert_eq!(total, 3);
    assert_eq!(written, 3);
}
