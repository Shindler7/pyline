//! Parser types and language defaults.

pub(crate) mod defaults;
mod models;
pub(crate) mod py;
pub(crate) mod rust;
pub mod traits;

pub use models::{CodeFilesStat, CodeLanguage, ParseResults};
pub use traits::CodeParser;

use crate::{FileData, errors::PyLineError};

pub fn run(files: &[FileData], lang: &CodeLanguage) -> Result<ParseResults, PyLineError> {
    use rayon::prelude::*;

    if files.is_empty() {
        return Err(PyLineError::NoFilesFound);
    }

    let parser = lang.get_parser();

    let final_stats = files
        .par_iter()
        .fold(ParseResults::new, |mut acc, file| {
            match parser.parse_file(file) {
                Ok(file_stats) => {
                    acc.merge(file_stats);
                }
                Err(_) => {
                    acc.stats.num_files_invalid += 1;
                    acc.stats.num_files_total += 1;
                }
            }

            acc
        })
        .reduce(ParseResults::new, |mut thread_acc_a, thread_acc_b| {
            thread_acc_a.merge(thread_acc_b);
            thread_acc_a
        });

    let mut final_results = ParseResults::new();
    final_results.merge(final_stats);

    Ok(final_results)
}
