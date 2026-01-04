//! Command-line argument parsing and validation module.
//!
//! This module handles:
//! - Parsing CLI arguments using `clap`
//! - Validating input paths and directories
//! - Providing sensible defaults when arguments are omitted
//! - Converting raw arguments into structured configuration for the application

use clap::{Parser, ValueEnum};
use pyline_libs::py::base::{
    EXCLUDE_DIRS, EXCLUDE_DOT_DIRS, EXCLUDE_FILENAMES, MARKER_FILE, VALID_EXTENSIONS,
};
use pyline_libs::rust::base::{
    RUST_EXCLUDE_DIRS, RUST_EXCLUDE_DOT_DIRS, RUST_EXCLUDE_FILENAMES, RUST_MARKER_FILE,
    RUST_VALID_EXTENSIONS,
};
use std::env;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(about = "A high-performance CLI tool for analyzing codebases with \
    intelligent filtering and detailed statistics collection.")]
#[clap(author, version, long_about = None)]
struct Args {
    /// Selects the programming language for parsing from predefined options.
    #[clap(short, long, required = true)]
    lang: CodeLang,

    /// Enables automatic configuration based on the selected programming
    /// language.
    ///
    /// If `false`, all other parameters (`--ext`, `--exclude-dirs`,
    /// `--exclude-files`, `--ignore-dot-dirs`) must be manually configured
    /// by the user or will use their default values. **Exception**: file
    /// extensions (`--ext`) always include basic language-specific extensions
    /// regardless of the `auto-config` flag.
    #[clap(short, long, default_value = "false")]
    auto_config: bool,

    /// Path to the directory with files to parse. If not specified,
    /// the current directory is analyzed.
    #[clap(short, long, value_name = "PATH")]
    path: Option<PathBuf>,

    /// Directories to exclude from collection.
    #[clap[short='x', long, value_name = "DIRECTORIES"]]
    exclude_dirs: Vec<String>,

    /// Marker files that cause their parent directories to be excluded from
    /// traversal.
    ///
    /// When a directory contains any of the specified marker files, the entire
    /// directory (including all subdirectories) will be skipped during file
    /// collection. This is useful for excluding directories based on the
    /// presence of configuration or metadata files.
    #[clap[short, long, value_name = "MARKER_FILE"]]
    marker_files: Vec<String>,

    /// Ignore directories starting with a dot (e.g., `.git`, `.config`)
    /// while traversing.
    ///
    /// When this flag is enabled (default: `true`), all directories whose
    /// names begin with a dot are automatically excluded from the file
    /// collection process.
    ///
    /// ⚠️ If `ignore_dot_dirs` is set to `true`, you **must not** manually
    /// specify such directories (e.g., `.git`, `.venv`) in the
    /// `--exclude-dirs` list. Doing so will cause the application to panic
    /// with an explanatory error. This is by design, as dot-directories are
    /// already handled separately by this flag.
    #[clap(short, long, default_value = "true")]
    ignore_dot_dirs: bool,

    /// File extensions to include in the collection. Can be specified
    /// multiple times.
    ///
    /// For the selected language, basic extensions (e.g., `.py` for Python)
    /// are automatically included alongside any explicitly provided extensions.
    #[clap(short, long, value_name = "EXTENSION")]
    ext: Vec<String>,

    /// Files to exclude from collection.
    #[clap(short = 'X', long, value_name = "FILENAMES")]
    exclude_files: Vec<String>,

    /// Do not skip access/read errors (default: errors are skipped)
    #[clap(short = 'E', long = "gather-errors", default_value = "false")]
    no_skip_gather_errors: bool,

    /// Enable verbose output with detailed logging information.
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Clone, ValueEnum, Debug, Default)]
pub enum CodeLang {
    /// alias `py`.
    #[clap(name = "python", alias = "py")]
    #[default]
    Python,
    #[clap(name = "rust")]
    Rust,
}

impl Display for CodeLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeLang::Python => f.write_str("Python, https://www.python.org/"),
            CodeLang::Rust => f.write_str("Rust, https://rust-lang.org/"),
        }
    }
}

