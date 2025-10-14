./CHANGELOG.md
```
1  # Changelog
2  
3  ## v0.2.0 - 2025-10-14
4  
```
./README.md
````
  1  # rssify
  2  
  3  A single-binary, Rust-first pipeline for collecting feeds, fetching items, extracting structure (e.g., keywords), and serving/exporting the results in forms other tools can consume. Designed to be AI-friendly: small, trait-first seams; one-file changes; strict line caps; and runnable tests to validate each step. 
  4  
  5  ---
  6  
  7  ## Overview
  8  
  9  rssify aims to:
 10  
 11  * Poll RSS/Atom/JSON feeds and fetch linked items.
 12  * Persist raw artifacts on disk and normalized metadata in a single, queryable fat JSON (and optionally other backends).
 13  * Run in-process, code-level orchestration: fetch -> extract -> persist, all under a trait-first architecture that makes it trivial for agents to implement or swap components.
 14  * Provide a CLI with focused subcommands that do one thing well and compose.
 15  * Respect AI-friendly repo rules: keep changes localized to one file, expose stable trait seams, enforce soft 300-line and hard 400-line caps per file, and keep unit tests close to the code.  
 16  
 17  ---
 18  
 19  ## Data model
 20  
 21  rssify treats content along three layers:
 22  
 23  * Raw: the exact HTML (or bytes) fetched from the entry link, stored on disk for reproducibility.
 24  * Metadata: a normalized record per item (url, title, published timestamp, fetched timestamp, path to raw, extracted main text, fingerprint, source label).
 25  * Derivations: structured outputs such as keyword lists with scores and the name/version of the extractor used.
 26  
 27  Principles guiding the model:
 28  
 29  * Pure data types live in the core domain; I/O details are adapter-specific and stay behind trait boundaries. 
 30  * Each module has one responsibility and remains under the file line caps; split when approaching the soft cap and document the seam. 
 31  
 32  ---
 33  
 34  ## Repository abstraction
 35  
 36  To make storage swappable and agent-friendly, rssify defines a thin repository interface in the core domain. Adapters implement it for different modalities (filesystem fat JSON now; SQLite, search indexes, or HTTP later).
 37  
 38  Key expectations for the repository seam:
 39  
 40  * Methods are small, stable, and trait-first so adapters can evolve independently. 
 41  * One file per adapter implementation where possible; if you need more, split by responsibility and keep each under the soft cap.  
 42  * Document the seam with a short header contract at the top of the file: purpose, inputs/outputs, invariants, examples, and the LOC rule. 
 43  
 44  Repository responsibilities:
 45  
 46  * load: initialize in-memory cache or indexes from disk.
 47  * save_item: persist or update the normalized item record.
 48  * save_keywords: persist derived keyword records for a given fingerprint.
 49  * exists: fast dedup by fingerprint or canonical URL.
 50  
 51  ---
 52  
 53  ## Fetch + extract pipeline
 54  
 55  The pipeline orchestrates three pure steps that can be tested and swapped independently:
 56  
 57  1. Feed parsing: given a feed URL, parse entries into a uniform entry list.
 58  2. Article fetching and readability: retrieve the linked resource, store raw bytes, and extract readable text for downstream analysis.
 59  3. Derivation: compute per-item features (e.g., keywords), recording the extractor name/version alongside scores.
 60  
 61  AI-friendly execution rules:
 62  
 63  * Keep orchestration code in its own small module; split helpers when a function grows past ~60-80 lines. 
 64  * Co-locate unit tests with the module implementing each step; add a doctest on the public entry point. 
 65  * If a new algorithm is added, expose it via a trait and implement in a single adapter file first; wire-up can follow as a separate change. 
 66  
 67  ---
 68  
 69  ## CLI skeleton
 70  
 71  The binary `rssify` exposes subcommands. Each subcommand is small and operates on the repository trait; it should be implementable in a single file and tested with a dummy repo.
 72  
 73  * fetch: read feeds.json, fetch and extract items, and persist raw artifacts plus normalized records.
 74  * stats: compute and print counts/aggregates (e.g., items per source, coverage by day).
 75  * import: load a newline-delimited list of URLs and create or append to the fat JSON without network I/O.
 76  * add: add a single URL either as metadata-only or fetch+extract; useful for on-demand enrichment.
 77  * serve: start a tiny HTTP API for read-only access to items and derivations; optionally start in-process cron jobs driven by feed-level or global schedules.
 78  * export: emit derived stats (JSON or CSV) for analysis elsewhere.
 79  
 80  Contribution style for the CLI:
 81  
 82  * Prefer one-file changes per subcommand.
 83  * Keep public APIs trait-first; the CLI is just an adapter over the core traits. 
 84  
 85  ---
 86  
 87  ## How the subcommands map to goals
 88  
 89  * fetch → Collection and normalization. Ensures raw + readable text exist for each new entry.
 90  * stats → Visibility into pipeline health and coverage (e.g., per-feed yields, median article size).
 91  * import → Offline seeding from curated URL lists; supports low-network or replay workflows.
 92  * add → Surgical enrichment of a single URL; ideal for quick experiments or backfilling.
 93  * serve → On-demand access via HTTP and optional in-process scheduling without external infra.
 94  * export → Downstream analytics and sharing of results without binding to a specific DB.
 95  
 96  ---
 97  
 98  ## Example feeds.json
 99  
100  You can attach optional labels and per-feed cron expressions. If you run `serve` with scheduler enabled, rssify uses these; otherwise, it treats them as metadata.
101  
102  ```json
103  [
104    { "url": "https://www.reuters.com/world/us/rss", "label": "reuters", "cron": "*/20 * * * *" },
105    { "url": "https://www.nature.com/nature.rss",    "label": "nature",  "cron": "0 * * * *" }
106  ]
107  ```
108  
109  ---
110  
111  ## Notes and options
112  
113  * AI-friendly repo shape: traits in core, implementations in adapters; keep crates and files small to fit a model’s working context.  
114  * Testing posture: co-locate focused unit tests; use doctests to document canonical usage; reserve snapshot/property tests for `tests/`. 
115  * Refactor playbook: when a file approaches 300 LOC, extract a module or trait seam; split multi-file changes into ordered, one-file PRs.  
116  * Naming and docs: prefer explicit names; comments explain why, not what; add a short module doc with a doctest. 
117  * Validation checklist for every file: LOC caps, header contract present, public seam via trait (if applicable), tests and lints clean. 
118  
119  ---
120  
121  ### Provenance
122  
123  This README follows your AI-friendly iteration and repo conventions so agents can implement or edit one file at a time with minimal context and fast convergence through tests. 
124  
````
./docs/AI-FRIENDLY.md
`````
  1  # AI-friendly Rust Development Guide
  2  
  3  This guide defines an iteration loop and repo/file conventions that make AI agents reliable at generating or editing one file at a time, with fast convergence through tests.
  4  
  5  ## Goals
  6  
  7  * Minimize context needed per edit.
  8  * Keep changes localized to one file at a time.
  9  * Make seams explicit so agents can implement behind traits.
 10  * Ensure every change is validated by runnable tests.
 11  
 12  ---
 13  
 14  ## Iteration Loop (per task)
 15  
 16  1. Pin the scope
 17  
 18     * Choose or create exactly one file to modify.
 19     * Add or update the top-of-file header contract (template below).
 20     * Write or update 1-3 focused tests the change must pass.
 21  
 22  2. Generate once
 23  
 24     * Ask the AI to edit only that file.
 25     * Include the header contract and relevant errors/tests in the prompt.
 26  
 27  3. Compile and test
 28  
 29     * Run `cargo check -q` then `cargo test -q`.
 30     * If it fails, paste only the compiler error and the current file back to the AI.
 31  
 32  4. Refactor to boundaries
 33  
 34     * If the file exceeds the soft cap (below), split by responsibility.
 35     * Introduce a trait boundary when extracting an impl.
 36  
 37  5. Document the seam
 38  
 39     * Add concise doc comments and a doctest showing usage.
 40     * Keep examples minimal but executable.
 41  
 42  ---
 43  
 44  ## File Size and Structure Rules
 45  
 46  * Soft cap: 300 lines per file.
 47  * Hard cap: 400 lines per file.
 48  * One responsibility per file.
 49  * Prefer descriptive names over explanatory comments.
 50  * Co-locate unit tests with the code they exercise.
 51  * Put broader integration/golden/property tests in `tests/`.
 52  
 53  ---
 54  
 55  ## Workspace Layout
 56  
 57  Use a workspace to keep crates small and contexts precise.
 58  
 59  ```
 60  .
 61  ├─ Cargo.toml                # [workspace] members = ["crates/*"]
 62  ├─ rust-toolchain.toml       # pin toolchain
 63  ├─ CONTRIBUTING.md
 64  ├─ AI-FRIENDLY.md            # this file
 65  └─ crates/
 66     ├─ core/
 67     │  ├─ Cargo.toml
 68     │  └─ src/
 69     │     ├─ lib.rs
 70     │     ├─ domain.rs        # traits, pure logic
 71     │     └─ algo/
 72     │        └─ foo.rs
 73     └─ adapters/
 74        ├─ cli/
 75        │  ├─ Cargo.toml
 76        │  └─ src/main.rs      # uses core traits
 77        └─ http/
 78           ├─ Cargo.toml
 79           └─ src/lib.rs       # HTTP handlers implementing core traits
 80  ```
 81  
 82  Guidelines:
 83  
 84  * Traits live in `core` with minimal, stable method sets.
 85  * Implementations live in adapter crates (CLI, HTTP, DB, FS).
 86  * Keep each crate small and focused to fit in an agent’s working context.
 87  
 88  ---
 89  
 90  ## Top-of-File Header Contract (copy/paste template)
 91  
 92  ```rust
 93  // Contract:
 94  // Purpose: One-sentence purpose of this file/module.
 95  // Inputs/Outputs: Describe function signatures and expected types.
 96  // Invariants: List key invariants or pre/post-conditions.
 97  // Examples: Show 1-2 minimal usage patterns.
 98  // Task: Maintain under 400 LOC. If exceeded, split into new files and extract a trait seam.
 99  // Tests: Must pass unit tests in this file and related doctests.
