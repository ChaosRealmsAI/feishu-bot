use super::*;

pub(in crate::app) fn push_query_opt(
    query: &mut Vec<(String, String)>,
    key: &str,
    value: Option<String>,
) {
    if let Some(value) = value {
        if !value.trim().is_empty() {
            query.push((key.to_string(), value));
        }
    }
}

pub(in crate::app) fn push_query_opt_i64(
    query: &mut Vec<(String, String)>,
    key: &str,
    value: Option<i64>,
) {
    if let Some(value) = value {
        query.push((key.to_string(), value.to_string()));
    }
}

pub(in crate::app) fn push_query_opt_u8(
    query: &mut Vec<(String, String)>,
    key: &str,
    value: Option<u8>,
) {
    if let Some(value) = value {
        query.push((key.to_string(), value.to_string()));
    }
}

pub(in crate::app) fn push_query_repeated(
    query: &mut Vec<(String, String)>,
    key: &str,
    values: Vec<String>,
) {
    for value in values {
        if !value.trim().is_empty() {
            query.push((key.to_string(), value));
        }
    }
}

pub(in crate::app) fn has_json_input(
    text: &Option<String>,
    file: &Option<PathBuf>,
    stdin: bool,
) -> bool {
    text.is_some() || file.is_some() || stdin
}

pub(in crate::app) fn clean_string_values(values: Vec<String>) -> Vec<String> {
    values
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .collect()
}

pub(in crate::app) fn validate_value_count(
    label: &str,
    count: usize,
    max: usize,
    required: bool,
) -> Result<()> {
    if required && count == 0 {
        bail!("at least one {label} is required");
    }
    if count > max {
        bail!("{label} cannot repeat more than {max} times");
    }
    Ok(())
}

pub(in crate::app) fn encode_path_segment(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut output = String::new();
    for byte in value.as_bytes() {
        let keep = byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~');
        if keep {
            output.push(*byte as char);
        } else {
            output.push('%');
            output.push(HEX[(byte >> 4) as usize] as char);
            output.push(HEX[(byte & 0x0f) as usize] as char);
        }
    }
    output
}

pub(in crate::app) fn insert_opt_string(
    object: &mut Map<String, Value>,
    key: &str,
    value: Option<String>,
) {
    if let Some(value) = value.filter(|value| !value.trim().is_empty()) {
        object.insert(key.to_string(), Value::String(value));
    }
}

pub(in crate::app) fn insert_opt_i64(
    object: &mut Map<String, Value>,
    key: &str,
    value: Option<i64>,
) {
    if let Some(value) = value {
        object.insert(key.to_string(), Value::Number(value.into()));
    }
}

pub(in crate::app) fn insert_opt_u8(object: &mut Map<String, Value>, key: &str, value: Option<u8>) {
    if let Some(value) = value {
        object.insert(key.to_string(), json!(value));
    }
}

pub(in crate::app) fn insert_string_array(
    object: &mut Map<String, Value>,
    key: &str,
    values: Vec<String>,
) {
    let values = values
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .map(Value::String)
        .collect::<Vec<_>>();
    if !values.is_empty() {
        object.insert(key.to_string(), Value::Array(values));
    }
}

pub(in crate::app) fn parse_query_pairs(values: Vec<String>) -> Result<Vec<(String, String)>> {
    parse_key_value_pairs(values, "query")
}

pub(in crate::app) fn parse_header_pairs(values: Vec<String>) -> Result<Vec<(String, String)>> {
    parse_key_value_pairs(values, "header")
}

pub(in crate::app) fn parse_key_value_pairs(
    values: Vec<String>,
    label: &str,
) -> Result<Vec<(String, String)>> {
    values
        .into_iter()
        .map(|item| {
            let (key, value) = item
                .split_once('=')
                .ok_or_else(|| anyhow!("{label} must be key=value, got {item}"))?;
            if key.is_empty() {
                bail!("{label} key cannot be empty: {item}");
            }
            Ok((key.to_string(), value.to_string()))
        })
        .collect()
}

pub(in crate::app) fn parse_file_part_pairs(values: Vec<String>) -> Result<Vec<(String, PathBuf)>> {
    values
        .into_iter()
        .map(|item| {
            let (key, value) = item
                .split_once('=')
                .ok_or_else(|| anyhow!("file part must be part_name=path, got {item}"))?;
            if key.is_empty() {
                bail!("file part name cannot be empty: {item}");
            }
            if value.trim().is_empty() {
                bail!("file part path cannot be empty: {item}");
            }
            Ok((key.to_string(), PathBuf::from(value)))
        })
        .collect()
}

pub(in crate::app) fn read_json_value(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    let text = read_content(text, file, stdin)?;
    parse_json_value(&text, "JSON")
}

