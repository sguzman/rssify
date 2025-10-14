# rssify cli

## crates/adapters/cli — command-line adapter

The CLI is a thin, scriptable shell over the `core` domain. It wires user-facing subcommands to `core` traits, returning quiet human output by default and machine-readable JSON when requested. It contains **no business logic** beyond argument parsing, input validation, and calling `core` contracts. 

## What it does

* Maps subcommands (`fetch`, `stats`, `import`, `add`, and future `serve`/`export`) onto `core` traits so the same binary can drive different repository backends. Each subcommand should be implementable in a single file.  
* Honors the repository seam (`--repo fs:<root>` / `--repo sqlite:<dsn>` when available) without reaching around it. 
* Emits structured logs and (when `--json` is set) machine-readable results suitable for pipelines.  

## What does not live here

* No parsing/scheduling/business rules: those belong in `core` and repository adapters. The CLI should not implement logic that belongs to `core`. 

## Subcommands (surface contract)

* `fetch` — quiet by default; `-v/--verbose` enables tracing. Supports `--from feeds.json`, `--one <url>`, `--since <ts>`.  
* `stats` — reads via repo trait only; never touches the network.  
* `import` — ingest URLs from a file, validate/dedup, and produce a canonical `feeds.json`. 
* `add` — add a single feed URL; returns the canonical feed id. 
* JSON mode — every subcommand must return machine-readable JSON when `--json` is set. 

## Expected module shape

* `main.rs` — arg parsing, top-level error handling, and subcommand dispatch. Keeps I/O and side effects here. 
* `cmd_fetch.rs`, `cmd_stats.rs`, `cmd_import.rs`, `cmd_add.rs` — one file per subcommand, each calling `core` traits and returning typed results convertible to JSON. 
* `output.rs` — JSON/human printers; stable keys that mirror domain types. 

## UX & observability

* Quiet by default; `-v/--verbose` adds tracing spans (schedule decision → HTTP → parse → storage). Log keys: `component`, `op`, `feed_id`, `elapsed_ms`, `items`.  
* Security posture: never echo secrets; validate inputs; bound sizes/timeouts; rely on `core` for canonicalization. 

## Testing posture

* Each `cmd_*` file includes focused unit tests using a dummy repo implementation; add a doctest at the module top showing canonical usage. 
* Keep tests and files within the AI-friendly caps; use the header contract template.  

## Contribution notes

* Prefer **one-file PRs** per subcommand; if a change spans multiple files, sequence them as separate commits. Always include a Conventional Commit line.  
* Dependency policy: prefer the latest **working** crate versions on the pinned toolchain; if the newest breaks, pin the newest working and note it. 
