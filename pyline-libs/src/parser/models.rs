//! Shared types for language parser and file statistics.

use crate::parser::{
    CodeParser,
    {py::PythonParser, rust::RustParser},
};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

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
    /// Returns a parser instance for this language.
    pub fn get_parser(&self) -> Box<dyn CodeParser> {
        match self {
            CodeLanguage::Python => Box::new(PythonParser),
            CodeLanguage::Rust => Box::new(RustParser),
        }
    }
}

/// Aggregated parse results: file statistics and keyword counts.
#[derive(Debug, Default, Clone)]
pub struct ParseResults {
    /// File statistics (lines, files, code lines).
    pub stats: CodeFilesStats,

    /// Keyword frequency counts.
    pub keywords: HashMap<String, usize>,
}

impl ParseResults {
    /// Creates an empty result.
    pub(crate) fn new() -> Self {
        ParseResults::default()
    }

    /// Merges `other` into `self`.
    pub(crate) fn merge(&mut self, other: Self) {
        self.stats.merge(&other.stats);
        for (keyword, count) in other.keywords {
            *self.keywords.entry(keyword).or_insert(0) += count;
        }
    }

    /// Builds results for a single parsed file.
    pub(crate) fn from_file(
        total_lines: usize,
        code_lines: usize,
        keywords: HashMap<String, usize>,
    ) -> Self {
        Self {
            stats: CodeFilesStats {
                total_lines,
                code_lines,
                total_files: 1,
                invalid_files: 0,
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
                write!(f, "\n  {keyword} = {count}")?;
            }
        }

        Ok(())
    }
}

/// Aggregate statistics over a set of analyzed files.
#[derive(Debug, Default, Clone)]
pub struct CodeFilesStats {
    /// Total number of files.
    pub total_files: usize,
    /// Number of files that could not be read or parsed.
    pub invalid_files: usize,
    /// Total number of lines.
    pub total_lines: usize,
    /// Number of lines that contain code.
    pub code_lines: usize,
}

impl CodeFilesStats {
    /// Adds `other` into `self`.
    pub fn merge(&mut self, other: &CodeFilesStats) {
        self.total_files += other.total_files;
        self.invalid_files += other.invalid_files;
        self.total_lines += other.total_lines;
        self.code_lines += other.code_lines;
    }
}

impl Display for CodeFilesStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Files: {}", self.total_files)?;
        writeln!(f, "Lines: {}", self.total_lines)?;
        write!(f, "  of which are code lines: {}", self.code_lines)?;
        if self.invalid_files > 0 {
            write!(f, "\nFailed to read files: {}", self.invalid_files)?;
        }
        Ok(())
    }
}
