//! Source line counter for Python applications.
//!
//! Analyzes *.py files, excluding comments and whitespace lines.
//! Produces statistical analysis of Python keyword usage.
//!
//! Shindler7, 2025.
use pyline_libs::traits::{CodeParsers, FileDataExt};
mod cli;
mod config;

use crate::cli::{ArgsResult, CodeLang};
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
    let cli_result = cli::read_cmd_args().normalize_by_lang();

    if cli_result.verbose {
        println!("{}", cli_result.verbose_display());
    } else {
        println!("\nSelected language: {}\n", cli_result.lang);
        println!(
            "The files in the directory are being examined: {}",
            cli_result.path.display()
        );
    }

    let files = collect_files(&cli_result).await?;

    if files.is_empty() {
        return Ok(());
    }

    println!(" Successfully gathered {} files.", files.len());

    if cli_result.verbose {
        println!("\n{}", files.join_verbose(""));
    }

    analyze_files(&cli_result, files).await?;

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

async fn analyze_files(cli_result: &ArgsResult, files: Vec<FileData>) -> Result<(), PyLineError> {
    print!("\nGathering code stats... ");

    match cli_result.lang {
        CodeLang::Python => {
            let python_stats = Python::new().parse(files).await?;

            print!("OK.");
            println!("\n{}\n", python_stats);
        }
    }

    Ok(())
}
