use super::*;

pub(in crate::app) fn markdown_to_blocks(content: &str) -> Vec<Value> {
    let mut blocks = Vec::new();
    let mut paragraph = Vec::new();
    let mut in_code = false;
    let mut code = Vec::new();
    let mut code_language: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim_end();
        let stripped = trimmed.trim_start();
        if stripped.starts_with("```") {
            if in_code {
                if !code.is_empty() {
                    blocks.push(code_block(&code.join("\n"), code_language.as_deref()));
                    code.clear();
                }
                code_language = None;
                in_code = false;
            } else {
                flush_paragraph(&mut blocks, &mut paragraph);
                code_language = parse_code_fence_language(stripped);
                in_code = true;
            }
            continue;
        }

        if in_code {
            code.push(trimmed.to_string());
            continue;
        }

        if trimmed.trim().is_empty() {
            flush_paragraph(&mut blocks, &mut paragraph);
            continue;
        }

        if is_divider(stripped) {
            flush_paragraph(&mut blocks, &mut paragraph);
            blocks.push(divider_block());
        } else if let Some((level, heading)) = parse_heading(stripped) {
            flush_paragraph(&mut blocks, &mut paragraph);
            let level = level.clamp(1, 9);
            let block_type = 2 + level as i64;
            let field = format!("heading{level}");
            blocks.push(text_block(block_type, &field, heading.trim()));
        } else if let Some((done, item)) = parse_todo(stripped) {
            flush_paragraph(&mut blocks, &mut paragraph);
            blocks.push(todo_block(item.trim(), done));
        } else if let Some(quote) = parse_quote(stripped) {
            flush_paragraph(&mut blocks, &mut paragraph);
            blocks.push(text_block(15, "quote", quote.trim()));
        } else if let Some(item) = parse_unordered(stripped) {
            flush_paragraph(&mut blocks, &mut paragraph);
            blocks.push(text_block(12, "bullet", item.trim()));
        } else if let Some(item) = parse_ordered(stripped) {
            flush_paragraph(&mut blocks, &mut paragraph);
            blocks.push(text_block(13, "ordered", item.trim()));
        } else {
            paragraph.push(stripped.to_string());
        }
    }

    if in_code && !code.is_empty() {
        blocks.push(code_block(&code.join("\n"), code_language.as_deref()));
    }
    flush_paragraph(&mut blocks, &mut paragraph);
    blocks
}

fn flush_paragraph(blocks: &mut Vec<Value>, paragraph: &mut Vec<String>) {
    if paragraph.is_empty() {
        return;
    }
    blocks.push(text_block(2, "text", &paragraph.join("\n")));
    paragraph.clear();
}

pub(in crate::app) fn text_block(block_type: i64, field: &str, content: &str) -> Value {
    text_block_with_style(block_type, field, content, json!({}))
}

fn text_block_with_style(block_type: i64, field: &str, content: &str, style: Value) -> Value {
    let mut block = Map::new();
    block.insert("block_type".to_string(), Value::Number(block_type.into()));
    block.insert(
        field.to_string(),
        json!({
            "elements": [{
                "text_run": {
                    "content": content,
                    "text_element_style": {}
                }
            }],
            "style": style
        }),
    );
    Value::Object(block)
}

pub(in crate::app) fn code_block(content: &str, language: Option<&str>) -> Value {
    let mut style = Map::new();
    style.insert("wrap".to_string(), Value::Bool(true));
    if let Some(language) = language.and_then(code_language_enum) {
        style.insert("language".to_string(), Value::Number(language.into()));
    }
    text_block_with_style(14, "code", content, Value::Object(style))
}

pub(in crate::app) fn todo_block(content: &str, done: bool) -> Value {
    text_block_with_style(17, "todo", content, json!({ "done": done }))
}

pub(in crate::app) fn divider_block() -> Value {
    json!({
        "block_type": 22,
        "divider": {}
    })
}

fn parse_heading(line: &str) -> Option<(usize, &str)> {
    let count = line.chars().take_while(|c| *c == '#').count();
    if count == 0 || count > 9 {
        return None;
    }
    let rest = &line[count..];
    if rest.starts_with(' ') {
        Some((count, rest))
    } else {
        None
    }
}

fn parse_quote(line: &str) -> Option<&str> {
    line.strip_prefix("> ")
        .or_else(|| line.strip_prefix(">"))
        .map(str::trim_start)
}

