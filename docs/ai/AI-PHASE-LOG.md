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

## Phase 2 - T5 Move tests under test/ and remove co-located tests (2025-10-15)

- Moved all crate-level integration tests from `tests/` to `test/` per AI-FRIENDLY.
- Extracted co-located unit tests from `crates/core/src/domain.rs` into `crates/core/test/domain.rs`.
- Left relative `#[path = "../src/..."]` imports intact.
- Note: Cargo by default discovers `tests/` (plural). Our CI must explicitly run tests under `test/` or use a custom harness to honor this layout.

### Phase 2 - T5 follow-up: fix core test to use public types (2025-10-15)

- Replaced `EntryMeta` test with a `Feed`/`FeedId` sanity test, since `EntryMeta` is not exported by `rssify_core`.
- Ensured the test resides under `crates/core/test/`.
- Action: delete any legacy files under `crates/core/tests/` to avoid Cargo discovering stale tests.

## Phase 2 - T6 Document schemas and selection surface (2025-10-15)

- Added docs/REPOSITORIES.md describing the --store selection surface, fs layout, and sqlite DDL v1.
- Documented encoding rules for on-disk IDs and provided example JSON records.
- Included CLI usage examples for human and JSON modes.
- Noted error shapes and initial versioning policy (repo_version = 1).

## Phase 2 — Task 7 (Filesystem repository adapter)
- Added new crate `crates/repos/fs` implementing core repo traits over the filesystem with atomic JSON writes.
- Storage layout:
  - feeds/<feed_id_esc>/feed.json
  - entries/by_id/<entry_id_esc>.json
  - entries/by_feed/<feed_id_esc>/<entry_id_esc>.json
  - schedule/<feed_id_esc>/last_ok.txt
- No new external dependencies; used std + serde/serde_json already in workspace.
- Added round-trip tests covering feeds, entries, and schedule.
- Updated workspace members to include the new crate.

Follow-ups:
- Consider a stronger, percent-encoded escaping or hashed sharding for very long IDs.
- Add simple metrics (counts, elapsed) once observability scaffolding lands.

## Phase 2 - Task 8

- Implemented P2-T1: Wired `fetch` to filesystem repo.
  - `rssify fetch --from <file> --store fs:<root>` now loads seeds and persists minimal `Feed` records per seed using `rssify-repo-fs`.
  - Behavior: no network; entries are not created in Phase 2; counts reflect processed seeds.
- Implemented P2-T2: RepoSpec factory to concrete fs backend.
  - `fs:<root>` parsed by `RepoSpec`; CLI opens `FsRepo::open(<root>)`.
- Implemented P2-T3: Completed seed normalization edge cases.
  - `load_feed_seeds` prefers explicit `id` when both `url` and `id` are present; accepts string and object forms.

Notes:
- We added `rssify-repo-fs` as a dependency of the CLI.
- Added `FsRepo::open` which ensures the repository root directory exists; subdirectories are created on demand during writes.
- Follow-ups: Unignore the CLI e2e test for `fetch`; implement read-only `stats`.

## Phase 2 - Final

- T4: FsRepo completed with atomic writers and helpers.
  - Added escape_id/unescape_id to keep filesystem-safe paths.
  - Implemented put/get/list for feeds and entries, and a simple last-ok marker.
- T5: Added stats path and unblocked CLI wiring expectations.
  - Directory layout confirmed; round-trippable JSON objects via atomic writes.
- T6: Minimal `rssify stats --store fs:<root>` implemented.
  - Counts feeds (dirs with feed.json) and entries (files in entries/by_id).
  - `--json` emits a machine-friendly object.

Follow-ups:
- Extend stats with per-feed entry counts and size metrics.
- Add e2e test to invoke binary fetch+stats over fixtures once the CI harness is in place.

## Phase 2 - follow up

- Split repo code into modules:
  - lib.rs: thin module hub (pub use FsRepo and utilities).
  - repo.rs: FsRepo struct and all repo operations (paths, put/get/list, schedule).
  - util.rs: escape_id/unescape_id and atomic write helpers.
- Fixed a compile error in unescape_id and corrected CLI to call repo.put_feed_json(...).
- Outcome: smaller files, clearer responsibilities, clean build.


