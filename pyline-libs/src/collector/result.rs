//! Result of a file collection operation, tracking both files and errors.

use crate::{FileData, errors::PyLineError};


/// Result of a file collection operation.
///
/// Holds successfully collected files and any errors encountered.
#[derive(Default)]
pub struct CollectorResult {
    /// Successfully collected files.
    result: Vec<FileData>,

    /// Errors encountered during file collection.
    errors: Vec<PyLineError>,
}

impl CollectorResult {
    /// Create instance with empty fields.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the collected files.
    pub fn files(&self) -> &Vec<FileData> {
        &self.result
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
        &self.errors
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

    /// Adds an error encountered during collection.
    pub fn add_err(&mut self, err: PyLineError) {
        self.errors.push(err);
    }

    /// Extends the collection with multiple successfully collected files.
    pub fn extend_results(&mut self, items: Vec<FileData>) {
        self.result.extend(items);
    }

    /// Extends the collection with multiple errors.
    pub fn extend_errors(&mut self, errs: Vec<PyLineError>) {
        self.errors.extend(errs);
    }

    /// Merges another `CollectorResult` into this one, consuming it.
    ///
    /// All files and errors from `other` are added to this result.
    pub fn absorb(&mut self, other: Self) {
        self.result.extend(other.result);
        self.errors.extend(other.errors);
    }
}
