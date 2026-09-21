//! Shared types for language parser and file statistics.

use crate::{define_lang_struct, display_for_lang};
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

    /// Like [`Self::merge`], but takes `other` by reference.
    pub fn merge_ref(&mut self, other: &CodeFilesStat) {
        self.num_files_total += other.num_files_total;
        self.num_files_invalid += other.num_files_invalid;
        self.lines_total += other.lines_total;
        self.code_lines += other.code_lines;
    }

    /// Returns the sum of `self` and `other`.
    pub fn combined(self, other: CodeFilesStat) -> Self {
        let mut result = self;
        result.merge(other);
        result
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

define_lang_struct!(Python);
define_lang_struct!(Rust);
