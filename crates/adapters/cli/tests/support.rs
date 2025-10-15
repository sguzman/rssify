/*
Module: rssify_cli::test::support
Purpose: Small helpers for tests (path building, temp dirs later)
*/

pub fn td(p: &str) -> std::path::PathBuf {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    here.join("test").join("testdata").join(p)
}

