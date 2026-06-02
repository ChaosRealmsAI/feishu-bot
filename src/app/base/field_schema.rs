use super::*;

mod body;
mod input;
mod spec;

pub(in crate::app) use body::{build_base_field_create_body, build_base_field_update_body};
pub(in crate::app) use spec::parse_base_table_field_spec;
