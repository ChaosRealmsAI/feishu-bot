use super::*;

pub(super) struct BaseFieldBuildInput {
    pub(super) name: Option<String>,
    pub(super) field_type: Option<i64>,
    pub(super) kind: Option<BaseFieldKindArg>,
    pub(super) property_json: Option<String>,
    pub(super) description_json: Option<String>,
    pub(super) ui_type: Option<String>,
    pub(super) options: Vec<String>,
    pub(super) formatter: Option<String>,
    pub(super) currency_code: Option<String>,
    pub(super) date_formatter: Option<String>,
    pub(super) auto_fill: Option<bool>,
    pub(super) multiple: Option<bool>,
    pub(super) linked_table_id: Option<String>,
    pub(super) formula: Option<String>,
    pub(super) location_input_type: Option<String>,
    pub(super) require_name_and_type: bool,
}
