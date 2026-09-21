//! Recursive directory traversal for [`Collector`].

use crate::{
    Collector, CollectorResult, FileData,
    collector::types::{CollectedErrors, CollectedFiles},
    errors::PyLineError,
};
use async_recursion::async_recursion;
use std::path::Path;
use tokio::fs;

impl Collector {
    /// Traverses the directory tree rooted at [`Collector::path`], applying
    /// all configured filters, and returns the collected files and errors.
    ///
    /// # Errors
    ///
    /// Returns an error if `skip_errors` is `false` and a directory cannot
    /// be read. When `skip_errors` is `true`, such errors are collected into
    /// [`CollectorResult`] instead.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::path::PathBuf;
    /// # use pyline_libs::collector::Collector;
    /// # use pyline_libs::CodeLanguage;
    /// # async fn example() -> Result<(), pyline_libs::errors::PyLineError> {
    /// let path = PathBuf::from("/path");
    ///
    /// let result = Collector::new(&path, CodeLanguage::Rust, true)
    ///     .with_extensions(["rs", "toml"])
    ///     .with_exclude_dirs(["target"])?
    ///     .complete()
    ///     .await?;
    ///
    /// println!("Found {} files", result.num_files());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn complete(&self) -> Result<CollectorResult, PyLineError> {
        let mut files = CollectedFiles::new();
        let mut errors = CollectedErrors::new();

        self.mapping_files(self.path(), &mut files, &mut errors)
            .await?;

        Ok(CollectorResult::from_collector(files, errors))
    }

    /// Recursively collects files under `path` that match the configured filters.
    #[async_recursion]
    async fn mapping_files(
        &self,
        path: &Path,
        files: &mut CollectedFiles,
        errors: &mut CollectedErrors,
    ) -> Result<(), PyLineError> {
        let mut dir_entries = match fs::read_dir(path).await {
            Ok(entries) => entries,
            Err(err) => {
                return if self.skip_errors() {
                    errors.push(err.into());
                    Ok(())
                } else {
                    Err(err.into())
                };
            }
        };

        'collect: while let Some(entry_res) = match dir_entries.next_entry().await {
            Ok(entry) => entry,
            Err(err) => {
                if self.skip_errors() {
                    errors.push(err.into());
                    continue 'collect;
                } else {
                    return Err(err.into());
                }
            }
        } {
            let elem = entry_res.path();
            let metadata = match entry_res.metadata().await {
                Ok(meta) => meta,
                Err(err) => {
                    if self.skip_errors() {
                        errors.push(err.into());
                        continue;
                    } else {
                        return Err(err.into());
                    }
                }
            };

            if self.is_collectable_dir(&elem) {
                // Subfolders
                if let Err(err) = self.mapping_files(&elem, files, errors).await {
                    if self.skip_errors() {
                        errors.push(err);
                    } else {
                        return Err(err);
                    }
                }
            } else if self.is_valid_file(&elem) {
                files.push(FileData::new(elem, metadata.len()));
            }
        }

        Ok(())
    }

    fn is_collectable_dir(&self, path: &Path) -> bool {
        path.is_dir() && !self.is_dir_excluded(path)
    }

    fn is_dir_excluded(&self, path: &Path) -> bool {
        let dir_name = match path.file_name().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => return false,
        };

        if self.ignore_dot_dirs() && dir_name.starts_with(".") {
            return true;
        }

        #[cfg(not(target_os = "windows"))]
        let dirs_exclude = self
            .exclude_dirs()
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

    /// Returns `true` if `dir_path` contains any of the configured marker
    /// files.
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
        self.exclude_files()
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
