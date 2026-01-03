use crate::collector::FileData;
use crate::errors::PyLineError;
use crate::impl_lang_parser;
use crate::parser::Rust;
use crate::rust::base::{RUST_KEYWORDS, RustKeywords};
use crate::traits::CodeParsers;
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

impl_lang_parser!(Rust);

enum RustResult {
    Code(HashMap<RustKeywords, usize>),
    NoCode,
    InBlockComment,
    EndBlockComment,
}

impl Rust {
    /// Parses lines from a buffered file reader and updates Rust code
    /// statistics.
    ///
    /// Analyzes each line to identify code lines, comments, and Rust
    /// keywords, updating the provided statistics structure accordingly.
    pub async fn parse_code_lines(
        cursor: BufReader<File>,
        code_stats: &mut Rust,
    ) -> Result<(), PyLineError> {
        let mut in_block_comment = false;

        let mut lines = cursor.lines();
        while let Some(line) = lines.next_line().await? {
            code_stats.count_line();

            match Self::parse_line(&line, in_block_comment) {
                RustResult::Code(stat) => {
                    code_stats.count_code_line();

                    for (k, v) in stat {
                        *code_stats.keywords.entry(k.to_string()).or_insert(0) += v;
                    }
                }
                RustResult::NoCode => {}
                RustResult::InBlockComment => {
                    in_block_comment = true;
                }
                RustResult::EndBlockComment => {
                    in_block_comment = false;
                }
            }
        }

        Ok(())
    }

    fn parse_line(line: &str, in_block_comment: bool) -> RustResult {
        let mut code_map: HashMap<RustKeywords, usize> = HashMap::new();
        let mut buf_keyword = String::new();
        let mut chars = line.char_indices().peekable();

        while let Some((_, ch)) = chars.next() {
            if in_block_comment {
                if ch == '*'
                    && let Some((_, next)) = chars.peek()
                    && *next == '/'
                {
                    // End of block comment
                    chars.next(); // skip '/'
                    return RustResult::EndBlockComment;
                }

                continue;
            }

            match ch {
                '/' => {
                    if let Some((_, next)) = chars.peek() {
                        match *next {
                            '/' => {
                                // Single-line comment
                                return if code_map.is_empty() {
                                    RustResult::NoCode
                                } else {
                                    RustResult::Code(code_map)
                                };
                            }
                            '*' => {
                                // Start block comment
                                chars.next(); // skip '*'
                                if code_map.is_empty() {
                                    return RustResult::InBlockComment;
                                } else {
                                    return RustResult::Code(code_map);
                                }
                            }
                            _ => {
                                buf_keyword.push(ch);
                            }
                        }
                    } else {
                        buf_keyword.push(ch);
                    }
                }

                ' ' | '\t' | '\u{00A0}' | '(' | ')' | '{' | '}' | ';' | ':' | ',' | '.' => {
                    buf_keyword.clear();
                }

                '"' | '\'' | 'r' => {
                    // Строковые литералы, пропускаем до конца
                    Self::consume_string_literal(ch, &mut chars);
                    buf_keyword.clear();
                }

                _ if ch.is_alphanumeric() || ch == '_' => {
                    buf_keyword.push(ch);
                    if let Some(keyword) = Self::parse_keywords(&buf_keyword) {
                        *code_map.entry(keyword).or_insert(0) += 1;
                        buf_keyword.clear();
                    }
                }

                _ => {
                    buf_keyword.clear();
                }
            }
        }

        if in_block_comment {
            RustResult::InBlockComment
        } else {
            RustResult::Code(code_map)
        }
    }

    /// Skip a string literal (regular or raw string)
    fn consume_string_literal(
        first_char: char,
        chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>,
    ) {
        if first_char == 'r' {
            // Handle raw string with possible multiple "#"
            let mut hash_count = 0;
            while let Some((_, '#')) = chars.peek() {
                chars.next();
                hash_count += 1;
            }

            if let Some((_, '"')) = chars.peek() {
                chars.next(); // skip opening quote

                let mut closing = String::from("\"");
                closing.push_str(&"#".repeat(hash_count));

                let mut literal = String::new();
                for (_, c) in chars.by_ref() {
                    literal.push(c);
                    if literal.ends_with(&closing) {
                        break;
                    }
                }
            }
        } else if first_char == '"' || first_char == '\'' {
            let quote = first_char;
            let mut escaped = false;
            for (_, c) in chars.by_ref() {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == quote {
                    break;
                }
            }
        }
    }

    fn parse_keywords(word: &str) -> Option<RustKeywords> {
        RUST_KEYWORDS.get(word).cloned()
    }
}
