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
