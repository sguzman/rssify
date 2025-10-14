# rssify core

## crates/core — domain & contracts

`core` is the project’s stable heart: the domain types, IDs, errors, and trait boundaries that every adapter (CLI, repos, HTTP, etc.) depends on. It is intentionally **pure** (no network/filesystem) and **small-surface** so both humans and AI agents can implement features behind clear seams without dragging in I/O or toolchain complexity.  

## What it does

* Defines the normalized **feed** and **entry** model, plus canonical ID rules to avoid duplicates across formats and mirrors. 
* Exposes the **repository trait(s)** used by storage backends (filesystem JSON, SQLite, etc.) and enforces atomicity/consistency at the boundary. 
* Houses the **scheduler state machine** and decision types used to select per-feed fetch cadence based on HTTP signals, publisher hints, and observed posting behavior. 
* Centralizes **error enums**, **logging key conventions**, and **deterministic contracts** (time/RNG injection via traits) so behaviors are reproducible and testable. 

## What does not live here

No direct HTTP, disk, clocks, or tracing exporters. Those belong to adapters that **depend on** `core` and talk through its traits. Keeping I/O at the edges preserves determinism, shrinks compile surfaces, and simplifies testing.  

## Where `core` is used

* **CLI adapter** maps subcommands like `fetch`, `stats`, and `import` onto `core` traits without re-implementing business logic. 
* **Repository adapters** (e.g., `repos/fs`, `repos/sqlite`) implement the `core` repo trait to persist raw blobs, normalized entries, schedule records, and derivations. 
* **Future adapters** (HTTP service, exporters) read through `core` types to expose APIs and taps without coupling to storage specifics. 

## Key modules (expected shape)

* `ids`: Canonical ID derivation for entries/feeds; golden tests pin stability. 
* `domain`: Core types (Feed, Entry, Enclosure, Derivation, etc.) with invariants documented at the type. 
* `repo`: Repository trait(s) and error semantics (`Conflict`, `NotFound`, `Transient`, etc.). 
* `sched`: Scheduler state/decision enums and cadence heuristics (EWMA, backoff, jitter, publisher hints). 
* `errors` & `observability`: boundary-scoped error enums and structured-log key set. 

## Design principles

* **Trait-first seams, tiny public APIs**: adapters evolve independently while tests target stable contracts. 
* **AI-friendly**: one concept per file, shallow modules, file size caps, and doctests that show canonical usage. 
* **Backwards-aware**: changes to public enums/traits are deliberate and documented; roadmap phases call out when surfaces stabilize.  

## Testing posture

* In-file **unit tests** for each module.
* **Golden/property tests** for IDs and scheduler math; deterministic fixtures for parser/normalizer behavior.
* **Integration/E2E** with in-memory or dummy repo implementations to validate trait contracts. 

## Versioning & stability

`core` aims for early stability in **Phase 0–2** (foundations, fetch+persist MVP, repo abstraction v1). Breaking changes should be rare and accompanied by migration notes once Phase 2 lands. 

## Contribution notes (for this crate)

* Keep files under the soft cap and include the required top-of-file header contract.
* Prefer latest **working** crate versions; if a bump breaks the pinned toolchain, pin to the newest working and note it.
* PRs should be one-file where possible, with a Conventional Commit line.  
