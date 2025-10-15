# AI Phase Log

## Phase 2 - Core repo seam and tests (2025-10-15)

- Added repo seam and error exports at crate root in crates/core/src/lib.rs.
- Moved tests to integration style under crates/core/tests/repo_trait.rs per project preference.
- No new external dependencies.
- Next: FsPaths builders (P2-T2), feed loader and CLI fetch wiring (P2-T3), logging facade (P2-T4).

Follow-up: AI-FRIENDLY currently mentions placing tests under src/test/. Consider updating that doc to reflect the project-wide convention of tests/ at the crate root.

