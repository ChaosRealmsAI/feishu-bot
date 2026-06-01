use super::*;
pub(super) fn get_string(value: &Value, path: &[&str]) -> Option<String> {
    let mut cursor = value;
    for segment in path {
        cursor = cursor.get(*segment)?;
    }
    cursor.as_str().map(ToString::to_string)
}

pub(super) fn get_i64(value: &Value, path: &[&str]) -> Option<i64> {
    let mut cursor = value;
    for segment in path {
        cursor = cursor.get(*segment)?;
    }
    cursor.as_i64()
}

pub(super) fn print_response(raw_json: bool, label: &str, value: Value) -> Result<()> {
    if raw_json {
        println!("{}", serde_json::to_string_pretty(&value)?);
    } else {
        println!("{label}");
        for (key, path) in [
            ("message_id", &["data", "message_id"][..]),
            ("reaction_id", &["data", "reaction_id"][..]),
            ("reaction_id", &["data", "reaction", "reaction_id"][..]),
            ("chat_id", &["data", "chat_id"][..]),
            ("document_id", &["data", "document", "document_id"][..]),
            ("app_token", &["data", "app", "app_token"][..]),
            ("base_url", &["data", "app", "url"][..]),
            ("default_table_id", &["data", "app", "default_table_id"][..]),
            ("table_id", &["data", "table", "table_id"][..]),
            ("default_view_id", &["data", "table", "default_view_id"][..]),
            ("view_id", &["data", "view", "view_id"][..]),
            ("field_id", &["data", "field", "field_id"][..]),
            ("record_id", &["data", "record", "record_id"][..]),
            ("record_id", &["data", "record", "id"][..]),
            ("task_guid", &["data", "task", "guid"][..]),
            ("tasklist_guid", &["data", "tasklist", "guid"][..]),
            ("comment_id", &["data", "comment", "id"][..]),
            ("comment_id", &["data", "comment_id"][..]),
            ("reply_id", &["data", "reply_id"][..]),
            ("version_id", &["data", "version"][..]),
            ("subscription_id", &["data", "subscription_id"][..]),
            ("subtask_guid", &["data", "subtask", "guid"][..]),
            ("image_key", &["data", "image_key"][..]),
            ("file_key", &["data", "file_key"][..]),
            ("file_token", &["data", "file_token"][..]),
            ("source_file_token", &["data", "source_file_token"][..]),
            ("ticket", &["data", "ticket"][..]),
            ("import_token", &["data", "result", "token"][..]),
            ("import_url", &["data", "result", "url"][..]),
            ("export_file_token", &["data", "result", "file_token"][..]),
            ("drive_token", &["data", "token"][..]),
            ("drive_url", &["data", "url"][..]),
            ("output", &["data", "output"][..]),
            ("output", &["output"][..]),
            ("member_id", &["data", "member", "member_id"][..]),
            ("member_type", &["data", "member", "member_type"][..]),
            ("perm", &["data", "member", "perm"][..]),
            ("calendar_id", &["data", "calendar", "calendar_id"][..]),
            ("event_id", &["data", "event", "event_id"][..]),
            ("minute_token", &["data", "minute", "token"][..]),
            ("minute_token", &["data", "minute_token"][..]),
            ("minute_token", &["data", "token"][..]),
            ("download_url", &["data", "download_url"][..]),
            ("data_source_id", &["data", "data_source", "id"][..]),
            ("schema_id", &["data", "schema", "schema_id"][..]),
            ("space_id", &["data", "space", "space_id"][..]),
            ("node_token", &["data", "node", "node_token"][..]),
            ("obj_token", &["data", "node", "obj_token"][..]),
            (
                "spreadsheet_token",
                &["data", "spreadsheet", "spreadsheet_token"][..],
            ),
            ("spreadsheet_url", &["data", "spreadsheet", "url"][..]),
            ("approval_code", &["data", "approval_code"][..]),
            ("instance_code", &["data", "instance_code"][..]),
            ("node_id", &["data", "node_id"][..]),
            ("helpdesk_chat_id", &["data", "chat_id"][..]),
            ("ticket_id", &["data", "ticket_id"][..]),
            ("message_id", &["data", "message_id"][..]),
            ("open_id", &["data", "open_id"][..]),
            (
                "wiki_member_add_example",
                &["data", "wiki_member_add_example"][..],
            ),
            ("faq_id", &["data", "faq", "faq_id"][..]),
            ("job_id", &["data", "job", "id"][..]),
            ("job_id", &["data", "job_detail", "basic_info", "id"][..]),
            ("talent_id", &["data", "talent", "id"][..]),
            ("talent_id", &["data", "id"][..]),
            ("application_id", &["data", "application", "id"][..]),
            (
                "application_id",
                &["data", "application_detail", "basic_info", "id"][..],
            ),
            ("attachment_id", &["data", "attachment", "id"][..]),
            ("attachment_url", &["data", "attachment", "url"][..]),
        ] {
            if let Some(value) = get_string(&value, path) {
                println!("{key}={value}");
            }
        }
        if let Some(ids) = value.pointer("/data/ids").and_then(Value::as_array) {
            println!("ids={}", ids.len());
        }
        if let Some(items) = value.pointer("/data/items").and_then(Value::as_array) {
            println!("items={}", items.len());
        }
        if let Some(employees) = value.pointer("/data/employees").and_then(Value::as_array) {
            println!("employees={}", employees.len());
        }
        if let Some(tickets) = value.pointer("/data/tickets").and_then(Value::as_array) {
            println!("tickets={}", tickets.len());
        }
        if let Some(ticket) = value.pointer("/data/ticket").and_then(Value::as_object) {
            if let Some(ticket_id) = ticket.get("ticket_id").and_then(Value::as_str) {
                println!("ticket_id={ticket_id}");
            }
        }
        if let Some(messages) = value.pointer("/data/messages").and_then(Value::as_array) {
            println!("messages={}", messages.len());
        }
        if let Some(categories) = value.pointer("/data/categories").and_then(Value::as_array) {
            println!("categories={}", categories.len());
        }
        if let Some(faqs) = value.pointer("/data/items").and_then(Value::as_array) {
            println!("items_or_faqs={}", faqs.len());
        }
        if let Some(records) = value.pointer("/data/records").and_then(Value::as_array) {
            println!("records={}", records.len());
        }
        if let Some(files) = value.pointer("/data/files").and_then(Value::as_array) {
            println!("files={}", files.len());
        }
        if let Some(bytes) = value.pointer("/data/bytes").and_then(Value::as_u64) {
            println!("bytes={bytes}");
        }
        if let Some(bytes) = value.pointer("/bytes").and_then(Value::as_u64) {
            println!("bytes={bytes}");
        }
        if let Some(calendars) = value
            .pointer("/data/calendar_list")
            .and_then(Value::as_array)
        {
            println!("calendars={}", calendars.len());
        }
        if let Some(events) = value.pointer("/data/items").and_then(Value::as_array) {
            println!("events_or_items={}", events.len());
        }
        if let Some(sheets) = value.pointer("/data/sheets").and_then(Value::as_array) {
            println!("sheets={}", sheets.len());
        }
        if let Some(okrs) = value.pointer("/data/okr_list").and_then(Value::as_array) {
            println!("okrs={}", okrs.len());
        }
        if let Some(okrs) = value.pointer("/data/okrs").and_then(Value::as_array) {
            println!("okrs={}", okrs.len());
        }
        if let Some(periods) = value.pointer("/data/periods").and_then(Value::as_array) {
            println!("periods={}", periods.len());
        }
        if let Some(period_rules) = value
            .pointer("/data/period_rules")
            .and_then(Value::as_array)
        {
            println!("period_rules={}", period_rules.len());
        }
        if let Some(groups) = value.pointer("/data/group_list").and_then(Value::as_array) {
            println!("attendance_groups={}", groups.len());
        }
        if let Some(shifts) = value.pointer("/data/shift_list").and_then(Value::as_array) {
            println!("attendance_shifts={}", shifts.len());
        }
        if let Some(schedules) = value
            .pointer("/data/user_daily_shifts")
            .and_then(Value::as_array)
        {
            println!("attendance_schedules={}", schedules.len());
        }
        if let Some(tasks) = value
            .pointer("/data/user_task_results")
            .and_then(Value::as_array)
        {
            println!("attendance_tasks={}", tasks.len());
        }
        if let Some(flows) = value
            .pointer("/data/user_flow_results")
            .and_then(Value::as_array)
        {
            println!("attendance_flows={}", flows.len());
        }
        if let Some(flows) = value
            .pointer("/data/flow_records")
            .and_then(Value::as_array)
        {
            println!("attendance_flows={}", flows.len());
        }
        if let Some(stats) = value.pointer("/data/user_datas").and_then(Value::as_array) {
            println!("attendance_stats={}", stats.len());
        }
        if let Some(failed) = value
            .pointer("/data/fail_record_ids")
            .and_then(Value::as_array)
        {
            println!("failed_record_ids={}", failed.len());
        }
        if let Some(addresses) = value
            .pointer("/data/sendable_addresses")
            .and_then(Value::as_array)
        {
            println!("sendable_addresses={}", addresses.len());
        }
        if let Some(mailboxes) = value
            .pointer("/data/accessible_mailboxes")
            .and_then(Value::as_array)
        {
            println!("accessible_mailboxes={}", mailboxes.len());
        }
        if let Some(has_more) = value.pointer("/data/has_more").and_then(Value::as_bool) {
            println!("has_more={has_more}");
        }
    }
    Ok(())
}

