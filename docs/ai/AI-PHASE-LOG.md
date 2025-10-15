# AI Phase Log

## Phase 2 - Core repo seam and tests (2025-10-15)
- Added repo seam and error exports at crate root in crates/core/src/lib.rs.
- Introduced integration tests under crates/core/tests/repo_trait.rs.
- No new external dependencies.

## Phase 2 - T2 FsPaths builders (2025-10-15)
- Implemented pure URL-safe encoder and path builders in crates/adapters/cli/src/repo_fs.rs.
- Added crates/adapters/cli/tests/repo_fs_paths.rs to validate layout and encoding.
- Removed colocated unit tests from source file to comply with project rule that all tests live under tests/.
- Next: P2-T3 feed loader and CLI fetch wiring; P2-T4 logging facade.

