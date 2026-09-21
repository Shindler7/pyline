//! File metadata types shared between the collector and the parser.

use crate::utils::format_file_size;
use std::{
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
};

/// Metadata for a source file: its path and size.
///
/// Passed from the collector to the parser, and used for verbose output.
#[derive(Debug, Default)]
pub struct FileData {
    /// Path to the source file.
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
            "File: {}\n  size: {}\n",
            self.path.display(),
            format_file_size(self.bytes)
        )
    }

    /// Returns the path to the source file.
    pub fn path(&self) -> &Path {
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
            "{} ({})",
            self.path.display(),
            format_file_size(self.bytes)
        )
    }
}

impl crate::collector::FileDataExt for &[FileData] {
    fn join_verbose(&self, sep: &str) -> String {
        self.iter()
            .map(FileData::verbose_display)
            .collect::<Vec<_>>()
            .join(sep)
    }
}
