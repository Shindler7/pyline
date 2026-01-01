use crate::collector::FileData;
use crate::errors::PyLineError;
use crate::impl_lang_parser;
use crate::parser::Python;
use crate::py::base::{PyKeywords, KEYWORDS};
use crate::py::py_methods::is_triple_quotes;
use crate::traits::CodeParsers;
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

impl_lang_parser!(Python);

enum PythonResult {
    Code(HashMap<PyKeywords, usize>),
    NoCode,
    InTripleQuotes(char),
    EndTripleQuotes,
}

impl Python {
    /// Parses lines from a buffered file reader and updates Python code
    /// statistics.
    ///
    /// Analyzes each line to identify code lines, comments, and Python
    /// keywords, updating the provided statistics structure accordingly.
    async fn parse_code_lines(
        cursor: BufReader<File>,
        code_stats: &mut Python,
    ) -> Result<(), PyLineError> {
        let mut triple_quotes: Option<char> = None;

        let mut lines = cursor.lines();
        while let Some(line) = lines.next_line().await? {
            code_stats.count_line();

            match Self::parse_line(&line, triple_quotes) {
                PythonResult::Code(stat) => {
                    code_stats.count_code_line();

                    for (k, v) in stat {
                        *code_stats.keywords.entry(k.to_string()).or_insert(0) += v;
                    }
                }
                PythonResult::NoCode => {}
                PythonResult::InTripleQuotes(quotes) => {
                    triple_quotes = Some(quotes);
                }
                PythonResult::EndTripleQuotes => {
                    triple_quotes = None;
                }
            };
        }

        Ok(())
    }

    /// Parse one line.
    fn parse_line(line: &str, triple_quotes: Option<char>) -> PythonResult {
        let (mut in_triple_quotes, mut quotes) = match triple_quotes {
            Some(quotes) => (true, quotes),
            None => (false, '\0'),
        };
        let mut code_map: HashMap<PyKeywords, usize> = HashMap::new();
        let mut buf_keyword = String::new();

        let mut chars = line.char_indices().peekable();
        while let Some((i, ch)) = chars.next() {
            match (in_triple_quotes, ch) {
                (false, '#') => {
                    return if code_map.is_empty() {
                        PythonResult::NoCode
                    } else {
                        PythonResult::Code(code_map)
                    };
                }

                (true | false, '\'' | '"') => {
                    if is_triple_quotes(&mut chars, &ch, i) {
                        if triple_quotes.is_some() && quotes == ch {
                            return PythonResult::EndTripleQuotes;
                        } else if triple_quotes.is_none() {
                            quotes = ch;
                            in_triple_quotes = true;
                        }
                    }
                    buf_keyword.clear();
                }

                (false, ' ' | '\u{00A0}' | '\t' | '=' | '(' | ')' | ':' | '.' | '{' | '}') => {
                    buf_keyword.clear()
                }

                (false, _) => {
                    buf_keyword.push(ch);
                    match Self::parse_keywords(&buf_keyword) {
                        Some(keywords) => {
                            *code_map.entry(keywords).or_insert(0) += 1;
                            buf_keyword.clear();
                        }
                        None => {
                            continue;
                        }
                    }
                }
                _ => continue,
            }
        }

        if in_triple_quotes {
            return PythonResult::InTripleQuotes(quotes);
        }

        PythonResult::Code(code_map)
    }

    fn parse_keywords(keyword: &str) -> Option<PyKeywords> {
        KEYWORDS.get(keyword.to_lowercase().as_str()).cloned()
    }
}
