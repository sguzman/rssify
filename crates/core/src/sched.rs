/*
Module: rssify_core::sched
Purpose: Scheduler decision contracts and enums for orchestration
Public API surface: SchedState, SchedDecision, SchedReason, Scheduler
Invariants: Pure function signatures; algorithm belongs in adapters/impls
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
    Hot,    // updating frequently
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
    WaitFor(i64), // seconds
}

/// Contract for a scheduler that maps telemetry to a decision.
pub trait Scheduler {
    fn decide(feed: &FeedId, state: SchedState, now_unix: i64) -> (SchedDecision, SchedReason);
}