#[derive(Default, Clone)]
pub struct ArgsResult {
    pub path: PathBuf,
    pub dirs: Vec<String>,
    pub marker_files: Vec<String>,
    pub extension: Vec<String>,
    pub filenames: Vec<String>,
    pub lang: CodeLang,
    pub verbose: bool,
    pub ignore_dot_dirs: bool,
    auto_config: bool,
    pub skip_gather_errors: bool,
}

impl ArgsResult {
    /// Creates a normalized copy of the arguments with language-aware extensions.
    ///
    /// This method returns a new instance where file extensions are processed to ensure:
    /// - All extensions have leading dots
    /// - Language-specific default extensions are included
    /// - Duplicate extensions are removed
    ///
    /// The original instance remains unchanged (following Rust's immutability principles).
    ///
    /// # Example
    ///
    /// ```
    /// let args = ArgsResult {
    ///     lang: CodeLang::Python,
    ///     ext: vec!["py".to_string(), ".txt".to_string()],
    ///     // other fields...
    /// };
    ///
    /// let normalized = args.normalize_by_lang();
    /// // normalized.ext will contain: [".py", ".txt"]
    /// // (".py" added by default for Python, ".txt" from user input)
    /// ```
    pub fn normalize_by_lang(&self) -> Self {
        let mut normalize_self = self.clone();
        normalize_self.extension = self.normalize_ext_by_lang();

        if !self.auto_config {
            return normalize_self;
        }

        // auto-config execution.
        normalize_self.dirs = self.exclude_dirs_by_lang();
        normalize_self.marker_files = self.exclude_marker_files_by_lang();
        normalize_self.filenames = self.exclude_filenames_by_lang();

        normalize_self
    }

    /// Normalizes the list of directories excluded by default for the current
    /// language.
    ///
    /// Merges language-specific default exclusions with user-provided
    /// directories, respecting the `ignore_dot_dirs` flag for handling hidden
    /// directories.
    fn exclude_dirs_by_lang(&self) -> Vec<String> {
        let (dirs, dot_dirs) = match self.lang {
            CodeLang::Python => (EXCLUDE_DIRS, EXCLUDE_DOT_DIRS),
            CodeLang::Rust => (RUST_EXCLUDE_DIRS, RUST_EXCLUDE_DOT_DIRS),
        };

        let combined_defaults: Vec<&str> = if self.ignore_dot_dirs {
            dirs.to_vec()
        } else {
            dirs.iter().chain(dot_dirs.iter()).copied().collect()
        };

        Self::normalize_list(&combined_defaults, &self.dirs, false)
    }

    /// Builds the list of marker files specific to the current language.
    ///
    /// Combines language-specific default markers with user-provided entries,
    /// ensuring uniqueness of items in the resulting list.
    fn exclude_marker_files_by_lang(&self) -> Vec<String> {
        let default = match self.lang {
            CodeLang::Python => MARKER_FILE,
            CodeLang::Rust => RUST_MARKER_FILE,
        };

        Self::normalize_list(default, &self.marker_files, false)
    }

    /// Generates the list of filenames to exclude based on language.
    ///
    /// Merges language-specific default exclusions with the user-provided list,
    /// removing duplicates and maintaining sorted order.
    fn exclude_filenames_by_lang(&self) -> Vec<String> {
        let default = match self.lang {
            CodeLang::Python => EXCLUDE_FILENAMES,
            CodeLang::Rust => RUST_EXCLUDE_FILENAMES,
        };

        Self::normalize_list(default, &self.filenames, false)
    }

    /// Normalizes the list of file extensions with language semantics.
    ///
    /// Adds language-specific default extensions to user-provided ones,
    /// ensuring uniqueness and canonical format (without leading dots).
    fn normalize_ext_by_lang(&self) -> Vec<String> {
        let default = match self.lang {
            CodeLang::Python => VALID_EXTENSIONS,
            CodeLang::Rust => RUST_VALID_EXTENSIONS,
        };

        Self::normalize_list(default, &self.extension, true)
    }

