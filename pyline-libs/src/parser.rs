//! Parser types and language defaults.

pub(crate) mod defaults;
pub(crate) mod macros;
mod models;
pub(crate) mod py;
pub(crate) mod rust;
pub mod traits;

pub use models::{CodeFilesStat, CodeLanguage, Python, Rust};
pub use traits::CodeParsers;
