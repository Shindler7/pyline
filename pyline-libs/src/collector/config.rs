//! Configuration for file collection.
//!
//! Defines [`Collector`] — the builder that holds filters and settings
//! for traversal. The traversal itself lives in
//! [`crate::collector::traversal`].

use crate::{
    CodeLanguage,
    collector::types::{Dirs, Extensions, Files},
    errors::PyLineError,
};
use std::path::{Path, PathBuf};

/// Configuration for collecting and filtering files from a directory tree.
///
/// Defines which files and directories are included or excluded during
/// the collection. All fields have defaults.
#[derive(Debug, Default)]
pub struct Collector {
    /// Root directory path from which to start file of collection.
    path: PathBuf,

    /// Target language for parsing.
    lang: CodeLanguage,

    /// File names whose presence causes the containing directory to be
    /// excluded.
    ///
    /// For example, a directory containing `.gitignore` is skipped entirely.
    marker_files: Files,

    /// List of directory names to exclude from traversal.
    exclude_dirs: Dirs,

    /// List of file names to exclude from the collection.
    exclude_files: Files,

    /// File extensions to include in the collection.
    ///
    /// Only files with these extensions are collected, e.g. `["py", "pyw"]`
    /// for Python. The language's default extensions are always included.
    extensions: Extensions,

    /// Whether to ignore directories starting with a dot (`.`).
    ignore_dot_dirs: bool,

    /// If `true`, access and read errors are ignored, and collection
    /// continues with accessible entries; otherwise, it halts on the first error.
    ///
    /// Default: `false`.
    skip_errors: bool,
}

impl Collector {
    /// Creates a new [`Collector`] for the given root `path` and language.
    ///
    /// # Arguments
    ///
    /// * `path` — root directory to scan.
    /// * `lang` — target language.
    /// * `auto_config` — if `true`, applies the language's default marker
    ///   files, excluded directories, and excluded filenames.
    ///
    /// The returned value can be refined with [`Self::with_exclude_dirs`],
    /// [`Self::with_exclude_files`], [`Self::with_extensions`], and other
    /// `with_*` methods.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use pyline_libs::collector::Collector;
    /// use pyline_libs::CodeLanguage;
    ///
    /// let path = PathBuf::from("/path");
    ///
    /// let collector = Collector::new(&path, CodeLanguage::Python, false)
    ///     .with_extensions(["py"])
    ///     .with_ignore_dot_dirs(false).unwrap()
    ///     .with_exclude_dirs(["target", "node_modules"]).unwrap();
    /// ```
    ///
    /// By default, `ignore_dot_dirs` is enabled (`true`): all directories
    /// starting with a dot are ignored.
    pub fn new(path: &Path, lang: CodeLanguage, auto_config: bool) -> Self {
        let lang_defaults = lang.defaults();

        let extensions = lang_defaults.valid_extensions();

        let (marker_files, exclude_dirs, exclude_files) = if auto_config {
            (
                lang_defaults.marker_files(),
                lang_defaults.exclude_dirs(),
                lang_defaults.exclude_filenames(),
            )
        } else {
            (Files::default(), Dirs::default(), Files::default())
        };

        Self {
            path: path.to_path_buf(),
            lang,
            extensions,
            marker_files,
            exclude_dirs,
            exclude_files,
            ignore_dot_dirs: true,
            ..Default::default()
        }
    }

    /// Excludes the given directories from the collection.
    ///
    /// Dot directories cannot be excluded this way. To exclude them, use
    /// [`Self::with_ignore_dot_dirs`] instead.
    ///
    /// # Arguments
    ///
    /// * `dirs` — directory names to exclude.
    ///
    /// # Errors
    ///
    /// Returns an error if any name starts with `.` while `ignore_dot_dirs`
    /// is enabled, since dot-directories are handled by that flag.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// # use pyline_libs::collector::Collector;
    /// # use pyline_libs::CodeLanguage;
    /// let path = PathBuf::from("/path");
    ///
    /// let collector = Collector::new(&path, CodeLanguage::Python, false)
    ///     .with_exclude_dirs(["node_modules", "target"])?;
    /// # Ok::<(), pyline_libs::errors::PyLineError>(())
    /// ```
    pub fn with_exclude_dirs<I, S>(mut self, dirs: I) -> Result<Self, PyLineError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let exclude_dirs: Dirs = dirs.into_iter().collect();

        self.validate_no_dot_dirs(Some(&exclude_dirs))?;
        self.exclude_dirs.extend(exclude_dirs.iter());

