//! Declarative macros for generating language parser implementations.
//!
//! Reduces boilerplate when adding a new language by generating:
//! - the parser struct with statistics and keyword tracking;
//! - the [`crate::CodeParsers`] trait implementation;
//! - the [`std::fmt::Display`] implementation for statistics.
//!
//! Language-specific logic (`parse_code_lines`, `is_code_line`,
//! `extract_keywords`) must be implemented manually.
//!
//! - [`define_lang_struct!`] — defines the parser struct;
//! - [`display_for_lang!`] — implements [`std::fmt::Display`];
//! - [`impl_lang_parser!`] — implements [`crate::CodeParsers`].

/// Implements [`std::fmt::Display`] for a language statistics struct.
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
                std::write!(f, "{}", self.stats)?;
                if !self.keywords.is_empty() {
                    std::write!(f, "\n\nKeywords:")?;

                    let mut sorted_keywords: Vec<_> = self.keywords.iter().collect();
                    sorted_keywords.sort_by(|a, b| b.1.cmp(a.1));
                    for (keyword, count) in sorted_keywords {
                        std::write!(f, "\n  {} = {}", keyword, count)?;
                    }
                }

                Ok(())
            }
        }
    };
}

/// Defines a language statistics struct with keyword tracking.
///
/// The generated struct implements [`std::fmt::Display`] via [`display_for_lang!`].
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
            pub stats: $crate::CodeFilesStat,

            /// Keyword frequency counts.
            pub keywords: std::collections::HashMap<String, usize>,
        }

        $crate::display_for_lang!($name);
    };
}

/// Implements [`crate::CodeParsers`] for a language type.
///
/// The type must implement `Default` and provide a method
/// `fn parse_code_lines(BufReader<File>) -> Result<Self, PyLineError>`.
///
/// The [`crate::CodeParsers`] trait must be in scope at the call site.
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
        impl $crate::CodeParsers for $Lang {
            fn new() -> $Lang {
                $Lang::default()
            }

            fn merge(&mut self, other: Self) {
                self.stats.merge(other.stats);
                for (keyword, count) in other.keywords {
                    *self.keywords.entry(keyword).or_insert(0) += count;
                }
            }

            fn parse(
                &mut self,
                files: &[$crate::FileData],
            ) -> Result<(), $crate::errors::PyLineError> {
                use rayon::prelude::*;

                if files.is_empty() {
                    return Err($crate::errors::PyLineError::NoFilesFound);
                }

                let final_stats = files
                    .par_iter()
                    .fold(
                        || Self::new(),
                        |mut acc, file| {
                            match Self::parse_file(file) {
                                Ok(file_stats) => {
                                    acc.merge(file_stats);
                                }
                                Err(_) => {
                                    acc.stats.num_files_invalid += 1;
                                    acc.stats.num_files_total += 1;
                                }
                            }

                            acc
                        },
                    )
                    .reduce(
                        || Self::new(),
                        |mut thread_acc_a, thread_acc_b| {
                            thread_acc_a.merge(thread_acc_b);
                            thread_acc_a
                        },
                    );

                self.merge(final_stats);

                Ok(())
            }

            /// Asynchronously parses a single code file and extracts statistics.
            ///
            /// Opens the file, reads it line by line, and analyzes code patterns.
            fn parse_file(file: &$crate::FileData) -> Result<Self, $crate::errors::PyLineError> {
                let file = std::fs::File::open(file.path())?;
                let cursor = std::io::BufReader::new(file);

                let code_stats = Self::parse_code_lines(cursor)?;

                Ok(code_stats)
            }
        }

        impl $Lang {

            /// Builds a parser from a single file's parsed statistics.
            pub(crate) fn from_parse(
                lines_total: usize,
                code_lines: usize,
                keywords: std::collections::HashMap<String, usize>,
            ) -> Self {
                Self {
                    stats: CodeFilesStat {
                        lines_total,
                        code_lines,
                        num_files_total: 1,
                        num_files_invalid: 0,
                    },
                    keywords,
                }
            }
        }
    };
}
