use super::*;

use super::input::BaseFieldBuildInput;

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

pub(super) fn build_base_field_body(input: BaseFieldBuildInput) -> Result<Value> {
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
