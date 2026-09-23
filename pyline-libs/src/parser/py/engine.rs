//! Python line-by-line parsing logic.

use crate::{
    errors::PyLineError,
    parser::{
        py::base::{KEYWORDS, PyKeywords},
        {CodeParser, ParseResults},
    },
};

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct PythonParser;

impl CodeParser for PythonParser {
    fn new() -> Self {
        Self
    }

    fn parse_code_lines(&self, cursor: &mut BufReader<File>) -> Result<ParseResults, PyLineError> {
        let mut lines_totals = 0;
        let mut code_lines = 0;

        let mut triple_quotes: Option<char> = None;
        let mut buf = String::new();

        let mut final_keywords: HashMap<String, usize> = HashMap::new();

        while cursor.read_line(&mut buf)? > 0 {
            lines_totals += 1;

            let line = buf.trim_end_matches(['\r', '\n']);
            let parsed = parse_line(line, triple_quotes);
            triple_quotes = parsed.triple_quotes;

            if parsed.is_code {
                code_lines += 1;

                for (k, v) in parsed.keywords {
                    *final_keywords.entry(k.to_string()).or_insert(0) += v;
                }
            }

            buf.clear();
        }

        Ok(ParseResults::from_parse(
            lines_totals,
            code_lines,
            final_keywords,
        ))
    }
}

/// Outcome of parsing a single line.
struct ParsedLine {
    /// Keywords found on the line, with their counts.
    keywords: HashMap<PyKeywords, usize>,

    /// Whether the line contains code (not a comment or blank).
    is_code: bool,

    /// Open triple-quote state carried to the next line, if any.
    triple_quotes: Option<char>,
}

/// Parses a single line, tracking triple-quote state across lines.
fn parse_line(line: &str, mut triple_quotes: Option<char>) -> ParsedLine {
    let mut code_map: HashMap<PyKeywords, usize> = HashMap::new();
    let mut token_start: Option<usize> = None;
    let mut has_code = false;

    let mut chars = line.char_indices().peekable();

    while let Some((i, ch)) = chars.next() {
        if let Some(open_q) = triple_quotes {
            if ch == open_q && is_triple_quote_at(line, i, ch) {
                chars.next();
                chars.next();
                triple_quotes = None;
            }
            continue;
        }

        match ch {
            '#' => {
                flush_keyword(line, token_start.take(), i, &mut code_map);
                break;
            }

            '\'' | '"' => {
                flush_keyword(line, token_start.take(), i, &mut code_map);

                if is_triple_quote_at(line, i, ch) {
                    chars.next();
                    chars.next();
                    triple_quotes = Some(ch);
                } else {
                    has_code = true;
                    skip_quoted_string(&mut chars, ch);
                }
            }

            _ if is_ident_char(ch) => {
                has_code = true;
                token_start.get_or_insert(i);
            }

            _ => {
                flush_keyword(line, token_start.take(), i, &mut code_map);
                if !ch.is_whitespace() {
                    has_code = true;
                }
            }
        }
    }

    flush_keyword(line, token_start.take(), line.len(), &mut code_map);

    ParsedLine {
        keywords: code_map,
        is_code: has_code,
        triple_quotes,
    }
}

/// Looks up `keyword` in [`KEYWORDS`].
#[inline]
fn parse_keywords(keyword: &str) -> Option<PyKeywords> {
    KEYWORDS.get(keyword).copied()
}

/// Returns `true` if `ch` can appear in an identifier.
#[inline]
fn is_ident_char(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_' || ch.is_ascii_digit()
}

/// Returns `true` if a triple quote starts at byte `idx`.
#[inline]
fn is_triple_quote_at(line: &str, idx: usize, quote: char) -> bool {
    let bytes = line.as_bytes();
    // `quote` is always `'\''` or `'"'`, both ASCII.
    let q = quote as u8;
    idx + 2 < bytes.len() && bytes[idx] == q && bytes[idx + 1] == q && bytes[idx + 2] == q
}

/// Records the token `line[start..end]` if it is a keyword.
fn flush_keyword(
    line: &str,
    start: Option<usize>,
    end: usize,
    out: &mut HashMap<PyKeywords, usize>,
) {
    let Some(start) = start else { return };
    if let Some(kw) = parse_keywords(&line[start..end]) {
        *out.entry(kw).or_insert(0) += 1;
    }
}

/// Advances `chars` past a quoted string delimited by `quote`.
///
/// Handles backslash escapes.
fn skip_quoted_string<I>(chars: &mut I, quote: char)
where
    I: Iterator<Item = (usize, char)>,
{
    let mut escaped = false;

    for (_, ch) in chars.by_ref() {
        if escaped {
            escaped = false;
            continue;
        }

        match ch {
            '\\' => escaped = true,
            c if c == quote => break,
            _ => {}
        }
    }
}
