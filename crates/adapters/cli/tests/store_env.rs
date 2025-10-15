/*
Module: rssify_cli::tests::store_env
Purpose: Verify precedence for repo resolution: flag > env > default.
*/

#[path = "../src/store.rs"]
mod store;

use store::{resolve_store_spec, ENV_REPO};

#[test]
fn precedence_flag_over_env() {
    std::env::set_var(ENV_REPO, "fs:/env");
    let got = resolve_store_spec(Some("fs:/flag".to_string()));
    assert_eq!(got, "fs:/flag");
    std::env::remove_var(ENV_REPO);
}

#[test]
fn precedence_env_over_default() {
    std::env::set_var(ENV_REPO, "fs:/env");
    let got = resolve_store_spec(None);
    assert_eq!(got, "fs:/env");
    std::env::remove_var(ENV_REPO);
}

#[test]
fn default_when_none_and_no_env() {
    std::env::remove_var(ENV_REPO);
    let got = resolve_store_spec(None);
    assert_eq!(got, "fs:.");
}

