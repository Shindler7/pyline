//! Parser types and language defaults.

pub(crate) mod defaults;
mod parsers;
pub mod macros;
pub mod py;
pub mod rust;
pub mod traits;

pub use parsers::{CodeFilesStat, CodeLanguage, Python, Rust};
