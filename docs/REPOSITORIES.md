# Repository backends and selection surface

This document describes how to select a repository backend at the CLI boundary and how data is laid out in each backend during Phase 2.

## Selecting a repository with --store

Flag: --store <spec>

Spec grammar:
- spec = "<kind>:<target>"
- kind is case-insensitive. Supported kinds: "fs", "sqlite".
- target must be non-empty. It is not parsed further by the selector, only validated for presence.
- surrounding whitespace is trimmed.

Examples:
- fs:/var/lib/rssify
- Fs:./data
- sqlite:/var/lib/rssify/data.db
- sqlite:./rssify.db

Invalid examples:
- unknown:/somewhere            (kind not supported)
- fs:                            (missing target)
- sqlite                         (missing colon and target)
- nocolon                        (no colon)

Notes:
- The current CLI flag is "--store". A future alias "--repo" may be added, but the stable surface today is "--store".
- The selector only validates shape. Instantiation of a concrete backend happens in later phases.

## Filesystem repository layout (fs:<root>)

All state lives under the provided root directory.

Root structure:
- <root>/feeds/<feed_id_encoded>/feed.json
- <root>/feeds/<feed_id_encoded>/last_blob.bin
- <root>/feeds/<feed_id_encoded>/entries/<entry_id_encoded>.json

Encoding:
- File and directory names use URL-safe percent encoding of the logical IDs.
- Examples:
  - "url:https://example.com/feed" -> "url%3Ahttps%3A%2F%2Fexample.com%2Ffeed"
  - "guid:ABC 123" -> "guid%3AABC%20123"
- Encoding is stable and case-preserving. All non-alphanumeric ASCII other than "_" and "-" should be percent-encoded as "%XX" with uppercase hex.

Feed JSON (feed.json):
```

{
"id": "url:[https://example.com/feed](https://example.com/feed)",
"url": "[https://example.com/feed](https://example.com/feed)",
"title": "Example Feed",
"site_url": "[https://example.com](https://example.com)",
"etag": null,
"last_modified": null,
"active": true
}

```

Last fetch blob (last_blob.bin):
- Opaque bytes related to the most recent network fetch for the feed. Exact semantics will be finalized with the network layer. Stored as raw bytes.

Entry JSON (entries/<entry_id>.json):
```

{
"id": "guid:ABC 123",
"feed_id": "url:[https://example.com/feed](https://example.com/feed)",
"url": "[https://example.com/p/abc](https://example.com/p/abc)",
"title": "Post title",
"published_rfc3339": "2024-08-01T12:34:56Z",
"readable_text": "extracted readable content or null",
"fingerprint": "sha256:...",
"source_label": "example.com"
}

```

Atomicity:
- Writes should be performed using write-temp-then-rename to avoid torn files.
- Multi-entry atomicity is not guaranteed by the filesystem backend. A coarse-grained batch write will be provided via a Tx facade in a later task.

## SQLite repository layout (sqlite:<dsn>)

The SQLite backend stores the same logical information in normalized tables.

Recommended DDL (v1):

```

PRAGMA journal_mode=WAL;
PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS meta (
key TEXT PRIMARY KEY,
value TEXT NOT NULL
);

-- repo schema versioning
INSERT OR REPLACE INTO meta(key, value) VALUES ('repo_version', '1');

CREATE TABLE IF NOT EXISTS feeds (
id TEXT PRIMARY KEY,
url TEXT NOT NULL,
title TEXT,
site_url TEXT,
etag TEXT,
last_modified TEXT,
active INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS entries (
id TEXT PRIMARY KEY,
feed_id TEXT NOT NULL REFERENCES feeds(id) ON DELETE CASCADE,
url TEXT NOT NULL,
title TEXT,
published_rfc3339 TEXT,
readable_text TEXT,
fingerprint TEXT NOT NULL,
created_ts INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);

CREATE INDEX IF NOT EXISTS idx_entries_feed ON entries(feed_id);
CREATE INDEX IF NOT EXISTS idx_entries_fingerprint ON entries(fingerprint);

```

Notes:
- Times are stored as RFC3339 strings at the model level. The "created_ts" is a unix epoch seconds helper for simple range scans.
- The "meta" table holds a small set of key value pairs. "repo_version" is required and set to "1" at initialization.
- The backend must enforce uniqueness of IDs and referential integrity between entries and their feed.

## CLI examples

Human output:
```

rssify fetch --from ./feeds.json --store fs:./data
Processed 12/12 feeds; items parsed=27, written=27

```

JSON output:
```

rssify fetch --from ./feeds.json --store sqlite:./rssify.db --json
{
"feeds_total": 12,
"feeds_processed": 12,
"items_parsed": 27,
"items_written": 27
}

```

## Error handling

Selector errors:
- invalid kind: "unknown:/x"
- missing target: "fs:"
- missing colon: "sqlite"

Backend errors:
- Filesystem: permission denied, path not found, partial writes.
- SQLite: failed to open database file, locking errors, schema mismatch.

Errors should be surfaced to the CLI as readable messages. In JSON mode they should be mapped to a single-line JSON object with "error" and "hint" fields in a later phase.

## Versioning and migrations

- Schema version is tracked as "repo_version" in both backends.
- Version 1 is the initial layout described here.
- Future migrations will bump the version and provide a forward-only upgrade path.
```
