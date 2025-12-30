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
pub trait PythonLineAnalysis {
    /// Checks if a line is a triple-quoted string literal (`"""` or `'''`).
    ///
    /// In Python, triple quotes are used for:
    /// - Multi-line strings
    /// - Docstrings (module, class, function documentation)
    /// - Single-line triple-quoted strings
    ///
    /// This method identifies lines that start **and** end with the same triple-quote
    /// marker on the same line, indicating a complete triple-quoted string literal.
    ///
    /// ## Examples
    ///
    /// ```python
    /// # True:
    /// """Single-line docstring"""
    /// '''Single-line with single quotes'''
    ///
    /// # False:
    /// """Multi-line string
    /// that continues"""
    ///
    /// x = """embedded triple quotes"""
    /// y = "regular string with \"\"\" inside"
    /// ```
    ///
    /// ## Returns
    ///
    /// `true` if the line:
    /// 1. Starts with `"""` or `'''`
    /// 2. Ends with the same triple-quote marker
    /// 3. Contains only the triple quotes and optional whitespace/comments
    ///
    /// `false` otherwise, including:
    /// - Multi-line triple-quoted strings
    /// - Lines with text before/after the triple quotes
    /// - Lines with mismatched quote types
    /// - Empty lines or regular strings
    ///
    /// ## Limitations
    ///
    /// ```python
    /// # These return false (as designed):
    /// '''start only
    /// end only'''
    /// prefix """suffix
    /// """content""" # comment
    ///
    /// # These might need special handling in calling code:
    /// r"""raw string"""
    /// f"""f-string {variable}"""
    /// u"""unicode string"""
    /// ```
    ///
    /// For prefix handling (r, f, u, b, etc.), consider trimming string prefixes
    /// before calling this method, or implementing a separate method that accounts
    /// for Python's string prefix syntax.
    fn is_triple_quotes_line(&self) -> bool;

    /// Check if a line contains only a Python comment.
    fn is_comment(&self) -> bool;

    /// Check if a line is empty.
    fn is_empty_line(&self) -> bool;

    /// Checks if the line contains no executable Python code.
    fn is_non_code(&self) -> bool;
}
