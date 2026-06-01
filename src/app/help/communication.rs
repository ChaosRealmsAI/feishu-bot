pub(in crate::app) const MESSAGE_SEND_AFTER_HELP: &str = r#"Examples:
  feishu-bot message send --to "$FEISHU_USER_ID" --text "hello"
  feishu-bot message loop-check --to "$FEISHU_USER_ID" --to-type open-id
  printf 'multi-line\nmessage\n' | feishu-bot message send --to "$FEISHU_USER_ID" --stdin
  feishu-bot message send --to oc_xxx --to-type chat-id --file ./message.txt
  feishu-bot message send-json --to "$FEISHU_USER_ID" --msg-type interactive --content-json '{"config":{"wide_screen_mode":true},"elements":[]}'
  feishu-bot message upload-image --file ./image.png
  feishu-bot message send-image --to "$FEISHU_USER_ID" --file ./image.png
  feishu-bot message upload-file --file ./demo.mp4 --file-type mp4 --duration 3000
  feishu-bot message send-file --to "$FEISHU_USER_ID" --file ./demo.mp4 --file-type mp4
  feishu-bot message send-file --to "$FEISHU_USER_ID" --file ./voice.opus --file-type opus --duration 3000
  feishu-bot message send-voice --to "$FEISHU_USER_ID" --file ./voice.mp3 --readback
  feishu-bot message send-voice --to "$FEISHU_USER_ID" --text "语音播报内容" --readback
  feishu-bot message reply-json --message-id om_xxx --msg-type text --content-json '{"text":"reply"}'
  feishu-bot message reply --message-id om_xxx --text "收到，我来处理"
  feishu-bot message ack --message-id om_xxx --emoji-type OK --reply-text "已读，开始处理"
  feishu-bot message poll --chat-id oc_xxx --from-now --mark-seen
  feishu-bot message poll --chat-id oc_xxx --ack-emoji OK --reply-text "收到" --mark-seen
  feishu-bot message edit-json --message-id om_xxx --msg-type text --content-json '{"text":"edited"}'
  feishu-bot message delete --message-id om_xxx
  feishu-bot message list --container-id oc_xxx --container-id-type chat --page-size 20
  feishu-bot message get --message-id om_xxx
  feishu-bot message read-users --message-id om_xxx
  feishu-bot message resource --message-id om_xxx --file-key file_xxx --type file --output ./download.bin
  feishu-bot message download-image --image-key img_v2_xxx --output ./image.png
  feishu-bot message download-file --file-key file_xxx --output ./download.bin
  feishu-bot message reaction list --message-id om_xxx
  feishu-bot message reaction add --message-id om_xxx --emoji-type SMILE
  feishu-bot message reaction delete --message-id om_xxx --reaction-id <reaction_id>
  feishu-bot message pin list --chat-id oc_xxx
  feishu-bot message pin add --message-id om_xxx
  feishu-bot message pin delete --message-id om_xxx

Receiver type inference:
  ou_... -> open_id
  oc_... -> chat_id
  on_... -> union_id
  contains @ -> email
  otherwise -> user_id

send-file message type:
  --msg-type auto maps --file-type mp4 to media/video, opus to audio, otherwise file.
  Use --cover-image-key <image_key> to set a video cover image when sending media.

send-voice:
  Use --file for MP3/WAV/M4A/OPUS input. Non-OPUS files are converted with ffmpeg
  and duration is detected with ffprobe. Use --text/--text-file/--stdin to call
  vox first, then send the generated voice as a Feishu audio message.

Use `message loop-check` for dogfood. It sends one text message, then reads back
the message by message_id, lists the target chat, reads chat metadata, lists chat
members, and checks read-users. Do not claim a human-visible send loop is proven
unless message_get/list/chat/member probes all pass and the target member is the
expected Feishu account.

Use `message poll --from-now --mark-seen` once per project chat to establish a
local cursor, then run `message poll --ack-emoji OK --reply-text "收到"` to pick
up user messages, add a reaction status, optionally reply, and move the cursor.
`message ack` uses Feishu reactions as workflow status markers; it is not an
official read receipt. Use `message read-users` only for Feishu read-user data on
messages sent by the bot.
"#;

