//// File: crates/adapters/cli/src/store.rs
//// Purpose: Resolve repository spec from CLI flag, env var, or default.
//// Precedence: --store flag > RSSIFY_REPO env > "fs:."
//// Notes: Keep pure and dependency-free.

pub const ENV_REPO: &str = "RSSIFY_REPO";

/// Resolve the repository spec string per precedence rules.
pub fn resolve_store_spec(store_flag: Option<String>) -> String {
    if let Some(s) = store_flag {
        return s;
    }
    if let Ok(envv) = std::env::var(ENV_REPO) {
        let trimmed = envv.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    "fs:.".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_beats_env_and_default() {
        std::env::set_var(ENV_REPO, "fs:/env");
        let got = resolve_store_spec(Some("fs:/flag".to_string()));
        assert_eq!(got, "fs:/flag");
        std::env::remove_var(ENV_REPO);
    }

    #[test]
    fn env_used_when_flag_missing() {
        std::env::set_var(ENV_REPO, "fs:/env");
        let got = resolve_store_spec(None);
        assert_eq!(got, "fs:/env");
        std::env::remove_var(ENV_REPO);
    }

    #[test]
    fn default_when_neither_present() {
        std::env::remove_var(ENV_REPO);
        let got = resolve_store_spec(None);
        assert_eq!(got, "fs:.");
    }

    #[test]
    fn empty_env_ignored() {
        std::env::set_var(ENV_REPO, "");
        let got = resolve_store_spec(None);
        assert_eq!(got, "fs:.");
        std::env::remove_var(ENV_REPO);
    }
}

