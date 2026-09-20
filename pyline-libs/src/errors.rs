//! Error types for `pyline-libs`.

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Error as IoError;

/// Errors that can occur during file scanning, parsing, and analysis.
#[derive(Debug)]
pub enum PyLineError {
    /// An I/O error from the standard library.
    IOError {
        /// The underlying error.
        error: IoError,
        /// String representation of the error.
        description: String,
    },

    /// An error during file collection.
    ScannerError {
        /// Human-readable description.
        description: String,
    },

    /// An error during source parsing.
    CounterError {
        /// Human-readable description.
        description: String,
    },

    /// No files were found to parse.
    NoFilesForParse,
}

impl From<IoError> for PyLineError {
    fn from(error: IoError) -> Self {
        let err_msg = error.to_string();
        PyLineError::IOError {
            error,
            description: err_msg,
        }
    }
}

impl Error for PyLineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IOError { error, .. } => Some(error),
            _ => None,
        }
    }
}

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
    /// Creates a [`PyLineError::ScannerError`] with the given description.
    ///
    /// # Examples
    ///
    /// ```
    /// use pyline_libs::errors::PyLineError;
    ///
    /// let err = PyLineError::scanner_error("failed to read directory");
    /// ```
    pub fn scanner_error(description: impl Into<String>) -> PyLineError {
        Self::ScannerError {
            description: description.into(),
        }
    }

    /// Creates a [`PyLineError::CounterError`] with the given description.
    ///
    /// # Examples
    ///
    /// ```
    /// use pyline_libs::errors::PyLineError;
    ///
    /// let err = PyLineError::counter_error("unexpected token");
    /// ```
    pub fn counter_error(description: impl Into<String>) -> PyLineError {
        Self::CounterError {
            description: description.into(),
        }
    }
}
