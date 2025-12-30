//! API for parsing code files.
//!
//! Contains two mechanisms:
//! - file collector — builds a dump of references to valid files;
//! - code file parser — consists of two components: universal methods and language-specific
//!   implementations (e.g., for Python).
//!
//! Custom error types defined in `errors.rs`.

pub mod collector;
pub mod errors;
pub mod parser;
pub mod py;
pub mod utils;
pub mod traits;
