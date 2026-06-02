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
