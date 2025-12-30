use crate::collector::FileData;
use crate::errors::PyLineError;
use crate::parser::Python;
use crate::py::base::{PyKeywords, KEYWORDS};
use crate::py::traits::PythonLineAnalysis;
use crate::traits::CodeParsers;
use futures::future::join_all;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};

impl CodeParsers for Python {
    type Code = Python;

    fn new() -> Python {
        Self {
            ..Default::default()
        }
    }

    fn new_one() -> Python {
        let mut code_stat = Self::new();
        code_stat.count_file();
        code_stat
    }

    fn update_with(&mut self, result: &Self::Code) {
        todo!()
    }

    async fn parse(&mut self, files: Vec<FileData>) -> Result<Python, PyLineError> {
        if files.is_empty() {
            return Err(PyLineError::NoFilesForParse);
        }

        self.parse_collector(&files).await?;
        Ok(self.clone())
    }

    fn count_file(&mut self) {
        self.stats.num_files_total += 1;
    }

    fn count_invalid_file(&mut self) {
        self.stats.num_files_not_valid += 1;
    }

    fn count_line(&mut self) {
        self.stats.lines_total += 1;
    }

    fn count_code_line(&mut self) {
        self.stats.code_lines += 1;
    }
}

impl Python {
    /// Asynchronously parses a collection of files and aggregates their
    /// statistics.
    ///
    /// Processes files in parallel using tasks, updates statistics for
    /// successfully parsed files, and counts invalid files separately.
    async fn parse_collector(&mut self, files: &[FileData]) -> Result<(), PyLineError> {
        let tasks: Vec<_> = files.iter().map(Self::parse_file).collect();
        let results = join_all(tasks).await;

        for result in results {
            match result {
                Ok(result) => {
                    self.update_with(&result);
                }
                Err(err) => {
                    self.count_invalid_file();
                }
            }
        }

        Ok(())
    }

    /// Asynchronously parses a single Python file and extracts code
    /// statistics.
    ///
    /// Opens the file, reads it line by line, and analyzes Python code
    /// patterns.
    async fn parse_file(file: &FileData) -> Result<Self, PyLineError> {
        let mut code_stats = Self::new_one();

        let code_file = File::open(&file.path).await?;
        let cursor = BufReader::new(code_file);
        Self::parse_code_lines(cursor, &mut code_stats).await?;

        Ok(code_stats)
    }

    /// Parses lines from a buffered file reader and updates Python code
    /// statistics.
    ///
    /// Analyzes each line to identify code lines, comments, and Python
    /// keywords, updating the provided statistics structure accordingly.
    async fn parse_code_lines(
        cursor: BufReader<File>,
        code_stats: &mut Python,
    ) -> Result<(), PyLineError> {

        let mut lines = cursor.lines();
        while let Some(line) = lines.next_line().await? {
            code_stats.count_line();

            let trimmed = line.trim();
            if line.is_non_code() {
                continue;
            }

            Self::inside_triple_quotes(&lines);



        }



        Ok(())
    }

    fn inside_triple_quotes(lines: &Lines<BufReader<File>>) {
        todo!()
    }

    fn parse_keywords(keyword: &str) -> Option<PyKeywords> {
        KEYWORDS.get(keyword.to_lowercase().as_str()).cloned()
    }
}
