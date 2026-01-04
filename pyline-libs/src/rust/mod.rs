//! Core parsing engine and language-specific implementations for Rust.
//!
//! This module provides the fundamental building blocks for code analysis:
//! - [`base`] — Basic data structures, enums, and constants shared across all parsers
//! - [`engine`] — Core parsing algorithms and state machines (language-independent
//!   logic)
//!
//! The architecture separates language-agnostic infrastructure from language-specific
//! implementations, enabling easy extension to new programming languages.
pub mod base;
#[macro_use]
pub(crate) mod engine;
