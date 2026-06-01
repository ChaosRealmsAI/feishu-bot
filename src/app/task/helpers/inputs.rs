use super::*;

pub(super) fn insert_task_time_create(
    body: &mut Map<String, Value>,
    field: &str,
    timestamp_ms: Option<String>,
    timestamp_at: Option<String>,
    date: Option<String>,
    is_all_day: bool,
) -> Result<()> {
    let Some((timestamp, is_all_day)) =
        resolve_task_time_value(field, timestamp_ms, timestamp_at, date, is_all_day)?
    else {
        if is_all_day {
            bail!("--{field}-all-day requires --{field}-ms, --{field}-at, or --{field}-date");
        }
        return Ok(());
    };
    body.insert(
        field.to_string(),
        json!({ "timestamp": timestamp, "is_all_day": is_all_day }),
    );
    Ok(())
}

pub(super) fn insert_task_time_update(
    task: &mut Map<String, Value>,
    update_fields: &mut Vec<Value>,
    field: &str,
    timestamp_ms: Option<String>,
    timestamp_at: Option<String>,
    date: Option<String>,
    is_all_day: bool,
    clear: bool,
) -> Result<()> {
    let timestamp = resolve_task_time_value(field, timestamp_ms, timestamp_at, date, is_all_day)?;
    if timestamp.is_some() && clear {
        bail!("task update cannot combine --{field}-ms/--{field}-at/--{field}-date with --clear-{field}");
    }
    if let Some((timestamp, is_all_day)) = timestamp {
        task.insert(
            field.to_string(),
            json!({ "timestamp": timestamp, "is_all_day": is_all_day }),
        );
        update_fields.push(Value::String(field.to_string()));
    } else if clear {
        update_fields.push(Value::String(field.to_string()));
    } else if is_all_day {
        bail!("--{field}-all-day requires --{field}-ms, --{field}-at, or --{field}-date");
    }
    Ok(())
}

fn resolve_task_time_value(
    field: &str,
    timestamp_ms: Option<String>,
    timestamp_at: Option<String>,
    date: Option<String>,
    is_all_day: bool,
) -> Result<Option<(String, bool)>> {
    let timestamp_ms = timestamp_ms.filter(|value| !value.trim().is_empty());
    let timestamp_at = timestamp_at.filter(|value| !value.trim().is_empty());
    let date = date.filter(|value| !value.trim().is_empty());
    let provided =
        timestamp_ms.is_some() as u8 + timestamp_at.is_some() as u8 + date.is_some() as u8;
    if provided > 1 {
        bail!(
            "task {field} time accepts only one of --{field}-ms, --{field}-at, or --{field}-date"
        );
    }
    if let Some(timestamp) = timestamp_ms {
        return Ok(Some((timestamp, is_all_day)));
    }
    if let Some(value) = timestamp_at {
        return Ok(Some((parse_task_timestamp_millis(&value)?, is_all_day)));
    }
    if let Some(value) = date {
        return Ok(Some((parse_task_date_millis(&value)?, true)));
    }
    Ok(None)
}

fn parse_task_timestamp_millis(value: &str) -> Result<String> {
    let value = value.trim();
    if value.chars().all(|char| char.is_ascii_digit()) {
        return Ok(value.to_string());
    }
    if let Ok(datetime) = DateTime::parse_from_rfc3339(value) {
        return Ok(datetime.timestamp_millis().to_string());
    }
    for format in [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%dT%H:%M",
    ] {
        if let Ok(naive) = NaiveDateTime::parse_from_str(value, format) {
            let datetime = Local
                .from_local_datetime(&naive)
                .single()
                .ok_or_else(|| anyhow!("task time is ambiguous in local timezone: {value}"))?;
            return Ok(datetime.timestamp_millis().to_string());
        }
    }
    bail!("task time must be milliseconds, RFC3339, or local 'YYYY-MM-DD HH:MM[:SS]': {value}");
}

fn parse_task_date_millis(value: &str) -> Result<String> {
    let date = NaiveDate::parse_from_str(value.trim(), "%Y-%m-%d")
        .with_context(|| format!("parse task all-day date: {value}"))?;
    let naive = date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| anyhow!("invalid task date: {value}"))?;
    let datetime = Local
        .from_local_datetime(&naive)
        .single()
        .ok_or_else(|| anyhow!("task date is ambiguous in local timezone: {value}"))?;
    Ok(datetime.timestamp_millis().to_string())
}

