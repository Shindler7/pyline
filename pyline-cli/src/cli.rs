//! Command-line argument parsing and validation module.
//!
//! This module handles:
//! - Parsing CLI arguments using `clap`
//! - Validating input paths and directories
//! - Providing sensible defaults when arguments are omitted
//! - Converting raw arguments into structured configuration for the application

use clap::{Parser, ValueEnum};
use pyline_libs::py::base::VALID_EXTENSIONS;
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

    /// Path to the directory with files to parse. If not specified,
    /// the current directory is analyzed.
    #[clap(short, long, value_name = "PATH")]
    path: Option<PathBuf>,

    /// Directories to exclude from collection.
    #[clap[short, long, value_name = "DIRECTORIES"]]
    dirs: Vec<String>,

    /// File extensions to include in the collection. Can be specified
    /// multiple times.
    ///
    /// For the selected language, basic extensions (e.g., `.py` for Python)
    /// are automatically included alongside any explicitly provided extensions.
    #[clap(short, long, value_name = "EXTENSION")]
    extension: Vec<String>,

    /// Files to exclude from collection.
    #[clap(short, long, value_name = "FILENAMES")]
    filenames: Vec<String>,

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
}

impl Display for CodeLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeLang::Python => f.write_str("PYTHON, https://www.python.org/"),
        }
    }
}

#[derive(Default, Clone)]
pub struct ArgsResult {
    pub path: PathBuf,
    pub dirs: Vec<String>,
    pub extension: Vec<String>,
    pub filenames: Vec<String>,
    pub lang: CodeLang,
    pub verbose: bool,
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

        normalize_self
    }

    fn normalize_ext_by_lang(&self) -> Vec<String> {
        let mut normalized_ext: Vec<String>;

        match self.lang {
            CodeLang::Python => {
                normalized_ext = VALID_EXTENSIONS.iter().map(|ext| ext.to_string()).collect();
            }
        }

        for ext in self.extension.iter() {
            let norma_ext = ext.trim_start_matches('.').to_lowercase();
            normalized_ext.push(norma_ext);
        }

        normalized_ext.sort();
        normalized_ext.dedup();

        normalized_ext
    }

    /// Returns a detailed string representation suitable for verbose output.
    ///
    /// Shows all fields with their values, formatted for readability.
    pub fn verbose_display(&self) -> String {
        let dirs = Self::join_or_wildcard(&self.dirs, ", ");
        let filenames = Self::join_or_wildcard(&self.filenames, ", ");

        format!(
            "Arguments:\n\
             ├─ Path: {}\n\
             ├─ Exclude Directories: {}\n\
             ├─ Extensions: {}\n\
             ├─ Exclude Filenames: {}\n\
             ├─ Language: {:?}\n\
             └─ Verbose: {}",
            self.path.display(),
            dirs,
            self.extension.join(", "),
            filenames,
            self.lang,
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
        dirs: args.dirs,
        extension: args.extension,
        filenames: args.filenames,
        lang: args.lang,
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
