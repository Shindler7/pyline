//! Сканирование и отбор файлов для расчётов.

use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use tokio::fs;

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
    /// Размер файла.
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
            "FileAnalysis [{} ({} кб)]",
            self.path.display(),
            self.size
        )
    }
}

impl Display for FileAnalysis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FileAnalysis ({} ({} кб))",
            self.path.display(),
            self.size
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

pub fn scan_files(path: PathBuf) -> Result<Vec<FileAnalysis>, Box<dyn std::error::Error>> {
    let mut files: Vec<FileAnalysis> = vec![];
    for i in (1000..=3000).step_by(1000) {
        let filename = format!("{i}.py");
        let new_path = path.join("scan").join(&filename);

        let file = FileAnalysis::new(new_path, i);
        files.push(file);
    }

    Ok(files)
}

/// Отобрать по заданному пути все файлы, соответствующие критериям.
///
/// Args:
/// * path — целевой путь для поиска файлов.
pub async fn collect_files(path: PathBuf) -> Result<Vec<FileAnalysis>, Box<dyn std::error::Error>> {
    let mut files: Vec<FileAnalysis> = vec![];

    mapping_dirs(&path).await;

    Ok(files)
}

async fn mapping_dirs(path: &PathBuf) {
    let mut dirs = fs::read_dir(path).await.unwrap();
    while let Some(dirs) = dirs.next_entry().await.unwrap() {
        if dirs.path().is_dir() {
            println!("{:?}", dirs.path());
        }
    }
}
