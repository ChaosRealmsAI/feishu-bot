pub(in crate::app) const DOC_AFTER_HELP: &str = r#"AI-safe workflow:
  1. feishu-bot doc preview --file ./doc.md
  2. feishu-bot doc create --title "Title" --file ./doc.md
  3. feishu-bot doc blocks --document-id <id>
  4. feishu-bot doc raw --document-id <id>
  5. feishu-bot doc send-link --document-id <id> --to "$FEISHU_USER_ID" --send-loop-check

For docx objects created by `feishu-bot wiki create-node --auth user`, keep using
the user token for writes and reads:
  feishu-bot doc append --auth user --document-id <obj_token> --file ./doc.md
  feishu-bot doc raw --auth user --document-id <obj_token>

Supported Markdown-ish input -> native Feishu docx blocks:
  #..######### headings -> heading1..heading9
  plain text           -> text
  - item               -> bullet
  1. item              -> ordered
  ```rust code fences  -> code with Feishu CodeLanguage when known
  ```mermaid fences    -> code/plain-text source, not rendered Mermaid plugin
  > quote              -> quote
  - [ ] / - [x]        -> todo
  ---                  -> divider

For rare/new Feishu blocks and subtype fields, use:
  feishu-bot doc template --kind all
  feishu-bot doc template --kind table-descendant
  feishu-bot doc insert-media --document-id <id> --kind image --file ./image.png
  feishu-bot doc insert-media --document-id <id> --kind file --file ./report.pdf
  feishu-bot doc append-json --document-id <id> --file ./children.json
  feishu-bot doc append-descendant --document-id <id> --file ./descendant-body.json

Run `feishu-bot doc capabilities` for the full AI writing boundary. Use
--send-loop-check whenever a document link is sent as dogfood; it proves the
exact link message through message get/list, chat metadata, chat members, and
read-users probes.
"#;

pub(in crate::app) const DOC_MEDIA_AFTER_HELP: &str = r#"AI-safe docx media workflow:
  feishu-bot doc insert-media --document-id <id> --kind image --file ./image.png --width 640 --align 2
  feishu-bot doc insert-media --document-id <id> --kind file --file ./report.pdf --view-type 1
  feishu-bot doc blocks --document-id <id>
  feishu-bot doc raw --document-id <id>

insert-media automates Feishu's required docx media sequence:
  1. append an image/file placeholder block under --block-id or the document root
  2. upload the local asset with drive/v1/medias/upload_all using docx_image/docx_file
  3. patch the block with replace_image or replace_file

The command needs docx block write scopes plus docs:document.media:upload. It is
for files up to 20 MB, matching Feishu's upload_all media endpoint.
"#;

pub(in crate::app) const DOC_CAPABILITIES: &str = r#"Feishu docx AI writing capabilities

Recommended writer choice:
  --writer official   Feishu Markdown/HTML converter; best for normal AI docs,
                      tables, links, inline styles, lists, headings, and code.
  --writer local      Predictable direct block creation; no converter scope needed.
  append-json         Raw child blocks under one parent block.
  append-descendant   Raw nested descendant request body with explicit block IDs.
  insert-media        One-shot image/file block insertion with Drive media upload.

Local Markdown-ish writer:
  #..######### headings -> heading1..heading9
  plain text           -> text
  - item               -> bullet
  1. item              -> ordered
  > quote              -> quote
  - [ ] / - [x]        -> todo
  ---                  -> divider
  ```rust fences       -> code with CodeLanguage when Feishu has that enum
  ```mermaid fences    -> code block with PlainText language; source is preserved

