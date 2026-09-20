//! Declarative macros for generating language parser implementations.
//!
//! Reduces boilerplate when adding a new language by generating:
//! - the parser struct with statistics and keyword tracking;
//! - the [`CodeParsers`] trait implementation;
//! - the [`Display`] implementation for statistics.
//!
//! Language-specific logic (`parse_code_lines`, `is_code_line`,
//! `extract_keywords`) must be implemented manually.

/// Implements [`Display`] for a language statistics struct.
///
/// The output contains the base statistics followed by keyword
/// frequencies sorted in descending order.
///
/// # Examples
///
/// ```
/// use pyline_libs::display_for_lang;
/// # use pyline_libs::parser::CodeFilesStat;
/// # use std::collections::HashMap;
///
/// struct Pascal {
///     stats: CodeFilesStat,
///     keywords: HashMap<String, usize>,
/// }
///
/// display_for_lang!(Pascal);
/// ```
#[macro_export]
macro_rules! display_for_lang {
    ($instance: ident) => {
        impl std::fmt::Display for $instance {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/// Defines a language statistics struct with keyword tracking.
///
/// The generated struct implements [`Display`] via
/// [`display_for_lang!`].
///
/// # Examples
///
/// ```
/// use pyline_libs::define_lang_struct;
/// # use pyline_libs::display_for_lang;
/// # use pyline_libs::parser::CodeFilesStat;
///
/// define_lang_struct!(Pascal);
/// ```
#[macro_export]
macro_rules! define_lang_struct {
    ($name:ident) => {

        #[doc = concat!("Statistics for the `", stringify!($name), "` parser.")]
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

/// Implements [`CodeParsers`] for a language type.
///
/// The type must derive `Default` and `Clone` and provide an async
/// `parse_code_lines` method with the signature
/// `async fn(BufReader<File>, &mut Self) -> Result<(), PyLineError>`.
/// The [`CodeParsers`] trait must be in scope at the call site.
///
/// # Examples
///
/// ```no_run
/// # use pyline_libs::impl_lang_parser;
/// // ...
/// ```
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
                self.stats.num_files_invalid += 1;
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

                let code_file = tokio::fs::File::open(&file.path()).await?;
                let cursor = tokio::io::BufReader::new(code_file);
                Self::parse_code_lines(cursor, &mut code_stats).await?;

                Ok(code_stats)
            }
        }
    };
}
