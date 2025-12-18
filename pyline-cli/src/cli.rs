//! Модуль взаимодействия с аргументами командной строки.

use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_name = "PATH")]
    path: Option<PathBuf>,
}

/// Считывание параметров командной строки с проверкой.
/// Управление не возвращается, пока от пользователя не будут получены валидные данные.
pub fn read_cmd_args() -> PathBuf {
    let args = Args::parse();

    parse_path(args.path)
}

/// Парсинг ключа PATH.
fn parse_path(args_path: Option<PathBuf>) -> PathBuf {
    match args_path {
        Some(path) => {
            if path.is_file() {
                println!(
                    "The Path is expected to point to a directory, but a file path was provided: {:?}",
                    path
                );
                exit(1);
            }

            if !path.exists() {
                println!("ERROR: The Path does not exist.");
                exit(1);
            }
            path
        }
        None => get_current_dir(),
    }
}

/// Вернуть текущую директорию.
fn get_current_dir() -> PathBuf {
    env::current_dir().expect("ERROR: Current directory could not be determined.")
}
