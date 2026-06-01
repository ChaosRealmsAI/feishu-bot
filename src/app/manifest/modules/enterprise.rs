use serde_json::{json, Value};

pub(in crate::app) fn enterprise_manifest_modules() -> Vec<Value> {
    vec![
        json!({
            "name": "okr",
            "command": "feishu-bot okr",
            "scope_group": "okr",
            "status": "tenant-token readable wrappers",
            "ai_use": "Read OKR periods, period rules, one user's OKR list, and batch fetch OKR details.",
            "help": ["feishu-bot okr --help", "feishu-bot okr period --help", "feishu-bot okr user-okrs --help", "feishu-bot okr batch-get --help"],
            "examples": [
                "feishu-bot okr period list --page-size 20",
                "feishu-bot okr period-rule list",
                "feishu-bot okr user-okrs --user-id \"$FEISHU_USER_ID\" --offset 0 --limit 5",
                "feishu-bot okr batch-get --okr-id <okr_id>"
            ],
            "known_permission_edges": [
                "OKR APIs require scopes such as okr:okr.period:readonly, okr:okr:readonly, or okr:okr.",
                "User OKR list reads may require okr:okr.content:readonly.",
                "Some tenants require Feishu OKR enterprise edition before period-rule or OKR reads are available."
            ]
        }),
        json!({
            "name": "attendance",
            "command": "feishu-bot attendance",
            "scope_group": "attendance",
            "status": "tenant-token wrappers with raw JSON write escape hatches",
            "ai_use": "Read attendance groups, shifts, user schedules, task results, flow records, and statistics; import/delete flow records with explicit raw JSON.",
            "help": [
                "feishu-bot attendance --help",
                "feishu-bot attendance group --help",
                "feishu-bot attendance shift --help",
                "feishu-bot attendance schedule query --help",
                "feishu-bot attendance task query --help",
                "feishu-bot attendance flow --help",
                "feishu-bot attendance stats query --help"
            ],
            "examples": [
                "feishu-bot attendance group list --page-size 20",
                "feishu-bot attendance shift list --page-size 20",
                "feishu-bot attendance schedule query --user-id <employee_id> --from 20260501 --to 20260531",
                "feishu-bot attendance task query --user-id <employee_id> --from 20260501 --to 20260531 --ignore-invalid-users",
                "feishu-bot attendance flow query --user-id <employee_id> --from-ts 1760000000 --to-ts 1760086400"
            ],
            "known_permission_edges": [
                "Attendance group and shift reads require attendance:rule or attendance:rule:readonly.",
                "Schedules, task results, flow records, and stats require attendance:task or attendance:task:readonly.",
                "Attendance APIs also depend on Feishu People/Attendance edition and attendance management data range.",
                "flow delete accepts at most 10 imported record IDs per request."
            ]
        }),
        json!({
            "name": "mail",
            "command": "feishu-bot mail",
            "scope_group": "mail",
            "status": "typed wrappers with user-token send and tenant/user-token reads",
            "ai_use": "List/read/send Mail messages and inspect folders, contacts, aliases, sendable addresses, accessible mailboxes, rules, and labels.",
            "help": [
                "feishu-bot mail --help",
                "feishu-bot mail message --help",
                "feishu-bot mail message send --help",
                "feishu-bot mail folder --help",
                "feishu-bot mail contact --help",
                "feishu-bot mail settings --help"
            ],
            "examples": [
                "feishu-bot mail message list --mailbox me --page-size 10",
                "feishu-bot mail message get --mailbox me --message-id <message_id> --format metadata",
                "feishu-bot mail folder list --mailbox me",
                "feishu-bot mail settings send-as --mailbox me",
                "feishu-bot mail message send --mailbox me --to user@example.com --subject \"hello\" --text \"body\""
            ],
            "known_permission_edges": [
                "mailbox=me and message send require FEISHU_USER_ACCESS_TOKEN.",
                "Tenant-token reads of explicit mailboxes require Mail data resource permissions.",
                "Full message bodies, subjects, addresses, and contact fields need separate Mail field scopes."
            ]
        }),
        json!({
            "name": "corehr",
            "command": "feishu-bot corehr",
            "scope_group": "corehr",
            "status": "tenant-token readable wrappers with raw JSON query escape hatches",
            "ai_use": "Search/batch-get CoreHR departments, list/get/batch-get jobs, query/get employee job data, get personal information, and list/get process instances.",
            "help": [
                "feishu-bot corehr --help",
                "feishu-bot corehr department --help",
                "feishu-bot corehr job --help",
                "feishu-bot corehr job-data --help",
                "feishu-bot corehr process --help"
            ],
            "examples": [
                "feishu-bot corehr department search --page-size 20 --field department_name",
                "feishu-bot corehr job list --page-size 20",
                "feishu-bot corehr job-data query --employment-id <id> --page-size 20",
                "feishu-bot corehr process list --modify-time-from <ms> --modify-time-to <ms>"
            ],
            "known_permission_edges": [
                "CoreHR APIs require both Open Platform scopes and Feishu People data-range grants.",
                "Sensitive fields such as department manager/custom fields, job levels, job data fields, and user_id need separate field scopes."
            ]
        }),
        json!({
            "name": "helpdesk",
            "command": "feishu-bot helpdesk",
            "scope_group": "helpdesk",
            "status": "typed wrappers with service-desk token header and raw JSON bodies",
            "ai_use": "List/get Helpdesk tickets, list ticket messages, start service conversations, send helpdesk bot messages, and read FAQ categories/articles.",
            "help": [
                "feishu-bot helpdesk --help",
                "feishu-bot helpdesk ticket --help",
                "feishu-bot helpdesk service --help",
                "feishu-bot helpdesk message --help",
                "feishu-bot helpdesk faq --help"
            ],
            "examples": [
                "feishu-bot helpdesk ticket list --page-size 20",
                "feishu-bot helpdesk ticket get --ticket-id <ticket_id>",
                "feishu-bot helpdesk ticket messages --ticket-id <ticket_id>",
                "feishu-bot helpdesk service start --open-id <open_id> --human-service",
                "feishu-bot helpdesk message send --receiver-id <open_id> --text \"hello\"",
                "feishu-bot helpdesk faq list --search \"登录\" --page-size 20"
            ],
            "known_permission_edges": [
                "Helpdesk APIs require FEISHU_HELPDESK_ID and FEISHU_HELPDESK_TOKEN from the Helpdesk admin API credential page.",
                "The CLI sends X-Lark-Helpdesk-Authorization as base64(helpdesk_id:helpdesk_token).",
                "Ticket and FAQ reads need helpdesk:all:readonly; service start needs helpdesk:helpdesk:access; bot message send needs helpdesk:all."
            ]
        }),
        json!({
            "name": "hire",
            "command": "feishu-bot hire",
            "scope_group": "hire",
            "status": "typed wrappers for core recruiting reads plus explicit raw JSON writes",
            "ai_use": "List/read Hire jobs, job schemas, talents, applications, application details, interviews, processes, requirement schemas, metadata, locations, and attachments; create talents and reopen jobs when explicitly requested.",
            "help": [
                "feishu-bot hire --help",
                "feishu-bot hire job --help",
                "feishu-bot hire talent --help",
                "feishu-bot hire application --help",
                "feishu-bot hire interview --help",
                "feishu-bot hire metadata --help",
                "feishu-bot hire location --help"
            ],
            "examples": [
                "feishu-bot hire job list --page-size 20",
                "feishu-bot hire job detail --job-id <job_id>",
                "feishu-bot hire talent list --keyword \"张三\" --page-size 10",
                "feishu-bot hire application detail --application-id <application_id> --option with_job --option with_talent",
                "feishu-bot hire interview by-talent --talent-id <talent_id>",
                "feishu-bot hire process list --page-size 50",
                "feishu-bot hire metadata resume-sources --page-size 20"
            ],
            "known_permission_edges": [
                "Hire APIs require Feishu Hire product availability and Hire data-range grants in addition to Open Platform scopes.",
                "Sensitive user_id fields require contact:user.employee_id:readonly.",
                "Application detail options such as with_offer, with_agency, with_referral, and with_portal require their corresponding hire:* readonly scopes.",
                "Tenant-specific custom fields and schema-bound writes should use --body-json/--file/--stdin copied from official OpenAPI explorer."
            ]
        }),
    ]
}
