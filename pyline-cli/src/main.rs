//! Source line counter for Python applications.
//!
//! Analyzes *.py files, excluding comments and whitespace lines.
//! Produces statistical analysis of Python keyword usage.
//!
//! Shindler7, 2025.
#![warn(missing_docs)]

use pyline_libs::traits::{CodeParsers, FileDataExt};
mod cli;
mod config;
mod tools;

use crate::{
    cli::{ArgsResult, CodeLang},
    tools::show_dot,
};
use anyhow::Result as AnyhowResult;
use pyline_libs::{
    collector::{Collector, CollectorResult, FileData},
    errors::PyLineError,
    parser::{Python, Rust},
};
use std::{
    process::exit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        exit(1);
    }

    Ok(())
}

/// Main loop function.
async fn run() -> Result<(), PyLineError> {
    let cli_result = cli::read_cmd_args().normalize_by_lang();

    println!("\nSelected language: {}\n", cli_result.lang);
    println!(
        "The files in the directory are being examined: {}",
        cli_result.path.display()
    );

    if cli_result.verbose {
        println!("\n{}", cli_result.verbose_display());
    }

    let files = collect_files(&cli_result).await?;

    // About errors and verbose.
    if files.has_errors() {
        println!(
            "\nWARNINGS! During the gathering process, {} errors occurred.",
            files.num_errors()
        );
        if cli_result.verbose {
            for err in files.errors() {
                eprintln!("\n{}", err);
            }
        }
    }

    if !files.has_files() {
        return Ok(());
    }

    println!(" Successfully gathered {} files.", files.num_files());

    if cli_result.verbose {
        println!("\n{}", files.files().join_verbose(""));
    }

    analyze_files(&cli_result, files.files()).await?;

    Ok(())
}

async fn collect_files(cli_result: &ArgsResult) -> Result<CollectorResult, PyLineError> {
    let running = Arc::new(AtomicBool::new(true));
    let spinner_handle = {
        let running = running.clone();
        tokio::spawn(show_dot(running))
    };

    print!("\nGathering files for analysis... ");

    let files = Collector::new(&cli_result.path)
        .ignore_dot_dirs(cli_result.ignore_dot_dirs)
        .extensions(&cli_result.extension)
        .exclude_dirs(&cli_result.dirs)
        .with_marker_files(&cli_result.marker_files)
        .exclude_files(&cli_result.filenames)
        .skip_errors(cli_result.skip_gather_errors)
        .complete()
        .await?;

    // Spinner stop.
    running.store(false, Ordering::Relaxed);
    let _ = spinner_handle.await;

    if files.has_files() {
        print!("OK.");
    } else {
        print!("NO FILES.");
    }

    println!();

    Ok(files)
}

async fn analyze_files(cli_result: &ArgsResult, files: &[FileData]) -> Result<(), PyLineError> {
    print!("\nGathering code stats... ");

    match cli_result.lang {
        CodeLang::Python => {
            let mut python_stats = Python::new();
            python_stats.parse(files).await?;

            print!("OK.");
            println!("\n{}\n", python_stats);
        }
        CodeLang::Rust => {
            let mut rust_stats = Rust::new();
            rust_stats.parse(files).await?;

            print!("OK.");
            println!("\n{}\n", rust_stats);
        }
    }

    Ok(())
}