    /// Universal method for normalizing string lists.
    ///
    /// Merges default values with user-provided entries, normalizing them
    /// to a common format. When `normalize_dot: true`, removes leading dots
    /// (for file extensions), guarantees uniqueness through sorting
    /// and deduplication.
    fn normalize_list(default: &[&str], user: &[String], normalize_dot: bool) -> Vec<String> {
        let mut result: Vec<String> = default.iter().map(|s| s.to_string()).collect();

        for item in user.iter() {
            let mut norm = Self::normalize_case(item);

            if normalize_dot {
                norm = norm.trim_start_matches('.').to_string();
            }

            result.push(norm);
        }

        result.sort();
        result.dedup();
        result
    }

    /// Normalizes string case according to platform conventions.
    ///
    /// On Windows, converts to lowercase for case-insensitive consistency.
    /// On other platforms, returns the string unchanged.
    #[cfg(windows)]
    fn normalize_case(s: &str) -> String {
        s.to_lowercase()
    }

    #[cfg(not(windows))]
    fn normalize_case(s: &str) -> String {
        s.to_string()
    }

    /// Returns a detailed string representation suitable for verbose output.
    ///
    /// Shows all fields with their values, formatted for readability.
    pub fn verbose_display(&self) -> String {
        let dirs = Self::join_or_wildcard(&self.dirs, ", ");
        let filenames = Self::join_or_wildcard(&self.filenames, ", ");
        let marker_files = Self::join_or_wildcard(&self.marker_files, ", ");

        format!(
            "Arguments:\n\
             ├─ Path: {}\n\
             ├─ Exclude Directories: {}\n\
             ├─ Marker_files: {}\n\
             ├─ Ignore dot dirs: {}\n\
             ├─ Extensions: {}\n\
             ├─ Exclude Filenames: {}\n\
             ├─ Language: {:?}\n\
             ├─ Skip gather errors: {}\n\
             └─ Verbose: {}",
            self.path.display(),
            dirs,
            marker_files,
            self.ignore_dot_dirs,
            self.extension.join(", "),
            filenames,
            self.lang,
            self.skip_gather_errors,
            self.verbose
        )
    }

    fn join_or_wildcard<T: AsRef<str>>(items: &[T], separator: &str) -> String {
        if items.is_empty() {
            "not set".to_string()
        } else {
            items
                .iter()
                .map(|item| item.as_ref())
                .collect::<Vec<_>>()
                .join(separator)
        }
    }
}

/// Reading command-line parameters with validation.
///
/// Control is not returned until valid data is received from the user.
pub fn read_cmd_args() -> ArgsResult {
    let args = Args::parse();

    let path = parse_path(args.path);

    ArgsResult {
        path,
        auto_config: args.auto_config,
        dirs: args.exclude_dirs,
        marker_files: args.marker_files,
        ignore_dot_dirs: args.ignore_dot_dirs,
        extension: args.ext,
        filenames: args.exclude_files,
        lang: args.lang,
        skip_gather_errors: !args.no_skip_gather_errors,
        verbose: args.verbose,
    }
}

/// Parses and validates the input path argument.
///
/// If a path is provided, validates it as an existing directory.
/// If no path is provided, returns the current working directory.
fn parse_path(args_path: Option<PathBuf>) -> PathBuf {
    match args_path {
        Some(path) => validate_directory_path(path),
        None => get_current_dir(),
    }
}

/// Validates that a given path exists and points to a directory.
///
/// # Panics
///
/// Terminates the program with an error message if:
/// - The path points to a file instead of a directory
/// - The path does not exist in the filesystem
fn validate_directory_path(path: PathBuf) -> PathBuf {
    if path.is_file() {
        exit_err(format!(
            "Path must be a directory, not a file: {}",
            path.display()
        ));
    }

    if !path.exists() {
        exit_err(format!("Directory not found: {}", path.display()));
    }

    path
}

/// Get path to the current directory.
fn get_current_dir() -> PathBuf {
    env::current_dir().expect("ERROR: Current directory could not be determined.")
}

/// Terminates the application with an error message.
///
/// This function prints the provided message to standard error and exits
/// the process with a non-zero status code, indicating failure.
/// The `!` return type indicates that this function never returns.
fn exit_err(message: impl Into<String>) -> ! {
    eprintln!("ERROR. {}", message.into());
    exit(1);
}
