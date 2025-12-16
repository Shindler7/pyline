//! Собственные ошибки приложения.

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PyLineError {
    ScannerError { description: String },
    CounterError { description: String },
}

impl Error for PyLineError {}

impl Display for PyLineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScannerError { description } => {
                write!(f, "Ошибка анализа древа файлов: {}", description)
            }
            Self::CounterError { description } => {
                write!(f, "Ошибка парсинга файла: {}", description)
            }
        }
    }
}
