use crate::{FileDataExt, utils::format_file_size};
use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

/// Metadata for a source code file to be processed.
///
/// Contains the file path and size information. Used throughout the parsing
/// pipeline to track files and provide detailed feedback in verbose mode.
#[derive(Debug, Default)]
pub struct FileData {
    /// Full path to the source file.
    pub path: PathBuf,

    /// File size in bytes.
    bytes: u64,
}

impl FileData {
    /// Creates a new `FileData` instance with the given path and size.
    pub fn new(path: PathBuf, bytes: u64) -> Self {
        Self { path, bytes }
    }

    /// Returns a detailed string representation suitable for verbose output.
    /// Includes both the raw byte count and a human-readable size format.
    ///
    /// Example output:
    ///
    /// ```bash
    ///  File: src/main.py
    ///  size: 2048 bytes (2.0 KB)
    /// ```
    pub fn verbose_display(&self) -> String {
        format!(
            "File: {}\n  size: {} bytes ({})\n",
            self.path.display(),
            self.bytes,
            format_file_size(self.bytes).unwrap_or("n/a".to_string())
        )
    }

    /// Returns the file size in bytes.
    pub fn size(&self) -> u64 {
        self.bytes
    }
}

impl Display for FileData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FileAnalysis ({} ({}))",
            self.path.display(),
            format_file_size(self.bytes).unwrap_or("n/a".to_string())
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
