use super::labels::{block_type_label, code_language_label};
use super::*;

pub(in crate::app) fn print_blocks_response(raw_json: bool, value: Value) -> Result<()> {
    if raw_json {
        println!("{}", serde_json::to_string_pretty(&value)?);
        return Ok(());
    }

    let items = value
        .pointer("/data/items")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("blocks response missing data.items: {value}"))?;
    println!("blocks={}", items.len());
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for item in items {
        if let Some(block_type) = item.get("block_type").and_then(Value::as_i64) {
            *counts.entry(block_type).or_insert(0) += 1;
        }
    }
    let mut pairs = counts.into_iter().collect::<Vec<_>>();
    pairs.sort_by_key(|(block_type, _)| *block_type);
    for (block_type, count) in pairs {
        println!(
            "block_type_{}_{}={}",
            block_type,
            block_type_label(block_type),
            count
        );
    }
    for item in items {
        if item.get("block_type").and_then(Value::as_i64) == Some(43) {
            if let Some(token) = item.pointer("/board/token").and_then(Value::as_str) {
                let block_id = item
                    .get("block_id")
                    .and_then(Value::as_str)
                    .unwrap_or("<unknown>");
                println!("board_token[{block_id}]={token}");
            }
        }
    }
    print_code_language_counts(items);
    Ok(())
}

pub(in crate::app) fn print_convert_response(raw_json: bool, value: Value) -> Result<()> {
    if raw_json {
        println!("{}", serde_json::to_string_pretty(&value)?);
        return Ok(());
    }
    let blocks = value
        .pointer("/data/blocks")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("convert response missing data.blocks: {value}"))?;
    let roots = value
        .pointer("/data/first_level_block_ids")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    println!("converted_blocks={}", blocks.len());
    println!("first_level_blocks={roots}");
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for block in blocks {
        if let Some(block_type) = block.get("block_type").and_then(Value::as_i64) {
            *counts.entry(block_type).or_insert(0) += 1;
        }
    }
    print_block_counts(counts);
    print_code_language_counts(blocks);
    Ok(())
}

pub(in crate::app) fn print_generated_blocks(raw_json: bool, blocks: &[Value]) -> Result<()> {
    if raw_json {
        println!("{}", serde_json::to_string_pretty(blocks)?);
        return Ok(());
    }

    println!("generated_blocks={}", blocks.len());
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for block in blocks {
        if let Some(block_type) = block.get("block_type").and_then(Value::as_i64) {
            *counts.entry(block_type).or_insert(0) += 1;
        }
    }
    let mut pairs = counts.into_iter().collect::<Vec<_>>();
    print_block_count_pairs(&mut pairs);
    print_code_language_counts(blocks);
    Ok(())
}

pub(in crate::app) fn print_block_counts(counts: HashMap<i64, usize>) {
    let mut pairs = counts.into_iter().collect::<Vec<_>>();
    print_block_count_pairs(&mut pairs);
}

pub(in crate::app) fn print_block_count_pairs(pairs: &mut [(i64, usize)]) {
    pairs.sort_by_key(|(block_type, _)| *block_type);
    for (block_type, count) in pairs.iter() {
        println!(
            "block_type_{}_{}={}",
            block_type,
            block_type_label(*block_type),
            count
        );
    }
}

pub(in crate::app) fn print_code_language_counts(blocks: &[Value]) {
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for block in blocks {
        if let Some(language) = block
            .pointer("/code/style/language")
            .and_then(Value::as_i64)
        {
            *counts.entry(language).or_insert(0) += 1;
        }
    }
    let mut pairs = counts.into_iter().collect::<Vec<_>>();
    pairs.sort_by_key(|(language, _)| *language);
    for (language, count) in pairs {
        println!(
            "code_language_{}_{}={}",
            language,
            code_language_label(language),
            count
        );
    }
}
