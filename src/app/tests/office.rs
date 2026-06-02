use super::super::*;

#[test]
fn parses_office_workflow_commands_for_ai() {
    let list = Cli::parse_from(["feishu", "office", "list", "--details"]);
    match list.command {
        Commands::Office(OfficeCommand::List(args)) => {
            assert!(args.details);
        }
        _ => panic!("expected office list"),
    }

    let bootstrap = Cli::parse_from([
        "feishu",
        "office",
        "bootstrap",
        "--project",
        "AI Project",
        "--user",
        "ou_1",
        "--space-id",
        "spc_1",
        "--send-summary",
        "--dry-run",
    ]);
    match bootstrap.command {
        Commands::Office(OfficeCommand::Bootstrap(args)) => {
            assert_eq!(args.project, "AI Project");
            assert_eq!(args.users, vec!["ou_1"]);
            assert_eq!(args.space_id.as_deref(), Some("spc_1"));
            assert!(args.send_summary);
            assert!(args.dry_run);
            assert!(!args.skip_wiki);
        }
        _ => panic!("expected office bootstrap"),
    }

    let report = Cli::parse_from([
        "feishu",
        "--json",
        "office",
        "report",
        "--project",
        "AI Project",
        "--title",
        "HTML Demo",
        "--content-type",
        "html",
        "--file",
        "demo.html",
        "--base-record",
        "--pin",
        "--dry-run",
    ]);
    match report.command {
        Commands::Office(OfficeCommand::Report(args)) => {
            assert_eq!(args.project, "AI Project");
            assert_eq!(args.title, "HTML Demo");
            assert!(matches!(args.content_type, ContentTypeArg::Html));
            assert_eq!(args.file.unwrap(), PathBuf::from("demo.html"));
            assert!(args.base_record);
            assert!(args.pin);
            assert!(args.dry_run);
        }
        _ => panic!("expected office report"),
    }

    let progress = Cli::parse_from([
        "feishu",
        "office",
        "progress",
        "--project",
        "AI Project",
        "--title",
        "Progress",
        "--status",
        "doing",
        "--summary",
        "Current status",
        "--wiki-report",
        "--pin",
    ]);
    match progress.command {
        Commands::Office(OfficeCommand::Progress(args)) => {
            assert_eq!(args.project, "AI Project");
            assert_eq!(args.title, "Progress");
            assert_eq!(args.status, "doing");
            assert_eq!(args.summary.as_deref(), Some("Current status"));
            assert!(args.wiki_report);
            assert!(args.pin);
            assert!(!args.no_base_record);
        }
        _ => panic!("expected office progress"),
    }

    let inbox = Cli::parse_from([
        "feishu",
        "office",
        "inbox",
        "--project",
        "AI Project",
        "--from-now",
        "--reply-text",
        "Received",
    ]);
    match inbox.command {
        Commands::Office(OfficeCommand::Inbox(args)) => {
            assert_eq!(args.project, "AI Project");
            assert!(args.from_now);
            assert_eq!(args.ack_emoji, "OK");
            assert_eq!(args.reply_text.as_deref(), Some("Received"));
            assert!(!args.no_mark_seen);
        }
        _ => panic!("expected office inbox"),
    }

    let cleanup = Cli::parse_from([
        "feishu",
        "office",
        "cleanup",
        "--project",
        "AI Project",
        "--dry-run",
    ]);
    match cleanup.command {
        Commands::Office(OfficeCommand::Cleanup(args)) => {
            assert_eq!(args.project, "AI Project");
            assert!(args.dry_run);
            assert!(!args.confirm);
        }
        _ => panic!("expected office cleanup"),
    }
}

