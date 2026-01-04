//! Module for selecting code files for subsequent analysis.

use crate::errors::PyLineError;
use crate::traits::FileDataExt;
use crate::utils::format_file_size;
use async_recursion::async_recursion;
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use tokio::fs;

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

/// Configuration for collecting and filtering files from a directory structure.
///
/// Used to define rules for which files and directories should be included
/// or excluded during file collection operations. All fields have sensible
/// defaults.
#[derive(Default)]
pub struct Collector {
    /// Root directory path from which to start file collection.
    path: PathBuf,

    /// List of file names that, when found, cause their parent directories
    /// to be excluded.
    ///
    /// For example, including `.gitignore` here would skip directories
    /// containing a `.gitignore` file.
    marker_files: Vec<String>,

    /// List of directory names to exclude from traversal.
    exclude_dirs: Vec<String>,

    /// List of file names to exclude from collection.
    exclude_files: Vec<String>,

    /// List of file extensions to include in collection.
    ///
    /// Only files with these extensions will be collected. For example,
    /// `vec!["py", "pyw"]` would collect only Python files. `None` means
    /// all file extensions are included.
    extensions: Vec<String>,

    /// Whether to ignore directories starting with a dot (`.`).    
    ignore_dot_dirs: bool,

    /// If `true`, access and read errors will be ignored, and the collection will be built only
    /// from accessible directories/files. Otherwise, the search will
    /// halt upon encountering any error.
    ///
    /// Default: `true`.
    skip_errors: bool,
}

impl Collector {
    /// Create an instance of the Collector struct.
    ///
    /// Required argument: `path` — the path to the top-level directory
    /// where file link collection will be performed.
    ///
    /// You can refine the search using the following extension methods:
    /// `exclude_dirs`, `exclude_files`, `extensions`.
    ///
    /// ## For example:
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use pyline_libs::collector::Collector;
    ///
    /// let path = PathBuf::from("/path");
    ///
    /// let c = Collector::new(&path)
    ///             .extensions(["py"])
    ///             .ignore_dot_dirs(false)
    ///             .exclude_dirs(["target", "node_modules"]);
    /// ```
    ///
    /// By default, the `ignore_dot_dirs` is enabled (set to true),
    /// meaning all directories starting with a dot (`.`) are ignored.
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            ignore_dot_dirs: true,
            skip_errors: true,
            ..Default::default()
        }
    }

    /// Excludes specified directories from file collection.
    ///
    /// Directories starting with '.' (dot-directories) cannot be excluded
    /// through this method. Use `ignore_dot_dirs(true)` instead to handle them.
    ///
    /// ## Arguments
    ///
    /// * `dirs` — An iterator of directory names or patterns to exclude
    ///
    /// ## Panics
    ///
    /// Panics if any directory name starts with '.', as dot-directories
    /// require special handling via the `ignore_dot_dirs` method.
    ///
    /// ## Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use pyline_libs::collector::Collector;
    ///
    /// let path = PathBuf::from("/path");
    ///
    /// Collector::new(&path)
    ///     .exclude_dirs(["node_modules", "target", "__pycache__"])
    ///     .complete();
    /// ```
    pub fn exclude_dirs<I, S>(mut self, dirs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.exclude_dirs =
            dirs.into_iter()
                .map(|s| {
                    let s_into = s.into();
                    if s_into.starts_with(".") {
                        panic!("Cannot exclude dot-directories (e.g., '.git') via `exclude_dirs` while `ignore_dot_dirs` is enabled. \
        Consider removing them from `exclude_dirs`, or disable `ignore_dot_dirs` with `.ignore_dot_dirs(false)`.");
                    }
                    s_into
                })
                .collect();
        self
    }

    /// Configures which files should trigger exclusion of their parent directories.
    ///
    /// When a file with any of the specified names is found in a directory,
    /// that entire directory (including subdirectories) will be skipped during
    /// file collection. This is useful for ignoring directories based on marker
    /// files like `.gitignore`, `.noscan`, etc.
    pub fn with_marker_files<I, S>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.marker_files = files.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Excludes specified files from collection by their names.
    ///
    /// This filter applies to exact filename matches. For pattern-based
    /// exclusion, consider implementing additional filtering logic.
    ///
    /// ## Arguments
    ///
    /// * `files` — An iterator of filenames to exclude from collection
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// use pyline_libs::collector::Collector;
    ///
    /// let path = PathBuf::from("/path");
    ///
    /// Collector::new(&path)
    ///     .exclude_files(["README.md", "LICENSE", ".gitignore"])
    ///     .complete();
    /// ```
    pub fn exclude_files<I, S>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.exclude_files = files.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Filters files by their extensions.
    ///
    /// Extensions should be provided without the leading dot (e.g., `"py"`, not `".py"`).
    /// The method automatically normalizes the input by removing any leading dots.
    ///
    /// ## Arguments
    ///
    /// * `ext` — An iterator of file extensions to include
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// use pyline_libs::collector::Collector;
    ///
    /// let path = PathBuf::from("/path");
    ///
    /// Collector::new(&path)
    ///     .extensions(["py", "rs", "toml"])  // Works with or without dots
    ///     .complete();
    /// ```
    pub fn extensions<I, S>(mut self, ext: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.extensions = ext
            .into_iter()
            .map(|s| {
                let ext = s.into();
                ext.trim_start_matches('.').to_string()
            })
            .collect();
        self
    }

    /// Controls whether directories starting with '.' should be ignored.
    ///
    /// Dot-directories (like `.git`, `.venv`, `.idea`) are typically hidden
    /// and often contain configuration or cache files rather than source code.
    ///
    /// ## Arguments
    ///
    /// * `ignore` — If `true`, all directories starting with '.' are skipped
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// use pyline_libs::collector::Collector;
    ///
    /// let path = PathBuf::from("/path");
    ///
    /// Collector::new(&path)
    ///     .ignore_dot_dirs(true)  // Skip .git, .venv, etc.
    ///     .complete();
    /// ```
    pub fn ignore_dot_dirs(mut self, ignore: bool) -> Self {
        self.ignore_dot_dirs = ignore;
        self
    }

    /// Sets whether to skip access/read errors and continue processing only accessible items.
    ///
    /// When `true` (default), errors are ignored and collection proceeds with accessible
    /// directories/files. When `false`, any error immediately halts the search.
    pub fn skip_errors(mut self, skip: bool) -> Self {
        self.skip_errors = skip;
        self
    }
}

