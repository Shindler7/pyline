//! Parser traits for language-specific implementations.

use crate::{collector::models::FileData, errors::PyLineError, parser::ParseResults};
use std::{fs::File, io::BufReader};

pub trait CodeParser: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn parse_file(&self, file: &FileData) -> Result<ParseResults, PyLineError> {
        let file = File::open(file.path())?;
        let mut cursor = BufReader::new(file);

        self.parse_code_lines(&mut cursor)
    }

    /// Parses a single code file and extracts statistics.
    ///
    /// Opens the file, reads it line by line, and analyzes code patterns.
    fn parse_code_lines(&self, cursor: &mut BufReader<File>) -> Result<ParseResults, PyLineError>;
}
