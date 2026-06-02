use super::*;

pub(super) async fn verify_dogfood(
    api: &mut FeishuClient,
    args: DogfoodVerifyArgs,
) -> Result<Value> {
    let started_at = Local::now().to_rfc3339();
    let mut probes = Vec::new();
    let mut expired_user_token_probes = Vec::new();
    let module_filters = dogfood_verify_module_filters(&args);

    if dogfood_module_selected(&module_filters, "auth", "auth.tenant_token") {
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
        if dogfood_module_selected(&module_filters, spec.module, spec.name) {
            let probe = run_dogfood_probe(api, spec.clone(), args.include_response).await;
            if args.auto_refresh_user_token
                && matches!(spec.auth, DogfoodProbeAuth::User)
                && dogfood_probe_has_status(&probe, "expired_user_token")
            {
                expired_user_token_probes.push((probes.len(), spec));
            }
            probes.push(probe);
        }
    }

    let auto_refresh = if args.auto_refresh_user_token && !expired_user_token_probes.is_empty() {
        refresh_user_token_and_retry(api, &args, &mut probes, expired_user_token_probes).await
    } else {
        Value::Null
    };

    if args.send_loop_check
        && dogfood_module_selected(&module_filters, "message", "message.loop_check")
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
        for probe in run_dogfood_write_probes(api, &args, &module_filters).await {
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
                "profile": args.profile.map(|profile| profile.as_str()).unwrap_or("custom"),
                "requested_modules": args.module,
                "module_filter": module_filters,
                "auto_refresh_user_token": args.auto_refresh_user_token,
                "strict": args.strict,
            },
            "environment": {
                "base_url": api.config.base_url,
                "app_id": mask_app_id(&api.config.app_id),
                "default_user_id": api.config.default_user_id.as_deref().map(mask_secret).unwrap_or_else(|| "missing".to_string()),
                "user_access_token": api.config.user_access_token.as_deref().map(mask_secret).unwrap_or_else(|| "missing".to_string()),
                "helpdesk_id": api.config.helpdesk_id.as_deref().map(mask_secret).unwrap_or_else(|| "missing".to_string()),
                "wiki_space_id": api.config.default_wiki_space_id.as_deref().map(mask_secret).unwrap_or_else(|| "missing".to_string()),
            },
            "auto_refresh": auto_refresh,
            "summary": summary,
            "probes": probes,
        }
    }))
}

async fn refresh_user_token_and_retry(
    api: &mut FeishuClient,
    args: &DogfoodVerifyArgs,
    probes: &mut [Value],
    retry_specs: Vec<(usize, DogfoodProbeSpec)>,
) -> Value {
    let env_file = dogfood_refresh_env_file(args.refresh_env_file.clone());
    let refresh = refresh_oauth_token(
        &api.config,
        OauthRefreshArgs {
            refresh_token: None,
            raw: true,
            print_env: false,
            save_env: true,
            env_file: Some(env_file.clone()),
        },
    )
    .await;

    let refresh = match refresh {
        Ok(value) => value,
        Err(error) => {
            return json!({
                "attempted": true,
                "ok": false,
                "env_file": env_file.display().to_string(),
                "error_excerpt": dogfood_truncate(&error.to_string(), 700),
                "retried": [],
            });
        }
    };

    let data = refresh.get("data").unwrap_or(&refresh);
    let Some(access_token) = get_string(data, &["access_token"]) else {
        return json!({
            "attempted": true,
            "ok": false,
            "env_file": env_file.display().to_string(),
            "error_excerpt": "OAuth refresh response did not include access_token",
            "retried": [],
        });
    };
    let refresh_token = get_string(data, &["refresh_token"]);
    api.config.user_access_token = Some(access_token.clone());

    let mut retried = Vec::new();
    for (index, spec) in retry_specs {
        let previous = probes.get(index).cloned().unwrap_or(Value::Null);
        let mut retry = run_dogfood_probe(api, spec, args.include_response).await;
        if let Some(object) = retry.as_object_mut() {
            object.insert("auto_refresh_retry".to_string(), Value::Bool(true));
            object.insert(
                "previous_status".to_string(),
                previous
                    .get("status")
                    .cloned()
                    .unwrap_or_else(|| Value::String("unknown".to_string())),
            );
            if let Some(error) = previous.get("error_excerpt") {
                object.insert("previous_error_excerpt".to_string(), error.clone());
            }
        }
        retried.push(json!({
            "module": retry.get("module").cloned().unwrap_or(Value::Null),
            "name": retry.get("name").cloned().unwrap_or(Value::Null),
            "status": retry.get("status").cloned().unwrap_or(Value::Null),
        }));
        if let Some(slot) = probes.get_mut(index) {
            *slot = retry;
        }
    }

    json!({
        "attempted": true,
        "ok": true,
        "env_file": env_file.display().to_string(),
        "saved_env_file": data.get("saved_env_file").cloned().unwrap_or_else(|| Value::String(env_file.display().to_string())),
        "access_token": mask_secret(&access_token),
        "refresh_token": refresh_token.as_deref().map(mask_secret),
        "retried": retried,
    })
}

fn dogfood_probe_has_status(probe: &Value, status: &str) -> bool {
    probe.get("status").and_then(Value::as_str) == Some(status)
}

fn dogfood_refresh_env_file(explicit: Option<PathBuf>) -> PathBuf {
    explicit
        .or_else(|| {
            std::env::var("FEISHU_ENV_FILE")
                .or_else(|_| std::env::var("LARK_ENV_FILE"))
                .ok()
                .map(PathBuf::from)
        })
        .unwrap_or_else(|| PathBuf::from("private/local.env"))
}

fn dogfood_truncate(value: &str, max_chars: usize) -> String {
    let mut output = String::new();
    for ch in value.chars().take(max_chars) {
        output.push(ch);
    }
    if value.chars().count() > max_chars {
        output.push_str("...");
    }
    output
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
