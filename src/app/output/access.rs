use super::*;

pub(in crate::app) fn get_string(value: &Value, path: &[&str]) -> Option<String> {
    let mut cursor = value;
    for segment in path {
        cursor = cursor.get(*segment)?;
    }
    cursor.as_str().map(ToString::to_string)
}

pub(in crate::app) fn get_i64(value: &Value, path: &[&str]) -> Option<i64> {
    let mut cursor = value;
    for segment in path {
        cursor = cursor.get(*segment)?;
    }
    cursor.as_i64()
}