pub(in crate::app) fn read_optional_json_value(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Option<Value>> {
    read_optional_content(text, file, stdin)?
        .map(|text| parse_json_value(&text, "JSON"))
        .transpose()
}

pub(in crate::app) fn parse_json_value(text: &str, label: &str) -> Result<Value> {
    serde_json::from_str(text).with_context(|| format!("parse {label}"))
}

pub(in crate::app) fn ensure_json_array(value: Value, label: &str) -> Result<Value> {
    if value.is_array() {
        Ok(value)
    } else {
        bail!("{label} must be a JSON array")
    }
}

pub(in crate::app) fn ensure_json_object(value: Value, label: &str) -> Result<Value> {
    if value.is_object() {
        Ok(value)
    } else {
        bail!("{label} must be a JSON object")
    }
}

pub(in crate::app) fn read_record_ids_json(
    mut ids: Vec<String>,
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if let Some(value) = read_optional_json_value(text, file, stdin)? {
        let record_ids = if let Some(record_ids) = value.get("record_ids") {
            record_ids.clone()
        } else if let Some(records) = value.get("records") {
            records.clone()
        } else {
            value
        };
        return ensure_json_array(record_ids, "record_ids");
    }
    ids.retain(|id| !id.trim().is_empty());
    if ids.is_empty() {
        bail!("provide --record-id at least once, or JSON via --record-ids-json/--records-json/--file/--stdin");
    }
    Ok(Value::Array(ids.into_iter().map(Value::String).collect()))
}

pub(in crate::app) fn read_table_ids_json(
    mut ids: Vec<String>,
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if let Some(value) = read_optional_json_value(text, file, stdin)? {
        let table_ids = if let Some(table_ids) = value.get("table_ids") {
            table_ids.clone()
        } else if let Some(tables) = value.get("tables") {
            tables.clone()
        } else {
            value
        };
        return ensure_json_array(table_ids, "table_ids");
    }
    ids.retain(|id| !id.trim().is_empty());
    if ids.is_empty() {
        bail!("provide --table-id at least once, or JSON via --table-ids-json/--file/--stdin");
    }
    Ok(Value::Array(ids.into_iter().map(Value::String).collect()))
}

pub(in crate::app) fn collect_json_string_array(
    mut values: Vec<String>,
    text: Option<String>,
    label: &str,
) -> Result<Option<Value>> {
    if let Some(text) = text {
        let value = parse_json_value(&text, label)?;
        let array = if let Some(nested) = value.get(label) {
            nested.clone()
        } else {
            value
        };
        return Ok(Some(ensure_json_array(array, label)?));
    }
    values.retain(|value| !value.trim().is_empty());
    if values.is_empty() {
        return Ok(None);
    }
    Ok(Some(Value::Array(
        values.into_iter().map(Value::String).collect(),
    )))
}

impl ReceiveIdTypeArg {
    pub(in crate::app) fn resolve(self, id: &str) -> &'static str {
        match self {
            ReceiveIdTypeArg::OpenId => "open_id",
            ReceiveIdTypeArg::UnionId => "union_id",
            ReceiveIdTypeArg::UserId => "user_id",
            ReceiveIdTypeArg::Email => "email",
            ReceiveIdTypeArg::ChatId => "chat_id",
            ReceiveIdTypeArg::Auto => infer_receive_id_type(id),
        }
    }
}

impl UserIdTypeArg {
    pub(in crate::app) fn resolve(self, sample: Option<&str>) -> &'static str {
        match self {
            UserIdTypeArg::OpenId => "open_id",
            UserIdTypeArg::UnionId => "union_id",
            UserIdTypeArg::UserId => "user_id",
            UserIdTypeArg::Auto => sample.map(infer_user_id_type).unwrap_or("open_id"),
        }
    }
}

impl OkrUserIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            OkrUserIdTypeArg::OpenId => "open_id",
            OkrUserIdTypeArg::UnionId => "union_id",
            OkrUserIdTypeArg::UserId => "user_id",
            OkrUserIdTypeArg::PeopleAdminId => "people_admin_id",
        }
    }
}

impl AttendanceEmployeeTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            AttendanceEmployeeTypeArg::EmployeeId => "employee_id",
            AttendanceEmployeeTypeArg::EmployeeNo => "employee_no",
        }
    }
}

impl DepartmentIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            DepartmentIdTypeArg::OpenDepartmentId => "open_department_id",
            DepartmentIdTypeArg::DepartmentId => "department_id",
        }
    }
}

impl ContentTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            ContentTypeArg::Markdown => "markdown",
            ContentTypeArg::Html => "html",
        }
    }
}

pub(in crate::app) fn infer_receive_id_type(id: &str) -> &'static str {
    if id.starts_with("oc_") {
        "chat_id"
    } else if id.starts_with("ou_") {
        "open_id"
    } else if id.starts_with("on_") {
        "union_id"
    } else if id.contains('@') {
        "email"
    } else {
        "user_id"
    }
}

pub(in crate::app) fn infer_user_id_type(id: &str) -> &'static str {
    match infer_receive_id_type(id) {
        "chat_id" | "email" => "open_id",
        other => other,
    }
}

pub(in crate::app) fn read_content(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<String> {
    read_optional_content(text, file, stdin)?
        .ok_or_else(|| anyhow!("provide --text/--content, --file, or --stdin"))
}

pub(in crate::app) fn read_optional_content(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Option<String>> {
    if let Some(text) = text {
        return Ok(Some(text));
    }
    if let Some(path) = file {
        return Ok(Some(
            fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?,
        ));
    }
    if stdin {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).context("read stdin")?;
        return Ok(Some(buf));
    }
    Ok(None)
}

pub(in crate::app) fn random_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub(in crate::app) fn unescape_newlines(value: &str) -> String {
    value.replace("\\n", "\n")
}
