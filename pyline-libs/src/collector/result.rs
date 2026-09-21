//! Result of a file collection operation, tracking both files and errors.

use crate::{
    FileData,
    collector::types::{CollectedErrors, CollectedFiles},
    errors::PyLineError,
};

/// Result of a file collection operation.
///
/// Holds successfully collected files and any errors encountered.
#[derive(Default)]
pub struct CollectorResult {
    /// Successfully collected files.
    result: CollectedFiles,

    /// Errors encountered during file collection.
    errors: CollectedErrors,
}

impl CollectorResult {
    /// Create instance with empty fields.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_collector(result: CollectedFiles, errors: CollectedErrors) -> Self {
        Self { result, errors }
    }

    /// Returns a reference to the collected files.
    pub fn files(&self) -> &Vec<FileData> {
        self.result.inner()
    }

    /// Returns `true` if any files were collected.
    pub fn has_files(&self) -> bool {
        !self.result.is_empty()
    }

    /// Returns the number of collected files.
    pub fn num_files(&self) -> usize {
        self.result.len()
    }

    /// Returns a reference to the error list.
    pub fn errors(&self) -> &Vec<PyLineError> {
        self.errors.inner()
    }

    /// Returns `true` if any errors occurred during collection.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns the number of errors encountered.
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }

    /// Adds a successfully collected file to the result.
    pub fn add_file(&mut self, item: FileData) {
        self.result.push(item);
    }
}
