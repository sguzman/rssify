# Phase 1 (MVP) — fetch + persist

Goal: `rssify fetch --from feeds.json --store fs:<root>` runs end-to-end for a tiny fixture set.

## Behaviors (must)
- Conditional HTTP fetch: send If-None-Match/If-Modified-Since when we have etag/last_modified; interpret 200/304.
- Parse minimal RSS/Atom:
  - Required: title, link/url, published(±updated) timestamps, summary or content.
  - Preserve the raw payload (ContentBlob) for the feed's last fetch.
- Canonical IDs:
  - FeedId = url:<feed_url>
  - EntryId = guid:, else link:, else hash: (per docs/ID_POLICY.md)
- Idempotent persistence in FS backend:
  - Create or update Feed and Entry records without duplicates.
  - Deterministic filenames/keys; safe to re-run.
- Basic metrics:
  - elapsed_ms for fetch
  - items written per feed
  - reason strings for NotModified / failures

## Inputs/Outputs
- Input: feeds.json (list of feed URLs and optional titles)
- Output: filesystem repo under fs:<root>

## Non-goals (Phase 2+)
- SQLite backend
- Full content extraction / sanitization
- Retry/backoff policies beyond trivial handling
- Advanced scheduling (only trivial SchedInput will be used)

## Observability
- Structured log keys: component, op, feed_id, elapsed_ms, items
- CLI `--json` prints a summary object per run

