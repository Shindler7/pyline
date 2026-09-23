//! Shared types for language parser and file statistics.

use crate::parser::{
    CodeParser,
    {py::PythonParser, rust::RustParser},
};
use std::fmt::{Display, Formatter};

/// Supported source languages.
#[derive(Debug, Default, Clone)]
pub enum CodeLanguage {
    /// Python (default).
    #[default]
    Python,

    /// Rust.
    Rust,
}

impl Display for CodeLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeLanguage::Python => f.write_str("Python, https://www.python.org/"),
            CodeLanguage::Rust => f.write_str("Rust, https://rust-lang.org/"),
        }
    }
}

impl CodeLanguage {
    pub fn get_parser(&self) -> Box<dyn CodeParser> {
        match self {
            CodeLanguage::Python => Box::new(PythonParser),
            CodeLanguage::Rust => Box::new(RustParser),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ParseResults {
    /// File statistics (lines, files, code lines).
    pub stats: CodeFilesStat,

    /// Keyword frequency counts.
    pub keywords: std::collections::HashMap<String, usize>,
}

impl ParseResults {
    pub(crate) fn new() -> Self {
        ParseResults::default()
    }

    pub fn merge(&mut self, other: Self) {
        self.stats.merge(other.stats);
        for (keyword, count) in other.keywords {
            *self.keywords.entry(keyword).or_insert(0) += count;
        }
    }

    pub(crate) fn from_parse(
        lines_total: usize,
        code_lines: usize,
        keywords: std::collections::HashMap<String, usize>,
    ) -> Self {
        Self {
            stats: CodeFilesStat {
                lines_total,
                code_lines,
                num_files_total: 1,
                num_files_invalid: 0,
            },
            keywords,
        }
    }
}

impl Display for ParseResults {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stats)?;
        if !self.keywords.is_empty() {
            write!(f, "\n\nKeywords:")?;

            let mut sorted_keywords: Vec<_> = self.keywords.iter().collect();
            sorted_keywords.sort_by(|a, b| b.1.cmp(a.1));
            for (keyword, count) in sorted_keywords {
                write!(f, "\n  {} = {}", keyword, count)?;
            }
        }

        Ok(())
    }
}

/// Aggregate statistics over a set of analyzed files.
#[derive(Debug, Default, Clone)]
pub struct CodeFilesStat {
    /// Total number of files.
    pub num_files_total: usize,
    /// Number of files that could not be read or parsed.
    pub num_files_invalid: usize,
    /// Total number of lines.
    pub lines_total: usize,
    /// Number of lines that contain code.
    pub code_lines: usize,
}

impl CodeFilesStat {
    /// Adds `other` into `self`.
    pub fn merge(&mut self, other: CodeFilesStat) {
        self.num_files_total += other.num_files_total;
        self.num_files_invalid += other.num_files_invalid;
        self.lines_total += other.lines_total;
        self.code_lines += other.code_lines;
    }
}

impl Display for CodeFilesStat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Files: {}", self.num_files_total)?;
        writeln!(f, "Lines: {}", self.lines_total)?;
        write!(f, "  of which are code lines: {}", self.code_lines)?;
        if self.num_files_invalid > 0 {
            write!(f, "\nFailed to read files: {}", self.num_files_invalid)?;
        }
        Ok(())
    }
}
