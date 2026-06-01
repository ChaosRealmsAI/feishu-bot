use super::*;

pub(super) async fn run_raw_api_command(
    api: &mut FeishuClient,
    command: ApiCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        ApiCommand::Get(args) => {
            let query = parse_query_pairs(args.query)?;
            let headers = parse_header_pairs(args.headers)?;
            api.request_json_with_auth(Method::GET, &args.path, &query, None, args.auth, &headers)
                .await?
        }
        ApiCommand::Post(args) => {
            let query = parse_query_pairs(args.query)?;
            let headers = parse_header_pairs(args.headers)?;
            let body = read_json_value(args.body_json, args.file, args.stdin)?;
            api.request_json_with_auth(
                Method::POST,
                &args.path,
                &query,
                Some(body),
                args.auth,
                &headers,
            )
            .await?
        }
        ApiCommand::Put(args) => {
            let query = parse_query_pairs(args.query)?;
            let headers = parse_header_pairs(args.headers)?;
            let body = read_json_value(args.body_json, args.file, args.stdin)?;
            api.request_json_with_auth(
                Method::PUT,
                &args.path,
                &query,
                Some(body),
                args.auth,
                &headers,
            )
            .await?
        }
        ApiCommand::Patch(args) => {
            let query = parse_query_pairs(args.query)?;
            let headers = parse_header_pairs(args.headers)?;
            let body = read_json_value(args.body_json, args.file, args.stdin)?;
            api.request_json_with_auth(
                Method::PATCH,
                &args.path,
                &query,
                Some(body),
                args.auth,
                &headers,
            )
            .await?
        }
        ApiCommand::Delete(args) => {
            let query = parse_query_pairs(args.query)?;
            let headers = parse_header_pairs(args.headers)?;
            let body = read_optional_json_value(args.body_json, args.file, args.stdin)?;
            api.request_json_with_auth(
                Method::DELETE,
                &args.path,
                &query,
                body,
                args.auth,
                &headers,
            )
            .await?
        }
        ApiCommand::Download(args) => {
            let query = parse_query_pairs(args.query)?;
            let headers = parse_header_pairs(args.headers)?;
            let bytes = api
                .request_binary_with_auth(
                    Method::GET,
                    &args.path,
                    &query,
                    args.auth,
                    &headers,
                    args.range.as_deref(),
                )
                .await?;
            write_output_file(&args.output, &bytes)?;
            json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "output": args.output.display().to_string(),
                    "bytes": bytes.len()
                }
            })
        }
        ApiCommand::Multipart(args) => {
            let query = parse_query_pairs(args.query)?;
            let headers = parse_header_pairs(args.headers)?;
            let fields = parse_key_value_pairs(args.fields, "field")?;
            let files = parse_file_part_pairs(args.files)?;
            api.request_multipart_with_auth(
                args.method.as_method(),
                &args.path,
                &query,
                fields,
                files,
                args.auth,
                &headers,
            )
            .await?
        }
    };
    print_response(raw_json, "api request completed", data)
}
