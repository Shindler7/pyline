//! Core library for `pyline`: file collection and source parsing.
//!
//! The crate provides two main parts:
//! - [`collector`] — gathers paths to files matching a set of filters;
//! - [`parser`] — a [`CodeParser`] trait with language-specific implementations
//!   (Python, Rust).
//!
//! Custom error types live in [`errors`].

pub mod collector;
pub mod errors;
pub mod parser;
pub mod utils;

pub use collector::{Collector, CollectorResult, models::FileData};
pub use parser::{CodeFilesStats, CodeLanguage, CodeParser};
