# Roadmap — rssify

Goal: A single Rust binary that fetches and stores syndication data reliably, exposes a clean repository abstraction, learns optimal fetch schedules, and offers a tightly scoped CLI for batch and interactive workflows — all while staying AI-friendly (small files, shallow modules, tiny public APIs, structured logs).

## Phase 0 — Foundations and Contracts (MVP-0)
Exit criteria:
- Repo compiles cleanly with `cargo check`.
- Global rules wired in: docs/AI-FRIENDLY.md, docs/CONTRIBUTING.md, header template, size checks.
- rust-toolchain.toml pinned; CI runs fmt, clippy, header/LOC check.
Subgoals:
- Define minimal domain model in `core/`:
  - FeedId, EntryId (opaque newtypes).
  - Feed, Entry, FetchOutcome, ContentBlob (raw XML/JSON bytes).
- Define repository traits (no I/O yet):
  - `FeedRepo`, `EntryRepo`, `ScheduleRepo`, `Tx`.
- Define error boundary enums per component.
- CLI skeleton: `rssify help` shows subcommands: fetch, stats, import, add.
- Structured logging keys decided: component, op, feed_id, elapsed_ms, items.

## Phase 1 — Fetch + Persist MVP (MVP-1)
Exit criteria:
- `rssify fetch --from feeds.json --store <path>` fetches feeds concurrently, parses minimally, saves raw entries and normalized fields via a file-backed repo.
- Idempotent writes; no dupes across runs.
Subgoals:
- HTTP client with timeouts, ETag/If-Modified-Since support.
- Parser supports RSS 2.0 + Atom core elements.
- Filesystem repository (v0): layout and index manifest.
- Basic metrics: total feeds, fetched, modified, new_entries.
- Backoff on 4xx/5xx with jitter; retry budget.

## Phase 2 — Repository Abstraction v1 (swappable backends)
Exit criteria:
- Same binary works with `--repo fs:<root>` or `--repo sqlite:<dsn>` behind a stable trait boundary.
- Round-trip tests for each backend.
Subgoals:
- Define compact on-disk schema for FS and SQLite.
- Transactions for multi-entry writes.
- Binary attachments: store raw XML/JSON blobs per entry.
- Simple migrations (repo version header, upgrader).

## Phase 3 — Scheduler v1 (frequency-aware)
Exit criteria:
- `rssify fetch` adjusts per-feed cadence using observed publish times and variance; avoids over-polling.
- Persisted schedule state survives restarts.
Subgoals:
- Per-feed moving-window stats: mean/median inter-arrival, stddev, burst detection.
- Heuristics: ceiling/floor polling intervals; cold-start defaults.
- Respect publisher hints: ttl, syndication module, crawl-delay when present.
- Priority queue executor; cooperative concurrency.

## Phase 4 — CLI Surface and UX
Exit criteria:
- Subcommands are coherent, orthogonal, and scriptable.
- Every subcommand returns machine-readable JSON when `--json` is set.
Subgoals:
- `fetch`: quiet by default, `-v` for detailed spans; supports `--from feeds.json`, `--one <url>`, `--since <ts>`.
- `add`: append a single feed URL into repo-managed feeds set; validates and probes.
- `import`: read a URLs file, de-dup, probe, emit a canonical feeds.json with discovered metadata.
- `stats`: print per-feed and global stats (success rate, freshness lag, cadence estimate).

## Phase 5 — Extraction Pipeline (normalized views)
Exit criteria:
- Deterministic extractor that maps raw blobs to a normalized Entry view with stable hashing.
- Provenance maintained (raw preserved, normalized linked).
Subgoals:
- Extract: title, link, guid, published_at, updated_at, author(s), categories, content hash.
- Content hashing for dedupe across mirrors/rel-feeds.
- Pluggable enrichers: URL canonicalization, site-specific fixes (opt-in table).

## Phase 6 — Resilience, Observability, Scale
Exit criteria:
- Can fetch 10k feeds reliably in bounded memory and time with clear SLOs.
- Restart-safe; corrupted writes recover or quarantine.
Subgoals:
- Bounded queues, backpressure; per-domain concurrency caps.
- Persistent checkpoints; resumable runs.
- Metrics export (stdout JSON now; Prometheus later); tracing spans with feed_id tags.
- Synthetic failure tests: network flaps, partial writes, parser errors.

## Phase 7 — Redundancy, Mirrors, and Advanced Features (Final)
Exit criteria:
- Final feature set shipped; docs complete; defaults sane.
- Redundant URLs per logical feed supported; scheduler treats them as mirrors.
- Configurable repo backends with safe migrations.
Subgoals:
- Multiple fetch URLs per feed with health scoring and failover.
- Optional content diffing to detect silent feed rewrites.
- Export commands: newline JSON, ndjson, and simple SQLite dump.
- Policy controls: robots/crawl-delay, per-feed max-size, blocklists/allowlists.
- Tunable learning scheduler with persisted priors and aging.
- Thorough docs: README quickstart, ARCHITECTURE design, CONTRIBUTING contracts, AI-FRIENDLY rules, example feeds.json and config.

