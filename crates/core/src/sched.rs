/*
Module: rssify_core::sched
Purpose: Scheduler decision contracts and enums for orchestration
Public API surface: SchedState, SchedDecision, SchedReason, SchedInput, Scheduler
Invariants: Pure contracts only; implementations live in adapters/backends
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

use crate::FeedId;

/// Coarse state known to the scheduler about a feed's recency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedState {
    Unknown,
    Quiet,  // rarely updates
    Normal, // typical cadence
    Hot,    // updates frequently
}

/// Reasons for the chosen decision (for observability).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedReason {
    NoHistory,
    RecentSuccess { seconds_ago: i64 },
    BackoffAfterError { seconds_ago: i64 },
    HotFeedHeuristic,
    QuietFeedHeuristic,
}

/// What to do next for a feed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedDecision {
    FetchNow,
    /// Wait for N seconds before next fetch attempt.
    WaitFor(i64),
}

/// Portable telemetry supplied by adapters to drive scheduling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedInput {
    pub feed: FeedId,
    pub now_unix: i64,
    /// Timestamp of last successful fetch, if any.
    pub last_ok_fetch_ts: Option<i64>,
    /// Timestamp of last error, if any (used for backoff).
    pub last_error_ts: Option<i64>,
    /// Typical interval (seconds) observed for this feed, if known.
    pub observed_interval_sec: Option<i64>,
    /// Coarse state tag to bias decisions.
    pub state: SchedState,
}

/// Contract for a scheduler that maps telemetry to a decision.
/// Implementations must be pure and deterministic.
pub trait Scheduler {
    fn decide(input: &SchedInput) -> (SchedDecision, SchedReason);
}
