use super::*;

use super::body::build_base_field_body;
use super::input::BaseFieldBuildInput;

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
