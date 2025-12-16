//! Сканирование и отбор файлов для расчётов.

use crate::tools::format_file_size_alt;
use async_recursion::async_recursion;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use tokio::fs;

const TECHNICAL_DIRS: &[&str] = &[
    "venv",
    ".venv",
    "env",
    ".env",
    "__pycache__",
    ".git",
    ".hg",
    ".svn",
    ".mypy_cache",
    ".pytest_cache",
    ".tox",
    ".idea",
    ".vscode",
    "build",
    "dist",
    ".eggs",
    ".cache",
];

#[cfg(debug_assertions)]
const VALID_EXTENSIONS: &[&str] = &["rs"];

#[cfg(not(debug_assertions))]
const VALID_EXTENSIONS: &[&str] = &["py"];

pub trait FileListFormatter {
    fn format(&self) -> String;
}

impl FileListFormatter for Vec<FileAnalysis> {
    fn format(&self) -> String {
        if self.is_empty() {
            return "[]".to_string();
        };
        let mut formatted = String::new();
        for file in self.iter() {
            formatted += &format!("{}\n", file);
        }
        formatted.to_string()
    }
}

/// Структура статистики отобранных для сканирования файлов.
pub struct FileAnalysis {
    /// Путь к файлу.
    pub path: PathBuf,
    /// Размер файла (в байтах).
    pub size: u64,
    /// Количество строк (всего).
    pub lines: usize,
    /// Количество строк кода.
    pub code_lines: usize,
}

impl Debug for FileAnalysis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FileAnalysis [{} ({})]",
            self.path.display(),
            format_file_size_alt(self.size)
        )
    }
}

impl Display for FileAnalysis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FileAnalysis ({} ({}))",
            self.path.display(),
            format_file_size_alt(self.size)
        )
    }
}

impl FileAnalysis {
    // Создание объекта FileAnalysis.
    pub fn new(path: PathBuf, size: u64) -> Self {
        Self {
            path,
            size,
            lines: 0,
            code_lines: 0,
        }
    }
}

/// Отобрать по заданному пути все файлы, соответствующие критериям.
///
/// Args:
/// * path — целевой путь для поиска файлов.
pub async fn collect_files(path: PathBuf) -> Result<Vec<FileAnalysis>, Box<dyn Error>> {
    let mut files: Vec<FileAnalysis> = mapping_dirs(&path).await?;
    Ok(files)
}

/// Организатор сборки древа файлов.
#[async_recursion]
async fn mapping_dirs(path: &PathBuf) -> Result<Vec<FileAnalysis>, Box<dyn Error>> {
    let mut files = Vec::<FileAnalysis>::new();
    let mut catalog = fs::read_dir(path).await?;

    while let Some(catalog_elems) = catalog.next_entry().await? {
        let elem = catalog_elems.path();

        if elem.is_dir() {
            if is_valid_dir(&elem) {
                let sub_files = mapping_dirs(&elem).await?;
                files.extend(sub_files);
            }
        } else {
            if is_valid_extension(&elem) {
                let length_file = length_file(&elem);
                files.push(FileAnalysis::new(elem, length_file));
            }
        }
    }

    Ok(files)
}

/// Определить размер файла.
fn length_file(file: &PathBuf) -> u64 {
    file.metadata().unwrap().len()
}

/// Проверка, что директория не в исключённых из поиска.
fn is_valid_dir(dir: &PathBuf) -> bool {
    dir.file_name()
        .and_then(|s| s.to_str())
        .map(|name| !(name.starts_with(".") || name.starts_with("~") || is_technical_dir(name)))
        .unwrap_or(false)
}

/// Проверка на соответствие имени директории константам технических директорий.
fn is_technical_dir(dir_name: &str) -> bool {
    // Список типов директорий, которые считаются техническими.

    TECHNICAL_DIRS.contains(&dir_name.to_lowercase().as_str())
}

/// Проверка, что расширение файла соответствует искомому.
fn is_valid_extension(file: &PathBuf) -> bool {
    file.extension()
        .and_then(|s| s.to_str())
        .map(|name| is_correct_ext(name))
        .unwrap_or(false)
}

/// Проверка на соответствие расширения файла константам расширений.
fn is_correct_ext(ext: &str) -> bool {
    VALID_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}
