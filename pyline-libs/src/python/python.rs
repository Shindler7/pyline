//! Исходные данные по работе с Python.

use std::collections::HashMap;

pub const TECHNICAL_DIRS: &[&str] = &[
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
pub const VALID_EXTENSIONS: &[&str] = &["py"];

pub const PYTHON_KEYWORDS: [&str; 35] = [
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
    "with", "yield",
];

#[derive(Debug, Default, Clone)]
pub struct Python {
    pub map_keywords: HashMap<String, usize>,
}

impl Python {
    /// Создать экземпляр для работы с парсингом Python.
    pub fn new() -> Python {
        let map_keywords = PYTHON_KEYWORDS
            .iter()
            .map(|&k| (k.to_lowercase(), 0))
            .collect::<HashMap<_, _>>();
        Self { map_keywords }
    }

    /// Идиоматичный метод для объединения статистики Python ключевых слов.
    ///
    /// # Примеры
    /// ```
    /// let mut python1 = Python::new();
    /// let mut python2 = Python::new();
    ///
    /// python1.map_keywords.insert("def".to_string(), 3);
    /// python2.map_keywords.insert("def".to_string(), 2);
    /// python2.map_keywords.insert("class".to_string(), 1);
    ///
    /// python1.update_with(&python2);
    ///
    /// assert_eq!(python1.map_keywords.get("def"), Some(&5));
    /// assert_eq!(python1.map_keywords.get("class"), Some(&1));
    /// ```
    pub fn update_with(&mut self, other: &Self) {
        for (keyword, count) in &other.map_keywords {
            if *count > 0 {
                if let Some(self_count) = self.map_keywords.get_mut(keyword) {
                    *self_count += count;
                }
                // Если ключевого слова нет в self.map_keywords, игнорируем.
            }
        }
    }
}
