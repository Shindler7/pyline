//! Rust line-by-line parsing logic.

use crate::{
    errors::PyLineError,
    parser::{
        CodeParser, ParseResults,
        rust::base::{RUST_KEYWORDS, RustKeywords},
    },
};

use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

/// Rust-specific parser.
pub(crate) struct RustParser;

impl CodeParser for RustParser {
    fn new() -> Self {
        Self
    }

    fn parse_code_lines(&self, cursor: &mut BufReader<File>) -> Result<ParseResults, PyLineError> {
        let mut lines_total = 0;
        let mut code_lines = 0;
        let mut in_block_comment = false;

        let mut local_keywords: HashMap<RustKeywords, usize> = HashMap::new();

        let mut line = String::new();

        loop {
            line.clear();
            let bytes_read = cursor.read_line(&mut line)?;
            if bytes_read == 0 {
                break; // EOF.
            }

            lines_total += 1;

            let has_code = process_line(&line, &mut in_block_comment, &mut local_keywords);
            if has_code {
                code_lines += 1;
            }
        }

        let final_keywords: HashMap<String, usize> = local_keywords
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        Ok(ParseResults::from_file(
            lines_total,
            code_lines,
            final_keywords,
        ))
    }
}

/// Processes a single line, updating comment state and keyword counts.
///
/// Returns `true` if the line contains any code.
fn process_line(
    line: &str,
    in_block_comment: &mut bool,
    keywords: &mut HashMap<RustKeywords, usize>,
) -> bool {
    let mut has_code = false;
    let mut iter = line.char_indices().peekable();

    while let Some((idx, ch)) = iter.next() {
        if *in_block_comment {
            if ch == '*'
                && let Some(&(_, '/')) = iter.peek()
            {
                iter.next(); // skip '/'
                *in_block_comment = false;
            }

            continue;
        }

        match ch {
            ' ' | '\t' | '\r' | '\n' | '\u{00A0}' => {}
            '/' => {
                if let Some(&(_, next_ch)) = iter.peek() {
                    if next_ch == '/' {
                        break;
                    } else if next_ch == '*' {
                        iter.next(); // skip '*'
                        *in_block_comment = true;
                        continue;
                    }
                }
                has_code = true;
            }
            '"' | '\'' => {
                has_code = true;
                skip_string(ch, &mut iter);
            }
            'r' => {
                has_code = true;
                let peek = iter.peek().map(|&(_, c)| c);
                if peek == Some('#') || peek == Some('"') {
                    skip_raw_string(&mut iter);
                } else {
                    consume_ident(line, idx, ch, &mut iter, keywords);
                }
            }
            c if c.is_alphabetic() || c == '_' => {
                has_code = true;
                consume_ident(line, idx, c, &mut iter, keywords);
            }
            _ => {
                has_code = true;
            }
        }
    }

    has_code
}

/// Consumes an identifier starting at `start_idx` and records it if it
/// is a keyword.
///
/// `first_ch` is the first character of the identifier; `iter` is
/// positioned right after it.
fn consume_ident(
    line: &str,
    start_idx: usize,
    first_ch: char,
    iter: &mut std::iter::Peekable<std::str::CharIndices<'_>>,
    keywords: &mut HashMap<RustKeywords, usize>,
) {
    let mut end_idx = start_idx + first_ch.len_utf8();

    while let Some(&(idx, ch)) = iter.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            end_idx = idx + ch.len_utf8();
            iter.next();
        } else {
            break;
        }
    }

    let word = &line[start_idx..end_idx];
    if let Some(keyword) = parse_keywords(word) {
        *keywords.entry(keyword).or_insert(0) += 1;
    }
}

/// Advances `iter` past a string literal delimited by `quote`.
///
/// Handles backslash escapes.
fn skip_string(quote: char, iter: &mut std::iter::Peekable<std::str::CharIndices<'_>>) {
    let mut escaped = false;
    for (_, ch) in iter.by_ref() {
        if escaped {
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == quote {
            break;
        }
    }
}

/// Advances `iter` past a raw string literal (`r"..."`, `r#"..."#`, …).
///
/// `iter` must be positioned at the first `#` or `"` after `r`.
fn skip_raw_string(iter: &mut std::iter::Peekable<std::str::CharIndices<'_>>) {
    let mut hashes = 0;

    while let Some(&(_, '#')) = iter.peek() {
        iter.next();
        hashes += 1;
    }

    if let Some(&(_, '"')) = iter.peek() {
        iter.next();

        let mut in_closing_sequence = false;
        let mut current_hashes = 0;

        for (_, ch) in iter.by_ref() {
            if in_closing_sequence {
                if ch == '#' {
                    current_hashes += 1;
                    if current_hashes == hashes {
                        break;
                    }
                } else if ch == '"' {
                    current_hashes = 0;
                } else {
                    in_closing_sequence = false;
                }
            } else if ch == '"' {
                in_closing_sequence = true;
                if hashes == 0 {
                    break;
                }
                current_hashes = 0;
            }
        }
    }
}

/// Looks up `word` in [`RUST_KEYWORDS`].
#[inline]
fn parse_keywords(word: &str) -> Option<RustKeywords> {
    RUST_KEYWORDS.get(word).copied()
}
