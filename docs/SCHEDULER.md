# Scheduler Contract

Goal: Decide when to fetch a feed next from simple, portable telemetry.

## Inputs (SchedInput)
- feed: FeedId (opaque)
- now_unix: current unix seconds
- last_ok_fetch_ts: Option<i64> (unix seconds)
- last_error_ts: Option<i64> (unix seconds)
- observed_interval_sec: Option<i64> (typical interval inferred externally)
- state: SchedState (Unknown | Quiet | Normal | Hot)

Notes:
- Core does not compute these values; adapters supply them from repos/telemetry.
- observed_interval_sec is a simple heuristic (e.g., EMA of prior publish cadence).

## Output
- (SchedDecision, SchedReason)

### SchedDecision
- FetchNow
- WaitFor(seconds)

### SchedReason
- NoHistory
- RecentSuccess { seconds_ago }
- BackoffAfterError { seconds_ago }
- HotFeedHeuristic
- QuietFeedHeuristic

Stability: This surface is versioned by the crate; changes require a migration note.

