//! Declarative macros for implementing language parsers with consistent
//! formatting.
//!
//! Provides a set of macros that reduce boilerplate when creating
//! language-specific parsing implementations. The macros enforce
//! a standardized structure for:
//! - Code statistics collection and display
//! - Parser trait implementation
//! - Result formatting and keyword tracking
//!
//! These macros follow the "inversion of control" pattern, generating
//! framework code while requiring manual implementation of language-specific
//! parsing logic.
//!
//! # Core Components
//! - [`display_for_lang!`] - Implements `Display` with standardized output
//!   format
//! - [`define_lang_struct!`] - Defines a language analysis structure
//!   with statistics
//! - [`impl_lang_parser!`] - Implements the `CodeParsers` trait with async
//!   file processing

/// Implements `Display` trait for code statistics structures.
///
/// Generates a standardized output format showing:
/// - Base statistics (lines, files, code lines)
/// - Keyword frequencies sorted by count (descending)
///
/// ## Usage
/// ```
/// use pyline_libs::display_for_lang;
/// use std::fmt::{Display, Formatter};
/// use std::collections::HashMap;
/// use pyline_libs::parser::CodeFilesStat;
///
///
/// struct Pascal {
///     pub stats: CodeFilesStat,
///     pub keywords: HashMap<String, usize>,
/// }
///
/// display_for_lang!(Pascal);
/// ```
#[macro_export]
macro_rules! display_for_lang {
    ($instance: ident) => {
        impl Display for $instance {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "{}", self.stats)?;
                if !self.keywords.is_empty() {
                    write!(f, "\n\nKeywords:")?;

                    let mut sorted_keywords: Vec<_> = self.keywords.iter().collect();
                    sorted_keywords.sort_by(|a, b| b.1.cmp(a.1));
                    for (keyword, count) in sorted_keywords {
                        write!(f, "\n  {} = {}", keyword, count)?;
                    }
                }

                Ok(())
            }
        }
    };
}

/// Defines a language-specific code analysis structure.
///
/// Creates a struct with file statistics and keyword tracking,
/// automatically implementing `Display` formatting.
///
/// ## Example
/// ```
/// use pyline_libs::define_lang_struct;
/// use pyline_libs::parser::CodeFilesStat;
/// use pyline_libs::display_for_lang;
/// use std::fmt::{Display, Formatter};
/// use std::collections::HashMap;
///
/// define_lang_struct!(CodeLang);
/// ```
#[macro_export]
macro_rules! define_lang_struct {
    ($name:ident) => {
        /// Structure for parsing Python files.
        #[derive(Debug, Default, Clone)]
        pub struct $name {
            /// File statistics (lines, files, code lines).
            pub stats: CodeFilesStat,
            /// Keyword frequency counts.
            pub keywords: std::collections::HashMap<String, usize>,
        }

        display_for_lang!($name);
    };
}

