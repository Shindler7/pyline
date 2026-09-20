use crate::{
    CodeLanguage,
    collector::{
        traits::LangDefaults,
        types::{Dirs, Extensions, Files},
    },
    errors::PyLineError,
    py::base as py_base,
    rust::base as rust_base,
};
use std::path::{Path, PathBuf};

/// Configuration for collecting and filtering files from a directory structure.
///
/// Used to define rules for which files and directories should be included
/// or excluded during file collection operations. All fields have sensible
/// defaults.
#[derive(Default)]
pub struct Collector {
    /// Root directory path from which to start file collection.
    path: PathBuf,

    lang: CodeLanguage,

    /// List of file names that, when found, cause their parent directories
    /// to be excluded.
    ///
    /// For example, including `.gitignore` here would skip directories
    /// containing a `.gitignore` file.
    marker_files: Files,

    /// List of directory names to exclude from traversal.
    exclude_dirs: Dirs,

    /// List of file names to exclude from collection.
    exclude_files: Files,

    /// List of file extensions to include in collection.
    ///
    /// Only files with these extensions will be collected. For example,
    /// `vec!["py", "pyw"]` would collect only Python files. `None` means
    /// all file extensions are included.
    extensions: Extensions,

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
    /// use pyline_libs::CodeLanguage;
    ///
    /// let path = PathBuf::from("/path");
    ///
    /// let c = Collector::new(&path, CodeLanguage::Python)
    ///             .with_extensions(["py"])
    ///             .with_ignore_dot_dirs(false)
    ///             .with_exclude_dirs(["target", "node_modules"]);
    /// ```
    ///
    /// By default, the `ignore_dot_dirs` is enabled (set to true),
    /// meaning all directories starting with a dot (`.`) are ignored.
    pub fn new(path: &Path, lang: CodeLanguage, auto_config: bool) -> Self {
        use CodeLanguage::*;

        let code_ext = match lang {
            Python => py_base::VALID_EXTENSIONS,
            Rust => rust_base::RUST_VALID_EXTENSIONS,
        };
        let extensions = Extensions::with_defaults(code_ext);

        let (marker_files, exclude_dirs, exclude_files) = if auto_config {
            match lang {
                Python => (
                    Files::with_defaults(py_base::MARKER_FILE),
                    Dirs::with_defaults(py_base::EXCLUDE_DIRS),
                    Files::with_defaults(py_base::EXCLUDE_FILENAMES),
                ),
                Rust => (
                    Files::with_defaults(rust_base::RUST_MARKER_FILE),
                    Dirs::with_defaults(rust_base::RUST_EXCLUDE_DIRS),
                    Files::with_defaults(rust_base::RUST_EXCLUDE_FILENAMES),
                ),
            }
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
            skip_errors: true,
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
    /// ```ignore
    /// use std::path::PathBuf;
    /// use pyline_libs::collector::Collector;
    ///
    /// let path = PathBuf::from("/path");
    ///
    /// Collector::new(&path)
    ///     .with_exclude_dirs(["node_modules", "target", "__pycache__"])?
    ///     .complete();
    /// ```
    pub fn with_exclude_dirs<I, S>(mut self, dirs: I) -> Result<Self, PyLineError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let exclude_dirs: Dirs = dirs.into_iter().collect();

        if self.ignore_dot_dirs && exclude_dirs.iter().any(|s| s.starts_with(".")) {
            return Err(PyLineError::scanner_error(
                "Cannot exclude dot-directories (e.g., '.git') \
                    via `exclude_dirs` while `ignore_dot_dirs` is enabled. \
                    Consider removing them from `exclude_dirs`, or disable \
                    `ignore_dot_dirs` with `.ignore_dot_dirs(false)`.",
            ));
        }

        self.exclude_dirs = exclude_dirs;

        Ok(self)
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
        self.marker_files = files.into_iter().collect();
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
    /// use pyline_libs::CodeLanguage;
    ///
    /// let path = PathBuf::from("/path");
    ///
    /// Collector::new(&path, CodeLanguage::Python)
    ///     .with_exclude_files(["README.md", "LICENSE", ".gitignore"])
    ///     .complete();
    /// ```
    pub fn with_exclude_files<I, S>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.exclude_files = files.into_iter().collect();
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
    /// use pyline_libs::CodeLanguage;
    ///
    /// let path = PathBuf::from("/path");
    ///
    /// Collector::new(&path, CodeLanguage::Rust)
    ///     .with_extensions(["rs", ".toml"])  // Works with or without dots
    ///     .complete();
    /// ```
    pub fn with_extensions<I, S>(mut self, ext: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.extensions.extend(ext);
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
    ///     .with_ignore_dot_dirs(true)  // Skip .git, .venv, etc.
    ///     .complete();
    /// ```
    pub fn with_ignore_dot_dirs(mut self, ignore: bool) -> Self {
        self.ignore_dot_dirs = ignore;
        self
    }

    /// Sets whether to skip access/read errors and continue processing only accessible items.
    ///
    /// When `true` (default), errors are ignored and collection proceeds with accessible
    /// directories/files. When `false`, any error immediately halts the search.
    pub fn with_skip_errors(mut self, skip: bool) -> Self {
        self.skip_errors = skip;
        self
    }
}

impl Collector {
    /// Returns the root directory path from which file collection starts.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the target code language.
    pub fn lang(&self) -> &CodeLanguage {
        &self.lang
    }
    //
    // /// Returns whether autoconfiguration is enabled.
    // pub fn auto_config(&self) -> bool {
    //     self.auto_config
    // }

    /// Returns the marker files whose presence excludes parent directories.
    pub fn marker_files(&self) -> &Files {
        &self.marker_files
    }

    /// Returns the directory names excluded from traversal.
    pub fn exclude_dirs(&self) -> &Dirs {
        &self.exclude_dirs
    }

    /// Returns the file names excluded from collection.
    pub fn exclude_files(&self) -> &Files {
        &self.exclude_files
    }

    /// Returns the file extensions included in collection.
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