pub(super) fn print_blocks_response(raw_json: bool, value: Value) -> Result<()> {
    if raw_json {
        println!("{}", serde_json::to_string_pretty(&value)?);
        return Ok(());
    }

    let items = value
        .pointer("/data/items")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("blocks response missing data.items: {value}"))?;
    println!("blocks={}", items.len());
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for item in items {
        if let Some(block_type) = item.get("block_type").and_then(Value::as_i64) {
            *counts.entry(block_type).or_insert(0) += 1;
        }
    }
    let mut pairs = counts.into_iter().collect::<Vec<_>>();
    pairs.sort_by_key(|(block_type, _)| *block_type);
    for (block_type, count) in pairs {
        println!(
            "block_type_{}_{}={}",
            block_type,
            block_type_label(block_type),
            count
        );
    }
    for item in items {
        if item.get("block_type").and_then(Value::as_i64) == Some(43) {
            if let Some(token) = item.pointer("/board/token").and_then(Value::as_str) {
                let block_id = item
                    .get("block_id")
                    .and_then(Value::as_str)
                    .unwrap_or("<unknown>");
                println!("board_token[{block_id}]={token}");
            }
        }
    }
    print_code_language_counts(items);
    Ok(())
}

pub(super) fn print_convert_response(raw_json: bool, value: Value) -> Result<()> {
    if raw_json {
        println!("{}", serde_json::to_string_pretty(&value)?);
        return Ok(());
    }
    let blocks = value
        .pointer("/data/blocks")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("convert response missing data.blocks: {value}"))?;
    let roots = value
        .pointer("/data/first_level_block_ids")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    println!("converted_blocks={}", blocks.len());
    println!("first_level_blocks={roots}");
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for block in blocks {
        if let Some(block_type) = block.get("block_type").and_then(Value::as_i64) {
            *counts.entry(block_type).or_insert(0) += 1;
        }
    }
    print_block_counts(counts);
    print_code_language_counts(blocks);
    Ok(())
}

