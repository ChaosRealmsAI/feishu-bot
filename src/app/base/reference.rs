use super::*;

pub(in crate::app) fn print_base_url_parse(args: BaseParseUrlArgs, raw_json: bool) -> Result<()> {
    let data = parse_base_reference(&args.url)?;
    if raw_json {
        println!("{}", serde_json::to_string_pretty(&data)?);
    } else {
        println!("base URL parsed");
        print_json_field_line(&data, "input_kind");
        print_json_field_line(&data, "host");
        print_json_field_line(&data, "app_token");
        print_json_field_line(&data, "table_id");
        print_json_field_line(&data, "view_id");
        print_json_field_line(&data, "record_id");
        print_json_field_line(&data, "wiki_node_token");
        print_json_field_line(&data, "resolution_hint");
    }
    Ok(())
}

fn print_json_field_line(data: &Value, key: &str) {
    if let Some(value) = data.get(key).and_then(Value::as_str) {
        println!("{key}={value}");
    } else if let Some(value) = data.get(key).and_then(Value::as_bool) {
        println!("{key}={value}");
    }
}

pub(in crate::app) fn parse_base_reference(input: &str) -> Result<Value> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        bail!("base parse-url requires a non-empty URL or app_token");
    }
    if !trimmed.contains("://") && !trimmed.contains('/') && !trimmed.contains('?') {
        return Ok(json!({
            "input_kind": "app_token",
            "app_token": trimmed,
            "resolution_hint": "Use this app_token directly with feishu-bot base commands."
        }));
    }

    let url_input = if trimmed.contains("://") {
        trimmed.to_string()
    } else {
        format!("https://placeholder.local/{trimmed}")
    };
    let url =
        reqwest::Url::parse(&url_input).with_context(|| format!("parse Base URL: {trimmed}"))?;
    let segments: Vec<String> = url
        .path_segments()
        .map(|segments| segments.map(str::to_string).collect())
        .unwrap_or_default();

    let app_token = segment_after(&segments, "base").or_else(|| segment_after(&segments, "app"));
    let wiki_node_token = segment_after(&segments, "wiki");
    let mut query = url
        .query_pairs()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect::<Vec<_>>();
    if let Some(fragment) = url.fragment() {
        query.extend(parse_fragment_pairs(fragment));
    }

    let table_id = query_value(&query, &["table", "table_id"]);
    let view_id = query_value(&query, &["view", "view_id"]);
    let record_id = query_value(&query, &["record", "record_id"]);
    let field_id = query_value(&query, &["field", "field_id"]);
    let form_id = query_value(&query, &["form", "form_id"]);
    let page_id = query_value(&query, &["page", "page_id", "pageId"]);
    let dashboard_id = query_value(&query, &["dashboard", "dashboard_id", "block_id"]);

    let mut output = Map::new();
    output.insert("input".to_string(), Value::String(trimmed.to_string()));
    output.insert("input_kind".to_string(), Value::String("url".to_string()));
    if let Some(host) = url.host_str() {
        output.insert("host".to_string(), Value::String(host.to_string()));
    }
    insert_opt_string(&mut output, "app_token", app_token);
    insert_opt_string(&mut output, "table_id", table_id);
    insert_opt_string(&mut output, "view_id", view_id);
    insert_opt_string(&mut output, "record_id", record_id);
    insert_opt_string(&mut output, "field_id", field_id);
    insert_opt_string(&mut output, "form_id", form_id);
    insert_opt_string(&mut output, "page_id", page_id);
    insert_opt_string(&mut output, "dashboard_id", dashboard_id);
    insert_opt_string(&mut output, "wiki_node_token", wiki_node_token.clone());
    output.insert(
        "is_wiki_url".to_string(),
        Value::Bool(wiki_node_token.is_some()),
    );

    let resolution_hint = if output.get("app_token").is_some() {
        "Use app_token/table_id/view_id directly with feishu-bot base commands."
    } else if wiki_node_token.is_some() {
        "This is a Wiki URL. Run `feishu-bot wiki node --token <wiki_node_token>` to resolve obj_token; if obj_type is bitable, obj_token is the Base app_token."
    } else {
        "No Base app_token found. Open a /base/<app_token> URL or pass a raw app_token."
    };
    output.insert(
        "resolution_hint".to_string(),
        Value::String(resolution_hint.to_string()),
    );
    Ok(Value::Object(output))
}

fn segment_after(segments: &[String], marker: &str) -> Option<String> {
    segments
        .windows(2)
        .find(|pair| pair[0] == marker)
        .and_then(|pair| non_empty_string(pair[1].clone()))
}

fn non_empty_string(value: String) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}

fn query_value(query: &[(String, String)], keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        query
            .iter()
            .find(|(query_key, value)| query_key == key && !value.trim().is_empty())
            .map(|(_, value)| value.clone())
    })
}

fn parse_fragment_pairs(fragment: &str) -> Vec<(String, String)> {
    let query = fragment
        .split_once('?')
        .map(|(_, query)| query)
        .unwrap_or(fragment);
    query
        .split('&')
        .filter_map(|part| part.split_once('='))
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}
