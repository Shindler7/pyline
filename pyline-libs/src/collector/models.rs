//! File metadata types shared between the collector and the parser.

use crate::{FileDataExt, utils::format_file_size};
use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

/// Metadata for a source file: its path and size.
///
/// Passed from the collector to the parser, and used for verbose output.
#[derive(Debug, Default)]
pub struct FileData {
    /// Returns a reference to full path of the source file.
    path: PathBuf,

    /// File size in bytes.
    bytes: u64,
}

impl FileData {
    /// Creates a new [`FileData`] from the given path and size.
    pub fn new(path: PathBuf, bytes: u64) -> Self {
        Self { path, bytes }
    }

    /// Returns a human-readable, multi-line description for verbose output.
    ///
    /// The result includes both the raw byte count and a formatted size, and
    /// ends with a newline.
    ///
    /// # Example
    ///
    /// ```text
    /// File: src/main.py
    ///   size: 2048 bytes (2.0 KB)
    /// ```
    pub fn verbose_display(&self) -> String {
        format!(
            "File: {}\n  size: {} bytes ({})\n",
            self.path.display(),
            self.bytes,
            format_file_size(self.bytes)
        )
    }

    /// Full path to the source file.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns the file size in bytes.
    pub fn bytes(&self) -> u64 {
        self.bytes
    }
}

impl Display for FileData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FileAnalysis ({} ({}))",
            self.path.display(),
            format_file_size(self.bytes)
        )
    }
}

impl FileDataExt for Vec<FileData> {
    fn join_verbose(&self, sep: &str) -> String {
        self.iter()
            .map(|f| f.verbose_display())
            .collect::<Vec<_>>()
            .join(sep)
    }
}
