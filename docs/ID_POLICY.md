# Canonical ID Policy

Goal: Every feed and entry gets a stable, portable, backend-agnostic identifier.

## FeedId
- Canonical form: "url:<original_url_trimmed>"
- We do not parse or mutate URLs in core; adapters may normalize but must preserve the original source URL they received.

## EntryId (precedence)
1) If GUID present: "guid:<guid>"
2) Else if canonical link present: "link:<link>"
3) Else: "hash:<u64_hex>" where the hash is over:
   - feed_id.as_str()
   - title (if any)
   - published_ts as decimal string (if any)

Rationale:
- Prefer publisher-provided GUID when available.
- Fallback to canonical link for typical feeds.
- Final fallback is a deterministic hash from stable content.

Notes:
- Hashing uses std DefaultHasher (SipHash) to avoid extra dependencies.
- Policy must remain stable across versions; changes require a migration note.

