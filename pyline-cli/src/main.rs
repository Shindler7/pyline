//! Source line counter for Python applications.
//!
//! Analyzes *.py files, excluding comments and whitespace lines.
//! Produces statistical analysis of Python keyword usage.
//!
//! Shindler7, 2025.
use pyline_libs::traits::CodeParsers;
mod cli;
mod config;

use crate::cli::ArgsResult;
use pyline_libs::collector::{Collector, FileData};
use pyline_libs::errors::PyLineError;
use pyline_libs::parser::Python;
use std::process::exit;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

async fn run() -> Result<(), PyLineError> {
    let cli_result = cli::read_cmd_args();

    println!(
        "OK.\nThe files in the directory are being examined: {}",
        cli_result.path.display()
    );

    let files = collect_files(&cli_result).await?;

    if files.is_empty() {
        return Ok(());
    }

    println!(" Successfully gathered {} files.", files.len());

    analyze_files(files).await?;

    Ok(())
}

async fn collect_files(cli_result: &ArgsResult) -> Result<Vec<FileData>, PyLineError> {
    print!("\nGathering files for analysis... ");

    let files = Collector::new(&cli_result.path)
        .ignore_dot_dirs(true)
        .extensions(&cli_result.extension)
        .exclude_dirs(&cli_result.dirs)
        .exclude_files(&cli_result.filenames)
        .complete()
        .await?;

    if files.is_empty() {
        print!("NO FILES.");
    } else {
        print!("OK.");
    }

    Ok(files)
}

async fn analyze_files(files: Vec<FileData>) -> Result<(), PyLineError> {
    print!("\nGathering code stats... ");

    let python_stats = Python::new().parse(files).await?;

    print!("OK.");
    println!("\n{}\n", python_stats.stats);

    Ok(())
}
