# Run all local preflight checks before pushing
check:
    python3 scripts/check_headers.py
    python3 scripts/check_loc.py
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo build --workspace --all-targets
    cargo test --workspace --all-features -- --quiet

# Individual guardrails
headers:
    python3 scripts/check_headers.py

loc:
    python3 scripts/check_loc.py

fmt:
    cargo fmt --all -- --check

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

build:
    cargo build --workspace --all-targets

test:
    cargo test --workspace --all-features -- --quiet

