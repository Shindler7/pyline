//! Parser types and language defaults.

pub(crate) mod defaults;
mod parsers;

pub use parsers::{CodeFilesStat, CodeLanguage, Python, Rust};
