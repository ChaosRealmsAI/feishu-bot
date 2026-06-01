use super::*;

#[derive(Clone, Copy)]
enum DogfoodProbeAuth {
    Tenant,
    User,
    Helpdesk,
}

struct DogfoodProbeSpec {
    module: &'static str,
    name: &'static str,
    command: &'static str,
    operation: &'static str,
    scope_group: &'static str,
    auth: DogfoodProbeAuth,
    method: Method,
    path: String,
    query: Vec<(String, String)>,
    body: Option<Value>,
}

pub(super) async fn verify_dogfood(
    api: &mut FeishuClient,
    args: DogfoodVerifyArgs,
) -> Result<Value> {
    let started_at = Local::now().to_rfc3339();
    let mut probes = Vec::new();

    if dogfood_module_selected(&args.module, "auth", "auth.tenant_token") {
        let token_probe = match api.tenant_token().await {
            Ok(token) => dogfood_probe_from_result(
                "auth",
                "auth.tenant_token",
                "feishu-bot --json token",
                "POST /auth/v3/tenant_access_token/internal",
                "auth",
                probe_value(Ok(json!({
                    "code": 0,
                    "msg": "success",
                    "data": {
                        "tenant_access_token": mask_secret(&token),
                    }
                }))),
                args.include_response,
                &api.config.app_id,
            ),
            Err(error) => dogfood_probe_from_result(
                "auth",
                "auth.tenant_token",
                "feishu-bot --json token",
                "POST /auth/v3/tenant_access_token/internal",
                "auth",
                probe_value(Err(error)),
                args.include_response,
                &api.config.app_id,
            ),
        };
        probes.push(token_probe);
    }

    for spec in dogfood_probe_specs() {
        if dogfood_module_selected(&args.module, spec.module, spec.name) {
            probes.push(run_dogfood_probe(api, spec, args.include_response).await);
        }
    }

    if args.send_loop_check
        && dogfood_module_selected(&args.module, "message", "message.loop_check")
    {
        probes.push(
            run_dogfood_message_loop_probe(
                api,
                args.to.clone(),
                args.to_type,
                args.include_response,
            )
            .await,
        );
    }

    if args.write {
        for probe in run_dogfood_write_probes(api, &args).await {
            probes.push(probe);
        }
    }

    let summary = summarize_dogfood_probes(&probes);
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "started_at": started_at,
            "finished_at": Local::now().to_rfc3339(),
            "mode": {
                "read_probes": true,
                "write_probes": args.write,
                "send_loop_check": args.send_loop_check,
                "include_response": args.include_response,
                "module_filter": args.module,
            },
            "environment": {
                "base_url": api.config.base_url,
                "app_id": mask_app_id(&api.config.app_id),
                "default_user_id": api.config.default_user_id.as_deref().map(mask_secret).unwrap_or_else(|| "missing".to_string()),
                "user_access_token": api.config.user_access_token.as_deref().map(mask_secret).unwrap_or_else(|| "missing".to_string()),
                "helpdesk_id": api.config.helpdesk_id.as_deref().map(mask_secret).unwrap_or_else(|| "missing".to_string()),
                "wiki_space_id": api.config.default_wiki_space_id.as_deref().map(mask_secret).unwrap_or_else(|| "missing".to_string()),
            },
            "summary": summary,
            "probes": probes,
        }
    }))
}

