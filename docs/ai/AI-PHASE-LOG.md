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

## Phase 2 - S4 CLI wiring (2025-10-15)

- Wired "rssify fetch" to "pipeline::fetch_from_file", honoring "--from" (default: feeds.json) and "--json".
- Added early validation for "--store" via "spec::RepoSpec" to enforce the repo seam format.
- Adopted the AI-FRIENDLY header block (line comments) in "crates/adapters/cli/src/main.rs".
- Minimal verbosity: "-v" prints start context; "-vv" includes completion trace.
- Left "stats", "import", and "add" as stubs that return success with explicit "not implemented" messaging (and JSON placeholders) to keep pipelines stable.

Follow-ups:
- Implement real "stats" reading via repository trait (Phase 3).
- Split subcommand handlers into dedicated files ("cmd_fetch.rs", etc.) once logic grows.
- Add integration test to exercise "rssify fetch --json" with fixtures.

