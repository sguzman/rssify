# RSS Fetcher Architecture

Status: draft
Owners: you
Scope: fetching and scheduling for RSS/Atom feeds; no reader UI

## 0. Executive summary

This service discovers, fetches, and parses syndicated feeds (RSS 2.0, Atom), stores both the raw responses and normalized entries, and exposes the results to other programs via simple interfaces. A smart, adaptive scheduler sets per-feed fetch cadence using a blend of publisher hints, HTTP signals, and observed posting behavior.

Non-goals: rendering/reading UI; social features; ML text classification; search UI.

---

## 1. Goals and constraints

* Fetch reliably, at scale, without hammering hosts.
* Preserve raw data exactly as served for reproducibility and future re-parsing.
* Provide a clean, stable normalized model for downstream consumers.
* Adapt fetch frequency per feed automatically.
* Be robust against malformed XML and hostile inputs.
* Keep operational visibility high: metrics, logs, sampling, tracing.

Operational constraints:

* Stateless workers where practical; persistence for feeds, payloads, and schedules.
* Horizontal scalability; per-host concurrency ceilings and backoff.
* Deterministic behavior under retry/restart.

---

## 2. Key terms

* Feed: a URL returning RSS or Atom XML.
* Entry/Item: a single post within a feed.
* Raw payload: exact bytes + headers from an HTTP response.
* Normalized entry: JSON record with consistent fields across RSS/Atom.
* Schedule state: the per-feed timer decision and rationale.

---

## 3. High-level architecture

```
+------------------+     +-------------------+     +-------------------+
| Subscription DB  | --> | Scheduler         | --> | Fetcher Workers   |
| feeds, state     |     | per-feed cadence  |     | HTTP + parsing    |
+------------------+     +-------------------+     +-------------------+
          |                          |                         |
          v                          v                         v
+------------------+     +-------------------+     +-------------------+
| Raw Store        |     | Entry Extractor   |     | Normalized Store  |
| bytes + headers  | <-- | XML -> fragments  | --> | entries + indexes |
+------------------+     +-------------------+     +-------------------+
          ^                                                     |
          |                                                     v
     +----------+                                       +----------------+
     | Events   |  NewEntry(feed_id, entry_id, ...)     | Public APIs    |
     | bus/queue| ------------------------------------> | REST/gRPC/NDJSON|
     +----------+                                       +----------------+
```

---

## 4. Components

### 4.1 Subscription & discovery

* Accept explicit feed URLs (primary).
* Optional: site autodiscovery by scanning HTML for
  link rel="alternate" type="application/rss+xml" or "application/atom+xml".
* Optional: sitemap seeding if you add crawler features later.
* Track feed metadata: content type, validators (ETag, Last-Modified), schedule state, last fetch, last success/failure, error budget.

### 4.2 Fetch layer (HTTP)

* Conditional GET: If-None-Match/ETag and If-Modified-Since/Last-Modified.
* Handle 304 Not Modified, 429/503 with Retry-After, gzip/br compression, timeouts, redirects.
* Per-host connection pooling and concurrency caps.
* Request and response headers fully recorded with the payload.

### 4.3 Parsing & normalization

* Hardened XML parsing with DTD/XXE disabled.
* Detect format (RSS 2.0 vs Atom) and parse:

  * Identity: guid/id (stable), link (canonical), title.
  * Timestamps: published, updated (Atom often reliable), first_seen.
  * Content: summary, content (HTML/text), authors, categories.
  * Media: enclosure(s) with type/length/url.
* Normalization rules:

  * Prefer Atom id; else RSS guid; else stable hash fallback.
  * Canonicalize URLs (strip fragments, normalize scheme/host as policy dictates).
  * Normalize timestamps to UTC RFC3339.

### 4.4 Deduplication

* Primary key: entry_uid computed by:

  1. format_native_id if present and stable,
  2. else SHA-256 over canonical tuple: (canon_link, title, published_ts, feed_id).
* Keep lineage: first_seen, last_seen, seen_count, last_hash.

### 4.5 Storage

* Raw store (immutable):

  * Object: feed_id, fetched_at, http_status, headers, body_bytes, content_hash.
  * Index by feed_id + fetched_at. Consider content-addressed storage.
* Normalized store:

  * Feeds: id, url, type, validators, schedule_state, stats, last_fetch, last_result.
  * Entries: entry_uid, feed_id, canonical_link, title, summary, content, authors[],
    categories[], published, updated, enclosures[], first_seen, last_seen, raw_refs[].
  * Entry raw fragments: store per-entry raw XML subtree snapshots when feasible.

