//! API for parsing code files.
//!
//! Contains two mechanisms:
//! - file collector — builds a dump of references to valid files;
//! - code file parser — consists of two components: universal methods and language-specific
//!   implementations (e.g., for Python).
//!
//! Custom error types defined in `errors.rs`.
#![warn(missing_docs)]
pub mod collector;
pub mod errors;
#[macro_use]
pub mod parser;
pub mod macros;
pub mod py;
pub mod rust;
pub mod traits;
pub mod utils;
