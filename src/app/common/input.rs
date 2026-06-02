use super::*;

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
