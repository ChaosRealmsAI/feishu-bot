#!/usr/bin/env bash
set -euo pipefail

scripts/open-source-preflight.sh
cargo fmt -- --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
