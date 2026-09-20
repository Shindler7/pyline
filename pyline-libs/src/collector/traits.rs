//! Traits for file collection and language-specific defaults.

/// Verbose formatting for collections of [`crate::FileData`].
pub trait FileDataExt {
    /// Joins files into a single string using each file's verbose
    /// representation, separated by `sep`.
    fn join_verbose(&self, sep: &str) -> String;
}

/// Collection types that can be constructed from a predefined set of
/// string values.
pub trait WithDefaults {
    /// Creates an instance populated with `values`.
    fn with_defaults(default: &[&str]) -> Self;
}
