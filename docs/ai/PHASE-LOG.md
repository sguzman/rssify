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

### Phase 2 - follow up

- Split repo code into modules:
  - lib.rs: thin module hub (pub use FsRepo and utilities).
  - repo.rs: FsRepo struct and all repo operations (paths, put/get/list, schedule).
  - util.rs: escape_id/unescape_id and atomic write helpers.
- Fixed a compile error in unescape_id and corrected CLI to call repo.put_feed_json(...).
- Outcome: smaller files, clearer responsibilities, clean build.
- Fix: removed erroneous re-export of load_feed_seeds from the CLI main to resolve E0364 in a binary crate.
- Fix: restore public pipeline and repo_fs modules for CLI tests.
- pipeline.rs: exposes FetchSummary and load_feed_seeds, with thiserror-based errors.
- repo_fs.rs: exposes FsPaths path builders used by path-shape tests.
- main.rs: adds parse_from for CLI parsing tests.
- Cargo.toml: add rssify-core and thiserror deps back to cli crate.
- Fix: make pipeline helpers public and path-friendly for tests; provide fetch_from_file, FeedSeed, FeedMetaDelta, PersistStats.
- Fix: FsPaths now returns String and includes last_blob/entry_json to match path-shape tests.
- Fix: add FsRepo::new alias; add dev-dep rssify-core for repo-fs tests.

### Phase 2 log — fs adapter trait impls (2025-10-15)
- Implemented rssify_core traits for FsRepo:
  - FeedRepo (get/put/list → JSON under feeds/<id>/feed.json).
  - EntryRepo (get/upsert/list_by_feed → JSON under entries/by_id and entries/by_feed).
  - ScheduleRepo (last_ok_fetch_ts/record_fetch_ts → schedule/<id>/last_ok.txt).
- Added FsTx and begin_tx() to match core::Tx and tests.
- Centralized JSON IO + ID escaping in util.rs; split files for clarity.

## Phase 3

### Phase 3 log — P3-T1 (2025-10-15)

- Change: CLI now parses `--store` via `spec::RepoSpec` in `main.rs` for both `fetch` and `stats`.
- Why: Centralize repo-spec validation and align with docs where the CLI “parses/validates” repository selection without embedding backend behavior.
- Behavior: No change for filesystem repos; `fs:<root>` continues to work. Non-`fs` kinds are recognized but return a clear “not supported in this phase” error.
- Follow-ups:
  - When a new backend adapter lands (e.g., SQLite), wire its kind here by matching `RepoKind::Sqlite` and constructing the concrete repo.
  - Consider promoting structured logging keys across CLI messages (component, op, items) consistently.

### Phase 3 log — P3-T2 (2025-10-15)

- Change: `rssify stats` now counts entries in the canonical per-feed layout `<root>/feeds/<feed>/entries/*.json`, and also includes legacy `<root>/entries/by_id/*.json`.
- Why: Align stats with the documented FS layout and `FsPaths` helpers; preserve backward compatibility with earlier fixtures.
- Follow-ups: Consider per-feed entry breakdown and size metrics; once legacy paths are fully removed, drop the fallback scan for `entries/by_id`.

### Phase 3 log — P3-T3 (2025-10-15)

- Change: Introduced a tiny structured logger in `crates/adapters/cli/src/log.rs` and routed CLI traces through it.
- Why: Comply with AI-FRIENDLY rule to avoid ad-hoc println-based logging and standardize key=value logs. This adds no external dependencies.
- Behavior:
  - Logs always go to stderr; stdout remains reserved for user-facing output or `--json`.
  - Verbosity mapping: default warn; `-v` sets info; `-vv` sets debug.
  - Common keys: `ts`, `level`, `component=cli`, `op`, plus contextual pairs like `feeds` and `written`.
- Follow-ups: Consider promoting the logger to a shared tiny crate if other adapters need it; add a `--quiet` that suppresses info lines even when `-v` is set.

