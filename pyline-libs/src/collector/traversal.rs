use crate::{Collector, CollectorResult, FileData, errors::PyLineError};
use async_recursion::async_recursion;
use std::path::Path;
use tokio::fs;

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
    /// ```ignore
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
    ///     .exclude_dirs(["target", ".git"])?
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
        self.mapping_files(self.path()).await
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
                return if self.skip_errors() {
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
                if self.skip_errors() {
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
                        if self.skip_errors() {
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

        if dir_name.starts_with(".") && self.ignore_dot_dirs() {
            return true;
        }

        #[cfg(target_os = "linux")]
        let dirs_exclude = self
            .exclude_dirs
            .iter()
            .any(|dir| dir.eq_ignore_ascii_case(dir_name));

        #[cfg(target_os = "windows")]
        let dirs_exclude = self
            .exclude_dirs()
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
        !self.marker_files().is_empty()
            && self.marker_files().iter().any(|file_name| {
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
            .exclude_files()
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
            .map(|ext| self.extensions().iter().any(|e| e == ext))
            .unwrap_or(false)
    }
}
