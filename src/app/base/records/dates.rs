use super::*;

pub(super) fn base_fields_contain_date_like_string(value: &Value) -> bool {
    match value {
        Value::String(text) => maybe_parse_base_record_date_millis(text)
            .ok()
            .flatten()
            .is_some(),
        Value::Array(values) => values.iter().any(base_fields_contain_date_like_string),
        Value::Object(map) => map.values().any(base_fields_contain_date_like_string),
        _ => false,
    }
}

pub(super) fn maybe_parse_base_record_date_millis(value: &str) -> Result<Option<i64>> {
    let value = value.trim();
    if value.is_empty() || value.chars().all(|char| char.is_ascii_digit()) {
        return Ok(None);
    }
    parse_base_record_datetime_millis(value)
        .map(Some)
        .or_else(|_| {
            parse_base_record_date_millis(value)
                .map(Some)
                .or_else(|_| Ok(None))
        })
}

pub(super) fn parse_base_record_datetime_millis(value: &str) -> Result<i64> {
    let value = value.trim();
    if value.chars().all(|char| char.is_ascii_digit()) {
        return value
            .parse::<i64>()
            .with_context(|| format!("parse base datetime milliseconds: {value}"));
    }
    if let Ok(datetime) = DateTime::parse_from_rfc3339(value) {
        return Ok(datetime.timestamp_millis());
    }
    for format in [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%dT%H:%M",
        "%Y/%m/%d %H:%M:%S",
        "%Y/%m/%d %H:%M",
    ] {
        if let Ok(naive) = NaiveDateTime::parse_from_str(value, format) {
            let datetime = Local
                .from_local_datetime(&naive)
                .single()
                .ok_or_else(|| anyhow!("base datetime is ambiguous in local timezone: {value}"))?;
            return Ok(datetime.timestamp_millis());
        }
    }
    bail!("base datetime must be milliseconds, RFC3339, or local 'YYYY-MM-DD HH:MM[:SS]': {value}");
}

pub(super) fn parse_base_record_date_millis(value: &str) -> Result<i64> {
    let value = value.trim();
    for format in ["%Y-%m-%d", "%Y/%m/%d"] {
        if let Ok(date) = NaiveDate::parse_from_str(value, format) {
            let naive = date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| anyhow!("invalid base date: {value}"))?;
            let datetime = Local
                .from_local_datetime(&naive)
                .single()
                .ok_or_else(|| anyhow!("base date is ambiguous in local timezone: {value}"))?;
            return Ok(datetime.timestamp_millis());
        }
    }
    bail!("base date must be YYYY-MM-DD or YYYY/MM/DD: {value}");
}
