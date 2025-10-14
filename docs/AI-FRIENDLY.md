# AI-friendly Rust Development Guide

## Scope

This file is global guidance for AI/human contributors across repos; project-specific rules live in CONTRIBUTING.md

This guide defines an iteration loop and repo/file conventions that make AI agents reliable at generating or editing one file at a time, with fast convergence through tests.

## Goals

* Minimize context needed per edit.
* Keep changes localized to one file at a time.
* Make seams explicit so agents can implement behind traits.
* Ensure every change is validated by runnable tests.

---

## Global File & API Rules
- File caps: aim for 100–200 LOC per file; refactor when ~300 LOC. Do not split cohesive logic just to hit a number.
- One concept per file; name files after the noun or verb they implement.
- No deep trees: max module depth of 3.
- Keep public APIs tiny; re-export types at the crate root.
- Error types: one error enum per boundary with stable, documented variants.
- Logging: structured logs with a stable set of keys; no println.
- Time and randomness are injected via traits so tests can control them.

---

## Iteration Loop (per task)

1. Pin the scope

   * Choose or create exactly one file to modify.
   * Add or update the top-of-file header contract (template below).
   * Write or update 1-3 focused tests the change must pass.

2. Generate once

   * Ask the AI to edit only that file.
   * Include the header contract and relevant errors/tests in the prompt.
   * If the change touches dependencies, prefer the latest **working** crate versions (update narrowly and verify with `cargo check -q`).

3. Compile and test

   * Run `cargo check -q` then `cargo test -q`.
   * If it fails, paste only the compiler error and the current file back to the AI.
   * If a new crate version fails, fall back to the latest **working** version and note the failure briefly in the PR description.
@@

4. Refactor to boundaries

   * If the file exceeds the soft cap (below), split by responsibility.
   * Introduce a trait boundary when extracting an impl.

5. Document the seam

   * Add concise doc comments and a doctest showing usage.
   * Keep examples minimal but executable.

---

## File Size and Structure Rules

* Soft cap: 300 lines per file.
* Hard cap: 400 lines per file.
* One responsibility per file.
* Prefer descriptive names over explanatory comments.
* Co-locate unit tests with the code they exercise.
* Put broader integration/golden/property tests in `tests/`.

---

## Workspace Layout

Use a workspace to keep crates small and contexts precise.

```
.
├─ Cargo.toml                # [workspace] members = ["crates/*"]
├─ rust-toolchain.toml       # pin toolchain
├─ CONTRIBUTING.md
├─ AI-FRIENDLY.md            # this file
└─ crates/
   ├─ core/
   │  ├─ Cargo.toml
   │  └─ src/
   │     ├─ lib.rs
   │     ├─ domain.rs        # traits, pure logic
   │     └─ algo/
   │        └─ foo.rs
   └─ adapters/
      ├─ cli/
      │  ├─ Cargo.toml
      │  └─ src/main.rs      # uses core traits
      └─ http/
         ├─ Cargo.toml
         └─ src/lib.rs       # HTTP handlers implementing core traits
```

Guidelines:

* Traits live in `core` with minimal, stable method sets.
* Implementations live in adapter crates (CLI, HTTP, DB, FS).
* Keep each crate small and focused to fit in an agent’s working context.

---

## Top-of-File Header Contract (copy/paste template)

```rust
// Contract:
// Purpose: One-sentence purpose of this file/module.
// Inputs/Outputs: Describe function signatures and expected types.
// Invariants: List key invariants or pre/post-conditions.
// Examples: Show 1-2 minimal usage patterns.
// Task: Maintain under 400 LOC. If exceeded, split into new files and extract a trait seam.
// Tests: Must pass unit tests in this file and related doctests.

```

Example for a trait seam:

````rust
/// Provides the abstract interface to fetch items.
/// Example:
/// ```
/// use crates_core::domain::{Fetcher, FetchItem};
/// struct Dummy;
/// impl Fetcher for Dummy {
///     fn fetch(&self, key: &str) -> Result<FetchItem, FetchError> {
///         Ok(FetchItem { key: key.to_string(), bytes: vec![] })
///     }
/// }
/// ```
pub trait Fetcher {
    fn fetch(&self, key: &str) -> Result<FetchItem, FetchError>;
}
````

---

## Testing Conventions

* In-file unit tests right below the code they validate:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_case() {
        // arrange
        // act
        // assert
        assert!(true);
    }
}
```

* Doctests on public functions and traits to show canonical usage.
* Golden or property tests in `tests/`:

  * Golden: use `insta` snapshots to pin expected outputs.
  * Property: use `proptest` for invariants.

Recommended dev commands:

* `cargo check -q`
* `cargo test -q`
* `cargo fmt --all`
* `cargo clippy --all-targets -- -D warnings`

---

## Prompting Conventions for Agents

When asking an AI to edit or generate code, keep prompts tight and file-scoped.

**Edit prompt template:**

```
Edit only: crates/core/src/algo/foo.rs
Constraints:
- Keep file under 300 LOC (hard cap 400).
- Preserve and update the header contract at top.
- Implement trait X and pass tests Y and Z.

