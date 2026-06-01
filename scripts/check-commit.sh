#!/usr/bin/env bash
set -euo pipefail

if [[ "${FEISHU_BOT_FULL_COMMIT_CHECK:-0}" == "1" ]]; then
  scripts/ci-local.sh
  exit 0
fi

scripts/open-source-preflight.sh
cargo fmt -- --check
cargo check --all-targets