### 4.6 Smart scheduler

* Inputs:

  * Publisher hints: RSS ttl (minutes), skipHours, skipDays.
  * HTTP signals: 304 Not Modified, Retry-After, recent status codes.
  * Observed cadence: inter-arrival stats from the last N entries.
  * Safety floors/ceilings and jitter.

* Baseline algorithm:

  * Start interval I0 = 15m (configurable).
  * If feed declares ttl, clamp I to [ttl, max_interval].
  * On 304, increase interval by factor b_up (e.g., 1.25) up to max_interval.
  * On 2xx with new entries, decrease interval by factor b_down (e.g., 0.75) down to min_interval.
  * On 429/503 with Retry-After, pause exactly Retry-After; otherwise exponential backoff with cap.
  * Observed cadence EWMA:

    * Let inter-arrival deltas be d1..dn from recent entries.
    * EWMA_t = alpha*d_t + (1-alpha)*EWMA_{t-1}, alpha in [0.2,0.4].
    * Target interval I* = clamp(EWMA, min_interval, max_interval).
    * Blend with current I via: I_next = clamp(lambda*I* + (1-lambda)*I, min_interval, max_interval).
  * Add random jitter J ~ U(-j, +j) proportionally (e.g., j = 0.15*I_next).
  * Honor skipHours/skipDays by shifting schedule out of publisher-declared quiet windows.

* Archive/backfill:

  * If RFC 5005 relations are present (prev-archive, next), walk archives once on a backfill queue.
  * After backfill completes, resume normal cadence on the current feed only.

* Push (when available):

  * WebSub: subscribe to hub and reduce poll cadence to a slow safety check (e.g., daily).

### 4.7 Politeness & governance

* Respect robots.txt when you crawl HTML pages for autodiscovery or article fetching.
* Per-host rate limits and concurrency.
* Hard ceilings on feed size, XML depth, and max items per document.

### 4.8 Exposure to other programs

* Event bus: publish NewEntry events on first_seen with a compact payload and raw_refs.
* Pull APIs:

  * REST/gRPC: list feeds, list entries by feed or time, get entry by uid, get raw payload by ref.
  * Flat files: write newline-delimited JSON streams to a directory for batch consumers.
* Optional: serve your own meta-feed for any collection, including Link headers (rel=self/next/hub).

### 4.9 Observability

* Metrics (per feed and aggregated):

  * fetch_count, success_rate, 304_rate, error_code_counts
  * bytes_in, compression_ratio, average_latency
  * scheduler_interval_chosen, ewma_interval, retry_counts
  * new_entries_count per fetch
* Logs:

  * Structured logs with feed_id, url, decision_reason, http_status, interval_before/after.
* Sampling:

  * Keep last N raw payloads per feed for debugging parser regressions.
* Tracing:

  * Trace spans: schedule decision -> HTTP request -> parse -> storage -> events.

### 4.10 Security posture

* Disable DTD, external entities, and XInclude in XML parsers.
* Enforce size/time limits on downloads and XML parse depth.
* Sanitize HTML content fields if you later render them anywhere.
* Validate and normalize URLs; restrict schemes to http/https for fetching.
* Isolate fetcher network egress via allowlist if you run in untrusted environments.

---

## 5. Data model (normalized)

Types shown in JSON-like notation.

### 5.1 Feed

```
Feed {
  id: string,            // stable UUID
  url: string,
  type: "rss" | "atom" | "unknown",
  validators: {
    etag: string|null,
    last_modified: string|null  // HTTP date
  },
  schedule: {
    interval_sec: int,
    min_interval_sec: int,
    max_interval_sec: int,
    last_decision_at: string,   // RFC3339
    reason: string,             // human-readable summary
    retry_after_sec: int|null
  },
  publisher_hints: {
    ttl_minutes: int|null,
    skip_hours: int[]|null,     // 0..23
    skip_days: string[]|null    // ["Mon",...]
  },
  stats: {
    last_fetch_at: string|null,
    last_success_at: string|null,
    consecutive_failures: int,
    ewma_interarrival_sec: int|null,
    new_entries_last_fetch: int
  }
}
```

### 5.2 Entry

