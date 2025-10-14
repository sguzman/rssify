/*
Module: rssify_cli::main
Purpose: Minimal CLI skeleton for subcommand parsing; no business logic or I/O
Public API surface: binary entrypoint only
Invariants: Adapters must not contain business logic; they call into core traits
Logging keys used: component, op, feed_id, elapsed_ms, items
Notes: Keep file <= 200 LOC if possible; refactor at 300.
*/

#![forbid(unsafe_code)]
use clap::Parser;
use rssify_cli::repo_selector::{open_from_spec, RepoKind};
use rssify_cli::repo_selector::{parse_repo_spec, RepoSpec};
use rssify_cli::repo_sqlite::SqliteRepo;
use rssify_core::RepoError;
use std::process::ExitCode;
// use your existing fs code path in the fs arm when it is ready

#[derive(Parser, Debug)]
#[command(name = "rssify", version, disable_help_subcommand = true)]
struct Cli {
    #[arg(long, env = "RSSIFY_REPO", default_value = "fs:./data")]
    repo: String,

    #[command(subcommand)]
    cmd: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Fetch {/* ... */},
    Stats {/* ... */},
    Import {/* ... */},
    Add {/* ... */},
    // Serve, Export, etc later
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {}", e);
            ExitCode::from(1)
        }
    }
}

use rssify_cli::repo_selector::{parse_repo_spec, RepoSpec};
use rssify_cli::repo_sqlite::SqliteRepo;
// use your existing fs code path in the fs arm when it is ready

fn run(cli: Cli) -> Result<(), RepoError> {
    let spec = parse_repo_spec(&cli.repo)?;
    match (spec, &cli.cmd) {
        (RepoSpec::Sqlite(dsn), Command::Fetch { /* ... */ }) => {
            let repo = SqliteRepo::open(dsn)?;
            fetch_impl(&repo /*, ... */)
        }
        (RepoSpec::Fs(path), Command::Fetch { /* ... */ }) => {
            // TODO: call your existing fs implementation here.
            // For now, return a clear error until fs is wired:
            Err(RepoError::Backend(format!("fs repo not wired yet: {}", path)))
        }
        // repeat the same pattern for other subcommands
        _ => Err(RepoError::Backend("command/backend combo not implemented".into())),
    }
}

fn cmd_fetch(repo: &RepoKind /*, .. */) -> Result<(), RepoError> {
    match repo {
        RepoKind::Fs(r) => fetch_impl(r /*, .. */),
        RepoKind::Sqlite(r) => fetch_impl(r /*, .. */),
    }
}
fn cmd_stats(repo: &RepoKind /*, .. */) -> Result<(), RepoError> {
    /* same pattern */
    Ok(())
}
fn cmd_import(repo: &RepoKind /*, .. */) -> Result<(), RepoError> {
    /* same pattern */
    Ok(())
}
fn cmd_add(repo: &RepoKind /*, .. */) -> Result<(), RepoError> {
    /* same pattern */
    Ok(())
}

// Generic implementation works for both adapters via the shared traits.
fn fetch_impl<R: rssify_core::FeedRepo + rssify_core::EntryRepo + rssify_core::ScheduleRepo>(
    r: &R, /*, .. */
) -> Result<(), RepoError> {
    // do work using r.get/put/upsert/list/... with tx=None per AI-FRIENDLY tiny API rules
    Ok(())
}