pub(super) fn insert_task_clearable_string(
    task: &mut Map<String, Value>,
    update_fields: &mut Vec<Value>,
    field: &str,
    value: Option<String>,
    clear: bool,
) -> Result<()> {
    if value.is_some() && clear {
        bail!("task update cannot combine --{field} with --clear-{field}");
    }
    if let Some(value) = value {
        task.insert(field.to_string(), Value::String(value));
        update_fields.push(Value::String(field.to_string()));
    } else if clear {
        update_fields.push(Value::String(field.to_string()));
    }
    Ok(())
}

pub(super) fn insert_task_clearable_object(
    task: &mut Map<String, Value>,
    update_fields: &mut Vec<Value>,
    field: &str,
    value: Option<String>,
    clear: bool,
    label: &str,
) -> Result<()> {
    if value.is_some() && clear {
        bail!("task update cannot combine --{field}-json with --clear-{field}");
    }
    if let Some(value) = value {
        task.insert(field.to_string(), task_object_from_json(value, label)?);
        update_fields.push(Value::String(field.to_string()));
    } else if clear {
        update_fields.push(Value::String(field.to_string()));
    }
    Ok(())
}

pub(super) fn insert_task_json_object(
    body: &mut Map<String, Value>,
    field: &str,
    value: Option<String>,
    label: &str,
) -> Result<()> {
    if let Some(value) = value {
        body.insert(field.to_string(), task_object_from_json(value, label)?);
    }
    Ok(())
}

pub(super) fn insert_task_json_array(
    body: &mut Map<String, Value>,
    field: &str,
    value: Option<String>,
    label: &str,
) -> Result<()> {
    if let Some(value) = value {
        body.insert(field.to_string(), task_array_from_json(value, label)?);
    }
    Ok(())
}

pub(super) fn insert_task_reminders(
    body: &mut Map<String, Value>,
    reminders_json: Option<String>,
    reminder_minute: Option<i64>,
    label: &str,
) -> Result<()> {
    if reminders_json.is_some() && reminder_minute.is_some() {
        bail!("task create cannot combine --reminders-json with --reminder-minute");
    }
    if let Some(value) = reminders_json {
        body.insert("reminders".to_string(), task_array_from_json(value, label)?);
    } else if let Some(reminder_minute) = reminder_minute {
        if body.get("due").is_none() {
            bail!("--reminder-minute requires --due-ms, --due-at, or --due-date");
        }
        body.insert(
            "reminders".to_string(),
            Value::Array(vec![task_relative_reminder(reminder_minute)?]),
        );
    }
    Ok(())
}

pub(super) fn insert_task_optional_u8(
    body: &mut Map<String, Value>,
    field: &str,
    value: Option<u8>,
) -> Result<()> {
    if let Some(value) = value {
        if field == "mode" && !(1..=2).contains(&value) {
            bail!("task mode must be 1 or 2");
        }
        body.insert(
            field.to_string(),
            Value::Number(serde_json::Number::from(value)),
        );
    }
    Ok(())
}

fn task_object_from_json(value: String, label: &str) -> Result<Value> {
    ensure_json_object(parse_json_value(&value, label)?, label)
}

pub(super) fn task_array_from_json(value: String, label: &str) -> Result<Value> {
    let value = parse_json_value(&value, label)?;
    if label.contains("reminders") {
        if let Some(array) = value.get("reminders") {
            return ensure_json_array(array.clone(), "reminders");
        }
    }
    if label.contains("custom-fields") {
        if let Some(array) = value.get("custom_fields") {
            return ensure_json_array(array.clone(), "custom_fields");
        }
    }
    ensure_json_array(value, label)
}

pub(super) fn task_relative_reminder(relative_fire_minute: i64) -> Result<Value> {
    if relative_fire_minute < 0 {
        bail!("task reminder minute cannot be negative");
    }
    Ok(json!({ "relative_fire_minute": relative_fire_minute }))
}
