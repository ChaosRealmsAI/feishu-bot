use super::*;

pub(in crate::app) fn build_base_app_update_body(args: BaseAppUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "base app update body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name);
    if let Some(is_advanced) = args.is_advanced {
        body.insert("is_advanced".to_string(), Value::Bool(is_advanced));
    }
    if body.is_empty() {
        bail!("base update needs --name, --is-advanced, or raw body");
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_base_copy_body(args: BaseCopyArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "base copy body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name);
    insert_opt_string(&mut body, "folder_token", args.folder_token);
    insert_opt_string(&mut body, "time_zone", args.time_zone);
    if let Some(without_content) = args.without_content {
        body.insert("without_content".to_string(), Value::Bool(without_content));
    }
    if body.is_empty() {
        bail!(
            "base copy needs --name, --folder-token, --without-content, --time-zone, or raw body"
        );
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_base_table_batch_create_body(
    args: BaseTableBatchCreateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "table batch_create body",
        );
    }
    if let Some(tables_json) = args.tables_json {
        let value = parse_json_value(&tables_json, "tables-json")?;
        if value.get("tables").is_some() {
            return ensure_json_object(value, "table batch_create body");
        }
        return Ok(json!({ "tables": ensure_json_array(value, "tables")? }));
    }
    let tables = args
        .name
        .into_iter()
        .filter(|name| !name.trim().is_empty())
        .map(|name| json!({ "name": name }))
        .collect::<Vec<_>>();
    if tables.is_empty() {
        bail!("table batch-create needs --name, --tables-json, or raw body");
    }
    Ok(json!({ "tables": tables }))
}

pub(in crate::app) fn build_base_table_create_body(args: BaseTableCreateArgs) -> Result<Value> {
    let mut table = Map::new();
    insert_opt_string(&mut table, "name", args.name);
    insert_opt_string(&mut table, "default_view_name", args.default_view_name);

    let mut fields = Vec::new();
    if let Some(value) =
        read_optional_json_value(args.fields_json, args.fields_file, args.fields_stdin)?
    {
        match ensure_json_array(value, "table.fields")? {
            Value::Array(items) => fields.extend(items),
            _ => unreachable!("ensure_json_array only returns arrays"),
        }
    }
    for spec in args.field_specs {
        fields.push(parse_base_table_field_spec(&spec)?);
    }
    if !fields.is_empty() {
        table.insert("fields".to_string(), Value::Array(fields));
    }

    Ok(json!({ "table": Value::Object(table) }))
}

pub(in crate::app) fn build_base_table_update_body(args: BaseTableUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "base table update body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name);
    if body.is_empty() {
        bail!("base table update needs --name or raw body");
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn parse_base_table_field_spec(spec: &str) -> Result<Value> {
    let mut parts = spec.splitn(3, ':');
    let name = parts
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("base table --field must be name:kind[:config]"))?;
    let kind_text = parts
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("base table --field must be name:kind[:config]"))?;
    let config = parts
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let kind = parse_base_field_kind(kind_text)?;

    let mut input = BaseFieldBuildInput {
        name: Some(name.to_string()),
        field_type: None,
        kind: Some(kind),
        property_json: None,
        description_json: None,
        ui_type: None,
        options: Vec::new(),
        formatter: None,
        currency_code: None,
        date_formatter: None,
        auto_fill: None,
        multiple: None,
        linked_table_id: None,
        formula: None,
        location_input_type: None,
        require_name_and_type: true,
    };

    if let Some(config) = config {
        if let Some(json) = config.strip_prefix("json:").or_else(|| {
            config
                .strip_prefix("property=")
                .or_else(|| config.strip_prefix("property-json="))
        }) {
            input.property_json = Some(json.to_string());
        } else if config.starts_with('{') {
            input.property_json = Some(config.to_string());
        } else {
            apply_base_table_field_config(&mut input, kind, config)?;
        }
    }

    build_base_field_body(input)
}

fn parse_base_field_kind(value: &str) -> Result<BaseFieldKindArg> {
    let normalized = value.trim().to_ascii_lowercase().replace('_', "-");
    match normalized.as_str() {
        "text" => Ok(BaseFieldKindArg::Text),
        "barcode" => Ok(BaseFieldKindArg::Barcode),
        "email" => Ok(BaseFieldKindArg::Email),
        "number" => Ok(BaseFieldKindArg::Number),
        "progress" => Ok(BaseFieldKindArg::Progress),
        "currency" => Ok(BaseFieldKindArg::Currency),
        "rating" => Ok(BaseFieldKindArg::Rating),
        "single-select" | "select" => Ok(BaseFieldKindArg::SingleSelect),
        "multi-select" | "multiple-select" => Ok(BaseFieldKindArg::MultiSelect),
        "date" => Ok(BaseFieldKindArg::Date),
        "checkbox" => Ok(BaseFieldKindArg::Checkbox),
        "user" => Ok(BaseFieldKindArg::User),
        "phone" => Ok(BaseFieldKindArg::Phone),
        "url" => Ok(BaseFieldKindArg::Url),
        "attachment" | "file" => Ok(BaseFieldKindArg::Attachment),
        "link" => Ok(BaseFieldKindArg::Link),
        "formula" => Ok(BaseFieldKindArg::Formula),
        "duplex-link" => Ok(BaseFieldKindArg::DuplexLink),
        "location" => Ok(BaseFieldKindArg::Location),
        "group" => Ok(BaseFieldKindArg::Group),
        "auto-number" | "autonumber" => Ok(BaseFieldKindArg::AutoNumber),
        _ => bail!("unknown base table --field kind: {value}"),
    }
}

fn apply_base_table_field_config(
    input: &mut BaseFieldBuildInput,
    kind: BaseFieldKindArg,
    config: &str,
) -> Result<()> {
    match kind {
        BaseFieldKindArg::SingleSelect | BaseFieldKindArg::MultiSelect => {
            input.options = split_base_table_field_config(config)
                .into_iter()
                .map(str::to_string)
                .collect();
        }
        BaseFieldKindArg::Currency => {
            for (index, token) in split_base_table_field_config(config)
                .into_iter()
                .enumerate()
            {
                if let Some((key, value)) = token.split_once('=') {
                    match normalize_base_table_config_key(key).as_str() {
                        "formatter" | "format" => input.formatter = Some(value.to_string()),
                        "currency" | "currency-code" | "currencycode" => {
                            input.currency_code = Some(value.to_string())
                        }
                        _ => bail!("unknown currency --field config key: {key}"),
                    }
                } else if index == 0 {
                    input.formatter = Some(token.to_string());
                } else if index == 1 {
                    input.currency_code = Some(token.to_string());
                } else {
                    bail!("currency --field config supports formatter and currency_code only");
                }
            }
        }
        BaseFieldKindArg::Date => {
            for token in split_base_table_field_config(config) {
                if let Some((key, value)) = token.split_once('=') {
                    match normalize_base_table_config_key(key).as_str() {
                        "formatter" | "date-formatter" | "dateformatter" => {
                            input.date_formatter = Some(value.to_string())
                        }
                        "auto-fill" | "autofill" => input.auto_fill = Some(parse_boolish(value)?),
                        _ => bail!("unknown date --field config key: {key}"),
                    }
                } else {
                    input.date_formatter = Some(token.to_string());
                }
            }
        }
        BaseFieldKindArg::Number | BaseFieldKindArg::Progress | BaseFieldKindArg::Rating => {
            input.formatter = Some(config.to_string());
        }
        BaseFieldKindArg::Formula => {
            input.formula = Some(config.to_string());
        }
        BaseFieldKindArg::Link | BaseFieldKindArg::DuplexLink => {
            let value = config
                .split_once('=')
                .map(|(_, value)| value)
                .unwrap_or(config)
                .trim();
            input.linked_table_id = Some(value.to_string());
        }
        BaseFieldKindArg::User | BaseFieldKindArg::Group => {
            let value = config
                .split_once('=')
                .map(|(_, value)| value)
                .unwrap_or(config)
                .trim();
            input.multiple = Some(value.eq_ignore_ascii_case("multiple") || parse_boolish(value)?);
        }
        BaseFieldKindArg::Location => {
            let value = config
                .split_once('=')
                .map(|(_, value)| value)
                .unwrap_or(config)
                .trim();
            input.location_input_type = Some(value.to_string());
        }
        _ => {
            bail!("--field config is not supported for kind {kind:?}; use json:{{...}} for raw property")
        }
    }
    Ok(())
}

fn split_base_table_field_config(config: &str) -> Vec<&str> {
    config
        .split('|')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect()
}

fn normalize_base_table_config_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace('_', "-")
}

fn parse_boolish(value: &str) -> Result<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "y" | "on" => Ok(true),
        "false" | "0" | "no" | "n" | "off" => Ok(false),
        other => bail!("expected boolean config value, got {other}"),
    }
}

