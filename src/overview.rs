use anyhow::Result;
use console::{Key, Term};
use std::fs;
use std::path::Path;

pub enum ViewResult {
    Include,
    Exclude,
    Back,
}

pub fn highlight_line(line: &str, ext: &str) -> String {
    match ext {
        "rs" => highlight_c_like(
            line,
            &[
                "fn", "let", "pub", "struct", "impl", "trait", "use", "mod", "match", "if", "else",
                "for", "in", "while", "return", "const", "static", "mut", "ref", "type", "enum",
                "as", "crate", "self", "Self", "where", "async", "await",
            ],
            &[
                "String", "Option", "Result", "u32", "i32", "u64", "i64", "usize", "isize", "str",
                "bool", "char", "f32", "f64",
            ],
        ),
        "js" | "ts" | "jsx" | "tsx" => highlight_c_like(
            line,
            &[
                "const",
                "let",
                "var",
                "function",
                "class",
                "import",
                "export",
                "from",
                "default",
                "return",
                "if",
                "else",
                "for",
                "in",
                "of",
                "while",
                "do",
                "switch",
                "case",
                "break",
                "continue",
                "new",
                "this",
                "super",
                "typeof",
                "instanceof",
                "async",
                "await",
                "yield",
                "interface",
                "type",
                "public",
                "private",
                "protected",
                "readonly",
                "any",
                "unknown",
                "never",
                "void",
            ],
            &[
                "string", "number", "boolean", "symbol", "any", "Promise", "Console",
            ],
        ),
        "go" => highlight_c_like(
            line,
            &[
                "func",
                "var",
                "const",
                "import",
                "package",
                "type",
                "struct",
                "interface",
                "map",
                "chan",
                "range",
                "go",
                "select",
                "defer",
                "if",
                "else",
                "for",
                "switch",
                "case",
                "default",
                "fallthrough",
                "break",
                "continue",
                "return",
            ],
            &[
                "string", "int", "int64", "uint64", "float64", "bool", "byte", "rune", "error",
            ],
        ),
        "py" => highlight_python_like(line),
        _ => line.to_string(),
    }
}

fn highlight_c_like(line: &str, keywords: &[&str], types: &[&str]) -> String {
    let mut output = String::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // 1. Single line comment
        if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
            output.push_str("\x1b[90m"); // Gray
            output.extend(&chars[i..]);
            output.push_str("\x1b[0m");
            break;
        }

        // 2. String literal
        if chars[i] == '"' || chars[i] == '\'' || chars[i] == '`' {
            let quote = chars[i];
            output.push_str("\x1b[32m"); // Green
            output.push(quote);
            i += 1;
            let mut escaped = false;
            while i < chars.len() {
                let c = chars[i];
                output.push(c);
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            output.push_str("\x1b[0m");
            continue;
        }

        // 3. Word boundaries (keywords / types)
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let mut word = String::new();
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                word.push(chars[i]);
                i += 1;
            }

            if keywords.contains(&word.as_str()) {
                output.push_str("\x1b[35m"); // Magenta
                output.push_str(&word);
                output.push_str("\x1b[0m");
            } else if types.contains(&word.as_str()) {
                output.push_str("\x1b[36m"); // Cyan
                output.push_str(&word);
                output.push_str("\x1b[0m");
            } else {
                output.push_str(&word);
            }
            continue;
        }

        // 4. Numbers
        if chars[i].is_ascii_digit() {
            output.push_str("\x1b[33m"); // Yellow
            while i < chars.len() && chars[i].is_ascii_digit() {
                output.push(chars[i]);
                i += 1;
            }
            output.push_str("\x1b[0m");
            continue;
        }

        // 5. Normal character
        output.push(chars[i]);
        i += 1;
    }
    output
}

