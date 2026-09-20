//! Library new types.
//!
//! Provides wrappers over primitive types for type-safe
//! representation of domain values.

use crate::collector::traits::LangDefaults;
use std::collections::HashSet;

fn normalize_verbatim(s: String) -> Option<String> {
    (!s.trim().is_empty()).then_some(s)
}

fn normalize_ext(mut s: String) -> Option<String> {
    if s.is_ascii() {
        s.make_ascii_lowercase();
    } else {
        s = s.to_lowercase();
    }

    let dots = s.bytes().take_while(|&b| b == b'.').count();
    if dots > 0 {
        s.drain(..dots);
    }

    normalize_verbatim(s)
}

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
            pub fn insert<S: Into<String>>(&mut self, raw: S) -> bool {
                match $normalize(raw.into()) {
                    Some(s) => self.0.insert(s),
                    None => false,
                }
            }

            pub fn iter(&self) -> impl Iterator<Item = &str> {
                self.0.iter().map(String::as_str)
            }

            pub fn into_inner(self) -> HashSet<String> {
                self.0
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl<T: Into<String>> Extend<T> for $name {
            fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
                for raw in iter {
                    self.insert(raw);
                }
            }
        }

        impl<T> FromIterator<T> for $name
        where
            T: Into<String>,
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
    };
}

string_set_type!(Files, "A collection of file paths.", normalize_verbatim);
string_set_type!(Dirs, "A collection of directory paths.", normalize_verbatim);
string_set_type!(
    Extensions,
    "A collection of file extensions.",
    normalize_ext
);

impl Extensions {
    pub fn join(&self, sep: &str) -> String {
        let mut v: Vec<&str> = self.0.iter().map(String::as_str).collect();
        v.sort_unstable();
        v.join(sep)
    }
}

impl LangDefaults for Extensions {
    fn with_defaults(default: &[&str]) -> Self {
        default.iter().copied().collect()
    }
}

impl LangDefaults for Files {
    fn with_defaults(default: &[&str]) -> Self {
        default.iter().copied().collect()
    }
}

impl LangDefaults for Dirs {
    fn with_defaults(default: &[&str]) -> Self {
        default.iter().copied().collect()
    }
}