        Ok(self)
    }

    fn validate_no_dot_dirs(&self, exclude_dirs: Option<&Dirs>) -> Result<(), PyLineError> {
        let dirs = exclude_dirs.unwrap_or(&self.exclude_dirs);

        if self.ignore_dot_dirs && dirs.iter().any(|s| s.starts_with('.')) {
            return Err(PyLineError::scanner_error(
                "Cannot exclude dot-directories (e.g., '.git') \
                    via `exclude_dirs` while `ignore_dot_dirs` is enabled. \
                    Consider removing them from `exclude_dirs`, or disable \
                    `ignore_dot_dirs` with `.ignore_dot_dirs(false)`.",
            ));
        }

        Ok(())
    }

    /// Sets marker files that exclude their parent directory.
    ///
    /// If a directory contains any of these files, the directory and all its
    /// subdirectories are skipped during the collection. Useful for ignoring
    /// directories marked by `.gitignore`, `.noscan`, and similar files.
    #[must_use]
    pub fn with_marker_files<I, S>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.marker_files.extend(files);
        self
    }

    /// Excludes files by name.
    ///
    /// Only exact filename matches are filtered; glob patterns are not
    /// supported.
    ///
    /// # Arguments
    ///
    /// * `files` — filenames to exclude.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// # use pyline_libs::collector::Collector;
    /// # use pyline_libs::CodeLanguage;
    /// let path = PathBuf::from("/path");
    ///
    /// let collector = Collector::new(&path, CodeLanguage::Python, false)
    ///     .with_exclude_files(["README.md", "LICENSE", ".gitignore"]);
    /// # Ok::<(), pyline_libs::errors::PyLineError>(())
    /// ```
    #[must_use]
    pub fn with_exclude_files<I, S>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.exclude_files.extend(files);
        self
    }

    /// Adds file extensions to include in the collection.
    ///
    /// Leading dots are stripped: `"py"` and `".py"` are equivalent.
    ///
    /// # Arguments
    ///
    /// * `ext` — extensions to include.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// # use pyline_libs::collector::Collector;
    /// # use pyline_libs::CodeLanguage;
    /// let path = PathBuf::from("/path");
    ///
    /// let collector = Collector::new(&path, CodeLanguage::Rust, false)
    ///     .with_extensions(["rs", ".toml"]);
    /// # Ok::<(), pyline_libs::errors::PyLineError>(())
    /// ```
    #[must_use]
    pub fn with_extensions<I, S>(mut self, ext: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.extensions.extend(ext);
        self
    }

    /// Sets whether dot-directories are ignored.
    ///
    /// Dot directories (`.git`, `.venv`, `.idea`) usually contain
    /// configuration or cache files rather than source code.
    ///
    /// # Arguments
    ///
    /// * `ignore` — if `true`, directories starting with `.` are skipped.
    ///
    /// # Errors
    ///
    /// Returns an error if `ignore` is `true` and dot-directories are already
    /// listed in `exclude_dirs` (see [`Self::with_exclude_dirs`]).
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// # use pyline_libs::collector::Collector;
    /// # use pyline_libs::CodeLanguage;
    /// let path = PathBuf::from("/path");
    ///
    /// let collector = Collector::new(&path, CodeLanguage::Python, false)
    ///     .with_ignore_dot_dirs(true)?;
    /// # Ok::<(), pyline_libs::errors::PyLineError>(())
    /// ```
    pub fn with_ignore_dot_dirs(mut self, ignore: bool) -> Result<Self, PyLineError> {
        if ignore {
            self.validate_no_dot_dirs(None)?;
        }
        self.ignore_dot_dirs = ignore;
        Ok(self)
    }

    /// Sets whether access and read errors are skipped.
    ///
    /// When `true`, the collection continues with accessible entries;
    /// when `false` (default), it halts on the first error.
    #[must_use]
    pub fn with_skip_errors(mut self, skip: bool) -> Self {
        self.skip_errors = skip;
        self
    }
}

impl Collector {
    /// Returns the root directory path from which file the collection starts.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the target code language.
    pub fn lang(&self) -> &CodeLanguage {
        &self.lang
    }

    /// Returns the marker files whose presence excludes parent directories.
    pub fn marker_files(&self) -> &Files {
        &self.marker_files
    }

    /// Returns the directory names excluded from traversal.
    pub fn exclude_dirs(&self) -> &Dirs {
        &self.exclude_dirs
    }

    /// Returns the file names excluded from the collection.
    pub fn exclude_files(&self) -> &Files {
        &self.exclude_files
    }

    /// Returns the file extensions included in the collection.
    pub fn extensions(&self) -> &Extensions {
        &self.extensions
    }

    /// Returns whether directories starting with a dot are ignored.
    pub fn ignore_dot_dirs(&self) -> bool {
        self.ignore_dot_dirs
    }

    /// Returns whether access and read errors are skipped during traversal.
    pub fn skip_errors(&self) -> bool {
        self.skip_errors
    }
}
