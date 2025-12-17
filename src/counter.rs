//! Подсчёт строк кода.

use crate::scanner::FileAnalysis;
use futures::future::join_all;
use std::fmt::{Display, Formatter};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Статистика подсчёта строк кода.
#[derive(Debug, Default, Clone, Copy)]
pub struct CodeStats {
    /// Количество файлов.
    pub files_count: usize,
    /// Количество файлов, которые не получилось обработать.
    pub files_invalid: usize,
    /// Всего обработано строк.
    pub total_lines: usize,
    /// Всего строк кода.
    pub code_lines: usize,
}

impl Display for CodeStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Files: {}", self.files_count)?;
        writeln!(f, "Lines: {}", self.total_lines)?;
        write!(f, "  of which are code lines: {}", self.code_lines)?;
        if self.files_invalid > 0 {
            write!(f, "\nFailed to read files: {}", self.files_invalid)?;
        }
        write!(f, "")
    }
}

/// Методы подсчёта статистики строк кода.
impl CodeStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Создание представления под обработку одного файла.
    fn new_one_file() -> Self {
        Self {
            files_count: 1,
            ..Default::default()
        }
    }

    /// Объединение значений с другим экземпляром структуры.
    pub fn update_with(&mut self, other: Self) {
        self.files_count += other.files_count;
        self.files_invalid += other.files_invalid;
        self.total_lines += other.total_lines;
        self.code_lines += other.code_lines;
    }

    /// Запустить парсинг переданного списка файлов.
    pub async fn parsing_files(
        &mut self,
        file_list: Vec<FileAnalysis>,
    ) -> Result<CodeStats, Box<dyn std::error::Error>> {
        if file_list.is_empty() {
            return Ok(*self);
        }

        // Диспетчеризация чтения файлов.
        self.parse_manager(file_list).await;

        // Результаты.
        Ok(*self)
    }

    /// Организатор обработки файлов в асинхронном режиме.
    async fn parse_manager(&mut self, file_list: Vec<FileAnalysis>) {
        let tasks = file_list
            .iter()
            .map(|file| Self::parse_file(file))
            .collect::<Vec<_>>();

        let results = join_all(tasks).await;

        for result in results {
            self.update_with(result);
        }
    }

    async fn parse_file(file: &FileAnalysis) -> CodeStats {
        let mut result = CodeStats::new_one_file();

        // Открытие файла.
        let mut code_file = match File::open(&file.path).await {
            Ok(result) => result,
            Err(_) => {
                result.files_invalid += 1;
                return result;
            }
        };

        let mut chunk = vec![0; 4096];
        loop {
            let len = match code_file.read(&mut chunk).await {
                Ok(len) => len,
                Err(_) => {
                    result.files_invalid += 1;
                    return result;
                }
            };

            if len == 0 {
                // Конец файла.
                break;
            }

            for &b in &chunk[..len] {
                if b == b'\n' {
                    result.total_lines += 1;
                }
            }
        }

        result
    }

    fn parse_line() {}
}
