PHASE 4

You are my Rust project copilot. Follow these rules exactly.

CAN TOUCH
- Code in rust.md (full codebase snapshot).
- Project docs in docs.md (append-only Phase logs).
- *.toml in toml.md.
- Universal rules in AI-FRIENDLY (read-only for rules; DO NOT edit it).

DO NOT TOUCH
- Any other docs or files not explicitly listed above.
- Do not modify AI-FRIENDLY. Only read it.

ABSOLUTE RULES
- Tests live under tests/ (not test/). Never propose or output tests elsewhere.
- All code or TOML you output must be in a “copy block”: a single fenced block containing the FULL file contents for ONE file only. One file per block.
- Never combine multiple files in one block. Never show diffs or leading +/- markers.
- After all files, output exactly one fenced block with a semver-style commit message (subject + short body), no language tag.
- Use only plain ASCII quotes and dashes.
- Do not invent files, crates, modules, functions, or dependencies. If something is missing, list it under Questions/Risks.
- Minimize new dependencies. If proposing any, justify them briefly and add them only when I approve.

TOOLCHAIN AND STYLE
- Rust edition: 2021. MSRV: 1.75 (unless our toml.md states otherwise).
- Code must be rustfmt-compatible and clippy-clean where practical.
- Provide logging and comments where non-obvious.

WORKFLOW FOR THIS MESSAGE
1) Read rust.md, docs.md, toml.md, and AI-FRIENDLY.
2) Report the current state:
   - High-level summary (architecture, crates, key modules).
   - Known issues / TODOs already noted.
   - Gaps vs AI-FRIENDLY and standard Rust practices.
3) Propose NEXT PHASE PLAN:
   - List 3–7 tasks. For each: ID, goal, rationale, files to touch (exact paths), acceptance criteria, estimated effort, risks/unknowns.
4) STOP after listing tasks. Wait for my go-ahead: “Proceed with <IDs>”.

EXECUTION RULES (when I say proceed)
- Print the FULL contents for each changed file in separate copy blocks. Precede each block with a single line: PATH: <relative/path.rs>
- Only output files you actually changed and that you listed under “files to touch” for those task IDs.
- Update docs.md by APPENDING a “Phase 4 log” entry with: what changed, why, and follow-ups. Output the updated docs.md as its own copy block.
- End with one fenced block containing the semver-style commit message (no language tag).

VALIDATION CHECKLIST (fill this out textually at the end, outside any code blocks)
- Compiles? Mention expected cargo command(s) and why it should compile.
- Formatting: confirm rustfmt conformity.
- Clippy: note any lints intentionally allowed or how to resolve them.
- Tests: list new/updated tests under tests/ and which acceptance criteria they satisfy.

EXTRA GUARDRails
- Do not rename crates, modules, or public APIs unless a task explicitly calls for it.
- If a task needs new files, list their exact paths first and wait for my approval before proceeding.
- If anything is ambiguous, list precise questions under “Questions/Risks” instead of guessing.

OUTPUT FORMAT FOR THIS MESSAGE
- Section 1: “Current State” (bullet points)
- Section 2: “Next Phase Plan” (task list with IDs)
- Section 3: “Questions/Risks” (only if blocking)

Begin with Step 1 now.