#[test]
fn parses_setup_automation_commands_for_ai() {
    let plan = Cli::parse_from(["feishu", "setup", "plan", "--group", "office"]);
    match plan.command {
        Commands::Setup(SetupCommand::Plan(args)) => {
            assert_eq!(args.groups, vec!["office"]);
            assert!(matches!(args.token_type, ApiAuthArg::Tenant));
        }
        _ => panic!("expected setup plan"),
    }

    let open = Cli::parse_from([
        "feishu",
        "setup",
        "open-scopes",
        "--group",
        "wiki",
        "--browser",
    ]);
    match open.command {
        Commands::Setup(SetupCommand::OpenScopes(args)) => {
            assert_eq!(args.groups, vec!["wiki"]);
            assert!(args.browser);
            assert!(!args.system_browser);
        }
        _ => panic!("expected setup open-scopes"),
    }

    let wiki_bot = Cli::parse_from([
        "feishu",
        "setup",
        "wiki-bot",
        "--space-id",
        "spc_1",
        "--auth",
        "user",
    ]);
    match wiki_bot.command {
        Commands::Setup(SetupCommand::WikiBot(args)) => {
            assert_eq!(args.space_id.as_deref(), Some("spc_1"));
            assert!(matches!(args.auth, ApiAuthArg::User));
        }
        _ => panic!("expected setup wiki-bot"),
    }

    let auto = Cli::parse_from(["feishu", "setup", "auto", "--open-browser"]);
    match auto.command {
        Commands::Setup(SetupCommand::Auto(args)) => {
            assert!(args.open_browser);
            assert!(!args.no_wiki_bot);
        }
        _ => panic!("expected setup auto"),
    }

    let quickstart = Cli::parse_from([
        "feishu",
        "setup",
        "quickstart",
        "--open-browser",
        "--system-browser",
        "--project",
        "AI Project",
    ]);
    match quickstart.command {
        Commands::Setup(SetupCommand::Quickstart(args)) => {
            assert!(args.open_browser);
            assert!(args.system_browser);
            assert_eq!(args.project, "AI Project");
            assert!(!args.no_wiki_bot);
        }
        _ => panic!("expected setup quickstart"),
    }
}

#[test]
fn setup_plan_uses_private_oauth_env_file() {
    let values = std::collections::HashMap::new();
    let oauth = setup_oauth_plan(&values);
    assert_eq!(oauth["env_file"], "private/local.env");
    assert!(oauth["token_command"]
        .as_str()
        .unwrap()
        .contains("--env-file 'private/local.env'"));
    let quickstart = setup_quickstart_plan(&values, "AI Project", &[]);
    assert!(quickstart["commands"]["save_oauth_code"]
        .as_str()
        .unwrap()
        .contains("--env-file 'private/local.env'"));

    let mut values = std::collections::HashMap::new();
    values.insert(
        "FEISHU_ENV_FILE".to_string(),
        "private/custom.env".to_string(),
    );
    let oauth = setup_oauth_plan(&values);
    assert_eq!(oauth["env_file"], "private/custom.env");
    assert!(oauth["refresh_command"]
        .as_str()
        .unwrap()
        .contains("--env-file 'private/custom.env'"));
}

#[test]
fn serializes_office_project_registry() {
    assert_eq!(office_project_key("  AI Project  ").unwrap(), "AI Project");
    assert!(office_project_key("   ").is_err());

    let mut registry = OfficeProjectRegistry::default();
    registry.projects.insert(
        "AI Project".to_string(),
        OfficeProject {
            project: "AI Project".to_string(),
            name: "AI Project".to_string(),
            chat_id: Some("oc_1".to_string()),
            wiki_space_id: Some("spc_1".to_string()),
            wiki_index_node_token: Some("wik_1".to_string()),
            wiki_index_obj_token: Some("docx_1".to_string()),
            base_app_token: Some("base_1".to_string()),
            base_table_id: Some("tbl_1".to_string()),
            ..OfficeProject::default()
        },
    );
    let text = serde_json::to_string(&registry).unwrap();
    assert!(text.contains("AI Project"));
    assert!(text.contains("base_1"));
    let parsed: OfficeProjectRegistry = serde_json::from_str(&text).unwrap();
    assert_eq!(
        parsed.projects["AI Project"]
            .wiki_index_obj_token
            .as_deref(),
        Some("docx_1")
    );
}

#[test]
fn filters_manifest_by_module_identity() {
    let base = json!({
        "name": "base",
        "command": "feishu-bot base",
        "scope_group": "base",
        "examples": ["feishu-bot base create --name \"AI Tasks\""]
    });
    let task = json!({
        "name": "task",
        "command": "feishu-bot task",
        "scope_group": "task"
    });
    assert!(!manifest_module_matches(&base, "task"));
    assert!(manifest_module_matches(&task, "task"));
    assert!(manifest_module_matches(&base, "feishu-bot base"));

    let setup = json!({
        "name": "setup",
        "command": "feishu-bot setup",
        "scope_group": "im,doc,wiki,base,search,user-token"
    });
    let office = json!({
        "name": "office",
        "command": "feishu-bot office",
        "scope_group": "im,wiki,doc,base,search"
    });
    let mut modules = vec![setup, office, base];
    retain_manifest_modules(&mut modules, "base");
    assert_eq!(modules.len(), 1);
    assert_eq!(modules[0]["name"], "base");

    let drive = json!({
        "name": "drive",
        "command": "feishu-bot drive",
        "aliases": ["permission", "drive permission"],
        "scope_group": "drive"
    });
    let mut modules = vec![task, drive];
    retain_manifest_modules(&mut modules, "permission");
    assert_eq!(modules.len(), 1);
    assert_eq!(modules[0]["name"], "drive");
}

