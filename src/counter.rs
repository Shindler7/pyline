//! Подсчёт строк кода.

use crate::scanner::FileAnalysis;
use std::fmt::{Display, Formatter};

/// Статистика подсчёта строк кода.
#[derive(Debug, Default)]
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
        writeln!(f, "Количество файлов: {}", self.files_count)?;
        writeln!(f, "Обработано строк: {}", self.total_lines)?;
        write!(f, "  из них строк с кодом: {}", self.code_lines)?;
        if self.files_invalid > 0 {
            write!(f, "\nНе удалось прочитать файлов: {}", self.files_invalid)?;
        }
        write!(f, "")
    }
}

/// Методы подсчёта статистики строк кода.
impl CodeStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Запустить парсинг переданного списка файлов.
    pub async fn parsing_files(
        &mut self,
        file_list: Vec<FileAnalysis>,
    ) -> Result<CodeStats, Box<dyn std::error::Error>> {
        let mut stats = CodeStats::new();
        if file_list.is_empty() {
            return Ok(stats);
        }

        self.files_count = file_list.len();

        // Диспетчеризация чтения файлов.

        Ok(stats)
    }
}
