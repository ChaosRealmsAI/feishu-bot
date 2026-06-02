use super::*;

mod batch;
mod dates;
mod fields;
mod normalize;

pub(in crate::app) use batch::read_base_record_batch_records;
pub(in crate::app) use fields::read_base_record_fields;
pub(super) use normalize::{
    normalize_base_record_write_fields, normalize_base_record_write_records,
};

pub(super) fn base_record_write_query(
    client_token: Option<String>,
    user_id_type: UserIdTypeArg,
    ignore_consistency_check: bool,
) -> Vec<(String, String)> {
    let mut query = vec![(
        "user_id_type".to_string(),
        user_id_type.resolve(None).to_string(),
    )];
    push_query_opt(&mut query, "client_token", client_token);
    if ignore_consistency_check {
        query.push(("ignore_consistency_check".to_string(), "true".to_string()));
    }
    query
}
