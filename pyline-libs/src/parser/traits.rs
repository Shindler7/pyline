//! Parser traits for language-specific implementations.

use crate::{collector::models::FileData, errors::PyLineError, parser::ParseResults};
use std::{fs::File, io::BufReader};

/// Language-specific code parser.
///
/// Implementors provide line-by-line parsing for a single language.
/// File opening, buffering, and per-file aggregation are handled by
/// the default [`Self::parse_file`].
pub trait CodeParser: Send + Sync {
    /// Creates an empty parser.
    fn new() -> Self
    where
        Self: Sized;

    /// Opens `file`, parses it line by line, and returns its results.
    ///
    /// # Errors
    ///
    /// Returns [`PyLineError`] if the file cannot be opened or parsed.
    fn parse_file(&self, file: &FileData) -> Result<ParseResults, PyLineError> {
        let handle = File::open(file.path())?;
        let mut cursor = BufReader::new(handle);

        self.parse_code_lines(&mut cursor)
    }

    /// Parses a single code file and extracts statistics.
    ///
    /// Opens the file, reads it line by line, and analyzes code patterns.
    fn parse_code_lines(&self, cursor: &mut BufReader<File>) -> Result<ParseResults, PyLineError>;
}