/// Result of a file collection operation with error tracking.
///
/// Contains both successfully collected files and any errors encountered
/// during the collection process. This allows for partial success scenarios
/// where some files are processed successfully while others fail.
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

impl Collector {
    /// Finalizes the configuration and performs the file collection
    /// operation.
    ///
    /// This is an **async** method that must be awaited. It traverses
    /// the directory tree starting from the configured `path`, applying
    /// all specified filters and exclusions to collect matching files.
    ///
    /// ## Order of Operations
    ///
    /// 1. All builder methods (`exclude_dirs`, `exclude_files`, `extensions`,
    ///    etc.) must be called **before** `complete()`.
    /// 2. `complete()` consumes the builder and returns a fully
    ///    populated `Collector`.
    /// 3. The collected files are available in the `files` field.
    ///
    /// ## Returns
    /// - `Ok(CollectorResult)` with collected files and errors (if
    ///   `skip_errors` is enabled)
    /// - `Err(PyLineError)` if `skip_errors` is `false` and an error occurs
    ///
    /// ## Async Behavior
    ///
    /// The method uses async I/O operations.
    ///
    /// ## Panics
    ///
    /// This method does not panic under normal circumstances. All expected
    /// error conditions are captured in the `Result` type.
    ///
    /// ## Example: Basic Usage
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// use pyline_libs::collector::Collector;
    /// use pyline_libs::errors::PyLineError;
    ///
    /// # async fn example() -> Result<(), PyLineError> {
    ///
    /// let path = PathBuf::from("/path");
    ///
    /// let collector = Collector::new(&path)
    ///     .extensions(["rs", "toml"])
    ///     .exclude_dirs(["target", ".git"])
    ///     .complete()
    ///     .await?;
    ///
    /// println!("Found {} Rust files", collector.num_files());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ## Notes
    ///
    /// - The operation respects all filters configured via builder methods
    /// - By default, dot-directories (starting with `.`) are excluded
    /// - File collection is recursive unless filtered by `exclude_dirs`
    /// - Symbolic links are followed according to platform behavior
    /// - The method has internal parallelism optimizations for large scans
    pub async fn complete(&self) -> Result<CollectorResult, PyLineError> {
        // Parsing...
        self.mapping_files(&self.path).await
    }

