//! Python source code parsing utilities.
//!
//! This module provides low-level functions and traits for analyzing Python
//! source code. It focuses on line-by-line parsing operations and syntactic
//! analysis without requiring a full Python interpreter
//! or complex AST parsing.
//!
//! ## Features
//!
//! - **Lightweight analysis**: Fast line classification without heavy
//!   dependencies
//! - **Zero-cost abstractions**: Works directly on string slices when possible
//! - **Extensible**: Designed to be extended with additional analysis methods

use std::iter::Peekable;

/// Checks if a quote character at a given position in a peekable iterator is part
/// of a triple-quote sequence.
///
/// ## Parameters:
/// - iter: Mutable reference to a peekable iterator over character positions
/// - quote: Reference to the quote character being checked
/// - index: Starting position of the quote character in the sequence
///
/// ## Returns
///
/// `true` if the next two characters at positions index+1 and index+2 match
/// the quote character, forming a triple-quote sequence.
pub fn is_triple_quotes<I>(iter: &mut Peekable<I>, quote: &char, index: usize) -> bool
where
    I: Iterator<Item = (usize, char)> + Clone,
{
    iter.next_if_eq(&(index + 1, *quote)).is_some()
        && iter.next_if_eq(&(index + 1, *quote)).is_some()
}
