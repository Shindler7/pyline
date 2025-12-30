//! Supporting traits of the application.

use crate::collector::FileData;
use crate::errors::PyLineError;

pub trait CodeParsers {
    type Code;

    /// Constructs a new empty instance of the type.
    fn new() -> Self;

    /// Creates an object for parsing a single file.
    fn new_one() -> Self;

    /// Idiomatic method for merging statistics from Python code files.
    fn update_with(&mut self, result: &Self::Code) {}

    /// Parses the provided list of files in asynchronous mode.
    ///
    /// ## Arguments
    /// * `files` — vector of [`FileData`] instances.
    ///
    /// ## Returns
    ///
    /// Statistics data if parsing is successful, or [`PyLineError`]
    /// if an error occurs during parsing.
    fn parse(
        &mut self,
        files: Vec<FileData>,
    ) -> impl Future<Output = Result<Self::Code, PyLineError>> + Send;

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
    /// Each file is represented using its [`verbose_display`] method,
    /// and items are separated by the specified delimiter.
    fn join_verbose(&self, sep: &str) -> String;
}
