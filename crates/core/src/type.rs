/*
Module: core::scheduler::types
Purpose: Data types for the scheduler.
Public API: Decision, Stats
Notes: Pure data; no I/O.
*/
#![forbid(unsafe_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    FetchNow { reason: &'static str },
    WaitFor { secs: u32, reason: &'static str },
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub last_ok_ts: Option<i64>,            // unix seconds
    pub last_err_ts: Option<i64>,           // unix seconds
    pub observed_interval_sec: Option<u32>, // rolling interval
    pub min_interval_sec: u32,              // lower bound
    pub max_interval_sec: u32,              // upper bound
    pub backoff_floor_sec: u32,             // base wait after error
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            last_ok_ts: None,
            last_err_ts: None,
            observed_interval_sec: None,
            min_interval_sec: 300,   // 5 min
            max_interval_sec: 43200, // 12 h
            backoff_floor_sec: 900,  // 15 min
        }
    }
}
