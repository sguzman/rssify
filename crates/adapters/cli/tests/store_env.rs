/*
Module: rssify_cli::tests::store_env
Purpose: Verify precedence for repo resolution without mutating the process environment.
*/

#[path = "../src/store.rs"]
mod store;

use store::{resolve_store_spec_with_env, ENV_REPO};

#[test]
fn precedence_flag_over_env() {
    let got = resolve_store_spec_with_env(
        |_k| Some("fs:/env".to_string()),
        Some("fs:/flag".to_string()),
    );
    assert_eq!(got, "fs:/flag");
}

#[test]
fn precedence_env_over_default() {
    let got = resolve_store_spec_with_env(|k| {
        if k == ENV_REPO { Some("fs:/env".to_string()) } else { None }
    }, None);
    assert_eq!(got, "fs:/env");
}

#[test]
fn default_when_neither_present() {
    let got = resolve_store_spec_with_env(|_| None, None);
    assert_eq!(got, "fs:.");
}

#[test]
fn empty_env_ignored() {
    let got = resolve_store_spec_with_env(|_| Some("".to_string()), None);
    assert_eq!(got, "fs:.");
}
