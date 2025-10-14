#![forbid(unsafe_code)]

use rssify_core::RepoError;

#[derive(Debug, Clone)]
pub enum RepoSpec<'a> {
    Fs(&'a str),
    Sqlite(&'a str),
}

pub fn parse_repo_spec(spec: &str) -> Result<RepoSpec<'_>, RepoError> {
    let (scheme, rest) = spec.split_once(':').unwrap_or(("fs", spec));
    match scheme.to_ascii_lowercase().as_str() {
        "sqlite" => Ok(RepoSpec::Sqlite(rest)),
        "fs" | "" => Ok(RepoSpec::Fs(rest)),
        other => Err(RepoError::Backend(format!(
            "unknown repo scheme: {}",
            other
        ))),
    }
}