pub(in crate::app) fn build_base_field_create_body(args: BaseFieldCreateArgs) -> Result<Value> {
    build_base_field_body(BaseFieldBuildInput {
        name: Some(args.name),
        field_type: args.field_type,
        kind: args.kind,
        property_json: args.property_json,
        description_json: args.description_json,
        ui_type: args.ui_type,
        options: args.options,
        formatter: args.formatter,
        currency_code: args.currency_code,
        date_formatter: args.date_formatter,
        auto_fill: args.auto_fill,
        multiple: args.multiple,
        linked_table_id: args.linked_table_id,
        formula: args.formula,
        location_input_type: args.location_input_type,
        require_name_and_type: true,
    })
}

pub(in crate::app) fn build_base_field_update_body(args: BaseFieldUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "base field update body",
        );
    }
    build_base_field_body(BaseFieldBuildInput {
        name: args.name,
        field_type: args.field_type,
        kind: args.kind,
        property_json: args.property_json,
        description_json: args.description_json,
        ui_type: args.ui_type,
        options: args.options,
        formatter: args.formatter,
        currency_code: args.currency_code,
        date_formatter: args.date_formatter,
        auto_fill: args.auto_fill,
        multiple: args.multiple,
        linked_table_id: args.linked_table_id,
        formula: args.formula,
        location_input_type: args.location_input_type,
        require_name_and_type: true,
    })
}

