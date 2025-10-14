# CONTRIBUTING.md

Welcome! This document explains how to contribute clean, testable code to this project. It encodes our architecture boundaries, AI-friendly authoring rules, testing strategy, and PR workflow so humans and AI agents can collaborate smoothly.

## 1. Core values

* Small, composable units. Prefer many focused files over one giant file.
* Pure core, impure edges. Parsing, I/O, clock, and HTTP live behind traits.
* Determinism by default. Same inputs should yield identical outputs and IDs.
* Observability first. Every decision is explainable via logs, metrics, traces.
* Backwards compatibility. Schema and output changes are versioned explicitly.

## 2. Workspace layout

We maintain strict boundaries. Adapters depend on `core`, never on each other.

```
crates/
  core/               # domain types, traits, logic (pure or easily mockable)
  adapters/
    cli/              # CLI surface; maps subcommands onto core traits
    http/             # future: HTTP service using the same core traits
  repos/
    fs/               # repository adapter: filesystem/JSON/NDJSON
    sqlite/           # repository adapter: SQLite (optional)
xtask/                # CI helpers, generators, fixture tooling (optional)
```

Rules:

* `core` has zero network or filesystem imports.
* `adapters/*` may do I/O but must speak only via core traits and types.
* `repos/*` implement the repository trait only; they contain no business logic.


