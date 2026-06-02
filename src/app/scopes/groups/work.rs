use super::ScopeGroup;

pub(super) const BASE: ScopeGroup = ScopeGroup {
    name: "base",
    scopes: &[
        "bitable:app",
        "bitable:app:readonly",
        "base:app:create",
        "base:app:read",
        "base:app:update",
        "base:table:create",
        "base:table:read",
        "base:table:update",
        "base:table:delete",
        "base:field:create",
        "base:field:read",
        "base:field:update",
        "base:field:delete",
        "base:view:read",
        "base:view:write_only",
        "base:record:create",
        "base:record:retrieve",
        "base:record:update",
        "base:record:delete",
        "base:dashboard:read",
        "base:dashboard:copy",
        "base:workflow:read",
        "base:workflow:write",
        "base:form:read",
        "base:form:update",
        "base:role:read",
        "base:role:create",
        "base:role:update",
        "base:role:delete",
        "base:collaborator:read",
        "base:collaborator:create",
        "base:collaborator:delete",
        "docs:document.media:upload",
        "docs:document.media:download",
    ],
};

pub(super) const TASK: ScopeGroup = ScopeGroup {
    name: "task",
    scopes: &[
        "task:task:write",
        "task:task:writeonly",
        "task:task:read",
        "task:task:readonly",
        "task:personnel:writeonly",
        "task:tasklist:read",
        "task:tasklist:write",
        "task:tasklist:writeonly",
        "task:section:read",
        "task:section:write",
        "task:section:writeonly",
        "task:custom_field:read",
        "task:custom_field:write",
        "task:custom_field:writeonly",
        "task:attachment:read",
        "task:attachment:write",
        "task:attachment:upload",
        "task:attachment:delete",
        "task:comment:read",
        "task:comment:write",
        "task:comment:writeonly",
        "task:comment:delete",
    ],
};

pub(super) const CALENDAR: ScopeGroup = ScopeGroup {
    name: "calendar",
    scopes: &[
        "calendar:calendar",
        "calendar:calendar:readonly",
        "calendar:calendar:read",
        "calendar:calendar.calendar:readonly",
        "calendar:calendar.free_busy:read",
        "calendar:calendar.event:read",
        "calendar:calendar.event:create",
        "calendar:calendar.event:update",
        "calendar:calendar.event:writeonly",
        "calendar:calendar.event:delete",
    ],
};

pub(super) const VC: ScopeGroup = ScopeGroup {
    name: "vc",
    scopes: &[
        "vc:meeting",
        "vc:meeting:readonly",
        "vc:meeting.all_meeting:readonly",
        "vc:meeting.meetingevent:read",
        "vc:meeting.participant:write",
        "vc:report:readonly",
        "vc:record",
        "vc:record:readonly",
        "vc:reserve",
        "vc:reserve:readonly",
        "vc:room",
        "vc:room:readonly",
        "vc:rooms.room.basicinfo:read",
        "vc:rooms.roomlevel:read",
    ],
};

pub(super) const MINUTES: ScopeGroup = ScopeGroup {
    name: "minutes",
    scopes: &[
        "minutes:minutes",
        "minutes:minutes:readonly",
        "minutes:minutes.basic:read",
        "minutes:minutes.search:read",
        "minutes:minutes.artifacts:read",
        "minutes:minute:download",
        "minutes:minutes.media:export",
        "minutes:minutes.transcript:export",
    ],
};

pub(super) const SEARCH: ScopeGroup = ScopeGroup {
    name: "search",
    scopes: &[
        "search:docs:read",
        "search:message",
        "search:data_source",
        "search:data_source:readonly",
    ],
};

pub(super) const OKR: ScopeGroup = ScopeGroup {
    name: "okr",
    scopes: &[
        "okr:okr.period:readonly",
        "okr:okr:readonly",
        "okr:okr.content:readonly",
        "okr:okr",
    ],
};

pub(super) const ATTENDANCE: ScopeGroup = ScopeGroup {
    name: "attendance",
    scopes: &[
        "attendance:rule",
        "attendance:rule:readonly",
        "attendance:task",
        "attendance:task:readonly",
    ],
};

pub(super) const MAIL: ScopeGroup = ScopeGroup {
    name: "mail",
    scopes: &[
        "mail:user_mailbox",
        "mail:user_mailbox:readonly",
        "mail:user_mailbox.message:readonly",
        "mail:user_mailbox.message:send",
        "mail:user_mailbox.message:modify",
        "mail:user_mailbox.message.subject:read",
        "mail:user_mailbox.message.address:read",
        "mail:user_mailbox.message.body:read",
        "mail:user_mailbox.folder:read",
        "mail:user_mailbox.folder:write",
        "mail:user_mailbox.mail_contact:read",
        "mail:user_mailbox.mail_contact:write",
        "mail:user_mailbox.mail_contact.mail_address:read",
        "mail:user_mailbox.mail_contact.phone:read",
        "mail:user_mailbox.rule:read",
        "mail:user_mailbox.rule:write",
        "contact:user.employee_id:readonly",
    ],
};

pub(super) const HELPDESK: ScopeGroup = ScopeGroup {
    name: "helpdesk",
    scopes: &[
        "helpdesk:all:readonly",
        "helpdesk:all",
        "helpdesk:helpdesk:access",
        "contact:user.employee_id:readonly",
    ],
};

pub(super) const APPROVAL: ScopeGroup = ScopeGroup {
    name: "approval",
    scopes: &[
        "approval:approval",
        "approval:approval:readonly",
        "approval:approval.list:readonly",
        "approval:definition",
        "approval:instance",
        "approval:instance:readonly",
        "approval:task",
        "approval:external_approval",
        "approval:external_instance",
        "approval:external_task",
    ],
};
