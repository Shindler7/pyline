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

use crate::errors::PyLineError;
use crate::py::traits::PythonLineAnalysis;
use std::str::FromStr;

const TRIPLE_SINGLE: &str = "'''";
const TRIPLE_DOUBLE: &str = r#"""""#;

#[derive(Copy, Clone)]
pub(crate) enum QuoteType {
    TripleSingle,
    TripleDouble,
}

impl FromStr for QuoteType {
    type Err = PyLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            TRIPLE_SINGLE => Ok(QuoteType::TripleSingle),
            TRIPLE_DOUBLE => Ok(QuoteType::TripleDouble),
            _ => Err(PyLineError::counter_error("Invalid string for QuoteType")),
        }
    }
}

impl QuoteType {
    const fn as_str(&self) -> &'static str {
        match self {
            QuoteType::TripleSingle => TRIPLE_SINGLE,
            QuoteType::TripleDouble => TRIPLE_DOUBLE,
        }
    }
}

impl<T: AsRef<str>> PythonLineAnalysis for T {
    fn is_triple_quotes_line(&self) -> bool {
        let line = self.as_ref();
        if line.len() < 6 {
            return false;
        }

        let quotes = match line.starts_with_quotes() {
            Some(s) => s,
            None => return false,
        };

        match line.rfind(quotes.as_str()) {
            Some(_) => true,
            None => false,
        }
    }

    fn starts_with_quotes(&self) -> Option<QuoteType> {
        if self.as_ref().starts_with(TRIPLE_SINGLE) {
            Some(QuoteType::TripleSingle)
        } else if self.as_ref().starts_with(TRIPLE_DOUBLE) {
            Some(QuoteType::TripleDouble)
        } else {
            None
        }
    }

    fn is_comment(&self) -> bool {
        self.as_ref().starts_with('#')
    }

    fn is_empty_line(&self) -> bool {
        self.as_ref().is_empty()
    }

    fn is_non_code(&self) -> bool {
        let line = self.as_ref().trim();
        line.is_empty_line() || line.is_comment() || line.is_triple_quotes_line()
    }
}