```
Entry {
  entry_uid: string,           // preferred native id; else hash
  feed_id: string,
  canonical_link: string|null,
  title: string|null,
  summary: string|null,        // may be HTML or text
  content: string|null,        // preferred full content if present
  authors: [{ name: string|null, email: string|null, uri: string|null }],
  categories: [string],
  enclosures: [{ url: string, type: string|null, length: int|null }],
  published: string|null,      // RFC3339
  updated: string|null,        // RFC3339
  first_seen: string,          // RFC3339
  last_seen: string,           // RFC3339
  seen_count: int,
  raw_refs: [RawRef],          // pointers into Raw store
  content_hash: string         // hash over normalized content for change detection
}

RawRef {
  fetch_id: string,            // id of a RawFetch object
  offset: int|null, length: int|null, // optional if you store entry subtree slices
  note: string|null
}
```

### 5.3 Raw fetch

```
RawFetch {
  fetch_id: string,            // UUID
  feed_id: string,
  fetched_at: string,          // RFC3339
  url: string,
  http_status: int,
  request_headers: { ... },
  response_headers: { ... },
  body_ref: BlobRef,
  body_sha256: string,
  content_type: string|null,
  content_length: int|null
}

BlobRef {
  storage: "localfs"|"s3"|"gcs"|"...",
  key: string
}
```

Indexes to consider:

* Entry(entry_uid) PK, Entry(feed_id, published), Entry(feed_id, first_seen)
* RawFetch(feed_id, fetched_at desc)
* Feed(url unique)

---

## 6. Scheduler details

### 6.1 Parameters

* min_interval_sec: default 300
* max_interval_sec: default 86400
* up_factor b_up: 1.25
* down_factor b_down: 0.75
* ewma_alpha: 0.3
* blend_lambda: 0.5
* jitter_ratio: 0.15
* backoff_base: 2.0
* backoff_cap_sec: 3600

### 6.2 Pseudocode

```
I = current_interval
if http_status == 429 or http_status == 503:
  if Retry-After present:
    pause = parse_retry_after()
    schedule_after(pause)
    reason = "retry-after"
  else:
    I = min(I * backoff_base, backoff_cap_sec)
    reason = "error-backoff"
elif http_status == 304:
  I = min(I * b_up, max_interval)
  reason = "not-modified"
elif http_status in 200..299:
  if new_entries > 0:
    I = max(I * b_down, min_interval)
    reason = "new-entries"
  else:
    I = min(I * b_up, max_interval)
    reason = "no-new-entries"

if ewma_interarrival available:
  I_star = clamp(ewma_interarrival, min_interval, max_interval)
  I = clamp(blend_lambda * I_star + (1 - blend_lambda) * I,
            min_interval, max_interval)

if publisher ttl exists:
  I = max(I, ttl_minutes * 60)

I = apply_skip_windows(I, skipHours, skipDays)
I = apply_jitter(I, jitter_ratio)

schedule_after(I)
record_decision(reason, I)
```

### 6.3 EWMA maintenance

* On each new entry sequence update, recompute inter-arrival deltas from the last N publication timestamps (e.g., N=20) and update ewma_interarrival.

---

## 7. Failure modes and handling

* Malformed XML: mark fetch as parse_error; store raw; do not advance last_success_at; continue with cautious backoff.
* Huge responses: abort on size/time limit; record truncated flag; increase interval.
* Flapping feeds (alternating errors): cap backoff but add jitter; open circuit if over error budget.
* ID instability (changing guid/title): rely on canonical_link and content hash; keep lineage for merges.
* Time skew: if published dates are missing/future, fall back to first_seen ordering.

---

## 8. Configuration

* Global:

  * min/max intervals, factors, jitter, backoff caps.
  * network timeouts, max redirects, max body size, allowed content types.
  * per-host concurrency ceilings and rate limits.
* Per-feed overrides:

  * min/max interval clamps, disable autodiscovery, enable push only, archive backfill enabled.

Example env keys (illustrative):

* FETCH_TIMEOUT_MS, MAX_BODY_BYTES, MAX_XML_DEPTH
* SCHED_MIN_INTERVAL_SEC, SCHED_MAX_INTERVAL_SEC, SCHED_JITTER_RATIO
* HOST_MAX_CONCURRENCY, HOST_RPS

---

## 9. Interfaces

### 9.1 Events

* Topic: new_entry
* Payload:

```
{
  "entry_uid": "...",
  "feed_id": "...",
  "published": "...",
  "canonical_link": "...",
  "raw_refs": [{ "fetch_id": "..." }],
  "ingested_at": "..."
}
```

### 9.2 REST (sketch)

* GET /feeds
* GET /feeds/{id}
* POST /feeds  // add subscription
* DELETE /feeds/{id}
* POST /feeds/{id}/refetch   // manual nudge
* GET /feeds/{id}/entries?since=...
* GET /entries/{entry_uid}
* GET /raw/{fetch_id}

