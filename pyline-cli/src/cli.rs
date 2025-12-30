//! Command-line argument parsing and validation module.
//!
//! This module handles:
//! - Parsing CLI arguments using `clap`
//! - Validating input paths and directories
//! - Providing sensible defaults when arguments are omitted
//! - Converting raw arguments into structured configuration for the application
//!
//! # Examples
//!
//! ```
//! use pyme::cli;
//! let config = cli::read_cmd_args();
//! ```

use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(about = "A high-performance CLI tool for analyzing codebases with \
    intelligent filtering and detailed statistics collection.")]
#[clap(author, version, long_about = None)]
struct Args {
    /// Path to the directory with files to parse. If not specified,
    /// the current directory is analyzed.
    #[clap(short, long, value_name = "PATH")]
    path: Option<PathBuf>,

    /// Directories to exclude from collection.
    #[clap[short, long, value_name = "DIRECTORIES"]]
    dirs: Vec<String>,

    /// File extensions to include in the collection. Can be specified
    /// multiple times. Required.
    #[clap(short, long, value_name = "EXTENSION", required = true)]
    extension: Vec<String>,

    /// Files to exclude from collection.
    #[clap(short, long, value_name = "FILENAMES")]
    filenames: Vec<String>,
}

#[derive(Default)]
pub struct ArgsResult {
    pub path: PathBuf,
    pub dirs: Vec<String>,
    pub extension: Vec<String>,
    pub filenames: Vec<String>,
}

/// Reading command-line parameters with validation.
///
/// Control is not returned until valid data is received from the user.
pub fn read_cmd_args() -> ArgsResult {
    let args = Args::parse();

    ArgsResult {
        path: parse_path(args.path),
        dirs: args.dirs,
        extension: args.extension,
        filenames: args.filenames,
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
