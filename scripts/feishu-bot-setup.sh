#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
BIN="${FEISHU_BOT_BIN:-feishu-bot}"

open_browser=0
args=()
for arg in "$@"; do
  case "${arg}" in
    --open-browser)
      open_browser=1
      ;;
    *)
      args+=("${arg}")
      ;;
  esac
done

if [[ "${FEISHU_BOT_SETUP_OPEN_BROWSER:-}" == "1" ]]; then
  open_browser=1
fi

quickstart_args=(--json setup quickstart)
if [[ "${open_browser}" == "1" ]]; then
  quickstart_args+=(--open-browser)
else
  echo "browser opening is off; pass --open-browser after confirming the intended Chrome account" >&2
fi
quickstart_args+=("${args[@]}")

if command -v "${BIN}" >/dev/null 2>&1; then
  exec "${BIN}" "${quickstart_args[@]}"
fi

exec cargo run --manifest-path "${ROOT_DIR}/Cargo.toml" --bin feishu-bot -- \
  "${quickstart_args[@]}"
