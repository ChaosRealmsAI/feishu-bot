#!/usr/bin/env bash
set -euo pipefail

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "not inside a git repository" >&2
  exit 1
fi

blocked=0
for path in .env dogfood dogfood-artifacts tmp target vox-feishu-test; do
  if git ls-files -- "${path}" | grep -q .; then
    echo "refusing to publish tracked private/generated path: ${path}" >&2
    blocked=1
  fi
done

if git ls-files -z -- private | xargs -0 -r basename | grep -Ev '^(\.gitignore|\.env\.example)$' | grep -q .; then
  echo "refusing to publish tracked private/ files except .gitignore and .env.example" >&2
  blocked=1
fi

if git ls-files | grep -Ei '\.(pem|key|p12|pfx|aiff|opus|mp3|mp4|mov|png|jpe?g|webp)$' | grep -q .; then
  echo "refusing to publish tracked local credentials or validation media" >&2
  blocked=1
fi

patterns=(
  'PLAYWRIGHT_MCP_EXTENSION_TOKEN'
  'github_pat_'
  'ghp_[A-Za-z0-9_]+'
  'sk-[A-Za-z0-9]{20,}'
  'AKIA[0-9A-Z]{16}'
  'AIza[0-9A-Za-z_-]{35}'
  'Bearer [A-Za-z0-9._~+/=-]{20,}'
  'xox[baprs]-[A-Za-z0-9-]{20,}'
)

tmp_file="$(mktemp)"
git ls-files -z >"${tmp_file}"
for pattern in "${patterns[@]}"; do
  if xargs -0 grep --exclude=open-source-preflight.sh -nE "${pattern}" <"${tmp_file}"; then
    echo "possible secret matched pattern: ${pattern}" >&2
    blocked=1
  fi
done
rm -f "${tmp_file}"

if git grep -nE '^(FEISHU|LARK)_(APP_ID|APP_SECRET|USER_ID|USER_ACCESS_TOKEN|REFRESH_TOKEN|WIKI_SPACE_ID|WIKI_PARENT_NODE_TOKEN|HELPDESK_ID|HELPDESK_TOKEN)=' -- \
  | grep -Ev '=(replace_me|xxx|cli_xxx|ou_xxx|u_xxx|r_xxx|wik_xxx|wiki_space_id|wiki_parent_node_token|helpdesk_id|helpdesk_token|ht_xxx|123456)$'; then
  echo "possible real Feishu/Lark credential or workspace identifier in tracked files" >&2
  blocked=1
fi

if [ "${blocked}" -ne 0 ]; then
  exit 1
fi

echo "open-source preflight passed"