fn dogfood_probe_specs() -> Vec<DogfoodProbeSpec> {
    vec![
        DogfoodProbeSpec {
            module: "bot",
            name: "bot.info",
            command: "feishu-bot --json bot info",
            operation: "GET /bot/v3/info",
            scope_group: "bot",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/bot/v3/info".to_string(),
            query: Vec::new(),
            body: None,
        },
        DogfoodProbeSpec {
            module: "message",
            name: "message.chat_list",
            command: "feishu-bot --json chat list --page-size 1",
            operation: "GET /im/v1/chats",
            scope_group: "im",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/im/v1/chats".to_string(),
            query: vec![("page_size".to_string(), "1".to_string())],
            body: None,
        },
        DogfoodProbeSpec {
            module: "contact",
            name: "contact.users.list",
            command: "feishu-bot --json contact user list --page-size 1",
            operation: "GET /contact/v3/users",
            scope_group: "contact",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/contact/v3/users".to_string(),
            query: vec![
                ("page_size".to_string(), "1".to_string()),
                ("user_id_type".to_string(), "open_id".to_string()),
                (
                    "department_id_type".to_string(),
                    "open_department_id".to_string(),
                ),
            ],
            body: None,
        },
        DogfoodProbeSpec {
            module: "drive",
            name: "drive.files.list",
            command: "feishu-bot --json drive list --page-size 1",
            operation: "GET /drive/v1/files",
            scope_group: "drive",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/drive/v1/files".to_string(),
            query: vec![
                ("page_size".to_string(), "1".to_string()),
                ("order_by".to_string(), "EditedTime".to_string()),
                ("direction".to_string(), "DESC".to_string()),
                ("user_id_type".to_string(), "open_id".to_string()),
            ],
            body: None,
        },
        DogfoodProbeSpec {
            module: "calendar",
            name: "calendar.primary",
            command: "feishu-bot --json calendar primary",
            operation: "GET /calendar/v4/calendars/primary",
            scope_group: "calendar",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/calendar/v4/calendars/primary".to_string(),
            query: Vec::new(),
            body: None,
        },
        DogfoodProbeSpec {
            module: "calendar",
            name: "calendar.list",
            command: "feishu-bot --json calendar list --page-size 50",
            operation: "GET /calendar/v4/calendars",
            scope_group: "calendar",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/calendar/v4/calendars".to_string(),
            query: vec![("page_size".to_string(), "50".to_string())],
            body: None,
        },
        DogfoodProbeSpec {
            module: "task",
            name: "task.my_tasks.list",
            command: "feishu-bot --json task list --completed false --type my_tasks",
            operation: "GET /task/v2/tasks",
            scope_group: "task",
            auth: DogfoodProbeAuth::User,
            method: Method::GET,
            path: "/task/v2/tasks".to_string(),
            query: vec![
                ("page_size".to_string(), "1".to_string()),
                ("completed".to_string(), "false".to_string()),
                ("type".to_string(), "my_tasks".to_string()),
                ("user_id_type".to_string(), "open_id".to_string()),
            ],
            body: None,
        },
        DogfoodProbeSpec {
            module: "task",
            name: "task.tenant_scope_probe",
            command: "feishu-bot --json task list --auth tenant --completed false --type my_tasks",
            operation: "GET /task/v2/tasks",
            scope_group: "task",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/task/v2/tasks".to_string(),
            query: vec![
                ("page_size".to_string(), "1".to_string()),
                ("completed".to_string(), "false".to_string()),
                ("type".to_string(), "my_tasks".to_string()),
                ("user_id_type".to_string(), "open_id".to_string()),
            ],
            body: None,
        },
        DogfoodProbeSpec {
            module: "wiki",
            name: "wiki.spaces.list",
            command: "feishu-bot --json wiki spaces --page-size 1",
            operation: "GET /wiki/v2/spaces",
            scope_group: "wiki",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/wiki/v2/spaces".to_string(),
            query: vec![("page_size".to_string(), "1".to_string())],
            body: None,
        },
        DogfoodProbeSpec {
            module: "search",
            name: "search.docs",
            command: "feishu-bot --json search docs --query dogfood --page-size 1",
            operation: "POST /search/v2/doc_wiki/search",
            scope_group: "search",
            auth: DogfoodProbeAuth::User,
            method: Method::POST,
            path: "/search/v2/doc_wiki/search".to_string(),
            query: Vec::new(),
            body: Some(json!({
                "query": "dogfood",
                "page_size": 1,
                "doc_filter": {},
                "wiki_filter": {},
            })),
        },
        DogfoodProbeSpec {
            module: "okr",
            name: "okr.periods.list",
            command: "feishu-bot --json okr period list --page-size 1",
            operation: "GET /okr/v1/periods",
            scope_group: "okr",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/okr/v1/periods".to_string(),
            query: vec![("page_size".to_string(), "1".to_string())],
            body: None,
        },
        DogfoodProbeSpec {
            module: "attendance",
            name: "attendance.groups.list",
            command: "feishu-bot --json attendance group list --page-size 1",
            operation: "GET /attendance/v1/groups",
            scope_group: "attendance",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/attendance/v1/groups".to_string(),
            query: vec![("page_size".to_string(), "1".to_string())],
            body: None,
        },
        DogfoodProbeSpec {
            module: "attendance",
            name: "attendance.shifts.list",
            command: "feishu-bot --json attendance shift list --page-size 1",
            operation: "GET /attendance/v1/shifts",
            scope_group: "attendance",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/attendance/v1/shifts".to_string(),
            query: vec![("page_size".to_string(), "1".to_string())],
            body: None,
        },
        DogfoodProbeSpec {
            module: "corehr",
            name: "corehr.jobs.list",
            command: "feishu-bot --json corehr job list --page-size 1",
            operation: "GET /corehr/v2/jobs",
            scope_group: "corehr",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/corehr/v2/jobs".to_string(),
            query: vec![("page_size".to_string(), "1".to_string())],
            body: None,
        },
        DogfoodProbeSpec {
            module: "mail",
            name: "mail.me.folders.list",
            command: "feishu-bot --json mail folder list --mailbox me",
            operation: "GET /mail/v1/user_mailboxes/me/folders",
            scope_group: "mail",
            auth: DogfoodProbeAuth::User,
            method: Method::GET,
            path: "/mail/v1/user_mailboxes/me/folders".to_string(),
            query: Vec::new(),
            body: None,
        },
        DogfoodProbeSpec {
            module: "minutes",
            name: "minutes.search",
            command: "feishu-bot --json minutes search --query dogfood --page-size 1",
            operation: "POST /minutes/v1/minutes/search",
            scope_group: "minutes",
            auth: DogfoodProbeAuth::User,
            method: Method::POST,
            path: "/minutes/v1/minutes/search".to_string(),
            query: vec![
                ("page_size".to_string(), "1".to_string()),
                ("user_id_type".to_string(), "open_id".to_string()),
            ],
            body: Some(json!({
                "query": "dogfood",
            })),
        },
        DogfoodProbeSpec {
            module: "vc",
            name: "vc.reports.daily",
            command: "feishu-bot --json vc report daily --start-time <unix> --end-time <unix>",
            operation: "GET /vc/v1/reports/get_daily",
            scope_group: "vc",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/vc/v1/reports/get_daily".to_string(),
            query: vec![
                (
                    "start_time".to_string(),
                    (Local::now().timestamp() - 86_400).to_string(),
                ),
                ("end_time".to_string(), Local::now().timestamp().to_string()),
            ],
            body: None,
        },
        DogfoodProbeSpec {
            module: "helpdesk",
            name: "helpdesk.faq.categories",
            command: "feishu-bot --json helpdesk faq categories",
            operation: "GET /helpdesk/v1/categories",
            scope_group: "helpdesk",
            auth: DogfoodProbeAuth::Helpdesk,
            method: Method::GET,
            path: "/helpdesk/v1/categories".to_string(),
            query: Vec::new(),
            body: None,
        },
        DogfoodProbeSpec {
            module: "hire",
            name: "hire.jobs.list",
            command: "feishu-bot --json hire job list --page-size 1",
            operation: "GET /hire/v1/jobs",
            scope_group: "hire",
            auth: DogfoodProbeAuth::Tenant,
            method: Method::GET,
            path: "/hire/v1/jobs".to_string(),
            query: vec![("page_size".to_string(), "1".to_string())],
            body: None,
        },
    ]
}

async fn run_dogfood_probe(
    api: &mut FeishuClient,
    spec: DogfoodProbeSpec,
    include_response: bool,
) -> Value {
    let result = match spec.auth {
        DogfoodProbeAuth::Tenant => {
            api.request_json(spec.method, &spec.path, &spec.query, spec.body)
                .await
        }
        DogfoodProbeAuth::User => {
            api.request_json_with_auth(
                spec.method,
                &spec.path,
                &spec.query,
                spec.body,
                ApiAuthArg::User,
                &[],
            )
            .await
        }
        DogfoodProbeAuth::Helpdesk => {
            api.request_helpdesk_json(spec.method, &spec.path, &spec.query, spec.body)
                .await
        }
    };
    dogfood_probe_from_result(
        spec.module,
        spec.name,
        spec.command,
        spec.operation,
        spec.scope_group,
        probe_value(result),
        include_response,
        &api.config.app_id,
    )
}