struct BaseFieldBuildInput {
    name: Option<String>,
    field_type: Option<i64>,
    kind: Option<BaseFieldKindArg>,
    property_json: Option<String>,
    description_json: Option<String>,
    ui_type: Option<String>,
    options: Vec<String>,
    formatter: Option<String>,
    currency_code: Option<String>,
    date_formatter: Option<String>,
    auto_fill: Option<bool>,
    multiple: Option<bool>,
    linked_table_id: Option<String>,
    formula: Option<String>,
    location_input_type: Option<String>,
    require_name_and_type: bool,
}

fn build_base_field_body(input: BaseFieldBuildInput) -> Result<Value> {
    let mut body = Map::new();
    let name = input
        .name
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("base field needs --name and --type/--kind, or raw body"))?;
    let (kind_type, kind_ui_type) = input.kind.map(base_field_kind_parts).unwrap_or((0, None));
    let field_type = match (input.field_type, input.kind) {
        (Some(field_type), Some(kind)) => {
            let (expected, _) = base_field_kind_parts(kind);
            if field_type != expected {
                bail!(
                    "base field --type {field_type} does not match --kind {kind:?} type {expected}"
                );
            }
            field_type
        }
        (Some(field_type), None) => field_type,
        (None, Some(_)) => kind_type,
        (None, None) if input.require_name_and_type => {
            bail!("base field needs --type or --kind unless raw body is used")
        }
        (None, None) => 0,
    };
    body.insert("field_name".to_string(), Value::String(name));
    body.insert("type".to_string(), Value::Number(field_type.into()));

    let property_value = input
        .property_json
        .map(|text| parse_json_value(&text, "property-json"))
        .transpose()?;
    let has_typed_property = !input.options.is_empty()
        || input.formatter.is_some()
        || input.currency_code.is_some()
        || input.date_formatter.is_some()
        || input.auto_fill.is_some()
        || input.multiple.is_some()
        || input.linked_table_id.is_some()
        || input.formula.is_some()
        || input.location_input_type.is_some();
    if has_typed_property {
        let mut property = match property_value {
            Some(value) => match ensure_json_object(value, "field.property")? {
                Value::Object(map) => map,
                _ => Map::new(),
            },
            None => Map::new(),
        };
        if !input.options.is_empty() {
            property.insert(
                "options".to_string(),
                Value::Array(
                    input
                        .options
                        .into_iter()
                        .map(base_field_option)
                        .collect::<Result<Vec<_>>>()?,
                ),
            );
        }
        insert_opt_string(&mut property, "formatter", input.formatter);
        insert_opt_string(&mut property, "currency_code", input.currency_code);
        insert_opt_string(&mut property, "date_formatter", input.date_formatter);
        insert_opt_string(&mut property, "table_id", input.linked_table_id);
        insert_opt_string(&mut property, "formula_expression", input.formula);
        if let Some(auto_fill) = input.auto_fill {
            property.insert("auto_fill".to_string(), Value::Bool(auto_fill));
        }
        if let Some(multiple) = input.multiple {
            property.insert("multiple".to_string(), Value::Bool(multiple));
        }
        if let Some(location_input_type) = input.location_input_type {
            property.insert(
                "location".to_string(),
                json!({ "input_type": location_input_type }),
            );
        }
        body.insert("property".to_string(), Value::Object(property));
    } else if let Some(property) = property_value {
        body.insert("property".to_string(), property);
    }
    if let Some(description) = input.description_json {
        body.insert(
            "description".to_string(),
            parse_json_value(&description, "description-json")?,
        );
    }
    let ui_type = input.ui_type.or_else(|| kind_ui_type.map(str::to_string));
    insert_opt_string(&mut body, "ui_type", ui_type);
    Ok(Value::Object(body))
}

