#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
BIN="${FEISHU_BOT_BIN:-feishu-bot}"

if command -v "${BIN}" >/dev/null 2>&1; then
  exec "${BIN}" --json setup quickstart --open-browser "$@"
fi

exec cargo run --manifest-path "${ROOT_DIR}/Cargo.toml" --bin feishu-bot -- \
  --json setup quickstart --open-browser "$@"