#[test]
fn resolves_office_scope_profile() {
    let all = scope_groups("all").unwrap();
    let all_names = all.iter().map(|(name, _)| *name).collect::<Vec<_>>();
    let unique_names = all_names
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(all.len(), 23);
    assert_eq!(
        all_names,
        vec![
            "user-token",
            "im",
            "contact",
            "directory",
            "doc",
            "board",
            "base",
            "task",
            "drive",
            "permission",
            "calendar",
            "vc",
            "minutes",
            "search",
            "okr",
            "attendance",
            "mail",
            "corehr",
            "helpdesk",
            "hire",
            "wiki",
            "sheet",
            "approval",
        ]
    );
    assert_eq!(unique_names.len(), all_names.len());
    assert!(all_names.contains(&"permission"));

    let groups = scope_groups("office").unwrap();
    let names = groups.iter().map(|(name, _)| *name).collect::<Vec<_>>();
    assert_eq!(
        names,
        vec!["im", "doc", "wiki", "base", "permission", "search"]
    );
    let scopes = groups
        .iter()
        .flat_map(|(_, scopes)| scopes.iter().copied())
        .collect::<Vec<_>>();
    assert!(scopes.contains(&"im:message"));
    assert!(scopes.contains(&"docx:document:create"));
    assert!(scopes.contains(&"docs:permission.member:create"));
    assert!(scopes.contains(&"search:docs:read"));
    assert!(scope_groups("missing").is_err());
}

#[test]
fn manifest_exposes_office_workflow_layer() {
    let manifest = build_manifest().unwrap();
    let workflow_modules = manifest
        .pointer("/layers/workflow_modules")
        .and_then(Value::as_array)
        .unwrap();
    assert!(workflow_modules.iter().any(|item| item == "office"));
    let setup_modules = manifest
        .pointer("/layers/setup_modules")
        .and_then(Value::as_array)
        .unwrap();
    assert!(setup_modules.iter().any(|item| item == "bot"));
    let atomic_modules = manifest
        .pointer("/layers/atomic_modules")
        .and_then(Value::as_array)
        .unwrap();
    assert!(atomic_modules.iter().any(|item| item == "notify"));
    let workflow_layer = manifest
        .get("workflow_layer")
        .and_then(Value::as_object)
        .unwrap();
    assert_eq!(workflow_layer["default_command"], "feishu-bot office");
    assert_eq!(
        workflow_layer["verification_command"],
        "feishu-bot dogfood verify"
    );
    assert!(workflow_layer["preferred_commands"]
        .as_array()
        .unwrap()
        .iter()
        .any(|command| command.as_str().unwrap().contains("office inbox")));
    assert!(workflow_layer["preferred_commands"]
        .as_array()
        .unwrap()
        .iter()
        .any(|command| command
            .as_str()
            .unwrap()
            .contains("--auto-refresh-user-token --strict")));
    assert!(workflow_layer["local_safe_commands"]
        .as_array()
        .unwrap()
        .iter()
        .any(|command| command.as_str().unwrap().contains("--dry-run")));
    let modules = manifest.get("modules").and_then(Value::as_array).unwrap();
    let office = modules
        .iter()
        .find(|module| module.get("name").and_then(Value::as_str) == Some("office"))
        .unwrap();
    assert_eq!(office["layer"], "workflow");
    assert!(office["examples"]
        .as_array()
        .unwrap()
        .iter()
        .any(|example| example.as_str().unwrap().contains("office report")));
    assert!(office["examples"]
        .as_array()
        .unwrap()
        .iter()
        .any(|example| example.as_str().unwrap().contains("office progress")));

    let setup = modules
        .iter()
        .find(|module| module.get("name").and_then(Value::as_str) == Some("setup"))
        .unwrap();
    assert_eq!(setup["layer"], "setup");
    assert!(setup["examples"]
        .as_array()
        .unwrap()
        .iter()
        .any(|example| example.as_str().unwrap().contains("setup quickstart")));
    assert!(setup["examples"]
        .as_array()
        .unwrap()
        .iter()
        .any(|example| example.as_str().unwrap().contains("setup auto")));
}
