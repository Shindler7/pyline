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
/// use pyline_libs::collector::models::FileData;
/// use pyline_libs::traits::FileDataExt;
/// use std::path::PathBuf;
/// use crate::FileData;
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

pub trait LangDefaults {
    fn with_defaults(default: &[&str]) -> Self;
}
