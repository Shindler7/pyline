//! Command-line argument parsing and validation module.
//!
//! This module handles:
//! - Parsing CLI arguments using `clap`
//! - Validating input paths and directories
//! - Providing sensible defaults when arguments are omitted
//! - Converting raw arguments into structured configuration for the application

use anyhow::{Context, Result as AnyhowResult, bail};
use clap::{Parser, ValueEnum};
use pyline_libs::{CodeLanguage, collector::Collector};
use std::{env, path::PathBuf};

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
    #[clap(short, long)]
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

impl std::fmt::Display for CodeLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeLang::Python => write!(f, "{}", CodeLanguage::Python),
            CodeLang::Rust => write!(f, "{}", CodeLanguage::Rust),
        }
    }
}

impl From<CodeLang> for CodeLanguage {
    fn from(lang: CodeLang) -> Self {
        match lang {
            CodeLang::Rust => CodeLanguage::Rust,
            CodeLang::Python => CodeLanguage::Python,
        }
    }
}

#[derive(Default)]
pub(super) struct ArgsResult {
    pub(super) collector: Collector,
    pub(super) verbose: bool,
}

impl ArgsResult {
    /// Reading command-line parameters with validation.
    ///
    /// Control is not returned until valid data is received from the user.
    pub(super) fn from_clap() -> AnyhowResult<Self> {
        let args = Args::parse();
        let path = parse_path(args.path)?;

        let collector = Collector::new(&path, args.lang.into(), args.auto_config)
            .with_ignore_dot_dirs(args.ignore_dot_dirs)
            .with_extensions(args.ext)
            .with_exclude_dirs(args.exclude_dirs)?
            .with_marker_files(args.marker_files)
            .with_exclude_files(args.exclude_files)
            .with_skip_errors(!args.no_skip_gather_errors);

        Ok(ArgsResult {
            collector,
            verbose: args.verbose,
        })
    }

    /// Returns a detailed string representation suitable for verbose output.
    ///
    /// Shows all fields with their values, formatted for readability.
    pub(super) fn verbose_display(&self) -> String {
        fn join_or_wildcard<I, T>(items: I, separator: &str) -> String
        where
            I: IntoIterator<Item = T>,
            T: AsRef<str>,
        {
            let mut iter = items.into_iter();
            if let Some(first) = iter.next() {
                let mut result = first.as_ref().to_string();
                for item in iter {
                    result.push_str(separator);
                    result.push_str(item.as_ref());
                }
                result
            } else {
                "not set".to_string()
            }
        }

        let collector = &self.collector;

        let dirs = join_or_wildcard(collector.exclude_dirs().iter(), ", ");
        let filenames = join_or_wildcard(collector.exclude_files().iter(), ", ");
        let marker_files = join_or_wildcard(collector.marker_files().iter(), ", ");

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
            collector.path().display(),
            dirs,
            marker_files,
            collector.ignore_dot_dirs(),
            collector.extensions().join(", "),
            filenames,
            collector.lang(),
            collector.skip_errors(),
            self.verbose
        )
    }
}

/// Parses and validates the input path argument.
///
/// If a path is provided, validates it as an existing directory.
/// If no path is provided, returns the current working directory.
fn parse_path(args_path: Option<PathBuf>) -> AnyhowResult<PathBuf> {
    let path = match args_path {
        Some(p) => p,
        None => env::current_dir().context("Current directory could not be determined.")?,
    };

    if path.is_file() {
        bail!("Path must be a directory, not a file: {}", path.display());
    }
    if !path.is_dir() {
        bail!("Directory not found: {}", path.display());
    }

    Ok(path)
}
