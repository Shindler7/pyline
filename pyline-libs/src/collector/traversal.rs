//! Recursive directory traversal for [`Collector`].

use crate::{
    Collector, CollectorResult, FileData,
    collector::types::{CollectedErrors, CollectedFiles},
    errors::PyLineError,
};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

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
    ///     .with_exclude_dirs(["target"])
    ///     .collect()?;
    ///
    /// println!("Found {} files", result.files().len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn collect(&self) -> Result<CollectorResult, PyLineError> {
        // Validators.
        self.validate_no_dot_dirs()?;

        // Executing.
        let mut files = CollectedFiles::new();
        let mut errors = CollectedErrors::new();

        let walker = WalkDir::new(self.path()).into_iter().filter_entry(|entry| {
            if entry.file_type().is_dir() {
                !self.is_dir_excluded(entry)
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

    fn validate_no_dot_dirs(&self) -> Result<(), PyLineError> {
        if self.ignore_dot_dirs() && self.exclude_dirs().iter().any(|s| s.starts_with('.')) {
            return Err(PyLineError::scanner_error(
                "Cannot exclude dot-directories (e.g., '.git') \
                    via `exclude_dirs` while `ignore_dot_dirs` is enabled. \
                    Consider removing them from `exclude_dirs`, or disable \
                    `ignore_dot_dirs` with `.ignore_dot_dirs(false)`.",
            ));
        }

        Ok(())
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

    fn is_dir_excluded(&self, dir_entry: &DirEntry) -> bool {
        let Some(dir_name) = dir_entry.file_name().to_str() else {
            return false;
        };

        if dir_entry.depth() > 0 && self.ignore_dot_dirs() && dir_name.starts_with('.') {
            return true;
        }

        if contains_this(self.exclude_dirs().iter(), dir_name) {
            return true;
        }

        self.should_exclude_dir_by_marker_file(dir_entry)
    }

    /// Returns `true` if `dir_path` contains any of the configured marker files.
    fn should_exclude_dir_by_marker_file(&self, dir_entry: &DirEntry) -> bool {
        let mut path = dir_entry.path().to_path_buf();
        self.marker_files().iter().any(|marker_file| {
            path.push(marker_file);
            let exists = path.exists();
            path.pop();
            exists
        })
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
        contains_this(self.exclude_files().iter(), file_name)
    }

    fn is_valid_extension(&self, file: &Path) -> bool {
        file.extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| {
                self.extensions()
                    .iter()
                    .any(|e| e.eq_ignore_ascii_case(ext))
            })
    }
}

/// Returns `true` if `elem` matches any item in `collection`.
///
/// Comparison is case-insensitive on Windows and exact elsewhere.
#[inline]
fn contains_this<I, S>(collection: I, elem: &str) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    collection.into_iter().any(|excluded| {
        let s = excluded.as_ref();
        if cfg!(target_os = "windows") {
            s.eq_ignore_ascii_case(elem)
        } else {
            s.eq(elem)
        }
    })
}
