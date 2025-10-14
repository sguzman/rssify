# rssify

A single-binary, Rust-first pipeline for collecting feeds, fetching items, extracting structure (e.g., keywords), and serving/exporting the results in forms other tools can consume. Designed to be AI-friendly: small, trait-first seams; one-file changes; strict line caps; and runnable tests to validate each step. 

---

## Overview

rssify aims to:

* Poll RSS/Atom/JSON feeds and fetch linked items.
* Persist raw artifacts on disk and normalized metadata in a single, queryable fat JSON (and optionally other backends).
* Run in-process, code-level orchestration: fetch -> extract -> persist, all under a trait-first architecture that makes it trivial for agents to implement or swap components.
* Provide a CLI with focused subcommands that do one thing well and compose.
* Respect AI-friendly repo rules: keep changes localized to one file, expose stable trait seams, enforce soft 300-line and hard 400-line caps per file, and keep unit tests close to the code.  

---

## Data model

rssify treats content along three layers:

* Raw: the exact HTML (or bytes) fetched from the entry link, stored on disk for reproducibility.
* Metadata: a normalized record per item (url, title, published timestamp, fetched timestamp, path to raw, extracted main text, fingerprint, source label).
* Derivations: structured outputs such as keyword lists with scores and the name/version of the extractor used.

Principles guiding the model:

* Pure data types live in the core domain; I/O details are adapter-specific and stay behind trait boundaries. 
* Each module has one responsibility and remains under the file line caps; split when approaching the soft cap and document the seam. 

---

## Repository abstraction

To make storage swappable and agent-friendly, rssify defines a thin repository interface in the core domain. Adapters implement it for different modalities (filesystem fat JSON now; SQLite, search indexes, or HTTP later).

Key expectations for the repository seam:

* Methods are small, stable, and trait-first so adapters can evolve independently. 
* One file per adapter implementation where possible; if you need more, split by responsibility and keep each under the soft cap.  
* Document the seam with a short header contract at the top of the file: purpose, inputs/outputs, invariants, examples, and the LOC rule. 

Repository responsibilities:

* load: initialize in-memory cache or indexes from disk.
* save_item: persist or update the normalized item record.
* save_keywords: persist derived keyword records for a given fingerprint.
* exists: fast dedup by fingerprint or canonical URL.

---

## Fetch + extract pipeline

The pipeline orchestrates three pure steps that can be tested and swapped independently:

1. Feed parsing: given a feed URL, parse entries into a uniform entry list.
2. Article fetching and readability: retrieve the linked resource, store raw bytes, and extract readable text for downstream analysis.
3. Derivation: compute per-item features (e.g., keywords), recording the extractor name/version alongside scores.

AI-friendly execution rules:

* Keep orchestration code in its own small module; split helpers when a function grows past ~60-80 lines. 
* Co-locate unit tests with the module implementing each step; add a doctest on the public entry point. 
* If a new algorithm is added, expose it via a trait and implement in a single adapter file first; wire-up can follow as a separate change. 

---

## CLI skeleton

The binary `rssify` exposes subcommands. Each subcommand is small and operates on the repository trait; it should be implementable in a single file and tested with a dummy repo.

* fetch: read feeds.json, fetch and extract items, and persist raw artifacts plus normalized records.
* stats: compute and print counts/aggregates (e.g., items per source, coverage by day).
* import: load a newline-delimited list of URLs and create or append to the fat JSON without network I/O.
* add: add a single URL either as metadata-only or fetch+extract; useful for on-demand enrichment.
* serve: start a tiny HTTP API for read-only access to items and derivations; optionally start in-process cron jobs driven by feed-level or global schedules.
* export: emit derived stats (JSON or CSV) for analysis elsewhere.

Contribution style for the CLI:

* Prefer one-file changes per subcommand.
* Keep public APIs trait-first; the CLI is just an adapter over the core traits. 

---

## How the subcommands map to goals

* fetch → Collection and normalization. Ensures raw + readable text exist for each new entry.
* stats → Visibility into pipeline health and coverage (e.g., per-feed yields, median article size).
* import → Offline seeding from curated URL lists; supports low-network or replay workflows.
* add → Surgical enrichment of a single URL; ideal for quick experiments or backfilling.
* serve → On-demand access via HTTP and optional in-process scheduling without external infra.
* export → Downstream analytics and sharing of results without binding to a specific DB.

---

## Example feeds.json

You can attach optional labels and per-feed cron expressions. If you run `serve` with scheduler enabled, rssify uses these; otherwise, it treats them as metadata.

```json
[
  { "url": "https://www.reuters.com/world/us/rss", "label": "reuters", "cron": "*/20 * * * *" },
  { "url": "https://www.nature.com/nature.rss",    "label": "nature",  "cron": "0 * * * *" }
]
```

---

## Notes and options

* AI-friendly repo shape: traits in core, implementations in adapters; keep crates and files small to fit a model’s working context.  
* Testing posture: co-locate focused unit tests; use doctests to document canonical usage; reserve snapshot/property tests for `tests/`. 
* Refactor playbook: when a file approaches 300 LOC, extract a module or trait seam; split multi-file changes into ordered, one-file PRs.  
* Naming and docs: prefer explicit names; comments explain why, not what; add a short module doc with a doctest. 
* Validation checklist for every file: LOC caps, header contract present, public seam via trait (if applicable), tests and lints clean. 

---

## Contributing

Contributing? See [CONTRIBUTING.md](./docs/CONTRIBUTING.md) for repo rules; see AI-FRIENDLY.md for AI/human iteration practices

---

### Provenance

This README follows your AI-friendly iteration and repo conventions so agents can implement or edit one file at a time with minimal context and fast convergence through tests. 

