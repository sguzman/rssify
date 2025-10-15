//// File: crates/adapters/cli/src/log.rs
//// Purpose: Minimal structured logging helper for the CLI without external deps.
//// Inputs/Outputs: Logger writes key=value lines to stderr. Public API: Logger::new(level), info/warn/error/debug.
//// Invariants: ASCII key names; values Display-escaped with basic whitespace compaction; never writes to stdout.
//// Examples:
////   let log = Logger::new(LogLevel::Info);
////   log.info("fetch", &[("items", 5), ("status", "ok")]);

use std::fmt::Display;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
}

impl LogLevel {
    pub fn from_verbosity(v: u8) -> Self {
        match v {
            0 => LogLevel::Warn,
            1 => LogLevel::Info,
            _ => LogLevel::Debug,
        }
    }
    fn as_str(self) -> &'static str {
        match self {
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
        }
    }
}

pub struct Logger {
    level: LogLevel,
    component: &'static str,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self { level, component: "cli" }
    }

    pub fn with_component(mut self, name: &'static str) -> Self {
        self.component = name;
        self
    }

    pub fn error(&self, op: &str, kvs: &[(&str, impl Display)]) {
        self.emit(LogLevel::Error, op, kvs);
    }
    pub fn warn(&self, op: &str, kvs: &[(&str, impl Display)]) {
        if self.level >= LogLevel::Warn {
            self.emit(LogLevel::Warn, op, kvs);
        }
    }
    pub fn info(&self, op: &str, kvs: &[(&str, impl Display)]) {
        if self.level >= LogLevel::Info {
            self.emit(LogLevel::Info, op, kvs);
        }
    }
    pub fn debug(&self, op: &str, kvs: &[(&str, impl Display)]) {
        if self.level >= LogLevel::Debug {
            self.emit(LogLevel::Debug, op, kvs);
        }
    }

    fn emit(&self, lvl: LogLevel, op: &str, kvs: &[(&str, impl Display)]) {
        let ts_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let mut line = format!("ts={} level={} component={} op={}", ts_ms, lvl.as_str(), self.component, op);
        for (k, v) in kvs {
            line.push(' ');
            line.push_str(*k);
            line.push('=');
            let mut s = format!("{}", v);
            if s.contains(char::is_whitespace) {
                // compact whitespace to underscores to keep one-token values
                s = s.split_whitespace().collect::<Vec<_>>().join("_");
            }
            // do not add quotes; keep ASCII-friendly tokens
            line.push_str(&s);
        }
        let _ = writeln!(io::stderr(), "{}", line);
    }
}
