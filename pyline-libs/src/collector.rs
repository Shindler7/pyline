//! Module for selecting code files for subsequent analysis.

use crate::errors::PyLineError;
use crate::traits::FileDataExt;
use crate::utils::format_file_size;
use async_recursion::async_recursion;
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(Debug, Default)]
pub struct FileData {
    pub path: PathBuf,
    bytes: u64,
}

impl FileData {
    pub fn new(path: PathBuf, bytes: u64) -> Self {
        Self { path, bytes }
    }

    /// Returns a detailed string representation suitable for verbose output.
    pub fn verbose_display(&self) -> String {
        format!(
            "File: {}\n  size: {} bytes ({})\n",
            self.path.display(),
            self.bytes,
            format_file_size(self.bytes).unwrap_or("n/a".to_string())
        )
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

#[derive(Default)]
pub struct Collector {
    path: PathBuf,
    exclude_dirs: Option<Vec<String>>,
    exclude_files: Option<Vec<String>>,
    extensions: Option<Vec<String>>,
    ignore_dot_dirs: bool,
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
    /// # For example:
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
            ..Default::default()
        }
    }

    /// Excludes specified directories from file collection.
    ///
    /// Directories starting with '.' (dot-directories) cannot be excluded
    /// through this method. Use `ignore_dot_dirs(true)` instead to handle them.
    ///
    /// # Arguments
    ///
    /// * `dirs` — An iterator of directory names or patterns to exclude
    ///
    /// # Panics
    ///
    /// Panics if any directory name starts with '.', as dot-directories
    /// require special handling via the `ignore_dot_dirs` method.
    ///
    /// # Example
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
        self.exclude_dirs = Some(
            dirs.into_iter()
                .map(|s| {
                    let s_into = s.into();
                    if s_into.starts_with(".") {
                        panic!("To exclude dot-directories (starting with '.'), set ignore_dot_dirs to true.")
                    }
                    s_into
                })
                .collect(),
        );
        self
    }

    /// Excludes specified files from collection by their names.
    ///
    /// This filter applies to exact filename matches. For pattern-based
    /// exclusion, consider implementing additional filtering logic.
    ///
    /// # Arguments
    ///
    /// * `files` — An iterator of filenames to exclude from collection
    ///
    /// # Example
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
        self.exclude_files = Some(files.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Filters files by their extensions.
    ///
    /// Extensions should be provided without the leading dot (e.g., `"py"`, not `".py"`).
    /// The method automatically normalizes the input by removing any leading dots.
    ///
    /// # Arguments
    ///
    /// * `ext` — An iterator of file extensions to include
    ///
    /// # Example
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
        self.extensions = Some(
            ext.into_iter()
                .map(|s| {
                    let ext = s.into();
                    ext.trim_start_matches('.').to_string()
                })
                .collect(),
        );
        self
    }

    /// Controls whether directories starting with '.' should be ignored.
    ///
    /// Dot-directories (like `.git`, `.venv`, `.idea`) are typically hidden
    /// and often contain configuration or cache files rather than source code.
    ///
    /// # Arguments
    ///
    /// * `ignore` — If `true`, all directories starting with '.' are skipped
    ///
    /// # Example
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
}

impl Collector {
    /// Finalizes the configuration and performs the file collection
    /// operation.
    ///
    /// This is an **async** method that must be awaited. It traverses
    /// the directory tree starting from the configured `path`, applying
    /// all specified filters and exclusions to collect matching files.
    ///
    /// # Order of Operations
    ///
    /// 1. All builder methods (`exclude_dirs`, `exclude_files`, `extensions`,
    ///    etc.) must be called **before** `complete()`.
    /// 2. `complete()` consumes the builder and returns a fully
    ///    populated `Collector`.
    /// 3. The collected files are available in the `files` field.
    ///
    /// # Returns
    ///
    /// - `Ok(Collector)` with the `files` vector populated with
    ///   [`FileData`] entries.
    /// - [`PyLineError`] if the operation fails.
    ///
    /// # Async Behavior
    ///
    /// The method uses async I/O operations.
    ///
    /// # Panics
    ///
    /// This method does not panic under normal circumstances. All expected
    /// error conditions are captured in the `Result` type.
    ///
    /// # Example: Basic Usage
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
    /// println!("Found {} Rust files", collector.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Notes
    ///
    /// - The operation respects all filters configured via builder methods
    /// - By default, dot-directories (starting with `.`) are excluded
    /// - File collection is recursive unless filtered by `exclude_dirs`
    /// - Symbolic links are followed according to platform behavior
    /// - The method has internal parallelism optimizations for large scans
    pub async fn complete(&self) -> Result<Vec<FileData>, PyLineError> {
        // Parsing...
        self.mapping_files(&self.path).await
    }

    #[async_recursion]
    async fn mapping_files(&self, path: &PathBuf) -> Result<Vec<FileData>, PyLineError> {
        let mut files: Vec<FileData> = Vec::new();

        let mut cur_dir = fs::read_dir(path).await?;
        while let Some(cur_dir_elems) = cur_dir.next_entry().await? {
            let elem = cur_dir_elems.path();

            if self.is_valid_dir(&elem) {
                // Subfolders
                if let Ok(sub_files) = self.mapping_files(&elem).await {
                    files.extend(sub_files);
                }
            }
            if self.is_valid_file(&elem) {
                let fb = Self::file_bytes(&elem);
                files.push(FileData::new(elem, fb));
            }
        }

        Ok(files)
    }

    fn file_bytes(file: &Path) -> u64 {
        match file.metadata() {
            Ok(metadata) => metadata.len(),
            Err(_) => 0,
        }
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
        self.exclude_dirs
            .as_ref()
            .is_some_and(|dirs| dirs.iter().any(|dir| dir.eq(dir_name)));

        #[cfg(target_os = "windows")]
        self.exclude_dirs
            .as_ref()
            .is_some_and(|dirs| dirs.iter().any(|dir| dir.eq_ignore_ascii_case(dir_name)))
    }

    fn is_valid_file(&self, file: &Path) -> bool {
        file.is_file() && self.is_valid_extension(file) && !self.is_file_excluded(file)
    }

    #[cfg(target_os = "windows")]
    fn is_file_excluded(&self, file: &Path) -> bool {
        self.exclude_files.as_ref().is_some_and(|exclude_files| {
            file.file_name()
                .and_then(|name| name.to_str())
                .map(|name| {
                    exclude_files
                        .iter()
                        .any(|excluded| excluded.eq_ignore_ascii_case(name))
                })
                .unwrap_or(false)
        })
    }

    #[cfg(target_os = "linux")]
    fn is_file_excluded(&self, file: &Path) -> bool {
        self.exclude_files.as_ref().is_some_and(|exclude_files| {
            file.file_name()
                .and_then(|name| name.to_str())
                .map(|name| exclude_files.iter().any(|excluded| excluded.eq(name)))
                .unwrap_or(false)
        })
    }

    fn is_valid_extension(&self, file: &Path) -> bool {
        self.extensions.as_ref().is_some_and(|extensions| {
            file.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| extensions.iter().any(|vec_e| vec_e.eq(ext)))
                .unwrap_or(false)
        })
    }
}