### 9.3 File taps

* Write NDJSON files to a directory partitioned by date:

  * out/entries/2025-10-14.ndjson
  * out/raw/2025-10-14/uuid.blob

---

## 10. Observability and SLOs

SLOs (initial targets):

* p99 fetch latency < 5s for 1MB payloads.
* Error budget: < 2% failed fetches per day per feed ignoring publisher/server faults.
* Scheduler freshness: 95% of feeds polled within 2x their chosen interval.

Dashboards:

* Success vs 304 vs error stacked over time.
* Interval chosen vs ewma per feed.
* Top feeds by new_entries.
* Top error codes and hosts.
* Bytes saved via conditional GETs.

---

## 11. Security and privacy

* Do not execute any active content; treat feed HTML as untrusted.
* Do not resolve data: URIs or file: URIs.
* Optionally hash or redact PII if you process author emails downstream.
* Isolate credentials if you support authenticated feeds later.

---

## 12. Extensions (optional modules)

* WebSub subscriptions and verification endpoint.
* RFC 5005 archive walker for historical import.
* Media fetcher for enclosures with their own queue/limits.
* Full-text article fetcher with HTML boilerplate removal.
* Duplicate clusterer across multiple feeds mirroring the same site.

---

## 13. Testing strategy

* Golden raw payloads and parser fixtures for RSS/Atom edge cases.
* Property tests on normalization (id stability, URL canonicalization).
* Fuzzing malformed XML.
* Deterministic scheduler tests:

  * sequences of 304/new entries/errors -> expected intervals.
* Load tests:

  * many small feeds vs few large feeds; simulate slow servers and 429s.

---

## 14. Rollout plan

Phase 1 (MVP):

* Feeds registry, HTTP fetcher with conditional GET, hardened parsing, raw + normalized storage, scheduler v1, minimal REST, metrics.

Phase 2:

* EWMA-based adaptation, skip windows, per-host governance, NDJSON tap, backfill via RFC 5005.

Phase 3:

* WebSub support, media fetcher, tracing, archive walker tooling.

Phase 4:

* Article fetcher, de-dupe across feeds, data export formats.

---

## 15. Implementation notes

* Make the fetcher worker idempotent per scheduled run and feed (dedupe by feed_id + scheduled_at).
* Store raw before parsing; parsing failures should not drop the payload.
* Keep a small on-disk cache of last good payload to allow diffing when a feed silently re-writes history.
* Use monotonic clocks for scheduling in process; persist absolute next_run_at in DB for cross-process handoff.
* Keep per-host pools keyed by host+port+scheme to avoid cross-talk.

---

## 16. Appendix: normalized field mapping cheatsheet

RSS -> normalized:

* item.guid[.isPermaLink=false] -> entry_uid
* item.link -> canonical_link
* item.title -> title
* item.pubDate -> published
* item.description -> summary
* item.content:encoded -> content
* item.enclosure -> enclosures[]
* channel.ttl -> publisher_hints.ttl_minutes
* channel.skipHours/skipDays -> publisher_hints.skip_*

Atom -> normalized:

* entry.id -> entry_uid
* entry.link rel="alternate" -> canonical_link
* entry.title -> title
* entry.published -> published
* entry.updated -> updated
* entry.summary -> summary
* entry.content -> content
* link rel="enclosure" -> enclosures[]

---

## 17. References you should be aware of (non-binding)

* RSS 2.0 (de facto spec), Atom (RFC 4287)
* HTTP semantics (conditional GET, Retry-After)
* WebSub (W3C), RFC 5005 (Feed Paging and Archiving)
* robots.txt (REP)
* OWASP XML security guidance

---

## 18. Quick checklist

* [ ] Conditional GET implemented
* [ ] Raw payloads persisted with headers
* [ ] XML parser hardened, limits enforced
* [ ] Normalization and stable entry_uid logic
* [ ] Scheduler v1 with EWMA blend and jitter
* [ ] Per-host concurrency and rate limits
* [ ] Metrics + logs + sample raw retention
* [ ] Minimal REST + NDJSON tap
* [ ] Backfill queue and archive handling

## Contracts index

- IDs: see docs/ID_POLICY.md and `crates/core/src/ids.rs`
- Domain records: `crates/core/src/model.rs`
- Errors: `crates/core/src/error.rs`
- Repositories: `crates/core/src/repo.rs`
- Scheduler: see docs/SCHEDULER.md and `crates/core/src/sched.rs`
- CLI surface: see docs/CLI_SURFACE.md (no business logic)
- Repository selection: see docs/REPOSITORIES.md

