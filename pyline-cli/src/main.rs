//! Source line counter for Python and Rust applications.
//!
//! Analyzes source files, excluding comments and whitespace lines,
//! and produces statistics on keyword usage.

mod cli;
mod tools;

use crate::{cli::ArgsResult, tools::show_dot};
use anyhow::Result as AnyhowResult;

use pyline_libs::parser::traits::CodeParsers;
use pyline_libs::{
    CodeLanguage, Collector, CollectorResult, FileData,
    collector::FileDataExt,
    errors::PyLineError,
    parser::{Python, Rust},
};
use std::{
    io::Write,
    process::exit,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    if let Err(e) = run().await {
        eprintln!("\n\n{}", e);
        exit(1);
    }

    Ok(())
}

/// Runs the application: parses CLI args, collects files, and prints stats.
async fn run() -> AnyhowResult<()> {
    let cli_result = ArgsResult::from_clap()?;

    println!("\nSelected language: {}\n", cli_result.collector.lang());
    println!(
        "The files in the directory are being examined: {}",
        cli_result.collector.path().display()
    );

    if cli_result.verbose {
        println!("\n{}", cli_result.verbose_display());
    }

    let collection = collect_files(&cli_result.collector).await?;

    if collection.has_errors() {
        println!(
            "\nWARNINGS! During the gathering process, {} errors occurred.",
            collection.num_errors()
        );
        if cli_result.verbose {
            for err in collection.errors() {
                eprintln!("\n{}", err);
            }
        }
    }

    if !collection.has_files() {
        return Ok(());
    }

    println!(" Successfully gathered {} files.", collection.num_files());

    if cli_result.verbose {
        println!("\n{}", collection.files().join_verbose(""));
    }

    analyze_files(&cli_result, collection.files()).await?;

    Ok(())
}

/// Collects files via the configured [`Collector`], showing a progress spinner.
async fn collect_files(collector: &Collector) -> AnyhowResult<CollectorResult> {
    let running = Arc::new(AtomicBool::new(true));
    let spinner_handle = {
        let running = running.clone();
        tokio::spawn(show_dot(running))
    };

    print!("\nGathering files for analysis... ");
    std::io::stdout().flush()?;

    let collector_result = collector.complete().await;

    running.store(false, Ordering::Relaxed);
    let _ = spinner_handle.await?;

    let files_batch = collector_result?;

    if files_batch.has_files() {
        print!("OK.");
    } else {
        print!("NO FILES.");
    }

    println!();

    Ok(files_batch)
}

/// Parses the collected files and prints keyword statistics.
async fn analyze_files(cli_result: &ArgsResult, files: &[FileData]) -> Result<(), PyLineError> {
    print!("\nGathering code stats... ");

    match cli_result.collector.lang() {
        CodeLanguage::Python => {
            let mut python_stats = Python::new();
            python_stats.parse(files).await?;

            print!("OK.");
            println!("\n{}\n", python_stats);
        }
        CodeLanguage::Rust => {
            let mut rust_stats = Rust::new();
            rust_stats.parse(files).await?;

            print!("OK.");
            println!("\n{}\n", rust_stats);
        }
    }

    Ok(())
}
