//! This module provides traits and implementations for common operations
//! when processing Python source code line by line. It offers lightweight,
//! zero-cost abstractions for checking line properties without requiring
//! full-scale parsing.

/// A trait providing utility methods for analyzing Python source code lines.
///
/// This trait offers methods to determine properties of Python code lines
/// without requiring a full Python parser. It is implemented for any type
/// that can be converted to a string slice (`AsRef<str>`), making it
/// convenient to use with various string types.
///
/// ## Performance
///
/// The implementations are designed to be zero-cost abstractions that
/// perform minimal allocation and work directly on string slices.
#[allow(dead_code)]
pub trait PythonLineAnalysis {
    /// Check if a line is empty.
    fn is_empty_line(&self) -> bool;
}
