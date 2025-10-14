/*
Module: main
Purpose: CLI surface; parse flags and dispatch to backend.
Public API: binary entrypoint only
Invariants: --repo selects backend; env RSSIFY_REPO respected.
Notes: Keep file <= 200 LOC; no business logic here.
*/

#![forbid(unsafe_code)]

use std::process::ExitCode;

use clap::Parser;
use rssify_cli::repo_selector::{RepoSpec, parse_repo_spec};
use rssify_cli::repo_sqlite::SqliteRepo;
use rssify_core::RepoError;

#[derive(Parser, Debug)]
#[command(name = "rssify", version, disable_help_subcommand = true)]
struct Cli {
    // requires Cargo.toml clap feature: "env"
    #[arg(long, env = "RSSIFY_REPO", default_value = "fs:./data")]
    repo: String,

    #[command(subcommand)]
    cmd: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Fetch {
        // add flags as needed later, kept minimal for compile
    },
    Stats {},
    Import {},
    Add {},
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

fn run(cli: Cli) -> Result<(), RepoError> {
    let spec = parse_repo_spec(&cli.repo)?;
    match (spec, &cli.cmd) {
        (RepoSpec::Sqlite(dsn), Command::Fetch { .. }) => {
            let repo = SqliteRepo::open(dsn)?;
            fetch_impl(&repo)
        }
        (RepoSpec::Fs(_path), Command::Fetch { .. }) => {
            // TODO: wire FS backend; returning clear placeholder for now
            Err(RepoError::Backend("fs repo not wired yet".into()))
        }
        // Stubs for other subcommands to keep compile green
        (_spec, Command::Stats { .. }) => Ok(()),
        (_spec, Command::Import { .. }) => Ok(()),
        (_spec, Command::Add { .. }) => Ok(()),
    }
}

// Generic impl used by both backends via shared traits.
// NOTE: underscore to silence unused on current stub.
fn fetch_impl<R: rssify_core::FeedRepo + rssify_core::EntryRepo + rssify_core::ScheduleRepo>(
    _r: &R,
) -> Result<(), RepoError> {
    Ok(())
}
