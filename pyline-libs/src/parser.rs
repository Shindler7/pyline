//! Core infrastructure for parsing and analyzing code files.

use std::collections::HashMap;
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

/// Structure for parsing Python files.
#[derive(Debug, Default, Clone)]
pub struct Python {
    /// Data structure with statistics of analyzed files.
    pub stats: CodeFilesStat,
    /// Statistics of identified Python keywords.
    pub keywords: HashMap<String, usize>,
}
