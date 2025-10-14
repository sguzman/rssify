/*
Module: lib
Purpose: Public surface for CLI adapters.
Public API: repo_selector, repo_sqlite
Invariants: Small modules; business logic elsewhere.
Notes: Keep exports minimal to avoid churn.
*/

pub mod repo_selector;
pub mod repo_sqlite;
