//! Структуры и методы для парсинга языков.

use crate::python::python::Python;
use std::fmt::{Display, Formatter};

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

impl std::ops::AddAssign for CodeStats {
    fn add_assign(&mut self, other: Self) {
        self.files_count += other.files_count;
        self.files_invalid += other.files_invalid;
        self.code_lines += other.code_lines;
        self.total_lines += other.total_lines;
    }
}

/// Статистика разбора строк кода Python.
#[derive(Debug, Default, Clone)]
pub struct CodeStatsPython {
    pub stats: CodeStats,
    /// Структура ключевых слов Python.
    pub keywords: Python,
}

impl Display for CodeStatsPython {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Code stats: {}", self.stats)?;
        write!(f, "")
    }
}

impl CodeStatsPython {
    pub fn new() -> Self {
        Self {
            keywords: Python::new(),
            ..Default::default()
        }
    }

    // Получить ссылку на базовую структуру.
    pub fn stats(&self) -> &CodeStats {
        &self.stats
    }

    // Получить мутабельную ссылку на базовую структуру.
    pub fn stats_mut(&mut self) -> &mut CodeStats {
        &mut self.stats
    }

    /// Объединение значений с другим экземпляром структуры.
    pub fn update_with(&mut self, other: Self) {
        *self.stats_mut() += *other.stats();

        // Объединить keywords
        self.keywords.update_with(&other.keywords);
    }

    /// Увеличить значение files_count на 1.
    pub fn count_file(&mut self) {
        self.stats.files_count += 1;
    }
    /// Увеличить значение files_invalid на 1.
    pub fn count_invalid_file(&mut self) {
        self.stats.files_invalid += 1;
    }
    /// Увеличить значение total_lines на 1.
    pub fn count_line(&mut self) {
        self.stats.total_lines += 1;
    }
    /// Увеличить значение code_lines на 1.
    pub fn count_code_line(&mut self) {
        self.stats.code_lines += 1;
    }
}
