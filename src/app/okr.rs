use super::*;

pub(super) async fn run_okr_command(
    api: &mut FeishuClient,
    command: OkrCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        OkrCommand::Period(OkrPeriodCommand::List(args)) => {
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/okr/v1/periods", &query).await?
        }
        OkrCommand::PeriodRule(OkrPeriodRuleCommand::List) => {
            api.get_json("/okr/v1/period_rules", &[]).await?
        }
        OkrCommand::UserOkrs(args) => {
            validate_okr_id_list("period-id", &args.period_ids, 10, false)?;
            if args.limit > 10 {
                bail!("okr user-okrs limit cannot exceed 10");
            }
            let mut query = build_okr_query(args.user_id_type, args.lang);
            query.push(("offset".to_string(), args.offset.to_string()));
            query.push(("limit".to_string(), args.limit.to_string()));
            push_query_repeated(&mut query, "period_ids", args.period_ids);
            let path = format!("/okr/v1/users/{}/okrs", args.user_id);
            api.get_json(&path, &query).await?
        }
        OkrCommand::BatchGet(args) => {
            validate_okr_id_list("okr-id", &args.okr_ids, 10, true)?;
            let mut query = build_okr_query(args.user_id_type, args.lang);
            push_query_repeated(&mut query, "okr_ids", args.okr_ids);
            api.get_json("/okr/v1/okrs/batch_get", &query).await?
        }
    };
    print_response(raw_json, "okr operation completed", data)
}

pub(super) fn build_okr_query(
    user_id_type: OkrUserIdTypeArg,
    lang: String,
) -> Vec<(String, String)> {
    vec![
        (
            "user_id_type".to_string(),
            user_id_type.as_api_value().to_string(),
        ),
        ("lang".to_string(), lang),
    ]
}

pub(super) fn validate_okr_id_list(
    label: &str,
    ids: &[String],
    max: usize,
    required: bool,
) -> Result<()> {
    let count = ids.iter().filter(|id| !id.trim().is_empty()).count();
    if required && count == 0 {
        bail!("at least one {label} is required");
    }
    if count > max {
        bail!("{label} cannot repeat more than {max} times");
    }
    Ok(())
}