/// A declarative macro for implementing language-specific parsers with minimal boilerplate.
///
/// This macro generates the complete implementation of the `CodeParsers` trait for a given
/// language type, handling common parsing logic while allowing customization through
/// language-specific methods.
///
/// # Design Philosophy
/// The macro follows the "inversion of control" principle - it provides the framework
/// (file handling, merging, statistics tracking) while delegating language-specific
/// parsing logic to methods that must be implemented manually.
///
/// # Prerequisites
/// The target type `$Lang` must implement:
/// - `Default` trait (for initialization)
/// - `Clone` trait (for result propagation)
/// - Have the following fields:
///   - `stats: CodeStats` - for statistical tracking
///   - `keywords: HashMap<LangKeyword, usize>` - for keyword frequency counting
///
/// # Required Manual Implementations
/// After using this macro, you MUST implement these methods on `$Lang`:
///
/// ```ignore
/// impl $Lang {
///     /// Core parsing logic that processes individual lines of code.
///     /// This is where language-specific syntax analysis happens.
///     async fn parse_code_lines(
///         reader: tokio::io::BufReader<tokio::fs::File>,
///         stats: &mut Self,
///     ) -> Result<(), PyLineError> { /* ... */ }
///
///     /// Determines if a line should be counted as code (not comment/empty).
///     /// Language-specific logic for identifying actual code lines.
///     fn is_code_line(line: &str) -> bool { /* ... */ }
///
///     /// Extracts and counts keywords from a line of code.
///     /// Language-specific keyword recognition logic.
///     fn extract_keywords(line: &str) { /* ... */ }
/// }
/// ```
///
/// # Generated Implementation
/// The macro generates:
/// 1. Complete `CodeParsers` trait implementation including:
///    - `new_one()` - Creates a new parser instance with file counting
///    - `merge()`/`merge_ref()` - Combines statistics from multiple parses
///    - `parse()` - Asynchronously processes multiple files
///    - Counting methods for files and lines
///
/// 2. A private `parse_file()` method that:
///    - Opens and reads files asynchronously
///    - Delegates line-by-line parsing to `parse_code_lines()`
///    - Handles file I/O errors gracefully
///
/// # Example Usage
/// ```no_run
/// use std::collections::HashMap;
/// use pyline_libs::collector::FileData;
/// use pyline_libs::traits::CodeParsers;
/// use pyline_libs::parser::CodeFilesStat;
/// use pyline_libs::errors::PyLineError;
/// use pyline_libs::impl_lang_parser;
/// use tokio::fs::File;
/// use tokio::io::BufReader;
///
///
/// #[derive(Default, Clone)]
/// struct PythonParser {
///     stats: CodeFilesStat,
///     keywords: HashMap<String, usize>,
/// }
///
/// // Generate the boilerplate implementation
/// impl_lang_parser!(PythonParser);
///
/// // Then implement the language-specific methods
/// impl PythonParser {
///     async fn parse_code_lines(reader: BufReader<File>, stats: &mut Self) -> Result<(), PyLineError> {
///         // Python-specific line parsing
///         Ok(())
///     }
///
///     fn is_code_line(line: &str) -> bool {
///         // Python-specific code line detection
///         !line.trim_start().starts_with('#') && !line.trim().is_empty()
///     }
///
///     fn extract_keywords(line: &str) {
///         // Python keyword extraction logic
///     }
/// }
/// ```
///
/// # Error Handling
/// The generated code handles:
/// - File not found errors (returns `PyLineError`)
/// - Invalid file errors (counts them in statistics)
/// - I/O errors during file reading
///
/// # Performance Characteristics
/// - Uses asynchronous I/O for parallel file processing
/// - Efficient merging of statistics using `HashMap` operations
/// - Minimal allocations through careful use of references
///
/// # Dependencies
/// Requires the following in scope:
/// - `futures::future::join_all` for parallel processing
/// - `tokio::fs::File` and `tokio::io::BufReader` for async I/O
/// - `$crate::errors::PyLineError` for error types
/// - `CodeParsers` trait definition
///
/// # Notes
/// - The macro assumes the use of Tokio runtime for async operations
/// - Files are processed in parallel when using `parse()`
/// - Statistics are aggregated incrementally to minimize memory usage
#[macro_export]
macro_rules! impl_lang_parser {
    (
        $Lang:ident
    ) => {
        impl CodeParsers for $Lang {
            type Code = $Lang;

            fn new_one() -> Self {
                let mut code_stat = Self::default();
                code_stat.count_file();
                code_stat
            }

            fn merge(&mut self, other: Self) {
                self.stats.merge(other.stats);
                for (keyword, count) in other.keywords {
                    *self.keywords.entry(keyword).or_insert(0) += count;
                }
            }

            fn merge_ref(&mut self, other: &Self) {
                self.stats.merge_ref(&other.stats);
                for (keyword, count) in &other.keywords {
                    *self.keywords.entry(keyword.clone()).or_insert(0) += count;
                }
            }

            async fn parse(
                &mut self,
                files: &[FileData],
            ) -> Result<(), $crate::errors::PyLineError> {
                if files.is_empty() {
                    return Err($crate::errors::PyLineError::NoFilesForParse);
                }

                let tasks: Vec<_> = files.iter().map(Self::parse_file).collect();
                let results = futures::future::join_all(tasks).await;

                for result in results {
                    match result {
                        Ok(result) => self.merge(result),
                        Err(_) => self.count_invalid_file(),
                    }
                }

                Ok(())
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

        impl $Lang {
            /// Asynchronously parses a single Python file and extracts code
            /// statistics.
            ///
            /// Opens the file, reads it line by line, and analyzes Python code
            /// patterns.
            async fn parse_file(file: &FileData) -> Result<Self, $crate::errors::PyLineError> {
                let mut code_stats = Self::new_one();

                let code_file = tokio::fs::File::open(&file.path).await?;
                let cursor = tokio::io::BufReader::new(code_file);
                Self::parse_code_lines(cursor, &mut code_stats).await?;

                Ok(code_stats)
            }
        }
    };
}