fn base_field_kind_parts(kind: BaseFieldKindArg) -> (i64, Option<&'static str>) {
    match kind {
        BaseFieldKindArg::Text => (1, None),
        BaseFieldKindArg::Barcode => (1, Some("Barcode")),
        BaseFieldKindArg::Email => (1, Some("Email")),
        BaseFieldKindArg::Number => (2, None),
        BaseFieldKindArg::Progress => (2, Some("Progress")),
        BaseFieldKindArg::Currency => (2, Some("Currency")),
        BaseFieldKindArg::Rating => (2, Some("Rating")),
        BaseFieldKindArg::SingleSelect => (3, None),
        BaseFieldKindArg::MultiSelect => (4, None),
        BaseFieldKindArg::Date => (5, None),
        BaseFieldKindArg::Checkbox => (7, None),
        BaseFieldKindArg::User => (11, None),
        BaseFieldKindArg::Phone => (13, None),
        BaseFieldKindArg::Url => (15, None),
        BaseFieldKindArg::Attachment => (17, None),
        BaseFieldKindArg::Link => (18, None),
        BaseFieldKindArg::Formula => (20, None),
        BaseFieldKindArg::DuplexLink => (21, None),
        BaseFieldKindArg::Location => (22, None),
        BaseFieldKindArg::Group => (23, None),
        BaseFieldKindArg::AutoNumber => (1005, None),
    }
}

fn base_field_option(value: String) -> Result<Value> {
    let mut option = Map::new();
    if let Some((name, color)) = value.rsplit_once(':') {
        if let Ok(color) = color.parse::<i64>() {
            option.insert("name".to_string(), Value::String(name.to_string()));
            option.insert("color".to_string(), Value::Number(color.into()));
            return Ok(Value::Object(option));
        }
    }
    option.insert("name".to_string(), Value::String(value));
    Ok(Value::Object(option))
}

pub(in crate::app) fn build_base_dashboard_copy_body(args: BaseDashboardCopyArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "dashboard copy body",
        );
    }
    let name = args
        .name
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("dashboard copy needs --name or raw body"))?;
    Ok(json!({ "name": name }))
}

pub(in crate::app) fn build_base_workflow_update_body(
    args: BaseWorkflowUpdateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "workflow update body",
        );
    }
    let status = args
        .status
        .ok_or_else(|| anyhow!("workflow update needs --status or raw body"))?;
    Ok(json!({ "status": status.as_feishu() }))
}