Context you may rely on:
- Trait definition: <paste small trait block if needed>
- Current compiler error: <paste error>
- Current file contents: <paste file>
```

**Create-file prompt template:**
Always include a commit message suggestion (required)
At the end of any code/doc proposal, add a single-line Conventional Commit message we can use as-is, e.g.:
- `feat(core): add EWMA clamp to scheduler to reduce jitter`

```
Create: crates/adapters/http/src/handlers/fetch.rs
Goal: HTTP handler that adapts core::domain::Fetcher to GET /v1/fetch?key=...
Constraints:
- Under 300 LOC.
- Do not modify other files.
- Add 2 unit tests with a dummy Fetcher impl.
```

---

## CONTRIBUTING Snippet (AI-Focused)

* One-file PRs whenever possible.
* If a change requires multiple files, sequence them as separate commits and PRs:

  1. Introduce/adjust trait in `core` (with doctest).
  2. Implement adapter in a single file.
  3. Wire-up and integration tests.
* Never exceed 400 LOC per file; split responsibly.
* Keep public APIs trait-first; adapters implement them.
* For errors, prefer enums with clear variants and `thiserror`.

---

## Minimal Cargo Examples

**Workspace Cargo.toml**

```toml
[workspace]
members = [
  "crates/core",
  "crates/adapters/cli",
  "crates/adapters/http",
]
resolver = "2"

[workspace.package]
edition = "2021"
```

**crates/core/Cargo.toml**

```toml
[package]
name = "crates-core"
version = "0.1.0"
edition = "2021"

[dependencies]
thiserror = "1"
```

**crates/adapters/cli/Cargo.toml**

```toml
[package]
name = "adapters-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
crates-core = { path = "../../core" }
clap = { version = "4", features = ["derive"] }
```

---

## Refactor Playbook

* If a file approaches 300 LOC:

  * Identify clusters of functions turning into a concept; extract to a module.
  * If a concept crosses a seam, extract a trait in `core` and move impls to the adapter.
  * Add a small example and a unit test for the extracted piece.

* If a function exceeds ~60-80 LOC:

  * Split into helpers keeping the original name as the orchestration point.
  * Ensure tests cover the orchestration path.

---

## Naming and Docs

* Prefer explicit names: no abbreviations except well-known ones (io, fs, http).
* Comments explain why, not what.
* Add a 3-5 line module-level doc comment to each public module with one doctest.

---

## Validation Checklist

* [ ] File under 300 LOC (hard cap 400).
* [ ] Header contract present and current.
* [ ] Public seam exposed as a trait (if applicable).
* [ ] Unit tests in-file pass.
* [ ] Doctest covers canonical usage.
* [ ] Integration/golden/property tests updated if behavior changed.
* [ ] `cargo fmt`, `clippy -D warnings` clean.
* [ ] Dependencies use the latest working crate versions and compile on the pinned toolchain.
* [ ] A Conventional Commit message suggestion is included with this change.

---

## Example Header Contract Filled

```rust
// Contract:
// Purpose: Parse feed headers and expose a trait-based normalizer.
// Inputs/Outputs: fn parse(input: &str) -> Result<Header, Error>; pure, no I/O.
// Invariants: Header::date is UTC; names trimmed; no panics.
// Examples: see doctest on Header::from_str.
// Task: Keep under 300 LOC; extract trait Normalizer if we add formats.
// Tests: pass tests in this module and doctests for Header.
```

---

## FAQ

* Why not cap at 100 lines per file?

  * It often forces over-fragmentation and harms readability. Aim for 150-300, hard cap 400. Split by responsibility, not by an arbitrary tiny limit.

* Why trait-first?

  * Traits give a stable target for agents, enable test doubles, and let adapters evolve independently.

* Why workspaces?

  * They keep each unit small, cache builds effectively, and isolate contexts for both humans and agents.

---

## Ready-to-use Tasks

* Add a new adapter:

  * Create one file in `crates/adapters/<kind>/src/` implementing an existing core trait.
  * Add 2 unit tests and a doctest on the public entry point.

* Extend core logic:

  * Modify or create one file in `crates/core/src/`.
  * If I/O is needed, introduce a trait here and implement it in an adapter instead.

---

## Agent Quickstart Command Hints

* Build fast: `cargo check -q`
* Run tests: `cargo test -q`
* Format: `cargo fmt --all`
* Lints: `cargo clippy --all-targets -- -D warnings`


---

## Global hard rules

* Always include a one-line Conventional Commit suggestion with any code/doc proposal.”
* Prefer latest compatible, working crate versions; if newest breaks, pin to latest working and note the failure.”

