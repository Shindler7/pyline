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
#[cfg(debug_assertions)]
pub const VALID_EXTENSIONS: &[&str] = &["rs"];
#[cfg(not(debug_assertions))]
const VALID_EXTENSIONS: &[&str] = &["py"];

pub const PYTHON_KEYWORDS: [&str; 35] = [
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
    "with", "yield",
];

#[derive(Debug)]
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
}
