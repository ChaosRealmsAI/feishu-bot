use super::ScopeGroup;

pub(super) const IM: ScopeGroup = ScopeGroup {
    name: "im",
    scopes: &[
        "im:message",
        "im:message:readonly",
        "im:message.history:readonly",
        "im:message:send_as_bot",
        "im:message:update",
        "im:message:recall",
        "im:message.group_at_msg:readonly",
        "im:message.group_msg",
        "im:message.p2p_msg:readonly",
        "im:message.reactions:read",
        "im:message.reactions:write_only",
        "im:message.pins:read",
        "im:message.pins:write_only",
        "im:resource",
        "im:resource:upload",
        "im:chat",
        "im:chat:create",
        "im:chat:operate_as_owner",
        "im:chat.group_info:readonly",
    ],
};

pub(super) const DOC: ScopeGroup = ScopeGroup {
    name: "doc",
    scopes: &[
        "docx:document",
        "docx:document:readonly",
        "docx:document:write_only",
        "docx:document:create",
        "docx:document.block:convert",
    ],
};

pub(super) const BOARD: ScopeGroup = ScopeGroup {
    name: "board",
    scopes: &["board:whiteboard:node:create", "board:whiteboard:node:read"],
};

pub(super) const DRIVE: ScopeGroup = ScopeGroup {
    name: "drive",
    scopes: &[
        "drive:drive",
        "drive:drive:readonly",
        "drive:file",
        "drive:file:readonly",
        "drive:file:upload",
        "drive:file:download",
        "docs:doc",
        "docs:document.media:upload",
        "docs:document.media:download",
        "docs:document:import",
        "docs:document:export",
        "drive:export:readonly",
        "docs:document.comment:read",
        "docs:document.comment:create",
        "docs:document.comment:update",
        "docs:document.comment:delete",
        "docs:document.comment:write_only",
        "docs:document.subscription",
        "docs:document.subscription:read",
        "drive:drive:version",
        "drive:drive:version:readonly",
        "drive:file:view_record:readonly",
        "contact:user.base:readonly",
        "contact:user.employee_id:readonly",
        "space:document:retrieve",
    ],
};

pub(super) const PERMISSION: ScopeGroup = ScopeGroup {
    name: "permission",
    scopes: &[
        "docs:permission.member",
        "docs:permission.member:read",
        "docs:permission.member:readonly",
        "docs:permission.member:retrieve",
        "docs:permission.member:create",
        "docs:permission.member:update",
        "docs:permission.member:delete",
        "docs:permission.member:auth",
        "docs:permission.setting",
        "docs:permission.setting:read",
        "docs:permission.setting:readonly",
        "docs:permission.setting:write_only",
    ],
};

pub(super) const WIKI: ScopeGroup = ScopeGroup {
    name: "wiki",
    scopes: &[
        "wiki:wiki",
        "wiki:wiki:readonly",
        "wiki:space:retrieve",
        "wiki:space:read",
        "wiki:space:write_only",
        "wiki:node:retrieve",
        "wiki:node:read",
        "wiki:node:create",
        "wiki:node:move",
        "wiki:node:copy",
        "wiki:node:update",
        "wiki:member:retrieve",
        "wiki:member:create",
        "wiki:member:update",
        "wiki:setting:write_only",
    ],
};

pub(super) const SHEET: ScopeGroup = ScopeGroup {
    name: "sheet",
    scopes: &[
        "sheets:spreadsheet",
        "sheets:spreadsheet:readonly",
        "sheets:spreadsheet:create",
        "sheets:spreadsheet:read",
        "sheets:spreadsheet:write_only",
        "sheets:spreadsheet.meta:read",
        "sheets:spreadsheet.meta:write_only",
        "drive:drive",
        "drive:drive:readonly",
    ],
};
