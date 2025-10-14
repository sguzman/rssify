/*
Module: core::scheduler::decide
Purpose: Pure decision function.
Public API: decide(now_sec, &Stats) -> Decision
Notes: No I/O; caller injects time.
*/
#![forbid(unsafe_code)]

use super::{Decision, Stats};

/// Rules:
/// 1) If never fetched OK -> FetchNow("never_fetched").
/// 2) If last error >= last ok -> WaitFor(backoff) where
///    backoff = clamp(max(backoff_floor, min_interval), max_interval).
/// 3) Else pace by interval: desired = clamp(observed or min, [min,max]).
///    If elapsed >= desired -> FetchNow("due") else WaitFor(remaining, "not_due").
pub fn decide(now_sec: i64, s: &Stats) -> Decision {
    if s.last_ok_ts.is_none() {
        return Decision::FetchNow {
            reason: "never_fetched",
        };
    }
    let last_ok = s.last_ok_ts.unwrap();

    if let Some(err_ts) = s.last_err_ts {
        if err_ts >= last_ok {
            let mut backoff = s.backoff_floor_sec.max(s.min_interval_sec);
            if backoff > s.max_interval_sec {
                backoff = s.max_interval_sec;
            }
            return Decision::WaitFor {
                secs: backoff,
                reason: "error_backoff",
            };
        }
    }

    let desired = s
        .observed_interval_sec
        .unwrap_or(s.min_interval_sec)
        .clamp(s.min_interval_sec, s.max_interval_sec);

    let elapsed = now_sec.saturating_sub(last_ok).max(0) as u32;

    if elapsed >= desired {
        Decision::FetchNow { reason: "due" }
    } else {
        Decision::WaitFor {
            secs: desired - elapsed,
            reason: "not_due",
        }
    }
}
