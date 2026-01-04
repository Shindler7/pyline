//! Supporting traits of the application.

use crate::collector::FileData;
use crate::errors::PyLineError;

/// Core trait for language-specific code parsers.
///
/// Defines the interface that all language parsers must implement. This trait provides
/// the framework for file processing, statistics collection, and result aggregation.
/// Language-specific implementations are typically generated using the [`impl_lang_parser!`]
/// macro, which handles common boilerplate while requiring manual implementation of
/// language-specific parsing logic.
///
/// # Type Parameters
/// - `Code` — The concrete parser type that implements this trait (usually `Self`).
///
/// # Architecture
/// The trait follows a two-phase processing model:
/// 1. **File-level processing**: `parse()` method handles multiple files asynchronously
/// 2. **Line-level processing**: Language-specific logic in `parse_code_lines()` (not part of
///    trait)
///
/// # Usage Pattern
/// 1. Implement or generate a struct with `stats: CodeFilesStat` and `keywords: HashMap`
/// 2. Use `impl_lang_parser!` macro to generate common implementation
/// 3. Manually implement language-specific line parsing methods
///
/// # Examples
/// See [`Python`] and [`Rust`] in the `parser` module for complete implementations.
pub trait CodeParsers {
    /// The concrete parser type (typically `Self`).
    type Code: Default;

    /// Constructs a new empty instance of the type.
    fn new() -> Self::Code {
        Default::default()
    }

    /// Creates an object for parsing a single file.
    ///
    /// This differs from `new()` in that it performs initial setup like
    /// counting the first file. Used internally by the parsing infrastructure.
    fn new_one() -> Self::Code;

    /// Merges another parser instance into this one, consuming it.
    ///
    /// Combines statistics and keyword counts from both instances.
    /// This is used to aggregate results from multiple files.
    fn merge(&mut self, other: Self::Code);

    /// Borrowing version of `merge()`.
    ///
    /// Useful when you need to preserve the original instance.
    fn merge_ref(&mut self, other: &Self::Code);

    /// Parses the provided list of files in asynchronous mode.
    ///
    /// ## Arguments
    /// * `files` — vector of [`FileData`] instances.
    ///
    /// ## Returns
    ///
    /// Statistics data if parsing is successful, or [`PyLineError`]
    /// if an error occurs during parsing.
    fn parse(&mut self, files: &[FileData])
    -> impl Future<Output = Result<(), PyLineError>> + Send;

    /// Increment the files_count value by 1.
    fn count_file(&mut self);

    /// Increment the files_invalid value by 1.
    fn count_invalid_file(&mut self);

    /// Increment the total_lines value by 1.
    fn count_line(&mut self);

    /// Increment the code_lines value by 1.
    fn count_code_line(&mut self);
}

/// Extension trait for collections of [`FileData`] providing verbose
/// formatting utilities.
///
/// This trait adds methods to format file data collections with detailed
/// information suitable for verbose output modes. It can be implemented for
/// any collection type containing [`FileData`] instances.
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// use pyline_libs::traits::FileDataExt;
/// use pyline_libs::collector::FileData;
///
/// let path = PathBuf::from("/test.py");
///
/// let files: Vec<FileData> = vec!(FileData::new(path, 999)); // ... get files
/// let verbose_list = files.join_verbose("\n");
/// println!("Files:\n{}", verbose_list);
/// ```
pub trait FileDataExt {
    /// Joins file data items into a single string with detailed information.
    ///
    /// Each file is represented using its `verbose_display` method,
    /// and items are separated by the specified delimiter.
    fn join_verbose(&self, sep: &str) -> String;
}
