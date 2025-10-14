/*
Module: tests::scheduler_decide
Purpose: Integration tests for core::scheduler public API.
Notes: Uses only public surface; : no internal imports.
*/
#![forbid(unsafe_code)]

use rssify_core::scheduler::{Decision, Stats, decide};

#[test]
fn never_fetched() {
    let s = Stats {
        last_ok_ts: None,
        ..Default::default()
    };
    assert_eq!(
        decide(1_700_000_000, &s),
        Decision::FetchNow {
            reason: "never_fetched"
        }
    );
}

#[test]
fn error_backoff_takes_precedence() {
    let s = Stats {
        last_ok_ts: Some(1_700_000_000),
        last_err_ts: Some(1_700_000_100),
        min_interval_sec: 600,
        max_interval_sec: 3600,
        backoff_floor_sec: 900,
        observed_interval_sec: Some(60),
    };
    assert_eq!(
        decide(1_700_000_200, &s),
        Decision::WaitFor {
            secs: 900,
            reason: "error_backoff"
        }
    );
}

#[test]
fn due_when_elapsed_ge_desired() {
    let s = Stats {
        last_ok_ts: Some(1_700_000_000),
        last_err_ts: Some(1_699_999_000),
        observed_interval_sec: Some(1200),
        min_interval_sec: 300,
        max_interval_sec: 7200,
        backoff_floor_sec: 600,
    };
    assert_eq!(
        decide(1_700_001_200, &s),
        Decision::FetchNow { reason: "due" }
    );
}

#[test]
fn not_due_waits_remaining() {
    let s = Stats {
        last_ok_ts: Some(1_700_000_000),
        last_err_ts: Some(1_699_999_000),
        observed_interval_sec: Some(1200),
        min_interval_sec: 300,
        max_interval_sec: 7200,
        backoff_floor_sec: 600,
    };
    assert_eq!(
        decide(1_700_000_600, &s),
        Decision::WaitFor {
            secs: 600,
            reason: "not_due"
        }
    );
}

#[test]
fn clamps_to_min_and_max() {
    let s = Stats {
        last_ok_ts: Some(1_700_000_000),
        last_err_ts: None,
        observed_interval_sec: Some(10), // below min
        min_interval_sec: 300,
        max_interval_sec: 3600,
        backoff_floor_sec: 10,
    };
    assert_eq!(
        decide(1_700_000_000, &s),
        Decision::WaitFor {
            secs: 300,
            reason: "not_due"
        }
    );

    let s2 = Stats {
        last_ok_ts: Some(1_700_000_000),
        last_err_ts: None,
        observed_interval_sec: Some(100_000), // above max
        min_interval_sec: 300,
        max_interval_sec: 3600,
        backoff_floor_sec: 10,
    };
    assert_eq!(
        decide(1_700_003_700, &s2),
        Decision::FetchNow { reason: "due" }
    );
}
