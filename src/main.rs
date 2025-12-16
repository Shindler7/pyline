//! Подсчёт строк кода в приложениях Python.
//!
//! Учитываются файлы *.py, в них игнорируются комментарии и пробелы.
//! С помощью пользователя или самостоятельно исключаются зависимости.
//!
//! Shindler7, 2025.
mod config;
mod counter;
mod errors;
mod scanner;
mod tools;

use counter::CodeStats;
use scanner::{collect_files, FileListFormatter};
use std::env;
use std::path::PathBuf;
use std::process::exit;

#[tokio::main]
async fn main() {
    // Подготовка древа файлов.
    let path = get_current_dir();
    let files = collect_files(path).await.unwrap();
    println!("This is files:\n{}", files.format());

    // Анализ файлов.
    let code_stats = CodeStats::new().parsing_files(files).await.unwrap();

    println!("Статистика:\n{}", code_stats)
}

/// Вернуть текущую директорию.
fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|err| {
        eprintln!("Не удалось определить текущую директорию: {}", err);
        exit(1);
    })
}
