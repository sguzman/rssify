# AI Phase Log

## Phase 2 - Core repo seam and tests (2025-10-15)
- Added repo seam and error exports at crate root in crates/core/src/lib.rs.
- Introduced integration tests under crates/core/tests/repo_trait.rs.
- No new external dependencies.

## Phase 2 - T2 FsPaths builders (2025-10-15)
- Implemented pure URL-safe encoder and path builders in crates/adapters/cli/src/repo_fs.rs.
- Added tests under crates/adapters/cli/tests/repo_fs_path.rs (fixed include path).
- Removed colocated unit tests from source file.

## Phase 2 - T3 Feeds loader and stub fetch (2025-10-15)
- Implemented crates/adapters/cli/src/pipeline.rs with:
  - PipelineError, FetchSummary, plus placeholder types FeedSeed, FeedMetaDelta, PersistStats
  - load_feed_seeds(path) and fetch_from_file(path)
- Added integration tests in crates/adapters/cli/tests/pipeline_fetch.rs
- No network or writes; no new dependencies.

