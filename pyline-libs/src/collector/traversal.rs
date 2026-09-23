//! Recursive directory traversal for [`Collector`].

use crate::{
    Collector, CollectorResult, FileData,
    collector::types::{CollectedErrors, CollectedFiles},
    errors::PyLineError,
};
use std::path::Path;
use walkdir::WalkDir;

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
    ///     .complete()?;
    ///
    /// println!("Found {} files", result.files().len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn complete(&self) -> Result<CollectorResult, PyLineError> {
        let mut files = CollectedFiles::new();
        let mut errors = CollectedErrors::new();

        let walker = WalkDir::new(self.path()).into_iter().filter_entry(|entry| {
            if entry.file_type().is_dir() {
                !self.is_dir_excluded(entry.path())
            } else {
                true
            }
        });

        for result in walker {
            match result {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        let path = entry.path();

                        if self.is_valid_file(path) {
                            match entry.metadata() {
                                Ok(meta) => {
                                    files.push(FileData::new(path.to_path_buf(), meta.len()));
                                }
                                Err(err) => {
                                    self.handle_error(err, &mut errors)?;
                                }
                            }
                        }
                    }
                }

                Err(err) => self.handle_error(err, &mut errors)?,
            }
        }

        Ok((files, errors).into())
    }

    #[inline]
    fn handle_error(
        &self,
        err: walkdir::Error,
        errors: &mut CollectedErrors,
    ) -> Result<(), PyLineError> {
        let py_err = err.into();
        if self.skip_errors() {
            errors.push(py_err);
            Ok(())
        } else {
            Err(py_err)
        }
    }

    fn is_dir_excluded(&self, path: &Path) -> bool {
        let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) else {
            return false;
        };

        if self.ignore_dot_dirs() && dir_name.starts_with('.') {
            return true;
        }

        let dirs_exclude = self.exclude_dirs().iter().any(|dir| {
            #[cfg(target_os = "windows")]
            {
                dir.eq_ignore_ascii_case(dir_name)
            }

            #[cfg(not(target_os = "windows"))]
            {
                dir.eq_ignore_ascii_case(dir_name)
            }
        });

        if dirs_exclude {
            return true;
        }

        // Exclude by marker files.
        self.should_exclude_dir_by_marker_file(path)
    }

    /// Returns `true` if `dir_path` contains any of the configured marker files.
    fn should_exclude_dir_by_marker_file(&self, dir_path: &Path) -> bool {
        let markers = self.marker_files();
        if markers.is_empty() {
            return false;
        }

        markers.iter().any(|marker| dir_path.join(marker).is_file())
    }

    fn is_valid_file(&self, file: &Path) -> bool {
        self.is_valid_extension(file) && !self.is_file_excluded(file)
    }

    fn is_file_excluded(&self, file: &Path) -> bool {
        file.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| self.is_excluded_contains_this(name))
    }

    fn is_excluded_contains_this(&self, file_name: &str) -> bool {
        self.exclude_files().iter().any(|excluded| {
            #[cfg(target_os = "windows")]
            {
                excluded.eq_ignore_ascii_case(file_name)
            }

            #[cfg(not(target_os = "windows"))]
            {
                excluded.eq(file_name)
            }
        })
    }

    fn is_valid_extension(&self, file: &Path) -> bool {
        file.extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| self.extensions().iter().any(|e| e == ext))
    }
}
