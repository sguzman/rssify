//// File: crates/adapters/cli/src/stats.rs
//// Purpose: Read-only repo stats for filesystem backend.
//// Inputs/Outputs: stats_fs(root) -> StatsSummary { feeds, entries }.
//// Invariants:
////   - A "feed" is counted iff <root>/feeds/<feed>/feed.json exists.
////   - "entries" are counted from the canonical per-feed layout:
////       <root>/feeds/<feed>/entries/<entry>.json
////     and (for backward-compatibility) from deprecated:
////       <root>/entries/by_id/<entry>.json
//// Examples:
////   let s = stats_fs("./data")?;
////   println!("feeds={} entries={}", s.feeds, s.entries);
//// Task: Keep pure read-only; no external deps; under 200 LOC.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct StatsSummary {
    pub feeds: usize,
    pub entries: usize,
}

pub fn stats_fs(root: &str) -> io::Result<StatsSummary> {
    let feeds_root = Path::new(root).join("feeds");
    let legacy_entries_root = Path::new(root).join("entries").join("by_id");

    // Count feeds: <root>/feeds/<feed>/feed.json
    let mut feed_count = 0usize;
    if feeds_root.exists() {
        for ent in fs::read_dir(&feeds_root)? {
            let ent = ent?;
            if ent.file_type()?.is_dir() {
                let candidate = ent.path().join("feed.json");
                if candidate.exists() {
                    feed_count += 1;
                }
            }
        }
    }

    // Count entries in canonical per-feed layout.
    // <root>/feeds/<feed>/entries/*.json
    let mut entry_count = 0usize;
    if feeds_root.exists() {
        for feed_dir in fs::read_dir(&feeds_root)? {
            let feed_dir = feed_dir?;
            if !feed_dir.file_type()?.is_dir() {
                continue;
            }
            let entries_dir = feed_dir.path().join("entries");
            entry_count += count_json_files_in_dir(&entries_dir)?;
        }
    }

    // Also count legacy flat entries directory if present.
    if legacy_entries_root.exists() {
        entry_count += count_json_files_in_dir(&legacy_entries_root)?;
    }

    Ok(StatsSummary {
        feeds: feed_count,
        entries: entry_count,
    })
}

fn count_json_files_in_dir(dir: &PathBuf) -> io::Result<usize> {
    if !dir.exists() {
        return Ok(0);
    }
    let mut n = 0usize;
    for ent in fs::read_dir(dir)? {
        let ent = ent?;
        if ent.file_type()?.is_file() {
            if ent
                .path()
                .extension()
                .map(|e| e.to_string_lossy().eq_ignore_ascii_case("json"))
                .unwrap_or(false)
            {
                n += 1;
            }
        }
    }
    Ok(n)
}

