use serde_json::{json, Value};

pub(in crate::app) fn communication_manifest_modules() -> Vec<Value> {
    vec![
        json!({
            "name": "message",
            "command": "feishu-bot message",
            "scope_group": "im",
            "status": "typed wrappers plus native JSON payloads",
            "ai_use": "Send/read/reply/edit/delete messages; upload and send images/files/videos/audio; download resources; poll project chats with a local cursor; ack user messages with reaction status markers; list read users, reactions, and pins.",
            "help": ["feishu-bot message --help", "feishu-bot message reply --help", "feishu-bot message ack --help", "feishu-bot message poll --help", "feishu-bot message upload-image --help", "feishu-bot message upload-file --help", "feishu-bot message list --help", "feishu-bot message reaction --help", "feishu-bot message pin --help"],
            "examples": [
                "feishu-bot message send --to \"$FEISHU_USER_ID\" --text \"hello\"",
                "feishu-bot message loop-check --to \"$FEISHU_USER_ID\" --to-type open-id",
                "feishu-bot message send-image --to \"$FEISHU_USER_ID\" --file ./image.png",
                "feishu-bot message send-file --to \"$FEISHU_USER_ID\" --file ./demo.mp4 --file-type mp4",
                "feishu-bot message send-file --to \"$FEISHU_USER_ID\" --file ./voice.opus --file-type opus --duration 3000",
                "feishu-bot message send-voice --to \"$FEISHU_USER_ID\" --file ./voice.mp3 --readback",
                "feishu-bot message send-voice --to \"$FEISHU_USER_ID\" --text \"语音播报内容\" --readback",
                "feishu-bot message reply --message-id om_xxx --text \"收到，我来处理\"",
                "feishu-bot message ack --message-id om_xxx --emoji-type OK --reply-text \"已读，开始处理\" --readback",
                "feishu-bot message poll --chat-id oc_xxx --from-now --mark-seen",
                "feishu-bot message poll --chat-id oc_xxx --ack-emoji OK --reply-text \"收到\" --mark-seen",
                "feishu-bot message list --container-id oc_xxx --container-id-type chat --page-size 20",
                "feishu-bot message resource --message-id om_xxx --file-key file_xxx --type file --output ./download.bin"
            ],
            "known_permission_edges": [
                "reaction list needs im:message.reactions:read",
                "message ack uses reactions as workflow status markers; it is not an official Feishu read receipt",
                "message read-users only reports Feishu read-user data for bot-sent messages within Feishu's sender/read-user limits",
                "message poll stores a local cursor under ~/.config/feishu/message-state.json by default and ignores app/bot/system messages unless explicitly included",
                "message image/file upload and resource download need im:resource or im:resource:upload",
                "image upload is limited to 10 MB; file/video upload is limited to 30 MB",
                "send-file --msg-type auto maps mp4 to media, opus to audio, and other files to file",
                "send-voice needs ffmpeg/ffprobe for non-OPUS files and vox when synthesizing from text",
                "Use message loop-check for dogfood; it proves send/get/list/chat/member/read-users through CLI before reporting that the human-visible send loop works."
            ]
        }),
        json!({
            "name": "chat",
            "command": "feishu-bot chat",
            "scope_group": "im",
            "status": "typed wrappers plus raw JSON escape hatches",
            "ai_use": "Discover chats, inspect metadata, create/update/delete project groups, manage members, set avatars, and operate chat tabs/menus.",
            "help": ["feishu-bot chat --help", "feishu-bot chat member --help", "feishu-bot chat tab --help", "feishu-bot chat menu --help"],
            "examples": [
                "feishu-bot chat list --page-size 20",
                "feishu-bot chat create --name \"AI 项目群\" --user \"$FEISHU_USER_ID\" --avatar-file ./avatar.png",
                "feishu-bot chat update --chat-id oc_xxx --name \"AI 项目群 v2\" --avatar-file ./avatar.png",
                "feishu-bot chat tab add --chat-id oc_xxx --name \"项目页\" --tab-type url --url https://example.com",
                "feishu-bot chat menu add --chat-id oc_xxx --body-file ./menu-tree.json",
                "feishu-bot chat member list --chat-id oc_xxx"
            ],
            "known_permission_edges": [
                "Create/update chat and member management need group/chat permissions and bot ability.",
                "Group avatars use image upload with image_type=avatar.",
                "Chat tabs only support typed doc/url create/update/delete; other tab types are client-only or read-only through OpenAPI.",
                "Chat menus require the bot/user to be in the group and may require group tab/menu/widget management permission.",
                "chat delete dissolves the group for everyone; it is not a client-side hide/remove-left-sidebar operation.",
                "Personal left-sidebar labels/folders in the Feishu client are not exposed by the group OpenAPI; use project groups, avatars, tabs, menus, pins, search, and feed-card APIs instead."
            ]
        }),
        json!({
            "name": "contact",
            "command": "feishu-bot contact",
            "scope_group": "contact",
            "status": "typed wrappers",
            "ai_use": "Resolve users/departments before sending or sharing.",
            "help": ["feishu-bot contact --help"],
            "examples": ["feishu-bot contact user get --user-id \"$FEISHU_USER_ID\""]
        }),
        json!({
            "name": "directory",
            "command": "feishu-bot directory",
            "scope_group": "directory",
            "status": "typed wrappers with tenant/user-token reads and raw JSON filter escape hatch",
            "ai_use": "Search employees by keyword, batch-get employee fields, and filter employees by email, mobile, department/status, or job number.",
            "help": [
                "feishu-bot directory --help",
                "feishu-bot directory employee --help",
                "feishu-bot directory employee search --help",
                "feishu-bot directory employee mget --help",
                "feishu-bot directory employee filter --help"
            ],
            "examples": [
                "feishu-bot directory employee search --query \"张三\" --page-size 10",
                "feishu-bot directory employee mget --employee-id <open_id> --field base_info.name",
                "feishu-bot directory employee filter --condition 'base_info.email=eq=\"user@example.com\"'"
            ],
            "known_permission_edges": [
                "Tenant-token reads follow the app contact range.",
                "User-token reads follow the admin range of FEISHU_USER_ACCESS_TOKEN.",
                "Fields must be requested explicitly and each sensitive field has its own directory:* field scope."
            ]
        }),
    ]
}
