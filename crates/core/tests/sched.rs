/*
Module: rssify_core::tests::sched
Purpose: Compile-time sanity for scheduler contracts (no logic yet)
Public API surface: tests only
Invariants: Contracts are constructible and stable for adapters
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Test files may exceed header rules; scripts skip /tests/.
*/

use rssify_core::{FeedId, SchedDecision, SchedInput, SchedReason, SchedState};

#[test]
fn sched_input_compiles_and_holds_values() {
    let input = SchedInput {
        feed: FeedId::from_url("https://ex.com/feed"),
        now_unix: 1_700_000_000,
        last_ok_fetch_ts: Some(1_699_999_000),
        last_error_ts: None,
        observed_interval_sec: Some(3600),
        state: SchedState::Normal,
    };
    // Basic shape checks
    assert_eq!(input.state, SchedState::Normal);
    assert!(matches!(
        SchedDecision::WaitFor(60),
        SchedDecision::WaitFor(_)
    ));
    assert!(matches!(
        SchedReason::RecentSuccess { seconds_ago: 123 },
        SchedReason::RecentSuccess { .. }
    ));
}
