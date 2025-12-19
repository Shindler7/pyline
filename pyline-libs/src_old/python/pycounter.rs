//! Подсчёт строк кода.

use crate::codestats::CodeStatsPython;
use crate::python::pybase::{Python, QuoteType};
use crate::scanner::FileAnalysis;
use futures::future::join_all;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

/// Методы подсчёта статистики строк кода.
impl CodeStatsPython {
    /// Создание представления под обработку одного файла.
    fn new_one_file() -> Self {
        let mut codestats = CodeStatsPython::new_with_keywords();
        codestats.count_file();
        codestats
    }

    /// Запустить парсинг переданного списка файлов.
    pub async fn parsing_files(
        &mut self,
        file_list: Vec<FileAnalysis>,
    ) -> Result<CodeStatsPython, Box<dyn std::error::Error>> {
        if file_list.is_empty() {
            return Ok(self.clone());
        }

        // Диспетчеризация чтения файлов.
        self.parse_manager(file_list).await;

        // Результаты.
        Ok(self.clone())
    }

    /// Организатор обработки файлов в асинхронном режиме.
    async fn parse_manager(&mut self, file_list: Vec<FileAnalysis>) {
        let tasks = file_list.iter().map(Self::parse_file).collect::<Vec<_>>();

        let results = join_all(tasks).await;

        for result in results {
            self.update_with(result);
        }
    }

    async fn parse_file(file: &FileAnalysis) -> CodeStatsPython {
        let mut result = CodeStatsPython::new_one_file();

        let maybe_file = File::open(&file.path).await;
        let code_file = match maybe_file {
            Ok(f) => f,
            Err(_) => {
                result.count_invalid_file();
                return result;
            }
        };

        let cursor = BufReader::new(code_file);
        Self::parse_code_lines(cursor, &mut result).await;

        result
    }

    /// Функция, которая отвечает непосредственно за разбор кода.
    async fn parse_code_lines(cursor: BufReader<File>, code_stats: &mut CodeStatsPython) {
        let mut triple_quotes: Option<QuoteType> = None;
        let mut inside_triple_quotes = false;

        let mut lines = cursor.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            code_stats.count_line();

            let trimmed = line.trim();

            // Проверка тройных кавычек.
            if inside_triple_quotes {
                // Входные тройные кавычки ранее были найдены. Проверяем, есть ли выходные.
                if Self::are_there_quotation(trimmed, triple_quotes) {
                    triple_quotes = None;
                    inside_triple_quotes = false;
                }
                continue;
            } else {
                // Тройные кавычки могут быть в одну строку: """docstrings this""".
                if Self::is_triple_quotes_line(trimmed) {
                    continue;
                }

                if let Some(quotes) = Self::check_quotes_on_start(trimmed) {
                    triple_quotes = Some(quotes);
                    inside_triple_quotes = true;
                    continue;
                }
            }

            // Игнорируем пустые строки и очевидные комментарии в коде.
            if Self::is_empty_or_comment(&line) {
                continue;
            }

            // Отсеяли комментарии, пустые строки, тройные кавычки. Скорее всего, здесь код.
            code_stats.count_code_line();

            // Разбор строки кода на составляющие.
            Self::parse_code_line(trimmed, code_stats.keywords_mut())
        }
    }

    /// Проверка на пустую линию или линию закрытую комментарием.
    fn is_empty_or_comment(line_trim: &str) -> bool {
        line_trim.is_empty() || line_trim.starts_with("#")
    }

    /// Проверка на строку закрытую тройными кавычками.
    ///
    /// ```python
    /// """This is docstring"""
    /// ```
    fn is_triple_quotes_line(line_trim: &str) -> bool {
        [
            QuoteType::TripleSingle.as_str(),
            QuoteType::TripleDouble.as_str(),
        ]
        .iter()
        .any(|triple| line_trim.starts_with(triple) && line_trim.ends_with(triple))
    }

    /// Проверка, что строка начинается с тройных кавычек.
    /// Если кавычки есть, возвращает найденный тип.
    fn check_quotes_on_start(line_trim: &str) -> Option<QuoteType> {
        if line_trim.starts_with(QuoteType::TripleSingle.as_str()) {
            Some(QuoteType::TripleSingle)
        } else if line_trim.starts_with(QuoteType::TripleDouble.as_str()) {
            Some(QuoteType::TripleDouble)
        } else {
            None
        }
    }

    /// Проверка, что в переданной строке есть заданные кавычки.
    ///
    /// Функция вызывается, когда ранее тройные кавычки были обнаружены и идёт поиск выхода.
    /// Закрывающие кавычки могут быть в начале строки и в конце строки.
    fn are_there_quotation(line_trim: &str, quotes: Option<QuoteType>) -> bool {
        quotes.is_some_and(|q| line_trim.starts_with(q.as_str()) || line_trim.ends_with(q.as_str()))
    }

    /// Разбор строки с кодом.
    fn parse_code_line(line_trim: &str, python: &mut Python) {

    }
}
