//! Parser types and language defaults.

pub(crate) mod defaults;
mod models;
pub(crate) mod py;
pub(crate) mod rust;
pub mod traits;

pub use models::{CodeFilesStats, CodeLanguage, ParseResults};
pub use traits::CodeParser;

use crate::{FileData, errors::PyLineError};
use rayon::prelude::*;

/// Parses `files` with the parser for `lang` and returns aggregate results.
///
/// # Errors
///
/// Returns [`PyLineError::NoFilesFound`] if `files` is empty.
pub fn run(files: &[FileData], lang: &CodeLanguage) -> Result<ParseResults, PyLineError> {
    if files.is_empty() {
        return Err(PyLineError::NoFilesFound);
    }

    let parser = lang.get_parser();

    let final_stats = files
        .par_iter()
        .fold(ParseResults::new, |mut acc, file| {
            if let Ok(files_stat) = parser.parse_file(file) {
                acc.merge(files_stat);
            } else {
                acc.stats.invalid_files += 1;
                acc.stats.total_files += 1;
            }

            acc
        })
        .reduce(ParseResults::new, |mut thread_acc_a, thread_acc_b| {
            thread_acc_a.merge(thread_acc_b);
            thread_acc_a
        });

    Ok(final_stats)
}
