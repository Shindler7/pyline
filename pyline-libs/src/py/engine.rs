use crate::collector::FileData;
use crate::errors::PyLineError;
use crate::parser::Python;
use crate::py::base::{KEYWORDS, PyKeywords};
use crate::py::py_methods::is_triple_quotes;
use crate::traits::CodeParsers;
use futures::future::join_all;
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

impl CodeParsers for Python {
    type Code = Python;

    fn new() -> Python {
        Self {
            ..Default::default()
        }
    }

    fn new_one() -> Python {
        let mut code_stat = Self::new();
        code_stat.count_file();
        code_stat
    }

    /// Merges another Python instance into this one, summing all fields.
    fn merge(&mut self, other: Python) {
        self.stats.merge(other.stats);

        for (keyword, count) in other.keywords {
            *self.keywords.entry(keyword).or_insert(0) += count;
        }
    }

    /// Alternative version that borrows the other instance.
    fn merge_ref(&mut self, other: &Python) {
        self.stats.merge_ref(&other.stats);

        for (keyword, count) in &other.keywords {
            *self.keywords.entry(keyword.clone()).or_insert(0) += count;
        }
    }

    /// Consumes both instances and returns a new merged instance
    /// (functional style).
    fn combined(self, other: Python) -> Python {
        let mut result = self;
        result.merge(other);
        result
    }

    async fn parse(&mut self, files: Vec<FileData>) -> Result<Python, PyLineError> {
        if files.is_empty() {
            return Err(PyLineError::NoFilesForParse);
        }

        self.parse_collector(&files).await?;
        Ok(self.clone())
    }

    fn count_file(&mut self) {
        self.stats.num_files_total += 1;
    }

    fn count_invalid_file(&mut self) {
        self.stats.num_files_not_valid += 1;
    }

    fn count_line(&mut self) {
        self.stats.lines_total += 1;
    }

    fn count_code_line(&mut self) {
        self.stats.code_lines += 1;
    }
}

enum PythonResult {
    Code(HashMap<PyKeywords, usize>),
    NoCode,
    InTripleQuotes(char),
    EndTripleQuotes,
}

impl Python {
    /// Asynchronously parses a collection of files and aggregates their
    /// statistics.
    ///
    /// Processes files in parallel using tasks, updates statistics for
    /// successfully parsed files, and counts invalid files separately.
    async fn parse_collector(&mut self, files: &[FileData]) -> Result<(), PyLineError> {
        let tasks: Vec<_> = files.iter().map(Self::parse_file).collect();
        let results = join_all(tasks).await;

        for result in results {
            match result {
                Ok(result) => {
                    self.merge(result);
                }
                Err(_) => {
                    self.count_invalid_file();
                }
            }
        }

        Ok(())
    }

    /// Asynchronously parses a single Python file and extracts code
    /// statistics.
    ///
    /// Opens the file, reads it line by line, and analyzes Python code
    /// patterns.
    async fn parse_file(file: &FileData) -> Result<Self, PyLineError> {
        let mut code_stats = Self::new_one();

        let code_file = File::open(&file.path).await?;
        let cursor = BufReader::new(code_file);
        Self::parse_code_lines(cursor, &mut code_stats).await?;

        Ok(code_stats)
    }

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

                (false, ' ' | '\u{00A0}' | '\t') => buf_keyword.clear(),

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