## 3. AI-friendly authoring rules
We follow the global rules in [AI-FRIENDLY.md — Global File & API Rules](./AI-FRIENDLY.md#global-file-and-api-rules). This repo ENFORCES:
- Required file header (CI fails if missing)
- Size check: warn above 200 LOC (excluding header/tests)
- Structured logging keys per component

### 3.1 Required file header

Every Rust file starts with this header block (ASCII only):

```text
// File: <crate>/<path>/<name>.rs
// Purpose: <one-line intent>
// Inputs: <types/traits it consumes>
// Outputs: <types/errors it returns>
// Side effects: <I/O, logging, metrics, none>
// Invariants:
//  - <bullet 1>
//  - <bullet 2>
// Tests: <file(s) providing golden/property/E2E coverage>
```

CI fails if the header is missing.

## 4. Code style

* Rust edition: 2021 (or project default).
* rustfmt: required. clippy: `-D warnings`.
* Naming: types UpperCamelCase, traits end with `Ext` only for extension traits.
* Errors implement `std::error::Error` and carry machine-readable codes.

## 5. Repository contract (must-implement)

All data storage goes through a single trait in `core`. Adapters in `repos/*` implement it. The exact signatures live in `crates/core/src/repo.rs`; this section defines the contract.

Operations and constraints:

* `put_raw(feed_id, payload, meta) -> Result<RawId>`
  Idempotent. Atomic write or fail. Stores verbatim fetch payload with headers and timestamps. Never mutates once written.
* `put_entry(entry: NormalizedEntry) -> Result<EntryId>`
  Upsert by canonical `EntryId`. Must be atomic at entry granularity.
* `get_last_fetch(feed_id) -> Result<Option<FetchRecord>>`
  Used by scheduler; must be O(1) on indexed backends.
* `record_fetch(feed_id, outcome: FetchOutcome) -> Result<()>`
  Append-only; enables EWMA and error-budget accounting.
* `scan_entries(feed_id, since: Option<Instant>) -> Iterator<NormalizedEntry>`
  Forward-only, stable order (by published then updated).
* `put_derivation(entry_id, kind, version, blob) -> Result<()>`
  Versioned derivations; never overwrite same `(entry_id, kind, version)`.

Error semantics:

* Distinguish `Conflict`, `NotFound`, `Unavailable`, `Corruption`, `Transient`.
* All writes are atomic per record. If not supported, adapter must do a temp-file or transaction dance to emulate.

## 6. Canonical IDs (must follow)

To avoid duplicates across mirrors or transport quirks, IDs are derived as follows (centralized in `core::ids`):

Order of precedence for `EntryId`:

1. `guid` if present and not marked `isPermaLink=false`, normalized.
2. Stable hash of tuple:

   * normalized link URL (after redirect resolution if available)
   * title text stripped of markup and collapsed whitespace
   * published timestamp (RFC3339, UTC, second precision)
   * content hash of `content:encoded` or `summary` if content missing

Hash function: xxhash64 over UTF-8 bytes of the tuple; output as 16-char lowercase hex. All callers must go through `core::ids::entry_id(entry_like)`.

Add golden tests: `tests/golden/ids/*.yaml` mapping inputs to expected IDs.

## 7. Scheduler state model (must follow)

The scheduler decides when to fetch each feed. Central enums live in `core::sched`:

* `State`: Healthy, CoolingDown, Backoff, Paused, Disabled
* `Decision`: FetchNow, Defer { next_at }, Backoff { until, reason }, Pause { until, reason }
* `Reason`: NewFeed, RetryAfter, HttpError, ParseError, BudgetExceeded, EWMAStable, Manual

Decision algorithm (high-level):

* Maintain per-feed EWMA of inter-arrival times and a global error budget.
* Respect HTTP cache semantics (ETag/If-None-Match, Last-Modified, Retry-After).
* Jitter within a small window to avoid thundering herds.
* Clamp with per-host concurrency caps and per-feed min/max intervals.

All decisions must log:

* `feed_id`, `state`, `decision`, `next_at`, `reason`, `ewma_secs`, `error_budget`, `retry_after_secs?`.

Goldens: `tests/golden/sched/*.yaml` with sequences of events -> expected decisions.

## 8. Testing strategy

We practice layered tests. Prefer fast tests and stable fixtures.

* Unit tests: in the same file as the unit under test.
* Property tests: for parsers, ID canonicalization, and scheduler math.
* Golden tests: YAML inputs -> expected normalized outputs and IDs.
* Integration tests: crate-level flows with in-memory repo and fake clock.
* E2E test: `feeds.json` with two small feeds. `fetch` once -> expect `NewEntry` events and two raw payloads.

Test layout:

```
crates/core/tests/
  golden_ids.rs
  golden_sched.rs
  e2e_minimal.rs
tests/fixtures/
  feeds.json
  feeds/
    atom_edge.xml
    rss_content_encoded.xml
    huge_truncated.xml
```

## 9. Backfill policy

Backfill (RFC 5005, archive walking) must not starve live updates.

* Live always wins. At most 1 backfill request per feed concurrently.
* Cap historical pages per run (default: 2) and per day (default: 20).
* Skip backfill while `State` is `Backoff` or `Paused`.
* Record progress markers; never re-crawl the same archive page in a day.

## 10. Observability and performance

* Logging: structured, no secrets. Required keys: `component`, `feed_id`, `decision`, `elapsed_ms`.
* Metrics: counters for fetch outcomes, histograms for durations and response sizes, gauges for backlog and error budget.
* Tracing: parent span for each fetch pipeline; child spans for DNS, TCP, TLS, HTTP, parse, normalize, store.
* Budgets: fail fast on timeouts; no unbounded retries.

## 11. Documentation duties

Any new public type or trait must have:

* A one-line summary and a short example.
* A note on invariants and failure modes.
* Cross-links to related types.

Update `ARCHITECTURE.md` if you add a boundary or change a contract. Update `README.md` if you add/rename a CLI subcommand.

## 12. CLI surface contract

Subcommands map 1:1 to use-cases and call only core traits:

* `fetch` — quiet by default; `--verbose` enables tracing.
* `stats` — reads only via repo trait; never touches the network.
* `import` — batch adds from a URLs file into the repo; validates and dedups.
* `add` — adds one URL; returns canonical `feed_id`.

No subcommand may implement logic that belongs in `core`.

## 13. PR process

Open a PR only when:

* All file headers are present.
* `cargo fmt`, `clippy -D warnings`, and tests pass locally.
* New or changed behavior has golden tests.
* Public API changes include docs and a brief migration note.

### 13.1 PR checklist

* [ ] I respected the workspace dependency rules (adapters -> core only).
* [ ] I kept files small and single-purpose.
* [ ] I used the repository trait and did not reach around it.
* [ ] I used `core::ids` for any IDs.
* [ ] I added or updated goldens and property tests.
* [ ] I documented invariants and failure modes.
* [ ] I added structured logs at decision points.

## 14. Commit messages

Use Conventional Commits with semantic intent so changelogs are automated:

* `feat(core): canonicalize IDs for Atom feeds`
* `fix(repo/fs): atomic writes with temp files`
* `perf(sched): clamp EWMA floor at 15m`
* `test(core): golden cases for Retry-After`
* `docs(architecture): backfill policy`


### 14.1 Agent submissions MUST include a commit message
When suggesting code or doc edits (in issues, PRs, or assistant responses), always include a one-line Conventional Commit message that we can copy-paste for the actual commit. This is mandatory for AI agents and human reviewers to keep the history coherent.

### 14.2 Dependency policy: latest working crate versions
When adding or modifying Rust dependencies:
* Prefer the latest **compatible, working** release that builds on our pinned toolchain.
* Run a targeted update (e.g., `cargo update -p crate_name`) and compile locally before proposing changes.
* If a newest release breaks our build, document the failure briefly and pin to the newest **working** version; open a follow-up issue to track the upgrade.
* Avoid broad, destabilizing updates in unrelated PRs; keep bumps scoped to the feature or fix at hand.

## 15. CI expectations

CI runs:

* Format and lint.
* Unit, property, golden, integration and E2E tests.
* Header check: every `.rs` file must include the required header block.
* Size check: warn on files > 200 LOC (excluding header and tests).
* Fixture determinism: re-run goldens and fail if outputs change without updated fixtures.

## 16. Security and licensing

* No secrets in code, tests, or fixtures.
* Treat all remote inputs as untrusted. Validate and bound parse sizes.
* License: CC0-1.0 for text; code license as declared in the repo.

## 17. Getting started (quick path)

1. Read `ARCHITECTURE.md` and skim `core::repo`, `core::ids`, `core::sched`.
2. Pick a unit of work from `ROADMAP.md` or open a proposal.
3. Create a small file in the right crate with the header and a unit test.
4. Add or extend a golden fixture if behavior changes.
5. Open a draft PR early to run CI and discuss boundaries.

---

Questions or proposals? Open a GitHub Discussion or a draft PR with a short design note. Thanks for helping keep this codebase clean, deterministic, and friendly to both humans and AI.

