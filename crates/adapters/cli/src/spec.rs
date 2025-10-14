/*
Module: rssify_cli::spec
Purpose: Parse and validate repository selection strings (no I/O)
Public API surface: RepoKind, RepoSpec, RepoSpec::parse, FromStr, Display
Invariants: Pure parsing only; adapters implement actual backends later
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Supported repository kinds. Extend in a backward-compatible way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoKind {
    Fs,
    Sqlite,
}

impl RepoKind {
    fn from_prefix(prefix: &str) -> Option<Self> {
        match prefix.to_ascii_lowercase().as_str() {
            "fs" => Some(Self::Fs),
            "sqlite" => Some(Self::Sqlite),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            RepoKind::Fs => "fs",
            RepoKind::Sqlite => "sqlite",
        }
    }
}

/// Parsed repository specification, e.g. "fs:/path", "sqlite:/path/to.db".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoSpec {
    pub kind: RepoKind,
    pub target: String,
}

impl RepoSpec {
    /// Parse without using FromStr ergonomics.
    pub fn parse(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();
        let Some((prefix, rest)) = trimmed.split_once(':') else {
            return Err("missing ':' separator".into());
        };
        let kind = RepoKind::from_prefix(prefix).ok_or_else(|| "unknown repo kind".to_string())?;
        if rest.is_empty() {
            return Err("empty repo target".into());
        }
        Ok(Self {
            kind,
            target: rest.to_string(),
        })
    }
}

impl FromStr for RepoSpec {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Display for RepoSpec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.kind.as_str(), self.target)
    }
}
