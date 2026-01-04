//! Custom error types module.

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Error as IoError;

/// Error types for the PyLine code analysis tool.
///
/// Categorizes different failure scenarios that can occur during
/// file scanning, parsing, and analysis operations.
#[derive(Debug)]
pub enum PyLineError {
    /// I/O error that occurred during file operations (reading,
    /// writing, etc.).
    IOError {
        /// The underlying I/O error from the standard library.
        error: IoError,
        /// Human-readable context about what operation failed.
        description: String,
    },

    /// Error building the file tree for parsing.
    ScannerError {
        /// Description of what went wrong during file scanning.
        description: String,
    },

    /// Error parsing code files.
    ///
    /// Indicates problems during the code analysis phase, such as syntax issues
    /// or unsupported language features.
    CounterError {
        /// Description of the parsing failure.
        description: String,
    },

    /// No files available for code parsing.
    NoFilesForParse,
}

impl From<IoError> for PyLineError {
    /// Converts an [`IoError`] into a [`PyLineError::IOError`] variant,
    /// preserving the original error and its string description.
    fn from(error: IoError) -> Self {
        let err_msg = error.to_string();
        PyLineError::IOError {
            error,
            description: err_msg,
        }
    }
}

impl Error for PyLineError {}

impl Display for PyLineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError { error, description } => {
                write!(f, "IO error: {}\n{}", error, description)
            }
            Self::ScannerError { description } => {
                write!(f, "ScannerError: {}", description)
            }
            Self::CounterError { description } => {
                write!(f, "CounterError: {}", description)
            }
            Self::NoFilesForParse => {
                write!(f, "No files available for code parsing.")
            }
        }
    }
}

impl PyLineError {
    /// Creates a new scanner error (File tree analysis error) with the
    /// given description.
    ///
    /// ## Examples
    ///
    /// ```
    /// use pyline_libs::errors::PyLineError;
    ///
    /// let error = PyLineError::scanner_error("this is error!");
    /// ```
    pub fn scanner_error(description: impl Into<String>) -> PyLineError {
        Self::ScannerError {
            description: description.into(),
        }
    }

    /// Creates a new counter error (File parsing error) with
    /// the given description.
    ///
    /// ## Examples
    ///
    /// ```
    /// use pyline_libs::errors::PyLineError;
    ///
    /// let error = PyLineError::counter_error("ooops! I did it again!..");
    /// ```
    pub fn counter_error(description: impl Into<String>) -> PyLineError {
        Self::CounterError {
            description: description.into(),
        }
    }
}
