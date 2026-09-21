//! Error types for `pyline-libs`.

use std::io::Error as IoError;
use thiserror::Error as ThisError;
use tokio::sync::mpsc::error::SendError;

/// Errors that can occur during file scanning, parsing, and analysis.
#[derive(Debug, ThisError)]
pub enum PyLineError {
    /// An I/O error from the standard library.
    #[error("{0}")]
    IOError(#[from] IoError),

    /// An error during file collection.
    #[error("{description}")]
    ScannerError {
        /// Human-readable description.
        description: String,
    },

    /// An error during source parsing.
    #[error("{description}")]
    CounterError {
        /// Human-readable description.
        description: String,
    },

    /// No files were found to parse.
    #[error("No files were found to parse.")]
    NoFilesForParse,

    #[error("{0}")]
    RuntimeError(String),
}

impl<T> From<SendError<T>> for PyLineError {
    fn from(err: SendError<T>) -> Self {
        Self::RuntimeError(err.to_string())
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