pub(super) fn print_generated_blocks(raw_json: bool, blocks: &[Value]) -> Result<()> {
    if raw_json {
        println!("{}", serde_json::to_string_pretty(blocks)?);
        return Ok(());
    }

    println!("generated_blocks={}", blocks.len());
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for block in blocks {
        if let Some(block_type) = block.get("block_type").and_then(Value::as_i64) {
            *counts.entry(block_type).or_insert(0) += 1;
        }
    }
    let mut pairs = counts.into_iter().collect::<Vec<_>>();
    print_block_count_pairs(&mut pairs);
    print_code_language_counts(blocks);
    Ok(())
}

pub(super) fn print_block_counts(counts: HashMap<i64, usize>) {
    let mut pairs = counts.into_iter().collect::<Vec<_>>();
    print_block_count_pairs(&mut pairs);
}

pub(super) fn print_block_count_pairs(pairs: &mut [(i64, usize)]) {
    pairs.sort_by_key(|(block_type, _)| *block_type);
    for (block_type, count) in pairs.iter() {
        println!(
            "block_type_{}_{}={}",
            block_type,
            block_type_label(*block_type),
            count
        );
    }
}

pub(super) fn print_code_language_counts(blocks: &[Value]) {
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for block in blocks {
        if let Some(language) = block
            .pointer("/code/style/language")
            .and_then(Value::as_i64)
        {
            *counts.entry(language).or_insert(0) += 1;
        }
    }
    let mut pairs = counts.into_iter().collect::<Vec<_>>();
    pairs.sort_by_key(|(language, _)| *language);
    for (language, count) in pairs {
        println!(
            "code_language_{}_{}={}",
            language,
            code_language_label(language),
            count
        );
    }
}

