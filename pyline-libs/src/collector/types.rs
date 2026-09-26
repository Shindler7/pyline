//! New type wrappers over collections of strings.
//!
//! Provides type-safe representations for file names, directory names,
//! and file extensions.

use crate::{FileData, collector::traits::WithDefaults, errors::PyLineError};
use std::{borrow::Cow, collections::HashSet};

fn normalize_verbatim(s: &str) -> Option<Cow<'_, str>> {
    let cleaned = s.trim().trim_matches('/');

    (!cleaned.is_empty()).then_some(Cow::Borrowed(cleaned))
}

fn normalize_ext(s: &str) -> Option<Cow<'_, str>> {
    let cleaned = s.trim_start_matches('.');

    let cleaned = match normalize_verbatim(cleaned)? {
        Cow::Owned(o) => return Some(Cow::Owned(o.to_lowercase())),
        Cow::Borrowed(b) => b,
    };

    if cleaned.chars().any(char::is_uppercase) {
        Some(Cow::Owned(cleaned.to_lowercase()))
    } else {
        Some(Cow::Borrowed(cleaned))
    }
}

/// Defines a new type wrapper around `HashSet<String>` with the given
/// name, doc comment, and normalization function.
///
/// # Parameters
///
/// * `$name` — identifier of the generated type.
/// * `$doc_expr` — doc string for the type.
/// * `$normalize` — function applied to each inserted value; returning
///   `None` skips the value.
macro_rules! string_set_type {
    (
        $name: ident,
        $doc_expr: expr,
        $normalize: path

    ) => {
        #[doc = $doc_expr]
        #[derive(Debug, Default, PartialEq, Eq)]
        pub struct $name(HashSet<String>);

        impl $name {
            /// Inserts a value after normalization.
            ///
            /// Returns `true` if the value was newly inserted, `false` if it was
            /// a duplicate or normalized to `None`.
            pub fn insert<S: AsRef<str>>(&mut self, raw: S) -> bool {
                match $normalize(raw.as_ref()) {
                    Some(normalized) => {
                        let norm_str: &str = &normalized;
                        if self.0.contains(norm_str) {
                            return false;
                        }
                        self.0.insert(normalized.into_owned())
                    }
                    None => false,
                }
            }

            /// Iterates over the stored values.
            pub fn iter(&self) -> impl Iterator<Item = &str> {
                self.0.iter().map(String::as_str)
            }

            /// Consumes `self` and returns the underlying set.
            pub fn into_inner(self) -> HashSet<String> {
                self.0
            }

            /// Returns `true` if the collection is empty.
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            /// Returns the stored values as vec.
            pub fn as_vec(&self) -> Vec<&str> {
                self.0.iter().map(String::as_str).collect()
            }
        }

        impl<T: AsRef<str>> Extend<T> for $name {
            fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
                for raw in iter {
                    self.insert(raw);
                }
            }
        }

        impl<T> FromIterator<T> for $name
        where
            T: AsRef<str>,
        {
            fn from_iter<I>(iter: I) -> Self
            where
                I: IntoIterator<Item = T>,
            {
                let mut out = Self::default();
                out.extend(iter);
                out
            }
        }

        impl AsRef<HashSet<String>> for $name {
            fn as_ref(&self) -> &HashSet<String> {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let mut items: Vec<&str> = self.as_vec();
                items.sort_unstable();

                write!(f, "[{}]", items.join(", "))
            }
        }
    };
}

string_set_type!(Files, "A set of file names.", normalize_verbatim);
string_set_type!(Dirs, "A set of directory names.", normalize_verbatim);
string_set_type!(Extensions, "A set of file extensions.", normalize_ext);

impl Extensions {
    /// Returns all extensions joined by `sep`, sorted for deterministic output.
    pub fn join(&self, sep: &str) -> String {
        let mut v: Vec<&str> = self.iter().collect();
        v.sort_unstable();
        v.join(sep)
    }
}

/// Implements [`WithDefaults`] for a string-set type by collecting the
/// provided defaults.
macro_rules! lang_defaults {
    ($name: ident) => {
        impl WithDefaults for $name {
            fn with_defaults(default: &[&str]) -> Self {
                default.iter().copied().collect()
            }
        }

        impl From<&[&str]> for $name {
            fn from(s: &[&str]) -> Self {
                Self::with_defaults(s)
            }
        }
    };
}

lang_defaults!(Files);
lang_defaults!(Dirs);
lang_defaults!(Extensions);

/// Defines a new type wrapper around `Vec<T>` with basic collection helpers.
///
/// # Parameters
///
/// * `$name` — identifier of the generated type.
/// * `$doc_expr` — doc string for the type.
/// * `$ty` — element type.
macro_rules! collection {
    ($name: ident, $doc_expr: expr, $ty: ty) => {
        #[doc = $doc_expr]
        #[derive(Debug, Default)]
        pub struct $name(Vec<$ty>);

        impl $name {
            /// Creates an empty collection.
            pub fn new() -> Self {
                Self::default()
            }

            /// Appends `value` to the collection.
            pub fn push(&mut self, value: $ty) {
                self.0.push(value);
            }

            /// Returns an iterator over the elements.
            pub fn iter(&self) -> impl Iterator<Item = &$ty> {
                self.0.iter()
            }

            /// Appends all elements of `other` to `self`.
            pub fn extend(&mut self, other: Self) {
                self.0.extend(other.0);
            }
        }

        impl std::ops::Deref for $name {
            type Target = [$ty];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

collection!(
    CollectedFiles,
    "Files collected during traversal.",
    FileData
);

collection!(
    CollectedErrors,
    "Errors encountered during traversal.",
    PyLineError
);
