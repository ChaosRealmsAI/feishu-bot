use super::*;

pub(in crate::app) fn print_response(raw_json: bool, label: &str, value: Value) -> Result<()> {
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