pub(super) fn block_type_label(block_type: i64) -> &'static str {
    match block_type {
        1 => "page",
        2 => "text",
        3 => "heading1",
        4 => "heading2",
        5 => "heading3",
        6 => "heading4",
        7 => "heading5",
        8 => "heading6",
        9 => "heading7",
        10 => "heading8",
        11 => "heading9",
        12 => "bullet",
        13 => "ordered",
        14 => "code",
        15 => "quote",
        17 => "todo",
        18 => "bitable",
        19 => "callout",
        20 => "chat_card",
        21 => "diagram",
        22 => "divider",
        23 => "file",
        24 => "grid",
        25 => "grid_column",
        26 => "iframe",
        27 => "image",
        28 => "isv",
        29 => "mindnote",
        30 => "sheet",
        31 => "table",
        32 => "table_cell",
        33 => "view",
        34 => "quote_container",
        35 => "task",
        36 => "okr",
        37 => "okr_objective",
        38 => "okr_key_result",
        39 => "okr_progress",
        40 => "add_ons",
        41 => "jira_issue",
        42 => "wiki_catalog",
        43 => "board",
        44 => "agenda",
        45 => "agenda_item",
        46 => "agenda_item_title",
        47 => "agenda_item_content",
        48 => "link_preview",
        49 => "source_synced",
        50 => "reference_synced",
        51 => "sub_page_list",
        52 => "ai_template",
        999 => "undefined",
        _ => "unknown",
    }
}

pub(super) fn code_language_label(language: i64) -> &'static str {
    match language {
        1 => "plain_text",
        2 => "abap",
        3 => "ada",
        4 => "apache",
        5 => "apex",
        6 => "assembly",
        7 => "bash",
        8 => "csharp",
        9 => "cpp",
        10 => "c",
        11 => "cobol",
        12 => "css",
        13 => "coffeescript",
        14 => "d",
        15 => "dart",
        16 => "delphi",
        17 => "django",
        18 => "dockerfile",
        19 => "erlang",
        20 => "fortran",
        21 => "foxpro",
        22 => "go",
        23 => "groovy",
        24 => "html",
        25 => "htmlbars",
        26 => "http",
        27 => "haskell",
        28 => "json",
        29 => "java",
        30 => "javascript",
        31 => "julia",
        32 => "kotlin",
        33 => "latex",
        34 => "lisp",
        35 => "logo",
        36 => "lua",
        37 => "matlab",
        38 => "makefile",
        39 => "markdown",
        40 => "nginx",
        41 => "objective_c",
        42 => "openedge_abl",
        43 => "php",
        44 => "perl",
        45 => "postscript",
        46 => "powershell",
        47 => "prolog",
        48 => "protobuf",
        49 => "python",
        50 => "r",
        51 => "rpg",
        52 => "ruby",
        53 => "rust",
        54 => "sas",
        55 => "scss",
        56 => "sql",
        57 => "scala",
        58 => "scheme",
        59 => "scratch",
        60 => "shell",
        61 => "swift",
        62 => "thrift",
        63 => "typescript",
        64 => "vbscript",
        65 => "visual",
        66 => "xml",
        67 => "yaml",
        68 => "cmake",
        69 => "diff",
        70 => "gherkin",
        71 => "graphql",
        72 => "glsl",
        73 => "properties",
        74 => "solidity",
        75 => "toml",
        _ => "unknown",
    }
}
