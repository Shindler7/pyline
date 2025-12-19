//! Подсчёт строк кода в приложениях Python.
//!
//! Учитываются файлы *.py, в них игнорируются комментарии и пробелы.
//! С помощью пользователя или самостоятельно исключаются зависимости.
//!
//! Shindler7, 2025.
mod cli;
mod config;

use pyline_libs::files_collector::Collector;
use pyline_libs::py::base::TECH_DIRS;
use std::process::exit;
#[tokio::main]
async fn main() {
    // Building file tree structure.
    let path = cli::read_cmd_args();
    println!(
        "OK.\nThe files in the directory are being examined: {}",
        path.display()
    );

    print!("\nGathering files for analysis... ");
    let files = Collector::new(path)
        .ignore_dot_dirs(true)
        .extensions(["py"])
        .exclude_dirs(["venv", "env"])
        .complete()
        .await
        .unwrap_or_else(|error| {
            eprint!("ERROR: {}", error);
            exit(1);
        });

    if files.is_empty() {
        print!("NO FILES.");
        exit(0);
    } else {
        print!("OK.");
        println!("\nSuccessfully gathered {} files", files.len());
    }

    // println!("\nDiscovered files:\n{}", files.format());
    //
    // // Анализ файлов.
    // print!("\nGathering code stats... ");
    // let code_stats = CodeStatsPython::new_with_keywords()
    //     .parsing_files(files)
    //     .await
    //     .unwrap_or_else(|err| {
    //         print!("ERROR: {}", err);
    //         exit(1);
    //     });
    // print!("OK.");
    // println!("\n{}\n", code_stats)
}
