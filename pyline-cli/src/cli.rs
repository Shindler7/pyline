//! Command-line argument parsing and validation.
//!
//! Responsibilities:
//!
//! - Parse CLI arguments with `clap`.
//! - Validate input paths and directories.
//! - Convert raw arguments into the application's [`Collector`] configuration.

use anyhow::{Context, Result as AnyhowResult, bail};
use clap::{Parser, ValueEnum};
use pyline_libs::{CodeLanguage, collector::Collector};
use std::{env, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(about = "A high-performance CLI tool for analyzing codebases with \
    intelligent filtering and detailed statistics collection.")]
#[clap(author, version, long_about = None)]
struct Args {
    /// Programming language to parse.
    #[clap(short, long, required = true)]
    lang: CodeLang,

    /// Enables automatic configuration based on the selected language.
    ///
    /// If `false`, the other parameters (`--ext`, `--exclude-dirs`,
    /// `--exclude-files`, `--ignore-dot-dirs`) must be configured manually
    /// or fall back to their defaults. Exception: `--ext` always includes
    /// the language's basic extensions regardless of this flag.
    #[clap(short, long, default_value = "false")]
    auto_config: bool,

    /// Directory to analyze. Defaults to the current working directory.
    #[clap(short, long, value_name = "PATH")]
    path: Option<PathBuf>,

    /// Directories to exclude from the collection.
    #[clap[short='x', long, value_name = "DIRECTORIES", value_delimiter = ',', num_args = 1..]]
    exclude_dirs: Vec<String>,

    /// Marker files that cause their parent directories to be excluded
    /// from traversal.
    ///
    /// When a directory contains any of the specified marker files, the
    /// directory and all its subdirectories are skipped. Useful for excluding
    /// directories based on configuration or metadata files.
    #[clap[short, long, value_name = "MARKER_FILE", value_delimiter = ',', num_args = 1..]]
    marker_files: Vec<String>,

    /// Ignore directories whose names start with a dot (e.g., `.git`, `.venv`).
    ///
    /// When enabled, such directories are excluded from the collection
    /// automatically. They must not be listed in `--exclude-dirs` — this
    /// is rejected with an error, since dot-directories are already handled
    /// by this flag.
    #[clap(short, long)]
    ignore_dot_dirs: bool,

    /// File extensions to include in the collection. Can be specified multiple times.
    ///
    /// The language's basic extensions (e.g., `.py` for Python) are always
    /// included alongside any explicitly provided ones.
    #[clap(short, long, value_name = "EXTENSION", value_delimiter = ',', num_args = 1..)]
    ext: Vec<String>,

    /// Files to exclude from the collection.
    #[clap(short = 'X', long, value_name = "FILENAMES", value_delimiter = ',', num_args = 1..)]
    exclude_files: Vec<String>,

    /// Collect access/read errors instead of silently skipping them.
    #[clap(short = 'E', long = "gather-errors", default_value = "false")]
    no_skip_gather_errors: bool,

    /// Enable verbose output.
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
    /// Parses CLI arguments and builds the resulting configuration.
    ///
    /// Returns an error if the input path is invalid or the collector
    /// configuration is rejected.
    pub(super) fn from_clap() -> AnyhowResult<Self> {
        let args = Args::parse();
        let path = parse_path(args.path)?;

        let collector = Collector::new(&path, args.lang.into(), args.auto_config)
            .with_ignore_dot_dirs(args.ignore_dot_dirs)?
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

    /// Returns a detailed string representation for verbose output.
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

/// Resolves and validates the input path.
///
/// If a path is given, it must be an existing directory. Otherwise, the
/// current working directory is used.
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
