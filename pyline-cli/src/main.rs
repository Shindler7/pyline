//! Source line counter for Python and Rust applications.
//!
//! Analyzes source files, excluding comments and whitespace lines,
//! and produces statistics on keyword usage.

mod cli;
mod tools;

use crate::{cli::ArgsResult, tools::show_dot};
use anyhow::Result as AnyhowResult;
use pyline_libs::{
    CodeLanguage, CodeParsers, Collector, CollectorResult, FileData,
    collector::FileDataExt,
    errors::PyLineError,
    parser::{Python, Rust},
};
use std::{
    fmt::Display,
    io::Write,
    process::ExitCode,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

fn main() -> AnyhowResult<ExitCode> {
    let cli_result = ArgsResult::from_clap()?;

    println!("\nSelected language: {}\n", cli_result.collector.lang());
    println!(
        "The files in the directory are being examined: {}",
        cli_result.collector.path().display()
    );

    if cli_result.verbose {
        println!("\n{}", cli_result.verbose_display());
    }

    run(&cli_result)?;

    Ok(ExitCode::SUCCESS)
}

/// Runs the application: collects files, and prints stats.
fn run(cli_result: &ArgsResult) -> AnyhowResult<()> {
    let collection = collect_files(&cli_result.collector)?;
    if collection.has_files() {
        println!("OK.\n");
    } else {
        println!("NO FILES.\n");
    }

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

    if collection.has_files() {
        println!("Successfully gathered {} files.", collection.num_files());

        if cli_result.verbose {
            println!("\n{}", collection.files().join_verbose(""));
        }

        match cli_result.collector.lang() {
            CodeLanguage::Python => analyze::<Python>(collection.files()),
            CodeLanguage::Rust => analyze::<Rust>(collection.files()),
        }?;
    }

    Ok(())
}

/// Collects files via the configured [`Collector`], showing a progress spinner.
fn collect_files(collector: &Collector) -> AnyhowResult<CollectorResult> {
    let running = Arc::new(AtomicBool::new(true));
    let spinner_handle = {
        let running = running.clone();
        thread::spawn(move || show_dot(running))
    };

    print!("\nGathering files for analysis... ");
    std::io::stdout().flush()?;

    let collector_result = collector.complete();

    running.store(false, Ordering::Relaxed);
    let _ = spinner_handle.join();

    Ok(collector_result?)
}

/// Parses the collected files and prints keyword statistics.
fn analyze<P: CodeParsers + Display>(files: &[FileData]) -> Result<(), PyLineError> {
    let mut parser = P::new();
    parser.parse(files)?;
    println!("\n{parser}\n");

    Ok(())
}
