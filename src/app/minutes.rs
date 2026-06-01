use std::fs;
use std::io::{self, Write};
use std::path::Path;

use super::*;

pub(super) async fn run_minutes_command(
    api: &mut FeishuClient,
    command: MinutesCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        MinutesCommand::Search(args) => {
            if args.page_size > 30 {
                bail!("minutes search page_size cannot exceed 30");
            }
            let page_size = args.page_size;
            let user_id_type = args.user_id_type.resolve(None).to_string();
            let page_token = args.page_token.clone();
            let body = build_minutes_search_body(args)?;
            let mut query = vec![
                ("page_size".to_string(), page_size.to_string()),
                ("user_id_type".to_string(), user_id_type),
            ];
            push_query_opt(&mut query, "page_token", page_token);
            api.post_json_user("/minutes/v1/minutes/search", &query, body)
                .await?
        }
        MinutesCommand::Get(args) => {
            let minute_token = extract_minute_token(&args.minute_token)?;
            let path = format!("/minutes/v1/minutes/{minute_token}");
            api.get_json(
                &path,
                &[(
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                )],
            )
            .await?
        }
        MinutesCommand::Artifacts(args) => {
            let minute_token = extract_minute_token(&args.minute_token)?;
            let path = format!("/minutes/v1/minutes/{minute_token}/artifacts");
            api.get_json(&path, &[]).await?
        }
        MinutesCommand::Media(args) => {
            let minute_token = extract_minute_token(&args.minute_token)?;
            let path = format!("/minutes/v1/minutes/{minute_token}/media");
            api.get_json(&path, &[]).await?
        }
        MinutesCommand::Transcript(args) => {
            let minute_token = extract_minute_token(&args.minute_token)?;
            let mut query = Vec::new();
            if args.need_speaker {
                query.push(("need_speaker".to_string(), "true".to_string()));
            }
            if args.need_timestamp {
                query.push(("need_timestamp".to_string(), "true".to_string()));
            }
            push_query_opt(&mut query, "file_format", args.file_format);
            let bytes = api
                .download_minutes_transcript(&minute_token, &query)
                .await?;
            if args.output.to_string_lossy() == "-" {
                io::stdout()
                    .write_all(&bytes)
                    .context("write transcript to stdout")?;
                return Ok(());
            }
            write_output_bytes(&args.output, &bytes)?;
            json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "minute_token": minute_token,
                    "output": args.output.display().to_string(),
                    "bytes": bytes.len()
                }
            })
        }
    };
    print_response(raw_json, "minutes operation completed", data)
}

fn write_output_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent().filter(|path| !path.as_os_str().is_empty()) {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

pub(super) fn extract_minute_token(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        bail!("minute token cannot be empty");
    }
    let without_fragment = trimmed.split('#').next().unwrap_or(trimmed);
    let without_query = without_fragment
        .split('?')
        .next()
        .unwrap_or(without_fragment)
        .trim_end_matches('/');
    let candidate = without_query
        .rsplit('/')
        .next()
        .unwrap_or(without_query)
        .trim_matches('=');
    if candidate.is_empty() {
        bail!("could not extract minute token from {input}");
    }
    Ok(candidate.to_string())
}

pub(super) fn build_minutes_search_body(args: MinutesSearchArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "minutes search body",
        );
    }

    let mut body = Map::new();
    insert_opt_string(&mut body, "query", args.query);
    if let Some(filter_json) = args.filter_json {
        let filter = ensure_json_object(
            parse_json_value(&filter_json, "minutes filter JSON")?,
            "minutes filter",
        )?;
        body.insert("filter".to_string(), filter);
    }
    if let Some(sorter) = args.sorter.filter(|value| !value.trim().is_empty()) {
        body.insert("sorter".to_string(), Value::String(sorter));
    }
    Ok(Value::Object(body))
}
