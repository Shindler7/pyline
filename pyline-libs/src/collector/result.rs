//! Result of a file collection operation, tracking both files and errors.

use crate::{
    FileData,
    collector::types::{CollectedErrors, CollectedFiles},
    errors::PyLineError,
};

/// Result of a file collection operation.
///
/// Holds successfully collected files and any errors encountered.
#[derive(Debug, Default)]
pub struct CollectorResult {
    /// Successfully collected files.
    result: CollectedFiles,

    /// Errors encountered during file collection.
    errors: CollectedErrors,
}

impl From<(CollectedFiles, CollectedErrors)> for CollectorResult {
    #[inline]
    fn from((result, errors): (CollectedFiles, CollectedErrors)) -> Self {
        Self { result, errors }
    }
}

impl CollectorResult {
    /// Returns a reference to the collected files.
    pub fn files(&self) -> &[FileData] {
        self.result.as_ref()
    }

    /// Returns a reference to the error list.
    pub fn errors(&self) -> &[PyLineError] {
        self.errors.as_ref()
    }

    /// Returns `true` if any errors occurred during collection.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns `true` if no files were collected and no errors occurred.
    pub fn is_empty(&self) -> bool {
        self.result.is_empty() && self.errors.is_empty()
    }
}