100  
101  ```
102  
103  Example for a trait seam:
104  
105  ````rust
106  /// Provides the abstract interface to fetch items.
107  /// Example:
108  /// ```
109  /// use crates_core::domain::{Fetcher, FetchItem};
110  /// struct Dummy;
111  /// impl Fetcher for Dummy {
112  ///     fn fetch(&self, key: &str) -> Result<FetchItem, FetchError> {
113  ///         Ok(FetchItem { key: key.to_string(), bytes: vec![] })
114  ///     }
115  /// }
116  /// ```
117  pub trait Fetcher {
118      fn fetch(&self, key: &str) -> Result<FetchItem, FetchError>;
119  }
120  ````
121  
122  ---
123  
124  ## Testing Conventions
125  
126  * In-file unit tests right below the code they validate:
127  
128  ```rust
129  #[cfg(test)]
130  mod tests {
131      use super::*;
132  
133      #[test]
134      fn parses_minimal_case() {
135          // arrange
136          // act
137          // assert
138          assert!(true);
139      }
140  }
141  ```
142  
143  * Doctests on public functions and traits to show canonical usage.
144  * Golden or property tests in `tests/`:
145  
146    * Golden: use `insta` snapshots to pin expected outputs.
147    * Property: use `proptest` for invariants.
148  
149  Recommended dev commands:
150  
151  * `cargo check -q`
152  * `cargo test -q`
153  * `cargo fmt --all`
154  * `cargo clippy --all-targets -- -D warnings`
155  
156  ---
157  
158  ## Prompting Conventions for Agents
159  
160  When asking an AI to edit or generate code, keep prompts tight and file-scoped.
161  
162  **Edit prompt template:**
163  
164  ```
165  Edit only: crates/core/src/algo/foo.rs
166  Constraints:
167  - Keep file under 300 LOC (hard cap 400).
168  - Preserve and update the header contract at top.
169  - Implement trait X and pass tests Y and Z.
170  
171  Context you may rely on:
172  - Trait definition: <paste small trait block if needed>
173  - Current compiler error: <paste error>
174  - Current file contents: <paste file>
175  ```
176  
177  **Create-file prompt template:**
178  
179  ```
180  Create: crates/adapters/http/src/handlers/fetch.rs
181  Goal: HTTP handler that adapts core::domain::Fetcher to GET /v1/fetch?key=...
182  Constraints:
183  - Under 300 LOC.
184  - Do not modify other files.
185  - Add 2 unit tests with a dummy Fetcher impl.
186  ```
187  
188  ---
189  
190  ## CONTRIBUTING Snippet (AI-Focused)
191  
192  * One-file PRs whenever possible.
193  * If a change requires multiple files, sequence them as separate commits and PRs:
194  
195    1. Introduce/adjust trait in `core` (with doctest).
196    2. Implement adapter in a single file.
197    3. Wire-up and integration tests.
198  * Never exceed 400 LOC per file; split responsibly.
199  * Keep public APIs trait-first; adapters implement them.
200  * For errors, prefer enums with clear variants and `thiserror`.
201  
202  ---
203  
204  ## Minimal Cargo Examples
205  
206  **Workspace Cargo.toml**
207  
208  ```toml
209  [workspace]
210  members = [
211    "crates/core",
212    "crates/adapters/cli",
213    "crates/adapters/http",
214  ]
215  resolver = "2"
216  
217  [workspace.package]
218  edition = "2021"
219  ```
220  
221  **crates/core/Cargo.toml**
222  
223  ```toml
224  [package]
225  name = "crates-core"
226  version = "0.1.0"
227  edition = "2021"
228  
229  [dependencies]
230  thiserror = "1"
231  ```
232  
233  **crates/adapters/cli/Cargo.toml**
234  
235  ```toml
236  [package]
237  name = "adapters-cli"
238  version = "0.1.0"
239  edition = "2021"
240  
241  [dependencies]
242  crates-core = { path = "../../core" }
243  clap = { version = "4", features = ["derive"] }
244  ```
245  
246  ---
247  
248  ## Refactor Playbook
249  
250  * If a file approaches 300 LOC:
251  
252    * Identify clusters of functions turning into a concept; extract to a module.
253    * If a concept crosses a seam, extract a trait in `core` and move impls to the adapter.
254    * Add a small example and a unit test for the extracted piece.
255  
256  * If a function exceeds ~60-80 LOC:
257  
258    * Split into helpers keeping the original name as the orchestration point.
259    * Ensure tests cover the orchestration path.
260  
261  ---
262  
263  ## Naming and Docs
264  
265  * Prefer explicit names: no abbreviations except well-known ones (io, fs, http).
266  * Comments explain why, not what.
267  * Add a 3-5 line module-level doc comment to each public module with one doctest.
268  
269  ---
270  
271  ## Validation Checklist
272  
273  * [ ] File under 300 LOC (hard cap 400).
274  * [ ] Header contract present and current.
275  * [ ] Public seam exposed as a trait (if applicable).
276  * [ ] Unit tests in-file pass.
277  * [ ] Doctest covers canonical usage.
278  * [ ] Integration/golden/property tests updated if behavior changed.
279  * [ ] `cargo fmt`, `clippy -D warnings` clean.
280  
281  ---
282  
283  ## Example Header Contract Filled
284  
285  ```rust
286  // Contract:
287  // Purpose: Parse feed headers and expose a trait-based normalizer.
288  // Inputs/Outputs: fn parse(input: &str) -> Result<Header, Error>; pure, no I/O.
289  // Invariants: Header::date is UTC; names trimmed; no panics.
290  // Examples: see doctest on Header::from_str.
291  // Task: Keep under 300 LOC; extract trait Normalizer if we add formats.
292  // Tests: pass tests in this module and doctests for Header.
293  ```
294  
295  ---
296  
297  ## FAQ
298  
299  * Why not cap at 100 lines per file?
300  
301    * It often forces over-fragmentation and harms readability. Aim for 150-300, hard cap 400. Split by responsibility, not by an arbitrary tiny limit.
302  
303  * Why trait-first?
304  
305    * Traits give a stable target for agents, enable test doubles, and let adapters evolve independently.
306  
307  * Why workspaces?
308  
309    * They keep each unit small, cache builds effectively, and isolate contexts for both humans and agents.
310  
311  ---
312  
313  ## Ready-to-use Tasks
314  
315  * Add a new adapter:
316  
317    * Create one file in `crates/adapters/<kind>/src/` implementing an existing core trait.
318    * Add 2 unit tests and a doctest on the public entry point.
319  
320  * Extend core logic:
321  
322    * Modify or create one file in `crates/core/src/`.
323    * If I/O is needed, introduce a trait here and implement it in an adapter instead.
324  
325  ---
326  
327  ## Agent Quickstart Command Hints
328  
329  * Build fast: `cargo check -q`
330  * Run tests: `cargo test -q`
331  * Format: `cargo fmt --all`
332  * Lints: `cargo clippy --all-targets -- -D warnings`
`````
./docs/ALL.md
``````
  1  ./CHANGELOG.md
  2  ```
  3  1  # Changelog
  4  2  
  5  3  ## v0.2.0 - 2025-10-14
  6  4  
  7  ```
  8  ./README.md
  9  ````
 10    1  # rssify
 11    2  
 12    3  A single-binary, Rust-first pipeline for collecting feeds, fetching items, extracting structure (e.g., keywords), and serving/exporting the results in forms other tools can consume. Designed to be AI-friendly: small, trait-first seams; one-file changes; strict line caps; and runnable tests to validate each step. 
 13    4  
 14    5  ---
 15    6  
 16    7  ## Overview
 17    8  
 18    9  rssify aims to:
 19   10  
 20   11  * Poll RSS/Atom/JSON feeds and fetch linked items.
 21   12  * Persist raw artifacts on disk and normalized metadata in a single, queryable fat JSON (and optionally other backends).
 22   13  * Run in-process, code-level orchestration: fetch -> extract -> persist, all under a trait-first architecture that makes it trivial for agents to implement or swap components.
 23   14  * Provide a CLI with focused subcommands that do one thing well and compose.
 24   15  * Respect AI-friendly repo rules: keep changes localized to one file, expose stable trait seams, enforce soft 300-line and hard 400-line caps per file, and keep unit tests close to the code.  
 25   16  
 26   17  ---
 27   18  
 28   19  ## Data model
 29   20  
 30   21  rssify treats content along three layers:
 31   22  
 32   23  * Raw: the exact HTML (or bytes) fetched from the entry link, stored on disk for reproducibility.
 33   24  * Metadata: a normalized record per item (url, title, published timestamp, fetched timestamp, path to raw, extracted main text, fingerprint, source label).
 34   25  * Derivations: structured outputs such as keyword lists with scores and the name/version of the extractor used.
 35   26  
 36   27  Principles guiding the model:
 37   28  
 38   29  * Pure data types live in the core domain; I/O details are adapter-specific and stay behind trait boundaries. 
 39   30  * Each module has one responsibility and remains under the file line caps; split when approaching the soft cap and document the seam. 
 40   31  
 41   32  ---
 42   33  
 43   34  ## Repository abstraction
 44   35  
 45   36  To make storage swappable and agent-friendly, rssify defines a thin repository interface in the core domain. Adapters implement it for different modalities (filesystem fat JSON now; SQLite, search indexes, or HTTP later).
 46   37  
 47   38  Key expectations for the repository seam:
 48   39  
 49   40  * Methods are small, stable, and trait-first so adapters can evolve independently. 
 50   41  * One file per adapter implementation where possible; if you need more, split by responsibility and keep each under the soft cap.  
 51   42  * Document the seam with a short header contract at the top of the file: purpose, inputs/outputs, invariants, examples, and the LOC rule. 
 52   43  
 53   44  Repository responsibilities:
 54   45  
 55   46  * load: initialize in-memory cache or indexes from disk.
 56   47  * save_item: persist or update the normalized item record.
 57   48  * save_keywords: persist derived keyword records for a given fingerprint.
 58   49  * exists: fast dedup by fingerprint or canonical URL.
 59   50  
 60   51  ---
 61   52  
 62   53  ## Fetch + extract pipeline
 63   54  
 64   55  The pipeline orchestrates three pure steps that can be tested and swapped independently:
 65   56  
 66   57  1. Feed parsing: given a feed URL, parse entries into a uniform entry list.
 67   58  2. Article fetching and readability: retrieve the linked resource, store raw bytes, and extract readable text for downstream analysis.
 68   59  3. Derivation: compute per-item features (e.g., keywords), recording the extractor name/version alongside scores.
 69   60  
 70   61  AI-friendly execution rules:
 71   62  
 72   63  * Keep orchestration code in its own small module; split helpers when a function grows past ~60-80 lines. 
 73   64  * Co-locate unit tests with the module implementing each step; add a doctest on the public entry point. 
 74   65  * If a new algorithm is added, expose it via a trait and implement in a single adapter file first; wire-up can follow as a separate change. 
 75   66  
 76   67  ---
 77   68  
 78   69  ## CLI skeleton
 79   70  
 80   71  The binary `rssify` exposes subcommands. Each subcommand is small and operates on the repository trait; it should be implementable in a single file and tested with a dummy repo.
 81   72  
 82   73  * fetch: read feeds.json, fetch and extract items, and persist raw artifacts plus normalized records.
 83   74  * stats: compute and print counts/aggregates (e.g., items per source, coverage by day).
 84   75  * import: load a newline-delimited list of URLs and create or append to the fat JSON without network I/O.
 85   76  * add: add a single URL either as metadata-only or fetch+extract; useful for on-demand enrichment.
 86   77  * serve: start a tiny HTTP API for read-only access to items and derivations; optionally start in-process cron jobs driven by feed-level or global schedules.
 87   78  * export: emit derived stats (JSON or CSV) for analysis elsewhere.
 88   79  
 89   80  Contribution style for the CLI:
 90   81  
 91   82  * Prefer one-file changes per subcommand.
 92   83  * Keep public APIs trait-first; the CLI is just an adapter over the core traits. 
 93   84  
 94   85  ---
 95   86  
 96   87  ## How the subcommands map to goals
 97   88  
 98   89  * fetch → Collection and normalization. Ensures raw + readable text exist for each new entry.
 99   90  * stats → Visibility into pipeline health and coverage (e.g., per-feed yields, median article size).
100   91  * import → Offline seeding from curated URL lists; supports low-network or replay workflows.
101   92  * add → Surgical enrichment of a single URL; ideal for quick experiments or backfilling.
102   93  * serve → On-demand access via HTTP and optional in-process scheduling without external infra.
103   94  * export → Downstream analytics and sharing of results without binding to a specific DB.
104   95  
105   96  ---
106   97  
107   98  ## Example feeds.json
108   99  
109  100  You can attach optional labels and per-feed cron expressions. If you run `serve` with scheduler enabled, rssify uses these; otherwise, it treats them as metadata.
110  101  
111  102  ```json
112  103  [
113  104    { "url": "https://www.reuters.com/world/us/rss", "label": "reuters", "cron": "*/20 * * * *" },
114  105    { "url": "https://www.nature.com/nature.rss",    "label": "nature",  "cron": "0 * * * *" }
115  106  ]
116  107  ```
117  108  
118  109  ---
119  110  
120  111  ## Notes and options
121  112  
122  113  * AI-friendly repo shape: traits in core, implementations in adapters; keep crates and files small to fit a model’s working context.  
123  114  * Testing posture: co-locate focused unit tests; use doctests to document canonical usage; reserve snapshot/property tests for `tests/`. 
124  115  * Refactor playbook: when a file approaches 300 LOC, extract a module or trait seam; split multi-file changes into ordered, one-file PRs.  
125  116  * Naming and docs: prefer explicit names; comments explain why, not what; add a short module doc with a doctest. 
126  117  * Validation checklist for every file: LOC caps, header contract present, public seam via trait (if applicable), tests and lints clean. 
127  118  
128  119  ---
129  120  
130  121  ### Provenance
131  122  
132  123  This README follows your AI-friendly iteration and repo conventions so agents can implement or edit one file at a time with minimal context and fast convergence through tests. 
133  124  
134  ````
135  ./docs/AI-FRIENDLY.md
136  `````
137    1  # AI-friendly Rust Development Guide
138    2  
139    3  This guide defines an iteration loop and repo/file conventions that make AI agents reliable at generating or editing one file at a time, with fast convergence through tests.
140    4  
141    5  ## Goals
142    6  
143    7  * Minimize context needed per edit.
144    8  * Keep changes localized to one file at a time.
145    9  * Make seams explicit so agents can implement behind traits.
146   10  * Ensure every change is validated by runnable tests.
147   11  
148   12  ---
149   13  
150   14  ## Iteration Loop (per task)
151   15  
152   16  1. Pin the scope
153   17  
154   18     * Choose or create exactly one file to modify.
155   19     * Add or update the top-of-file header contract (template below).
156   20     * Write or update 1-3 focused tests the change must pass.
157   21  
158   22  2. Generate once
159   23  
160   24     * Ask the AI to edit only that file.
161   25     * Include the header contract and relevant errors/tests in the prompt.
162   26  
163   27  3. Compile and test
164   28  
165   29     * Run `cargo check -q` then `cargo test -q`.
166   30     * If it fails, paste only the compiler error and the current file back to the AI.
167   31  
168   32  4. Refactor to boundaries
169   33  
170   34     * If the file exceeds the soft cap (below), split by responsibility.
171   35     * Introduce a trait boundary when extracting an impl.
172   36  
173   37  5. Document the seam
174   38  
175   39     * Add concise doc comments and a doctest showing usage.
176   40     * Keep examples minimal but executable.
177   41  
178   42  ---
179   43  
180   44  ## File Size and Structure Rules
181   45  
182   46  * Soft cap: 300 lines per file.
183   47  * Hard cap: 400 lines per file.
184   48  * One responsibility per file.
185   49  * Prefer descriptive names over explanatory comments.
186   50  * Co-locate unit tests with the code they exercise.
187   51  * Put broader integration/golden/property tests in `tests/`.
188   52  
189   53  ---
190   54  
191   55  ## Workspace Layout
192   56  
193   57  Use a workspace to keep crates small and contexts precise.
194   58  
195   59  ```
196   60  .
197   61  ├─ Cargo.toml                # [workspace] members = ["crates/*"]
198   62  ├─ rust-toolchain.toml       # pin toolchain
199   63  ├─ CONTRIBUTING.md
200   64  ├─ AI-FRIENDLY.md            # this file
201   65  └─ crates/
202   66     ├─ core/
203   67     │  ├─ Cargo.toml
204   68     │  └─ src/
205   69     │     ├─ lib.rs
206   70     │     ├─ domain.rs        # traits, pure logic
207   71     │     └─ algo/
208   72     │        └─ foo.rs
209   73     └─ adapters/
210   74        ├─ cli/
211   75        │  ├─ Cargo.toml
212   76        │  └─ src/main.rs      # uses core traits
213   77        └─ http/
214   78           ├─ Cargo.toml
215   79           └─ src/lib.rs       # HTTP handlers implementing core traits
216   80  ```
217   81  
218   82  Guidelines:
219   83  
220   84  * Traits live in `core` with minimal, stable method sets.
221   85  * Implementations live in adapter crates (CLI, HTTP, DB, FS).
222   86  * Keep each crate small and focused to fit in an agent’s working context.
223   87  
224   88  ---
225   89  
226   90  ## Top-of-File Header Contract (copy/paste template)
227   91  
228   92  ```rust
229   93  // Contract:
230   94  // Purpose: One-sentence purpose of this file/module.
231   95  // Inputs/Outputs: Describe function signatures and expected types.
232   96  // Invariants: List key invariants or pre/post-conditions.
233   97  // Examples: Show 1-2 minimal usage patterns.
234   98  // Task: Maintain under 400 LOC. If exceeded, split into new files and extract a trait seam.
235   99  // Tests: Must pass unit tests in this file and related doctests.
236  100  
237  101  ```
238  102  
239  103  Example for a trait seam:
240  104  
241  105  ````rust
242  106  /// Provides the abstract interface to fetch items.
243  107  /// Example:
244  108  /// ```
245  109  /// use crates_core::domain::{Fetcher, FetchItem};
246  110  /// struct Dummy;
247  111  /// impl Fetcher for Dummy {
248  112  ///     fn fetch(&self, key: &str) -> Result<FetchItem, FetchError> {
249  113  ///         Ok(FetchItem { key: key.to_string(), bytes: vec![] })
250  114  ///     }
251  115  /// }
252  116  /// ```
253  117  pub trait Fetcher {
254  118      fn fetch(&self, key: &str) -> Result<FetchItem, FetchError>;
255  119  }
256  120  ````
257  121  
258  122  ---
259  123  
260  124  ## Testing Conventions
261  125  
262  126  * In-file unit tests right below the code they validate:
263  127  
264  128  ```rust
265  129  #[cfg(test)]
266  130  mod tests {
267  131      use super::*;
268  132  
269  133      #[test]
270  134      fn parses_minimal_case() {
271  135          // arrange
272  136          // act
273  137          // assert
274  138          assert!(true);
275  139      }
276  140  }
277  141  ```
278  142  
279  143  * Doctests on public functions and traits to show canonical usage.
280  144  * Golden or property tests in `tests/`:
281  145  
282  146    * Golden: use `insta` snapshots to pin expected outputs.
283  147    * Property: use `proptest` for invariants.
284  148  
285  149  Recommended dev commands:
286  150  
287  151  * `cargo check -q`
288  152  * `cargo test -q`
289  153  * `cargo fmt --all`
290  154  * `cargo clippy --all-targets -- -D warnings`
291  155  
292  156  ---
293  157  
294  158  ## Prompting Conventions for Agents
295  159  
296  160  When asking an AI to edit or generate code, keep prompts tight and file-scoped.
297  161  
298  162  **Edit prompt template:**
299  163  
300  164  ```
301  165  Edit only: crates/core/src/algo/foo.rs
302  166  Constraints:
303  167  - Keep file under 300 LOC (hard cap 400).
304  168  - Preserve and update the header contract at top.
305  169  - Implement trait X and pass tests Y and Z.
306  170  
307  171  Context you may rely on:
308  172  - Trait definition: <paste small trait block if needed>
309  173  - Current compiler error: <paste error>
310  174  - Current file contents: <paste file>
311  175  ```
312  176  
313  177  **Create-file prompt template:**
314  178  
315  179  ```
316  180  Create: crates/adapters/http/src/handlers/fetch.rs
317  181  Goal: HTTP handler that adapts core::domain::Fetcher to GET /v1/fetch?key=...
318  182  Constraints:
319  183  - Under 300 LOC.
320  184  - Do not modify other files.
321  185  - Add 2 unit tests with a dummy Fetcher impl.
322  186  ```
323  187  
324  188  ---
325  189  
326  190  ## CONTRIBUTING Snippet (AI-Focused)
327  191  
328  192  * One-file PRs whenever possible.
329  193  * If a change requires multiple files, sequence them as separate commits and PRs:
330  194  
331  195    1. Introduce/adjust trait in `core` (with doctest).
332  196    2. Implement adapter in a single file.
333  197    3. Wire-up and integration tests.
334  198  * Never exceed 400 LOC per file; split responsibly.
335  199  * Keep public APIs trait-first; adapters implement them.
336  200  * For errors, prefer enums with clear variants and `thiserror`.
337  201  
338  202  ---
339  203  
340  204  ## Minimal Cargo Examples
341  205  
342  206  **Workspace Cargo.toml**
343  207  
344  208  ```toml
345  209  [workspace]
346  210  members = [
347  211    "crates/core",
348  212    "crates/adapters/cli",
349  213    "crates/adapters/http",
350  214  ]
351  215  resolver = "2"
352  216  
353  217  [workspace.package]
354  218  edition = "2021"
355  219  ```
356  220  
357  221  **crates/core/Cargo.toml**
358  222  
359  223  ```toml
360  224  [package]
361  225  name = "crates-core"
362  226  version = "0.1.0"
363  227  edition = "2021"
364  228  
365  229  [dependencies]
366  230  thiserror = "1"
367  231  ```
368  232  
369  233  **crates/adapters/cli/Cargo.toml**
370  234  
371  235  ```toml
372  236  [package]
373  237  name = "adapters-cli"
374  238  version = "0.1.0"
375  239  edition = "2021"
376  240  
377  241  [dependencies]
378  242  crates-core = { path = "../../core" }
379  243  clap = { version = "4", features = ["derive"] }
380  244  ```
381  245  
382  246  ---
383  247  
384  248  ## Refactor Playbook
385  249  
386  250  * If a file approaches 300 LOC:
387  251  
388  252    * Identify clusters of functions turning into a concept; extract to a module.
389  253    * If a concept crosses a seam, extract a trait in `core` and move impls to the adapter.
390  254    * Add a small example and a unit test for the extracted piece.
391  255  
392  256  * If a function exceeds ~60-80 LOC:
393  257  
394  258    * Split into helpers keeping the original name as the orchestration point.
395  259    * Ensure tests cover the orchestration path.
396  260  
397  261  ---
398  262  
399  263  ## Naming and Docs
400  264  
401  265  * Prefer explicit names: no abbreviations except well-known ones (io, fs, http).
402  266  * Comments explain why, not what.
403  267  * Add a 3-5 line module-level doc comment to each public module with one doctest.
404  268  
405  269  ---
406  270  
407  271  ## Validation Checklist
408  272  
409  273  * [ ] File under 300 LOC (hard cap 400).
410  274  * [ ] Header contract present and current.
411  275  * [ ] Public seam exposed as a trait (if applicable).
412  276  * [ ] Unit tests in-file pass.
413  277  * [ ] Doctest covers canonical usage.
414  278  * [ ] Integration/golden/property tests updated if behavior changed.
415  279  * [ ] `cargo fmt`, `clippy -D warnings` clean.
416  280  
417  281  ---
418  282  
419  283  ## Example Header Contract Filled
420  284  
421  285  ```rust
422  286  // Contract:
423  287  // Purpose: Parse feed headers and expose a trait-based normalizer.
424  288  // Inputs/Outputs: fn parse(input: &str) -> Result<Header, Error>; pure, no I/O.
425  289  // Invariants: Header::date is UTC; names trimmed; no panics.
426  290  // Examples: see doctest on Header::from_str.
427  291  // Task: Keep under 300 LOC; extract trait Normalizer if we add formats.
428  292  // Tests: pass tests in this module and doctests for Header.
429  293  ```
430  294  
431  295  ---
432  296  
433  297  ## FAQ
434  298  
435  299  * Why not cap at 100 lines per file?
436  300  
437  301    * It often forces over-fragmentation and harms readability. Aim for 150-300, hard cap 400. Split by responsibility, not by an arbitrary tiny limit.
438  302  
439  303  * Why trait-first?
440  304  
441  305    * Traits give a stable target for agents, enable test doubles, and let adapters evolve independently.
442  306  
443  307  * Why workspaces?
444  308  
445  309    * They keep each unit small, cache builds effectively, and isolate contexts for both humans and agents.
446  310  
447  311  ---
448  312  
449  313  ## Ready-to-use Tasks
450  314  
451  315  * Add a new adapter:
452  316  
453  317    * Create one file in `crates/adapters/<kind>/src/` implementing an existing core trait.
454  318    * Add 2 unit tests and a doctest on the public entry point.
455  319  
456  320  * Extend core logic:
457  321  
458  322    * Modify or create one file in `crates/core/src/`.
459  323    * If I/O is needed, introduce a trait here and implement it in an adapter instead.
460  324  
461  325  ---
462  326  
463  327  ## Agent Quickstart Command Hints
464  328  
465  329  * Build fast: `cargo check -q`
466  330  * Run tests: `cargo test -q`
467  331  * Format: `cargo fmt --all`
468  332  * Lints: `cargo clippy --all-targets -- -D warnings`
469  `````
``````
./docs/ARCHITECTURE.md
````
  1  # RSS Fetcher Architecture
  2  
  3  Status: draft
  4  Owners: you
  5  Scope: fetching and scheduling for RSS/Atom feeds; no reader UI
  6  
  7  ## 0. Executive summary
  8  
  9  This service discovers, fetches, and parses syndicated feeds (RSS 2.0, Atom), stores both the raw responses and normalized entries, and exposes the results to other programs via simple interfaces. A smart, adaptive scheduler sets per-feed fetch cadence using a blend of publisher hints, HTTP signals, and observed posting behavior.
 10  
 11  Non-goals: rendering/reading UI; social features; ML text classification; search UI.
 12  
 13  ---
 14  
 15  ## 1. Goals and constraints
 16  
 17  * Fetch reliably, at scale, without hammering hosts.
 18  * Preserve raw data exactly as served for reproducibility and future re-parsing.
 19  * Provide a clean, stable normalized model for downstream consumers.
 20  * Adapt fetch frequency per feed automatically.
 21  * Be robust against malformed XML and hostile inputs.
 22  * Keep operational visibility high: metrics, logs, sampling, tracing.
 23  
 24  Operational constraints:
 25  
 26  * Stateless workers where practical; persistence for feeds, payloads, and schedules.
 27  * Horizontal scalability; per-host concurrency ceilings and backoff.
 28  * Deterministic behavior under retry/restart.
 29  
 30  ---
 31  
 32  ## 2. Key terms
 33  
 34  * Feed: a URL returning RSS or Atom XML.
 35  * Entry/Item: a single post within a feed.
 36  * Raw payload: exact bytes + headers from an HTTP response.
 37  * Normalized entry: JSON record with consistent fields across RSS/Atom.
 38  * Schedule state: the per-feed timer decision and rationale.
 39  
 40  ---
 41  
 42  ## 3. High-level architecture
 43  
 44  ```
 45  +------------------+     +-------------------+     +-------------------+
 46  | Subscription DB  | --> | Scheduler         | --> | Fetcher Workers   |
 47  | feeds, state     |     | per-feed cadence  |     | HTTP + parsing    |
 48  +------------------+     +-------------------+     +-------------------+
 49            |                          |                         |
 50            v                          v                         v
 51  +------------------+     +-------------------+     +-------------------+
 52  | Raw Store        |     | Entry Extractor   |     | Normalized Store  |
 53  | bytes + headers  | <-- | XML -> fragments  | --> | entries + indexes |
 54  +------------------+     +-------------------+     +-------------------+
 55            ^                                                     |
 56            |                                                     v
 57       +----------+                                       +----------------+
 58       | Events   |  NewEntry(feed_id, entry_id, ...)     | Public APIs    |
 59       | bus/queue| ------------------------------------> | REST/gRPC/NDJSON|
 60       +----------+                                       +----------------+
 61  ```
 62  
 63  ---
 64  
 65  ## 4. Components
 66  
 67  ### 4.1 Subscription & discovery
 68  
 69  * Accept explicit feed URLs (primary).
 70  * Optional: site autodiscovery by scanning HTML for
 71    link rel="alternate" type="application/rss+xml" or "application/atom+xml".
 72  * Optional: sitemap seeding if you add crawler features later.
 73  * Track feed metadata: content type, validators (ETag, Last-Modified), schedule state, last fetch, last success/failure, error budget.
 74  
 75  ### 4.2 Fetch layer (HTTP)
 76  
 77  * Conditional GET: If-None-Match/ETag and If-Modified-Since/Last-Modified.
 78  * Handle 304 Not Modified, 429/503 with Retry-After, gzip/br compression, timeouts, redirects.
 79  * Per-host connection pooling and concurrency caps.
 80  * Request and response headers fully recorded with the payload.
 81  
 82  ### 4.3 Parsing & normalization
 83  
 84  * Hardened XML parsing with DTD/XXE disabled.
 85  * Detect format (RSS 2.0 vs Atom) and parse:
 86  
 87    * Identity: guid/id (stable), link (canonical), title.
 88    * Timestamps: published, updated (Atom often reliable), first_seen.
 89    * Content: summary, content (HTML/text), authors, categories.
 90    * Media: enclosure(s) with type/length/url.
 91  * Normalization rules:
 92  
 93    * Prefer Atom id; else RSS guid; else stable hash fallback.
 94    * Canonicalize URLs (strip fragments, normalize scheme/host as policy dictates).
 95    * Normalize timestamps to UTC RFC3339.
 96  
 97  ### 4.4 Deduplication
 98  
 99  * Primary key: entry_uid computed by:
100  
101    1. format_native_id if present and stable,
102    2. else SHA-256 over canonical tuple: (canon_link, title, published_ts, feed_id).
103  * Keep lineage: first_seen, last_seen, seen_count, last_hash.
104  
105  ### 4.5 Storage
106  
107  * Raw store (immutable):
108  
109    * Object: feed_id, fetched_at, http_status, headers, body_bytes, content_hash.
110    * Index by feed_id + fetched_at. Consider content-addressed storage.
111  * Normalized store:
112  
113    * Feeds: id, url, type, validators, schedule_state, stats, last_fetch, last_result.
114    * Entries: entry_uid, feed_id, canonical_link, title, summary, content, authors[],
115      categories[], published, updated, enclosures[], first_seen, last_seen, raw_refs[].
116    * Entry raw fragments: store per-entry raw XML subtree snapshots when feasible.
117  
118  ### 4.6 Smart scheduler
119  
120  * Inputs:
121  
122    * Publisher hints: RSS ttl (minutes), skipHours, skipDays.
123    * HTTP signals: 304 Not Modified, Retry-After, recent status codes.
124    * Observed cadence: inter-arrival stats from the last N entries.
125    * Safety floors/ceilings and jitter.
126  
127  * Baseline algorithm:
128  
129    * Start interval I0 = 15m (configurable).
130    * If feed declares ttl, clamp I to [ttl, max_interval].
131    * On 304, increase interval by factor b_up (e.g., 1.25) up to max_interval.
132    * On 2xx with new entries, decrease interval by factor b_down (e.g., 0.75) down to min_interval.
133    * On 429/503 with Retry-After, pause exactly Retry-After; otherwise exponential backoff with cap.
134    * Observed cadence EWMA:
135  
136      * Let inter-arrival deltas be d1..dn from recent entries.
137      * EWMA_t = alpha*d_t + (1-alpha)*EWMA_{t-1}, alpha in [0.2,0.4].
138      * Target interval I* = clamp(EWMA, min_interval, max_interval).
139      * Blend with current I via: I_next = clamp(lambda*I* + (1-lambda)*I, min_interval, max_interval).
140    * Add random jitter J ~ U(-j, +j) proportionally (e.g., j = 0.15*I_next).
141    * Honor skipHours/skipDays by shifting schedule out of publisher-declared quiet windows.
142  
143  * Archive/backfill:
144  
145    * If RFC 5005 relations are present (prev-archive, next), walk archives once on a backfill queue.
146    * After backfill completes, resume normal cadence on the current feed only.
147  
148  * Push (when available):
149  
150    * WebSub: subscribe to hub and reduce poll cadence to a slow safety check (e.g., daily).
151  
152  ### 4.7 Politeness & governance
153  
154  * Respect robots.txt when you crawl HTML pages for autodiscovery or article fetching.
155  * Per-host rate limits and concurrency.
156  * Hard ceilings on feed size, XML depth, and max items per document.
157  
158  ### 4.8 Exposure to other programs
159  
160  * Event bus: publish NewEntry events on first_seen with a compact payload and raw_refs.
161  * Pull APIs:
162  
163    * REST/gRPC: list feeds, list entries by feed or time, get entry by uid, get raw payload by ref.
164    * Flat files: write newline-delimited JSON streams to a directory for batch consumers.
165  * Optional: serve your own meta-feed for any collection, including Link headers (rel=self/next/hub).
166  
167  ### 4.9 Observability
168  
169  * Metrics (per feed and aggregated):
170  
171    * fetch_count, success_rate, 304_rate, error_code_counts
172    * bytes_in, compression_ratio, average_latency
173    * scheduler_interval_chosen, ewma_interval, retry_counts
174    * new_entries_count per fetch
175  * Logs:
176  
177    * Structured logs with feed_id, url, decision_reason, http_status, interval_before/after.
178  * Sampling:
179  
180    * Keep last N raw payloads per feed for debugging parser regressions.
181  * Tracing:
182  
183    * Trace spans: schedule decision -> HTTP request -> parse -> storage -> events.
184  
185  ### 4.10 Security posture
186  
187  * Disable DTD, external entities, and XInclude in XML parsers.
188  * Enforce size/time limits on downloads and XML parse depth.
189  * Sanitize HTML content fields if you later render them anywhere.
190  * Validate and normalize URLs; restrict schemes to http/https for fetching.
191  * Isolate fetcher network egress via allowlist if you run in untrusted environments.
192  
193  ---
194  
195  ## 5. Data model (normalized)
196  
197  Types shown in JSON-like notation.
198  
199  ### 5.1 Feed
200  
201  ```
202  Feed {
203    id: string,            // stable UUID
204    url: string,
205    type: "rss" | "atom" | "unknown",
206    validators: {
207      etag: string|null,
208      last_modified: string|null  // HTTP date
209    },
210    schedule: {
211      interval_sec: int,
212      min_interval_sec: int,
213      max_interval_sec: int,
214      last_decision_at: string,   // RFC3339
215      reason: string,             // human-readable summary
216      retry_after_sec: int|null
217    },
218    publisher_hints: {
219      ttl_minutes: int|null,
220      skip_hours: int[]|null,     // 0..23
221      skip_days: string[]|null    // ["Mon",...]
222    },
223    stats: {
224      last_fetch_at: string|null,
225      last_success_at: string|null,
226      consecutive_failures: int,
227      ewma_interarrival_sec: int|null,
228      new_entries_last_fetch: int
229    }
230  }
231  ```
232  
233  ### 5.2 Entry
234  
235  ```
236  Entry {
237    entry_uid: string,           // preferred native id; else hash
238    feed_id: string,
239    canonical_link: string|null,
240    title: string|null,
241    summary: string|null,        // may be HTML or text
242    content: string|null,        // preferred full content if present
243    authors: [{ name: string|null, email: string|null, uri: string|null }],
244    categories: [string],
245    enclosures: [{ url: string, type: string|null, length: int|null }],
246    published: string|null,      // RFC3339
247    updated: string|null,        // RFC3339
248    first_seen: string,          // RFC3339
249    last_seen: string,           // RFC3339
250    seen_count: int,
251    raw_refs: [RawRef],          // pointers into Raw store
252    content_hash: string         // hash over normalized content for change detection
253  }
254  
255  RawRef {
256    fetch_id: string,            // id of a RawFetch object
257    offset: int|null, length: int|null, // optional if you store entry subtree slices
258    note: string|null
259  }
260  ```
261  
262  ### 5.3 Raw fetch
263  
264  ```
265  RawFetch {
266    fetch_id: string,            // UUID
267    feed_id: string,
268    fetched_at: string,          // RFC3339
269    url: string,
270    http_status: int,
271    request_headers: { ... },
272    response_headers: { ... },
273    body_ref: BlobRef,
274    body_sha256: string,
275    content_type: string|null,
276    content_length: int|null
277  }
278  
279  BlobRef {
280    storage: "localfs"|"s3"|"gcs"|"...",
281    key: string
282  }
283  ```
284  
285  Indexes to consider:
286  
287  * Entry(entry_uid) PK, Entry(feed_id, published), Entry(feed_id, first_seen)
288  * RawFetch(feed_id, fetched_at desc)
289  * Feed(url unique)
290  
291  ---
292  
293  ## 6. Scheduler details
294  
295  ### 6.1 Parameters
296  
297  * min_interval_sec: default 300
298  * max_interval_sec: default 86400
299  * up_factor b_up: 1.25
300  * down_factor b_down: 0.75
301  * ewma_alpha: 0.3
302  * blend_lambda: 0.5
303  * jitter_ratio: 0.15
304  * backoff_base: 2.0
305  * backoff_cap_sec: 3600
306  
307  ### 6.2 Pseudocode
308  
309  ```
310  I = current_interval
311  if http_status == 429 or http_status == 503:
312    if Retry-After present:
313      pause = parse_retry_after()
314      schedule_after(pause)
315      reason = "retry-after"
316    else:
317      I = min(I * backoff_base, backoff_cap_sec)
318      reason = "error-backoff"
319  elif http_status == 304:
320    I = min(I * b_up, max_interval)
321    reason = "not-modified"
322  elif http_status in 200..299:
323    if new_entries > 0:
324      I = max(I * b_down, min_interval)
325      reason = "new-entries"
326    else:
327      I = min(I * b_up, max_interval)
328      reason = "no-new-entries"
329  
330  if ewma_interarrival available:
331    I_star = clamp(ewma_interarrival, min_interval, max_interval)
332    I = clamp(blend_lambda * I_star + (1 - blend_lambda) * I,
333              min_interval, max_interval)
334  
335  if publisher ttl exists:
336    I = max(I, ttl_minutes * 60)
337  
338  I = apply_skip_windows(I, skipHours, skipDays)
339  I = apply_jitter(I, jitter_ratio)
340  
341  schedule_after(I)
342  record_decision(reason, I)
343  ```
344  
345  ### 6.3 EWMA maintenance
346  
347  * On each new entry sequence update, recompute inter-arrival deltas from the last N publication timestamps (e.g., N=20) and update ewma_interarrival.
348  
349  ---
350  
351  ## 7. Failure modes and handling
352  
353  * Malformed XML: mark fetch as parse_error; store raw; do not advance last_success_at; continue with cautious backoff.
354  * Huge responses: abort on size/time limit; record truncated flag; increase interval.
355  * Flapping feeds (alternating errors): cap backoff but add jitter; open circuit if over error budget.
356  * ID instability (changing guid/title): rely on canonical_link and content hash; keep lineage for merges.
357  * Time skew: if published dates are missing/future, fall back to first_seen ordering.
358  
359  ---
360  
361  ## 8. Configuration
362  
363  * Global:
364  
365    * min/max intervals, factors, jitter, backoff caps.
366    * network timeouts, max redirects, max body size, allowed content types.
367    * per-host concurrency ceilings and rate limits.
368  * Per-feed overrides:
369  
370    * min/max interval clamps, disable autodiscovery, enable push only, archive backfill enabled.
371  
372  Example env keys (illustrative):
373  
374  * FETCH_TIMEOUT_MS, MAX_BODY_BYTES, MAX_XML_DEPTH
375  * SCHED_MIN_INTERVAL_SEC, SCHED_MAX_INTERVAL_SEC, SCHED_JITTER_RATIO
376  * HOST_MAX_CONCURRENCY, HOST_RPS
377  
378  ---
379  
380  ## 9. Interfaces
381  
382  ### 9.1 Events
383  
384  * Topic: new_entry
385  * Payload:
386  
387  ```
388  {
389    "entry_uid": "...",
390    "feed_id": "...",
391    "published": "...",
392    "canonical_link": "...",
393    "raw_refs": [{ "fetch_id": "..." }],
394    "ingested_at": "..."
395  }
396  ```
397  
398  ### 9.2 REST (sketch)
399  
400  * GET /feeds
401  * GET /feeds/{id}
402  * POST /feeds  // add subscription
403  * DELETE /feeds/{id}
404  * POST /feeds/{id}/refetch   // manual nudge
405  * GET /feeds/{id}/entries?since=...
406  * GET /entries/{entry_uid}
407  * GET /raw/{fetch_id}
408  
409  ### 9.3 File taps
410  
411  * Write NDJSON files to a directory partitioned by date:
412  
413    * out/entries/2025-10-14.ndjson
414    * out/raw/2025-10-14/uuid.blob
415  
416  ---
417  
418  ## 10. Observability and SLOs
419  
420  SLOs (initial targets):
421  
422  * p99 fetch latency < 5s for 1MB payloads.
423  * Error budget: < 2% failed fetches per day per feed ignoring publisher/server faults.
424  * Scheduler freshness: 95% of feeds polled within 2x their chosen interval.
425  
426  Dashboards:
427  
428  * Success vs 304 vs error stacked over time.
429  * Interval chosen vs ewma per feed.
430  * Top feeds by new_entries.
431  * Top error codes and hosts.
432  * Bytes saved via conditional GETs.
433  
434  ---
435  
436  ## 11. Security and privacy
437  
438  * Do not execute any active content; treat feed HTML as untrusted.
439  * Do not resolve data: URIs or file: URIs.
440  * Optionally hash or redact PII if you process author emails downstream.
441  * Isolate credentials if you support authenticated feeds later.
442  
443  ---
444  
445  ## 12. Extensions (optional modules)
446  
447  * WebSub subscriptions and verification endpoint.
448  * RFC 5005 archive walker for historical import.
449  * Media fetcher for enclosures with their own queue/limits.
450  * Full-text article fetcher with HTML boilerplate removal.
451  * Duplicate clusterer across multiple feeds mirroring the same site.
452  
453  ---
454  
455  ## 13. Testing strategy
456  
457  * Golden raw payloads and parser fixtures for RSS/Atom edge cases.
458  * Property tests on normalization (id stability, URL canonicalization).
459  * Fuzzing malformed XML.
460  * Deterministic scheduler tests:
461  
462    * sequences of 304/new entries/errors -> expected intervals.
463  * Load tests:
464  
465    * many small feeds vs few large feeds; simulate slow servers and 429s.
466  
467  ---
468  
469  ## 14. Rollout plan
470  
471  Phase 1 (MVP):
472  
473  * Feeds registry, HTTP fetcher with conditional GET, hardened parsing, raw + normalized storage, scheduler v1, minimal REST, metrics.
474  
475  Phase 2:
476  
477  * EWMA-based adaptation, skip windows, per-host governance, NDJSON tap, backfill via RFC 5005.
478  
479  Phase 3:
480  
481  * WebSub support, media fetcher, tracing, archive walker tooling.
482  
483  Phase 4:
484  
485  * Article fetcher, de-dupe across feeds, data export formats.
486  
487  ---
488  
489  ## 15. Implementation notes
490  
491  * Make the fetcher worker idempotent per scheduled run and feed (dedupe by feed_id + scheduled_at).
492  * Store raw before parsing; parsing failures should not drop the payload.
493  * Keep a small on-disk cache of last good payload to allow diffing when a feed silently re-writes history.
494  * Use monotonic clocks for scheduling in process; persist absolute next_run_at in DB for cross-process handoff.
495  * Keep per-host pools keyed by host+port+scheme to avoid cross-talk.
496  
497  ---
498  
499  ## 16. Appendix: normalized field mapping cheatsheet
500  
501  RSS -> normalized:
502  
503  * item.guid[.isPermaLink=false] -> entry_uid
504  * item.link -> canonical_link
505  * item.title -> title
506  * item.pubDate -> published
507  * item.description -> summary
508  * item.content:encoded -> content
509  * item.enclosure -> enclosures[]
510  * channel.ttl -> publisher_hints.ttl_minutes
511  * channel.skipHours/skipDays -> publisher_hints.skip_*
512  
513  Atom -> normalized:
514  
515  * entry.id -> entry_uid
516  * entry.link rel="alternate" -> canonical_link
517  * entry.title -> title
518  * entry.published -> published
519  * entry.updated -> updated
520  * entry.summary -> summary
521  * entry.content -> content
522  * link rel="enclosure" -> enclosures[]
523  
524  ---
525  
526  ## 17. References you should be aware of (non-binding)
527  
528  * RSS 2.0 (de facto spec), Atom (RFC 4287)
529  * HTTP semantics (conditional GET, Retry-After)
530  * WebSub (W3C), RFC 5005 (Feed Paging and Archiving)
531  * robots.txt (REP)
532  * OWASP XML security guidance
533  
534  ---
535  
536  ## 18. Quick checklist
537  
538  * [ ] Conditional GET implemented
539  * [ ] Raw payloads persisted with headers
540  * [ ] XML parser hardened, limits enforced
541  * [ ] Normalization and stable entry_uid logic
542  * [ ] Scheduler v1 with EWMA blend and jitter
543  * [ ] Per-host concurrency and rate limits
544  * [ ] Metrics + logs + sample raw retention
545  * [ ] Minimal REST + NDJSON tap
546  * [ ] Backfill queue and archive handling
````
./docs/CONTRIBUTING.md
````
  1  # CONTRIBUTING.md
  2  
  3  Welcome! This document explains how to contribute clean, testable code to this project. It encodes our architecture boundaries, AI-friendly authoring rules, testing strategy, and PR workflow so humans and AI agents can collaborate smoothly.
  4  
  5  ## 1. Core values
  6  
  7  * Small, composable units. Prefer many focused files over one giant file.
  8  * Pure core, impure edges. Parsing, I/O, clock, and HTTP live behind traits.
  9  * Determinism by default. Same inputs should yield identical outputs and IDs.
 10  * Observability first. Every decision is explainable via logs, metrics, traces.
 11  * Backwards compatibility. Schema and output changes are versioned explicitly.
 12  
 13  ## 2. Workspace layout
 14  
 15  We maintain strict boundaries. Adapters depend on `core`, never on each other.
 16  
 17  ```
 18  crates/
 19    core/               # domain types, traits, logic (pure or easily mockable)
 20    adapters/
 21      cli/              # CLI surface; maps subcommands onto core traits
 22      http/             # future: HTTP service using the same core traits
 23    repos/
 24      fs/               # repository adapter: filesystem/JSON/NDJSON
 25      sqlite/           # repository adapter: SQLite (optional)
 26  xtask/                # CI helpers, generators, fixture tooling (optional)
 27  ```
 28  
 29  Rules:
 30  
 31  * `core` has zero network or filesystem imports.
 32  * `adapters/*` may do I/O but must speak only via core traits and types.
 33  * `repos/*` implement the repository trait only; they contain no business logic.
 34  
 35  ## 3. AI-friendly authoring rules
 36  
 37  These rules make files easy to generate, review, and test.
 38  
 39  * File caps: aim for 60–120 LOC per file. Do not split cohesive logic just to hit a number; prefer clarity. If a unit exceeds ~200 LOC, consider refactoring.
 40  * One concept per file. Name files after the noun or verb they implement.
 41  * No deep trees. Max module depth of 3.
 42  * Keep public APIs tiny. Re-export types at the crate root.
 43  * Error types: one error enum per boundary, with stable, documented variants.
 44  * Logging: structured logs with a stable set of keys. No println.
 45  * Time and randomness are injected via traits so tests can control them.
 46  
 47  ### 3.1 Required file header
 48  
 49  Every Rust file starts with this header block (ASCII only):
 50  
 51  ```text
 52  // File: <crate>/<path>/<name>.rs
 53  // Purpose: <one-line intent>
 54  // Inputs: <types/traits it consumes>
 55  // Outputs: <types/errors it returns>
 56  // Side effects: <I/O, logging, metrics, none>
 57  // Invariants:
 58  //  - <bullet 1>
 59  //  - <bullet 2>
 60  // Tests: <file(s) providing golden/property/E2E coverage>
 61  ```
 62  
 63  CI fails if the header is missing.
 64  
 65  ## 4. Code style
 66  
 67  * Rust edition: 2021 (or project default).
 68  * rustfmt: required. clippy: `-D warnings`.
 69  * Naming: types UpperCamelCase, traits end with `Ext` only for extension traits.
 70  * Errors implement `std::error::Error` and carry machine-readable codes.
 71  
 72  ## 5. Repository contract (must-implement)
 73  
 74  All data storage goes through a single trait in `core`. Adapters in `repos/*` implement it. The exact signatures live in `crates/core/src/repo.rs`; this section defines the contract.
 75  
 76  Operations and constraints:
 77  
 78  * `put_raw(feed_id, payload, meta) -> Result<RawId>`
 79    Idempotent. Atomic write or fail. Stores verbatim fetch payload with headers and timestamps. Never mutates once written.
 80  * `put_entry(entry: NormalizedEntry) -> Result<EntryId>`
 81    Upsert by canonical `EntryId`. Must be atomic at entry granularity.
 82  * `get_last_fetch(feed_id) -> Result<Option<FetchRecord>>`
 83    Used by scheduler; must be O(1) on indexed backends.
 84  * `record_fetch(feed_id, outcome: FetchOutcome) -> Result<()>`
 85    Append-only; enables EWMA and error-budget accounting.
 86  * `scan_entries(feed_id, since: Option<Instant>) -> Iterator<NormalizedEntry>`
 87    Forward-only, stable order (by published then updated).
 88  * `put_derivation(entry_id, kind, version, blob) -> Result<()>`
 89    Versioned derivations; never overwrite same `(entry_id, kind, version)`.
 90  
 91  Error semantics:
 92  
 93  * Distinguish `Conflict`, `NotFound`, `Unavailable`, `Corruption`, `Transient`.
 94  * All writes are atomic per record. If not supported, adapter must do a temp-file or transaction dance to emulate.
 95  
 96  ## 6. Canonical IDs (must follow)
 97  
 98  To avoid duplicates across mirrors or transport quirks, IDs are derived as follows (centralized in `core::ids`):
 99  
100  Order of precedence for `EntryId`:
101  
102  1. `guid` if present and not marked `isPermaLink=false`, normalized.
103  2. Stable hash of tuple:
104  
105     * normalized link URL (after redirect resolution if available)
106     * title text stripped of markup and collapsed whitespace
107     * published timestamp (RFC3339, UTC, second precision)
108     * content hash of `content:encoded` or `summary` if content missing
109  
110  Hash function: xxhash64 over UTF-8 bytes of the tuple; output as 16-char lowercase hex. All callers must go through `core::ids::entry_id(entry_like)`.
111  
112  Add golden tests: `tests/golden/ids/*.yaml` mapping inputs to expected IDs.
113  
114  ## 7. Scheduler state model (must follow)
115  
116  The scheduler decides when to fetch each feed. Central enums live in `core::sched`:
117  
118  * `State`: Healthy, CoolingDown, Backoff, Paused, Disabled
119  * `Decision`: FetchNow, Defer { next_at }, Backoff { until, reason }, Pause { until, reason }
120  * `Reason`: NewFeed, RetryAfter, HttpError, ParseError, BudgetExceeded, EWMAStable, Manual
121  
122  Decision algorithm (high-level):
123  
124  * Maintain per-feed EWMA of inter-arrival times and a global error budget.
125  * Respect HTTP cache semantics (ETag/If-None-Match, Last-Modified, Retry-After).
126  * Jitter within a small window to avoid thundering herds.
127  * Clamp with per-host concurrency caps and per-feed min/max intervals.
128  
129  All decisions must log:
130  
131  * `feed_id`, `state`, `decision`, `next_at`, `reason`, `ewma_secs`, `error_budget`, `retry_after_secs?`.
132  
133  Goldens: `tests/golden/sched/*.yaml` with sequences of events -> expected decisions.
134  
135  ## 8. Testing strategy
136  
137  We practice layered tests. Prefer fast tests and stable fixtures.
138  
139  * Unit tests: in the same file as the unit under test.
140  * Property tests: for parsers, ID canonicalization, and scheduler math.
141  * Golden tests: YAML inputs -> expected normalized outputs and IDs.
142  * Integration tests: crate-level flows with in-memory repo and fake clock.
143  * E2E test: `feeds.json` with two small feeds. `fetch` once -> expect `NewEntry` events and two raw payloads.
144  
145  Test layout:
146  
147  ```
148  crates/core/tests/
149    golden_ids.rs
150    golden_sched.rs
151    e2e_minimal.rs
152  tests/fixtures/
153    feeds.json
154    feeds/
155      atom_edge.xml
156      rss_content_encoded.xml
157      huge_truncated.xml
158  ```
159  
160  ## 9. Backfill policy
161  
162  Backfill (RFC 5005, archive walking) must not starve live updates.
163  
164  * Live always wins. At most 1 backfill request per feed concurrently.
165  * Cap historical pages per run (default: 2) and per day (default: 20).
166  * Skip backfill while `State` is `Backoff` or `Paused`.
167  * Record progress markers; never re-crawl the same archive page in a day.
168  
169  ## 10. Observability and performance
170  
171  * Logging: structured, no secrets. Required keys: `component`, `feed_id`, `decision`, `elapsed_ms`.
172  * Metrics: counters for fetch outcomes, histograms for durations and response sizes, gauges for backlog and error budget.
173  * Tracing: parent span for each fetch pipeline; child spans for DNS, TCP, TLS, HTTP, parse, normalize, store.
174  * Budgets: fail fast on timeouts; no unbounded retries.
175  
176  ## 11. Documentation duties
177  
178  Any new public type or trait must have:
179  
180  * A one-line summary and a short example.
181  * A note on invariants and failure modes.
182  * Cross-links to related types.
183  
184  Update `ARCHITECTURE.md` if you add a boundary or change a contract. Update `README.md` if you add/rename a CLI subcommand.
185  
186  ## 12. CLI surface contract
187  
188  Subcommands map 1:1 to use-cases and call only core traits:
189  
190  * `fetch` — quiet by default; `--verbose` enables tracing.
191  * `stats` — reads only via repo trait; never touches the network.
192  * `import` — batch adds from a URLs file into the repo; validates and dedups.
193  * `add` — adds one URL; returns canonical `feed_id`.
194  
195  No subcommand may implement logic that belongs in `core`.
196  
197  ## 13. PR process
198  
199  Open a PR only when:
200  
201  * All file headers are present.
202  * `cargo fmt`, `clippy -D warnings`, and tests pass locally.
203  * New or changed behavior has golden tests.
204  * Public API changes include docs and a brief migration note.
205  
206  ### 13.1 PR checklist
207  
208  * [ ] I respected the workspace dependency rules (adapters -> core only).
209  * [ ] I kept files small and single-purpose.
210  * [ ] I used the repository trait and did not reach around it.
211  * [ ] I used `core::ids` for any IDs.
212  * [ ] I added or updated goldens and property tests.
213  * [ ] I documented invariants and failure modes.
214  * [ ] I added structured logs at decision points.
215  
216  ## 14. Commit messages
217  
218  Use Conventional Commits with semantic intent so changelogs are automated:
219  
220  * `feat(core): canonicalize IDs for Atom feeds`
221  * `fix(repo/fs): atomic writes with temp files`
222  * `perf(sched): clamp EWMA floor at 15m`
223  * `test(core): golden cases for Retry-After`
224  * `docs(architecture): backfill policy`
225  
226  ## 15. CI expectations
227  
228  CI runs:
229  
230  * Format and lint.
231  * Unit, property, golden, integration and E2E tests.
232  * Header check: every `.rs` file must include the required header block.
233  * Size check: warn on files > 200 LOC (excluding header and tests).
234  * Fixture determinism: re-run goldens and fail if outputs change without updated fixtures.
235  
236  ## 16. Security and licensing
237  
238  * No secrets in code, tests, or fixtures.
239  * Treat all remote inputs as untrusted. Validate and bound parse sizes.
240  * License: CC0-1.0 for text; code license as declared in the repo.
241  
242  ## 17. Getting started (quick path)
243  
244  1. Read `ARCHITECTURE.md` and skim `core::repo`, `core::ids`, `core::sched`.
245  2. Pick a unit of work from `ROADMAP.md` or open a proposal.
246  3. Create a small file in the right crate with the header and a unit test.
247  4. Add or extend a golden fixture if behavior changes.
248  5. Open a draft PR early to run CI and discuss boundaries.
249  
250  ---
251  
252  Questions or proposals? Open a GitHub Discussion or a draft PR with a short design note. Thanks for helping keep this codebase clean, deterministic, and friendly to both humans and AI.
253  
````