pub(in crate::app) const CONTACT_AFTER_HELP: &str = r#"AI-safe contact workflow:
  feishu-bot contact user get --user-id "$FEISHU_USER_ID"
  feishu-bot contact user list --department-id 0 --page-size 10
  feishu-bot contact department children --department-id 0 --page-size 10
  feishu-bot contact department get --department-id 0
  feishu-bot contact department search --query "研发"

Tenant-token access is limited by the app's contact scope and visible
department range. Use `feishu-bot scopes --group contact` when permissions are
missing.
"#;

pub(in crate::app) const DIRECTORY_AFTER_HELP: &str = r#"AI-safe Directory workflow:
  feishu-bot directory employee search --query "张三" --page-size 10
  feishu-bot directory employee search --query user@example.com --field base_info.employee_id --field base_info.email
  feishu-bot directory employee mget --employee-id <open_id> --field base_info.name --field work_info.job_title
  feishu-bot directory employee filter --condition 'base_info.email=eq="user@example.com"'
  feishu-bot directory employee filter --condition 'work_info.job_number=eq="E12345"' --field base_info.name

Directory v1 is the newer admin org-directory API. It supports tenant and user
tokens; tenant mode follows the app contact range, user mode follows the admin
range of FEISHU_USER_ACCESS_TOKEN. Pass --body-json/--file/--stdin for full
official filter bodies.
"#;

pub(in crate::app) const NOTIFY_AFTER_HELP: &str = r#"Examples:
  feishu-bot notify --to "$FEISHU_USER_ID" --status done --task "build" --summary "passed"
  feishu-bot notify --project my-project --status error --summary "tests failed" --details "cargo test failed|see logs"
  feishu-bot notify --project my-project --link "https://example.com/report" --text "full report"

Without --to:
  The CLI creates/reuses a private project chat, stores the mapping in
  ~/.config/feishu/projects.json.
"#;

pub(in crate::app) const CHAT_AFTER_HELP: &str = r#"AI-safe chat workflow:
  feishu-bot chat list
  feishu-bot chat search --query "项目"
  feishu-bot chat get --chat-id oc_xxx
  feishu-bot chat create --name "AI 项目群" --user "$FEISHU_USER_ID" --avatar-file ./avatar.png
  feishu-bot chat update --chat-id oc_xxx --name "AI 项目群 v2" --avatar-file ./avatar.png
  feishu-bot chat member list --chat-id oc_xxx
  feishu-bot chat member add --chat-id oc_xxx --id "$FEISHU_USER_ID"
  feishu-bot chat member is-in-chat --chat-id oc_xxx
  feishu-bot chat member delete --chat-id oc_xxx --id <open_id>
  feishu-bot chat tab list --chat-id oc_xxx
  feishu-bot chat tab add --chat-id oc_xxx --name "项目页" --tab-type url --url https://example.com
  feishu-bot chat tab add --chat-id oc_xxx --name "知识库" --tab-type doc --doc https://my.feishu.cn/wiki/xxx
  feishu-bot chat menu get --chat-id oc_xxx
  feishu-bot chat menu add --chat-id oc_xxx --body-file ./menu-tree.json
  feishu-bot chat delete --chat-id oc_xxx

Use `chat list` or `chat search` to discover oc_ chat IDs before sending group
messages. Member add/delete defaults to open_id; use --member-id-type app-id
when adding a bot by App ID. For AI project isolation, prefer one group per
project/topic, set a recognizable avatar, add doc/url tabs for durable context,
pin important messages, and use group menus for common links/actions. Feishu's
personal left-sidebar labels are client-side and are not exposed by the group
OpenAPI; use naming prefixes, avatars, tabs, menus, pins, and optional feed-card
APIs as the automatable substitute. `chat delete` dissolves the group for
everyone; it is not a client-side "hide/remove this conversation from my left
sidebar" operation.
"#;
