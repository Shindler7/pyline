//! Подсчёт строк кода.

use crate::codestats::CodeStatsPython;
use crate::scanner::FileAnalysis;
use futures::future::join_all;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

const BUFFER_SIZE: usize = 1024 * 8;

/// Методы подсчёта статистики строк кода.
impl CodeStatsPython {
    /// Создание представления под обработку одного файла.
    fn new_one_file() -> Self {
        let mut codestats = CodeStatsPython::new();
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
        let tasks = file_list
            .iter()
            .map(|file| Self::parse_file(file))
            .collect::<Vec<_>>();

        let results = join_all(tasks).await;

        for result in results {
            self.update_with(result);
        }
    }

    async fn parse_file(file: &FileAnalysis) -> CodeStatsPython {
        let mut result = CodeStatsPython::new_one_file();

        // Открытие файла.
        let mut code_file = match File::open(&file.path).await {
            Ok(result) => result,
            Err(_) => {
                result.count_invalid_file();
                return result;
            }
        };

        // Обработка содержимого.
        let mut chunk = vec![0; BUFFER_SIZE];
        loop {
            let len = match code_file.read(&mut chunk).await {
                Ok(len) => len,
                Err(_) => {
                    result.count_invalid_file();
                    return result;
                }
            };

            if len == 0 {
                break;
            }

            // Передача данных для парсинга сайта.
            Self::parse_codeline(&chunk, len, &mut result);
        }

        result
    }

    /// Функция, которая отвечает непосредственно за разбор кода.
    fn parse_codeline(chunk: &Vec<u8>, len_chunk: usize, code_stats: &mut CodeStatsPython) {
        for &b in &chunk[..len_chunk] {
            match b {
                b'\n' => code_stats.count_line(),
                _ => continue,
            }
        }
    }
}
