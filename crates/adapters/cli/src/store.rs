//// File: crates/adapters/cli/src/store.rs
//// Purpose: Resolve repository spec from CLI flag, env var, or default.
//// Precedence: --store flag > RSSIFY_REPO env > "fs:."
//// Notes: Keep pure and dependency-free. Provide an injectable resolver for tests.

pub const ENV_REPO: &str = "RSSIFY_REPO";

/// Resolve the repository spec string per precedence rules using the real process environment.
pub fn resolve_store_spec(store_flag: Option<String>) -> String {
    resolve_store_spec_with_env(|k| std::env::var(k).ok(), store_flag)
}

/// Resolve the repository spec with a provided env getter (for tests).
pub fn resolve_store_spec_with_env<F>(get_env: F, store_flag: Option<String>) -> String
where
    F: Fn(&str) -> Option<String>,
{
    if let Some(s) = store_flag {
        return s;
    }
    if let Some(envv) = get_env(ENV_REPO) {
        let trimmed = envv.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    "fs:.".to_string()
}