Mermaid boundary:
  Feishu's public docx OpenAPI exposes diagram blocks with diagram_type
  1=flowchart and 2=UML, but does not expose a Mermaid source field. The
  official Markdown converter also maps ```mermaid to a normal code block.
  Therefore this CLI preserves Mermaid source as code in docx. For rendered
  Mermaid/PlantUML, create or locate a board block and use:
    feishu-bot board import --whiteboard-id <id> --syntax mermaid --file diagram.mmd

Raw subtype coverage:
  For block types or subtype fields not modeled by the local writer, generate
  Feishu's native JSON and call append-json or append-descendant. This is how AI
  should write table/table_cell descendants, grid/grid_column, iframe with
  iframe.component.type/url, file/image tokens, bitable, sheet, callout,
  isv/add_ons, board, agenda, link_preview, sub_page_list, and future writable
  block types.

Image/file media:
  Use `feishu-bot doc insert-media` for normal images and attachments. It creates
  the target block, uploads with drive media, then patches the block token.

Known non-writable public docx blocks:
  diagram/rendered Mermaid, mindnote, task blocks, synced blocks, and AI
  template blocks are not writable through the public docx OpenAPI today. Do not
  invent JSON for them.

Known BlockType labels:
  1 page, 2 text, 3..11 heading1..heading9, 12 bullet, 13 ordered, 14 code,
  15 quote, 17 todo, 18 bitable, 19 callout, 20 chat_card, 21 diagram,
  22 divider, 23 file, 24 grid, 25 grid_column, 26 iframe, 27 image, 28 isv,
  29 mindnote, 30 sheet, 31 table, 32 table_cell, 33 view, 34 quote_container,
  35 task, 36 okr, 37 okr_objective, 38 okr_key_result, 39 okr_progress,
  40 add_ons, 41 jira_issue, 42 wiki_catalog, 43 board, 44 agenda,
  45 agenda_item, 46 agenda_item_title, 47 agenda_item_content,
  48 link_preview, 49 source_synced, 50 reference_synced, 51 sub_page_list,
  52 ai_template, 999 undefined.
"#;

pub(in crate::app) const DOC_TEMPLATE_AFTER_HELP: &str = r#"Examples:
  feishu-bot doc template --kind all
  feishu-bot doc template --kind support-matrix
  feishu-bot doc template --kind mermaid-code-child > mermaid.json
  feishu-bot doc append-json --document-id <id> --file mermaid.json
  feishu-bot doc template --kind image-child > image.json
  feishu-bot doc template --kind link-preview-child > link.json
  feishu-bot doc template --kind table-descendant > table.json
  feishu-bot doc append-descendant --document-id <id> --file table.json

Template classes:
  *-child        Request body for doc append-json / children API.
  *-descendant  Request body for doc append-descendant / descendant API.
  support-matrix Machine-readable write strategy for common docx block types.

Mermaid note:
  docx Mermaid is stored as source code. For rendered Mermaid, create or locate a
  board block, get its whiteboard_id from `feishu-bot doc blocks`, then run
  `feishu-bot board import --syntax mermaid --whiteboard-id <id>`.
"#;

pub(in crate::app) const DOC_PREVIEW_AFTER_HELP: &str = r#"Examples:
  feishu-bot doc preview --file ./guide.md
  feishu-bot --json doc preview --file ./guide.md
  printf '# Title\n\n- item\n' | feishu-bot doc preview --stdin

This command does not call Feishu and does not need FEISHU_APP_ID/SECRET.
"#;

pub(in crate::app) const DOC_CREATE_AFTER_HELP: &str = r#"Examples:
  feishu-bot doc create --title "Runbook" --file ./runbook.md
  feishu-bot doc create --title "Runbook" --writer official --content-type markdown --file ./runbook.md
  feishu-bot doc create --title "HTML import" --writer official --content-type html --file ./page.html
  feishu-bot doc create --title "Runbook" --stdin < ./runbook.md
  feishu-bot doc create --title "Runbook" --file ./runbook.md --send-to "$FEISHU_USER_ID" --send-loop-check
  feishu-bot doc create --title "Dogfood" --writer official --file ./demo.md --wiki --wiki-space-id <space_id> --wiki-fallback-ok
  feishu-bot wiki create-node --auth user --space-id <space_id> --title "AI 演示" --obj-type docx
  feishu-bot doc append --auth user --document-id <wiki_obj_token> --writer official --file ./demo.md
  FEISHU_WIKI_SPACE_ID=<space_id> feishu-bot doc create --title "Dogfood" --file ./demo.md --wiki
  FEISHU_DOC_CREATE_WIKI_DEFAULT=true FEISHU_WIKI_SPACE_ID=<space_id> feishu-bot doc create --title "Dogfood" --file ./demo.md
  FEISHU_DOC_CREATE_WIKI_DEFAULT=true FEISHU_WIKI_SPACE_ID=<space_id> feishu-bot doc create --title "Strict Wiki" --file ./demo.md --wiki-strict
  feishu-bot doc create --title "Private draft" --file ./draft.md --no-wiki

The root page block_id equals document_id, so appended content is inserted under
the document root by default.

Wiki publishing creates the docx first, then calls Wiki move_docs_to_wiki. Use
FEISHU_DOC_CREATE_WIKI_DEFAULT=true plus FEISHU_WIKI_SPACE_ID and optional
FEISHU_WIKI_PARENT_NODE_TOKEN to make this the default dogfood route. Use
--no-wiki for one-off local docs. When FEISHU_DOC_CREATE_WIKI_DEFAULT=true,
Wiki move failures keep and return the fallback docx unless --wiki-strict is
passed. Use --wiki-fallback-ok for explicit one-off --wiki commands that must
also return and send the fallback docx when Wiki permissions are not ready.
Use --send-loop-check whenever --send-to is part of dogfood; it proves the exact
doc link message with message get/list, chat metadata, chat members, and
read-users probes.
"#;

pub(in crate::app) const DOC_CONVERT_AFTER_HELP: &str = r#"Examples:
  feishu-bot doc convert --file ./guide.md
  feishu-bot doc convert --content-type html --file ./page.html
  feishu-bot --json doc convert --file ./guide.md

This calls Feishu's official Markdown/HTML -> docx blocks converter and needs
the docx:document.block:convert app scope.
"#;

pub(in crate::app) const DOC_RAW_BLOCK_AFTER_HELP: &str = r#"Advanced AI escape hatch:
  feishu-bot doc append-json --document-id <id> --file ./children.json
  feishu-bot doc append-json --document-id <id> --raw-json '[{"block_type":2,...}]'
  feishu-bot doc append-descendant --document-id <id> --file ./descendant-body.json

append-json accepts either:
  [{...block...}, {...block...}]
  {"children":[{...block...}]}

append-descendant accepts the full Feishu descendant request body, for example:
  {"index":-1,"children_id":["block_a"],"descendants":[{"block_id":"block_a",...}]}

Use this when the AI needs a newer/rarer Feishu block that the local writer does
not model yet.
"#;

pub(in crate::app) const BOARD_AFTER_HELP: &str = r#"AI-safe Board workflow:
  feishu-bot board template --style brutal-note --title "系统流程" > board.svg
  feishu-bot board check-svg --file board.svg --external
  feishu-bot board svg --file board.svg --print-nodes --check --external-check --render-output board.png
  feishu-bot board create --title "系统流程画板" --file board.svg --check --external-check --send-to <chat_id> --send-to-type chat-id
  feishu-bot doc template --kind board-child > board.json
  feishu-bot doc append-json --document-id <doc_id> --file board.json
  feishu-bot doc blocks --document-id <doc_id>
  feishu-bot board import --whiteboard-id <whiteboard_id> --syntax mermaid --file ./diagram.mmd
  feishu-bot board import --whiteboard-id <whiteboard_id> --syntax plantuml --file ./diagram.puml
  feishu-bot board node-create --whiteboard-id <whiteboard_id> --file ./nodes.json

`board template` prints local native-shape SVG starters. `board check-svg` runs
local medium checks; `--external` also runs @larksuite/whiteboard-cli through
npx. `board svg` converts SVG into Board OpenAPI nodes and either prints them
or writes them to an existing whiteboard. `board create` creates a docx, appends
a whiteboard block, converts the SVG, writes editable nodes, and can send the
link with delivery proof. SVG conversion requires Node/npm and npx access to
@larksuite/whiteboard-cli.

The docx `diagram` block is not writable through the public docx OpenAPI. The
Board API is the supported rendered Mermaid/PlantUML path when a whiteboard
block exists in the document.
"#;

pub(in crate::app) const DRIVE_AFTER_HELP: &str = r#"AI-safe Drive workflow:
  feishu-bot drive list --folder-token <folder_token>
  feishu-bot drive folder create --name "AI 输出" --folder-token ""
  feishu-bot drive upload --folder-token <folder_token> --file ./report.pdf
  feishu-bot drive upload-large --folder-token <folder_token> --file ./large-video.mp4
  feishu-bot drive media upload --parent-type docx_image --parent-node <image_block_id> --drive-route-token <document_id> --file ./image.png
  feishu-bot drive media upload --parent-type bitable_file --parent-node <app_token> --drive-route-token <app_token> --file ./video.mp4
  feishu-bot drive media download --file-token <media_token> --output ./asset.bin
  feishu-bot drive import file --file ./page.html --type docx --folder-token "" --title "HTML Preview"
  feishu-bot drive import get --ticket <ticket>
  feishu-bot drive export file --token <docx_token> --type docx --file-extension pdf --output ./doc.pdf
  feishu-bot drive export create --token <sheet_token> --type sheet --file-extension xlsx
  feishu-bot drive comment create --file-token <docx_token> --file-type docx --text "需要复核"
  feishu-bot drive comment list --file-token <docx_token> --file-type docx --is-whole
  feishu-bot drive version create --file-token <docx_token> --obj-type docx --name "AI 修订版"
  feishu-bot drive view-record --file-token <docx_token> --file-type docx
  feishu-bot drive download --file-token <file_token> --output ./report.pdf
  feishu-bot drive permission public-get --token <docx_token> --file-type docx
  feishu-bot drive permission member-list --token <docx_token> --file-type docx
  feishu-bot drive permission member-add --token <docx_token> --file-type docx --member-id "$FEISHU_USER_ID" --perm edit
  feishu-bot drive stats --file-token <token> --file-type docx
  feishu-bot drive copy --file-token <token> --file-type docx --folder-token <folder_token>

Folder token "" means the root folder for create-folder/import. Existing
user-owned folders still need the app to have document/folder access. `upload`
uses drive/v1/files/upload_all for Drive files. `media upload` uses
drive/v1/medias/upload_all for doc/sheet/Base assets and HTML/Markdown import
staging. Both single-call upload paths support non-empty files up to 20 MB; use
`upload-large` for Drive files that need the official multipart
upload_prepare/upload_part/upload_finish flow.
`export` creates/polls/downloads asynchronous docx/sheet/Base export tasks.
`comment` manages global comments and replies; use raw JSON for complex comment
elements. `subscription` uses user_access_token because Feishu's subscription
API is user-token only.
`permission member-list` is the readback step after sharing a docx/sheet/Base
with a user or chat; verify collaborators before claiming the recipient can
access the artifact.
For media assets embedded in docs/sheets/Base, use the matching media endpoint
and Feishu block/parent token semantics.
"#;
