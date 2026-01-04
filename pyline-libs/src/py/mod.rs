//! Core parsing engine and language-specific implementations for Python.
//!
//! This module provides the fundamental building blocks for code analysis:
//! - [`base`] — Basic data structures, enums, and constants shared across all parsers
//! - [`engine`] — Core parsing algorithms and state machines (language-independent logic)
//! - [`py_methods`] — Python-specific parsing logic and keyword handling
//!
//! The architecture separates language-agnostic infrastructure from language-specific
//! implementations, enabling easy extension to new programming languages.
pub mod base;
#[macro_use]
pub(crate) mod engine;
pub(crate) mod py_methods;
