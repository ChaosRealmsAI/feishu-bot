use super::*;

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