fn parse_todo(line: &str) -> Option<(bool, &str)> {
    for prefix in ["- [ ] ", "* [ ] ", "+ [ ] "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some((false, rest));
        }
    }
    for prefix in ["- [x] ", "- [X] ", "* [x] ", "* [X] ", "+ [x] ", "+ [X] "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some((true, rest));
        }
    }
    None
}

fn is_divider(line: &str) -> bool {
    matches!(line, "---" | "***" | "___")
}

fn parse_unordered(line: &str) -> Option<&str> {
    for prefix in ["- ", "* ", "+ "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some(rest);
        }
    }
    None
}

fn parse_ordered(line: &str) -> Option<&str> {
    let dot = line.find(". ")?;
    if dot == 0 || !line[..dot].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(&line[dot + 2..])
}

fn parse_code_fence_language(line: &str) -> Option<String> {
    let rest = line.trim_start_matches('`').trim();
    let language = rest
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .trim_matches(|c: char| c == '{' || c == '}' || c == '.')
        .trim();
    if language.is_empty() {
        None
    } else {
        Some(language.to_ascii_lowercase())
    }
}

fn code_language_enum(language: &str) -> Option<i64> {
    let normalized = language
        .trim()
        .trim_start_matches('.')
        .to_ascii_lowercase()
        .replace(['_', '-'], "");
    match normalized.as_str() {
        "text" | "txt" | "plain" | "plaintext" | "mermaid" | "mmd" => Some(1),
        "abap" => Some(2),
        "ada" => Some(3),
        "apache" => Some(4),
        "apex" => Some(5),
        "asm" | "assembly" => Some(6),
        "bash" => Some(7),
        "csharp" | "cs" => Some(8),
        "cpp" | "cplusplus" | "c++" | "cc" | "cxx" | "hpp" => Some(9),
        "c" | "h" => Some(10),
        "cobol" => Some(11),
        "css" => Some(12),
        "coffeescript" | "coffee" => Some(13),
        "d" => Some(14),
        "dart" => Some(15),
        "delphi" | "pascal" => Some(16),
        "django" => Some(17),
        "dockerfile" | "docker" => Some(18),
        "erlang" | "erl" => Some(19),
        "fortran" => Some(20),
        "foxpro" => Some(21),
        "go" | "golang" => Some(22),
        "groovy" => Some(23),
        "html" | "htm" => Some(24),
        "htmlbars" | "handlebars" | "hbs" => Some(25),
        "http" => Some(26),
        "haskell" | "hs" => Some(27),
        "json" | "jsonc" => Some(28),
        "java" => Some(29),
        "javascript" | "js" | "jsx" | "node" => Some(30),
        "julia" | "jl" => Some(31),
        "kotlin" | "kt" | "kts" => Some(32),
        "latex" | "tex" => Some(33),
        "lisp" | "clisp" | "commonlisp" | "elisp" => Some(34),
        "logo" => Some(35),
        "lua" => Some(36),
        "matlab" => Some(37),
        "makefile" | "make" => Some(38),
        "markdown" | "md" | "mdx" => Some(39),
        "nginx" => Some(40),
        "objectivec" | "objc" => Some(41),
        "openedgeabl" | "abl" => Some(42),
        "php" => Some(43),
        "perl" | "pl" => Some(44),
        "postscript" => Some(45),
        "powershell" | "pwsh" | "ps1" => Some(46),
        "prolog" => Some(47),
        "protobuf" | "proto" => Some(48),
        "python" | "py" | "python3" => Some(49),
        "r" => Some(50),
        "rpg" => Some(51),
        "ruby" | "rb" => Some(52),
        "rust" | "rs" => Some(53),
        "sas" => Some(54),
        "scss" => Some(55),
        "sql" => Some(56),
        "scala" => Some(57),
        "scheme" => Some(58),
        "scratch" => Some(59),
        "shell" | "sh" | "zsh" | "fish" => Some(60),
        "swift" => Some(61),
        "thrift" => Some(62),
        "typescript" | "ts" | "tsx" => Some(63),
        "vbscript" | "vbs" => Some(64),
        "visual" => Some(65),
        "xml" => Some(66),
        "yaml" | "yml" => Some(67),
        "cmake" => Some(68),
        "diff" | "patch" => Some(69),
        "gherkin" | "feature" | "cucumber" => Some(70),
        "graphql" | "gql" => Some(71),
        "glsl" | "opengl" => Some(72),
        "properties" | "ini" | "conf" => Some(73),
        "solidity" | "sol" => Some(74),
        "toml" => Some(75),
        _ => None,
    }
}
