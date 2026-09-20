// use crate::cli::{ArgsResult, CodeLang};
// use CodeLang::*;
// use pyline_libs::{py::base as py_base, rust::base as rust_base};
//
// pub(super) fn normalize_ext_by_lang(extensions: &mut Vec<String>, lang: &CodeLang) {
//     let basic_ext = match lang {
//         Python => py_base::VALID_EXTENSIONS,
//         Rust => rust_base::RUST_VALID_EXTENSIONS,
//     };
//
//     normalize_list(basic_ext, extensions, true)
// }
//
// /// Universal method for normalizing string lists.
// ///
// /// Merges default values with user-provided entries, normalizing them
// /// to a common format. When `normalize_dot: true`, removes leading dots
// /// (for file extensions), guarantees uniqueness through sorting
// /// and deduplication.
// fn normalize_list(user_data: &mut [String], defaults: &[&str], normalize_dot: bool) -> Vec<String> {
//
//     let mut result = Vec::with_capacity(user_data.len() + defaults.len());
//
//     result.extend(defaults.iter().map(|s| s.to_string()));
//
//     for item in user_data.iter_mut() {
//         let mut norm = std::mem::take(item);
//
//
//         normalize_case(item);
//
//
//         let mut norm = normalize_case(item);
//
//         if normalize_dot {
//             norm = norm.trim_start_matches('.').to_string();
//         }
//
//         result.push(norm);
//     }
// }
//
// /// Normalizes string case according to platform conventions.
// ///
// /// On Windows, converts to lowercase for case-insensitive consistency.
// /// On other platforms, returns the string unchanged.
// #[cfg(windows)]
// fn normalize_case(s: &mut String) {
//     let lowered = s.to_lowercase();
//     *s = lowered;
// }
//
// #[cfg(not(windows))]
// fn normalize_case(_s: &mut String) -> String {}
//
//
// impl ArgsResult {
//     /// Creates a normalized copy of the arguments with language-aware extensions.
//     ///
//     /// This method returns a new instance where file extensions are processed to ensure:
//     /// - All extensions have leading dots
//     /// - Language-specific default extensions are included
//     /// - Duplicate extensions are removed
//     ///
//     /// The original instance remains unchanged (following Rust's immutability principles).
//     ///
//     /// # Example
//     ///
//     /// ```
//     /// let args = ArgsResult {
//     ///     lang: CodeLang::Python,
//     ///     ext: vec!["py".to_string(), ".txt".to_string()],
//     ///     // other fields...
//     /// };
//     ///
//     /// let normalized = args.normalize_by_lang();
//     /// // normalized.ext will contain: [".py", ".txt"]
//     /// // (".py" added by default for Python, ".txt" from user input)
//     /// ```
//     pub fn normalize_by_lang(&self) -> Self {
//         let mut normalize_self = self.clone();
//         normalize_self.extension = self.normalize_ext_by_lang();
//
//         if !self.auto_config {
//             return normalize_self;
//         }
//
//         // auto-config execution.
//         normalize_self.dirs = self.exclude_dirs_by_lang();
//         normalize_self.marker_files = self.exclude_marker_files_by_lang();
//         normalize_self.filenames = self.exclude_filenames_by_lang();
//
//         normalize_self
//     }
//
//     /// Normalizes the list of directories excluded by default for the current
//     /// language.
//     ///
//     /// Merges language-specific default exclusions with user-provided
//     /// directories, respecting the `ignore_dot_dirs` flag for handling hidden
//     /// directories.
//     fn exclude_dirs_by_lang(&self) -> Vec<String> {
//         let (dirs, dot_dirs) = match self.lang {
//             CodeLang::Python => (py_base::EXCLUDE_DIRS, py_base::EXCLUDE_DOT_DIRS),
//             CodeLang::Rust => (
//                 rust_base::RUST_EXCLUDE_DIRS,
//                 rust_base::RUST_EXCLUDE_DOT_DIRS,
//             ),
//         };
//
//         let combined_defaults: Vec<&str> = if self.ignore_dot_dirs {
//             dirs.to_vec()
//         } else {
//             dirs.iter().chain(dot_dirs.iter()).copied().collect()
//         };
//
//         Self::normalize_list(&combined_defaults, &self.dirs, false)
//     }
//
//     /// Builds the list of marker files specific to the current language.
//     ///
//     /// Combines language-specific default markers with user-provided entries,
//     /// ensuring uniqueness of items in the resulting list.
//     fn exclude_marker_files_by_lang(&self) -> Vec<String> {
//         let default = match self.lang {
//             CodeLang::Python => py_base::MARKER_FILE,
//             CodeLang::Rust => rust_base::RUST_MARKER_FILE,
//         };
//
//         Self::normalize_list(default, &self.marker_files, false)
//     }
//
//     /// Generates the list of filenames to exclude based on language.
//     ///
//     /// Merges language-specific default exclusions with the user-provided list,
//     /// removing duplicates and maintaining sorted order.
//     fn exclude_filenames_by_lang(&self) -> Vec<String> {
//         let default = match self.lang {
//             CodeLang::Python => py_base::EXCLUDE_FILENAMES,
//             CodeLang::Rust => rust_base::RUST_EXCLUDE_FILENAMES,
//         };
//
//         Self::normalize_list(default, &self.filenames, false)
//     }
//
//     /// Normalizes the list of file extensions with language semantics.
//     ///
//     /// Adds language-specific default extensions to user-provided ones,
//     /// ensuring uniqueness and canonical format (without leading dots).
//     fn normalize_ext_by_lang(&self) -> Vec<String> {
//         let default = match self.lang {
//             CodeLang::Python => py_base::VALID_EXTENSIONS,
//             CodeLang::Rust => rust_base::RUST_VALID_EXTENSIONS,
//         };
//
//         Self::normalize_list(default, &self.extension, true)
//     }
//
//     // /// Universal method for normalizing string lists.
//     // ///
//     // /// Merges default values with user-provided entries, normalizing them
//     // /// to a common format. When `normalize_dot: true`, removes leading dots
//     // /// (for file extensions), guarantees uniqueness through sorting
//     // /// and deduplication.
//     // fn normalize_list(default: &[&str], user: &[String], normalize_dot: bool) -> Vec<String> {
//     //     let mut result: Vec<String> = default.iter().map(|s| s.to_string()).collect();
//     //
//     //     for item in user.iter() {
//     //         let mut norm = Self::normalize_case(item);
//     //
//     //         if normalize_dot {
//     //             norm = norm.trim_start_matches('.').to_string();
//     //         }
//     //
//     //         result.push(norm);
//     //     }
//     //
//     //     result.sort();
//     //     result.dedup();
//     //     result
//     // }
//     //
//     // /// Normalizes string case according to platform conventions.
//     // ///
//     // /// On Windows, converts to lowercase for case-insensitive consistency.
//     // /// On other platforms, returns the string unchanged.
//     // #[cfg(windows)]
//     // fn normalize_case(s: &str) -> String {
//     //     s.to_lowercase()
//     // }
//     //
//     // #[cfg(not(windows))]
//     // fn normalize_case(s: &str) -> String {
//     //     s.to_string()
//     // }
//
//     // /// Returns a detailed string representation suitable for verbose output.
//     // ///
//     // /// Shows all fields with their values, formatted for readability.
//     // pub fn verbose_display(&self) -> String {
//     //     let dirs = Self::join_or_wildcard(&self.dirs, ", ");
//     //     let filenames = Self::join_or_wildcard(&self.filenames, ", ");
//     //     let marker_files = Self::join_or_wildcard(&self.marker_files, ", ");
//     //
//     //     format!(
//     //         "Arguments:\n\
//     //          ├─ Path: {}\n\
//     //          ├─ Exclude Directories: {}\n\
//     //          ├─ Marker_files: {}\n\
//     //          ├─ Ignore dot dirs: {}\n\
//     //          ├─ Extensions: {}\n\
//     //          ├─ Exclude Filenames: {}\n\
//     //          ├─ Language: {:?}\n\
//     //          ├─ Skip gather errors: {}\n\
//     //          └─ Verbose: {}",
//     //         self.path.display(),
//     //         dirs,
//     //         marker_files,
//     //         self.ignore_dot_dirs,
//     //         self.extension.join(", "),
//     //         filenames,
//     //         self.lang,
//     //         self.skip_gather_errors,
//     //         self.verbose
//     //     )
//     // }
//
//     // fn join_or_wildcard<T: AsRef<str>>(items: &[T], separator: &str) -> String {
//     //     if items.is_empty() {
//     //         "not set".to_string()
//     //     } else {
//     //         items
//     //             .iter()
//     //             .map(|item| item.as_ref())
//     //             .collect::<Vec<_>>()
//     //             .join(separator)
//     //     }
//     // }
// }
//
// macro_rules! impl_string_collection {
//     ($name: ident, $normalize:path) => {
//         impl<T> FromIterator<T> for $name
//         where
//             T: Into<String>,
//         {
//             fn from_iter<I>(iter: I) -> Self
//             where
//                 I: IntoIterator<Item = T>,
//             {
//                 Self(iter.into_iter().map(|s| s.into()).collect())
//             }
//         }
//
//         impl Deref for $name {
//             type Target = HashSet<String>;
//             fn deref(&self) -> &Self::Target {
//                 &self.0
//             }
//         }
//     };
// }