fn highlight_python_like(line: &str) -> String {
    let keywords = &[
        "def", "class", "return", "if", "elif", "else", "for", "while", "in", "is", "and", "or",
        "not", "import", "from", "as", "try", "except", "finally", "raise", "assert", "with",
        "pass", "break", "continue", "global", "nonlocal", "lambda", "yield", "async", "await",
        "None", "True", "False",
    ];
    let types = &[
        "str", "int", "float", "bool", "list", "dict", "set", "tuple", "object", "self",
    ];

    let mut output = String::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '#' {
            output.push_str("\x1b[90m"); // Gray
            output.extend(&chars[i..]);
            output.push_str("\x1b[0m");
            break;
        }

        if chars[i] == '"' || chars[i] == '\'' {
            let quote = chars[i];
            output.push_str("\x1b[32m"); // Green
            output.push(quote);
            i += 1;
            let mut escaped = false;
            while i < chars.len() {
                let c = chars[i];
                output.push(c);
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            output.push_str("\x1b[0m");
            continue;
        }

        if chars[i].is_alphabetic() || chars[i] == '_' {
            let mut word = String::new();
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                word.push(chars[i]);
                i += 1;
            }

            if keywords.contains(&word.as_str()) {
                output.push_str("\x1b[35m"); // Magenta
                output.push_str(&word);
                output.push_str("\x1b[0m");
            } else if types.contains(&word.as_str()) {
                output.push_str("\x1b[36m"); // Cyan
                output.push_str(&word);
                output.push_str("\x1b[0m");
            } else {
                output.push_str(&word);
            }
            continue;
        }

        if chars[i].is_ascii_digit() {
            output.push_str("\x1b[33m"); // Yellow
            while i < chars.len() && chars[i].is_ascii_digit() {
                output.push(chars[i]);
                i += 1;
            }
            output.push_str("\x1b[0m");
            continue;
        }

        output.push(chars[i]);
        i += 1;
    }
    output
}

pub fn show_viewer(path: &Path, rel_path: &str, extension: &str) -> Result<ViewResult> {
    let term = Term::stdout();
    let content_bytes = fs::read(path)?;
    let content = String::from_utf8_lossy(&content_bytes).into_owned();
    let lines: Vec<&str> = content.lines().collect();

    // Enter alternate screen, hide cursor
    term.write_str("\x1b[?1049h")?;
    term.hide_cursor()?;
    term.flush()?;

    let mut scroll_top = 0;

    let result = (|| -> Result<ViewResult> {
        loop {
            let (term_height, term_width) = term.size();
            let term_height = if term_height == 0 {
                24
            } else {
                term_height as usize
            };
            let term_width = if term_width == 0 {
                80
            } else {
                term_width as usize
            };

            // Clear screen & cursor to home
            term.write_str("\x1b[2J\x1b[H")?;

            // Header
            term.write_line(&format!(
                "\x1b[1;37;44m File: {} ({} lines) | Up/Down to scroll, Y/N to decide, O/ESC to return \x1b[0m",
                rel_path, lines.len()
            ))?;

            // Content
            let content_height = term_height.saturating_sub(3);
            let display_lines = content_height.min(lines.len().saturating_sub(scroll_top));

            for idx in 0..display_lines {
                let line_idx = scroll_top + idx;
                let original_line = lines[line_idx];

                let truncated_line = if original_line.chars().count() > term_width {
                    let mut s: String = original_line
                        .chars()
                        .take(term_width.saturating_sub(3))
                        .collect();
                    s.push_str("...");
                    s
                } else {
                    original_line.to_string()
                };

                let highlighted = highlight_line(&truncated_line, extension);
                term.write_line(&highlighted)?;
            }

            // Fill remaining blank space
            let empty_needed = content_height.saturating_sub(display_lines);
            for _ in 0..empty_needed {
                term.write_line("")?;
            }

            // Footer / Prompt
            term.write_str("\x1b[1;33mInclude this file in merge? (y/n/o to exit overview/arrows to scroll): \x1b[0m")?;
            term.flush()?;

            let key = term.read_key()?;
            match key {
                Key::ArrowUp => {
                    if scroll_top > 0 {
                        scroll_top -= 1;
                    }
                }
                Key::ArrowDown => {
                    if scroll_top + content_height < lines.len() {
                        scroll_top += 1;
                    }
                }
                Key::PageUp => {
                    if scroll_top > content_height {
                        scroll_top -= content_height;
                    } else {
                        scroll_top = 0;
                    }
                }
                Key::PageDown => {
                    if scroll_top + content_height < lines.len() {
                        scroll_top += content_height;
                    } else {
                        scroll_top = lines.len().saturating_sub(content_height);
                    }
                }
                Key::Char('y') | Key::Char('Y') => {
                    return Ok(ViewResult::Include);
                }
                Key::Char('n') | Key::Char('N') => {
                    return Ok(ViewResult::Exclude);
                }
                Key::Char('o') | Key::Char('O') | Key::Escape => {
                    return Ok(ViewResult::Back);
                }
                _ => {}
            }
        }
    })();

    // Exit alternate screen, restore cursor
    let _ = term.write_str("\x1b[?1049l");
    let _ = term.show_cursor();
    let _ = term.flush();

    result
}
