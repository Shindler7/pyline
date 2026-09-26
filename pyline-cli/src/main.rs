//! Source line counter for Python and Rust applications.
//!
//! Analyzes source files, excluding comments and whitespace lines,
//! and produces statistics on keyword usage.

mod cli;
mod tools;

use crate::{cli::ArgsResult, tools::show_dot};
use anyhow::Result as AnyhowResult;
use pyline_libs::{Collector, CollectorResult, collector::FileDataExt, parser::run as parser_run};
use std::{
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
    if collection.files().is_empty() {
        println!("NO FILES.\n");
    } else {
        println!("OK.\n");
    }

    if collection.has_errors() {
        println!(
            "\nWARNINGS! During the gathering process, {} errors occurred.",
            collection.errors().len()
        );
        if cli_result.verbose {
            for err in collection.errors() {
                eprintln!("\n{err}");
            }
        }
    }

    if !collection.files().is_empty() {
        println!("Successfully gathered {} files.", collection.files().len());

        if cli_result.verbose {
            println!("\n{}", collection.files().join_verbose(""));
        }

        let result = parser_run(collection.files(), cli_result.collector.lang())?;
        println!("\n{result}\n");
    }

    Ok(())
}

/// Collects files via the configured [`Collector`], showing a progress spinner.
fn collect_files(collector: &Collector) -> AnyhowResult<CollectorResult> {
    let running = Arc::new(AtomicBool::new(true));
    let spinner_handle = {
        let running = Arc::clone(&running);
        thread::spawn(move || show_dot(&running))
    };

    print!("\nGathering files for analysis... ");
    std::io::stdout().flush()?;

    let collector_result = collector.collect();

    running.store(false, Ordering::Relaxed);
    let _ = spinner_handle.join();

    Ok(collector_result?)
}
