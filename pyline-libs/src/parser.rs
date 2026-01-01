//! Core infrastructure for parsing and analyzing code files.

use crate::{define_lang_struct, display_for_lang};
use std::fmt::{Display, Formatter};

/// Data structure with statistics of analyzed files.
#[derive(Debug, Default, Clone)]
pub struct CodeFilesStat {
    /// Number of analyzed files (total).
    pub num_files_total: usize,
    /// Number of unsuitable files. Invalid files include those that could not
    /// be analyzed or those where code syntax errors were detected.
    pub num_files_not_valid: usize,
    /// Number of lines in files (total).
    pub lines_total: usize,
    /// Number of code lines.
    pub code_lines: usize,
}

impl CodeFilesStat {
    /// Merges another CodeFilesStat instance into this one, summing all fields.
    pub fn merge(&mut self, other: CodeFilesStat) {
        self.num_files_total += other.num_files_total;
        self.num_files_not_valid += other.num_files_not_valid;
        self.lines_total += other.lines_total;
        self.code_lines += other.code_lines;
    }

    /// Alternative version that borrows the other instance.
    pub fn merge_ref(&mut self, other: &CodeFilesStat) {
        self.num_files_total += other.num_files_total;
        self.num_files_not_valid += other.num_files_not_valid;
        self.lines_total += other.lines_total;
        self.code_lines += other.code_lines;
    }

    /// Consumes both instances and returns a new merged instance
    /// (functional style).
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
        if self.num_files_not_valid > 0 {
            write!(f, "\nFailed to read files: {}", self.num_files_not_valid)?;
        }
        write!(f, "")
    }
}

define_lang_struct!(Python);
define_lang_struct!(Rust);
