//// File: crates/adapters/cli/src/stats.rs
//// Purpose: Minimal read-only stats over the filesystem repo for Phase 2.

use std::path::Path;

pub struct StatsSummary {
    pub feeds: usize,
    pub entries: usize,
}

pub fn stats_fs(root: &str) -> std::io::Result<StatsSummary> {
    let feeds_root = Path::new(root).join("feeds");
    let entries_root = Path::new(root).join("entries").join("by_id");

    let mut feed_count = 0usize;
    if feeds_root.exists() {
        for ent in std::fs::read_dir(&feeds_root)? {
            let ent = ent?;
            if ent.file_type()?.is_dir() {
                // Consider it a feed when feed.json exists within.
                let candidate = ent.path().join("feed.json");
                if candidate.exists() {
                    feed_count += 1;
                }
            }
        }
    }

    let mut entry_count = 0usize;
    if entries_root.exists() {
        for ent in std::fs::read_dir(&entries_root)? {
            let ent = ent?;
            if ent.file_type()?.is_file() {
                entry_count += 1;
            }
        }
    }

    Ok(StatsSummary {
        feeds: feed_count,
        entries: entry_count,
    })
}