    /// Recursively collects files matching the configured criteria.
    ///
    /// Traverses directories depth-first, applying all configured filters and exclusions.
    /// Returns a [`CollectorResult`] containing both successfully collected files
    /// and any encountered errors (depending on the `skip_errors` setting).
    #[async_recursion]
    async fn mapping_files(&self, path: &Path) -> Result<CollectorResult, PyLineError> {
        // let mut files: Vec<FileData> = Vec::new();
        let mut collector_result = CollectorResult::new();

        // Ok or skip_errors?
        let mut dir_entries = match fs::read_dir(path).await {
            Ok(entries) => entries,
            Err(err) => {
                return if self.skip_errors {
                    collector_result.add_err(err.into());
                    Ok(collector_result)
                } else {
                    Err(err.into())
                };
            }
        };

        'collect: while let Some(entry_res) = match dir_entries.next_entry().await {
            Ok(entry) => entry,
            Err(err) => {
                if self.skip_errors {
                    collector_result.add_err(err.into());
                    continue 'collect;
                } else {
                    return Err(err.into());
                }
            }
        } {
            let elem = entry_res.path();
            let metadata = entry_res.metadata().await?;

            if self.is_valid_dir(&elem) {
                // Subfolders
                match self.mapping_files(&elem).await {
                    Ok(sub_dirs) => collector_result.absorb(sub_dirs),
                    Err(err) => {
                        if self.skip_errors {
                            collector_result.add_err(err);
                        } else {
                            return Err(err);
                        }
                    }
                }
            } else if self.is_valid_file(&elem) {
                let file_data = FileData::new(elem, metadata.len());
                collector_result.add_file(file_data);
            }
        }

        Ok(collector_result)
    }

    fn is_valid_dir(&self, path: &Path) -> bool {
        path.is_dir() && !self.is_dir_excluded(path)
    }

    fn is_dir_excluded(&self, path: &Path) -> bool {
        let dir_name = match path.file_name().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => return false,
        };

        if dir_name.starts_with(".") && self.ignore_dot_dirs {
            return true;
        }

        #[cfg(target_os = "linux")]
        let dirs_exclude = self
            .exclude_dirs
            .iter()
            .any(|dir| dir.eq_ignore_ascii_case(dir_name));

        #[cfg(target_os = "windows")]
        let dirs_exclude = self
            .exclude_dirs
            .iter()
            .any(|dir| dir.eq_ignore_ascii_case(dir_name));

        // Exclude by marker files.
        dirs_exclude || self.should_exclude_dir_by_marker_file(path)
    }

    /// Checks if a directory contains any marker files that warrant exclusion.
    ///
    /// Returns `true` if the directory contains any file specified in `marker_files`.
    /// When a marker file is found, the entire directory tree is skipped.
    fn should_exclude_dir_by_marker_file(&self, dir_path: &Path) -> bool {
        !self.marker_files.is_empty()
            && self.marker_files.iter().any(|file_name| {
                let marker_path = dir_path.join(file_name);
                marker_path.exists() && marker_path.is_file()
            })
    }

    fn is_valid_file(&self, file: &Path) -> bool {
        file.is_file() && self.is_valid_extension(file) && !self.is_file_excluded(file)
    }

    fn is_file_excluded(&self, file: &Path) -> bool {
        file.file_name()
            .and_then(|name| name.to_str())
            .map(|name| self.is_excluded_contains_this(name))
            .unwrap_or(false)
    }

    fn is_excluded_contains_this(&self, file_name: &str) -> bool {
        #[cfg(target_os = "windows")]
        return self
            .exclude_files
            .iter()
            .any(|excluded| excluded.eq_ignore_ascii_case(file_name));

        #[cfg(not(target_os = "windows"))]
        self.exclude_files
            .iter()
            .any(|excluded| excluded.eq(file_name))
    }

    fn is_valid_extension(&self, file: &Path) -> bool {
        file.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.extensions.iter().any(|e| e == ext))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn file_data_display_format() {
        let file = FileData::new(PathBuf::from("main.rs"), 100);
        let text = format!("{}", file);
        assert!(text.contains("main.rs"));
        assert!(text.contains("100"));
    }

    #[test]
    fn verbose_display_contains_filename_and_size() {
        let file = FileData::new(PathBuf::from("test.py"), 1024);
        let v = file.verbose_display();
        assert!(v.contains("File:"));
        assert!(v.contains("1024"));
    }

    #[test]
    fn remove_dot_from_extensions() {
        let c = Collector::new(Path::new("/some/path")).extensions([".py", "rs", ".toml"]);
        assert_eq!(c.extensions, vec!["py", "rs", "toml"]);
    }
}
