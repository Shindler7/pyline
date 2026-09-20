//! Library new types.
//!
//! Provides wrappers over primitive types for type-safe
//! representation of domain values.

use std::ops::Deref;

macro_rules! impl_string_collection {
    ($name: ident) => {
        impl<T> FromIterator<T> for $name
        where
            T: Into<String>,
        {
            fn from_iter<I>(iter: I) -> Self
            where
                I: IntoIterator<Item = T>,
            {
                Self(iter.into_iter().map(|s| s.into()).collect())
            }
        }

        impl Deref for $name {
            type Target = Vec<String>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

/// A collection of file paths.
#[derive(Default)]
pub struct Files(Vec<String>);

#[derive(Default)]
pub struct Dirs(Vec<String>);

impl_string_collection!(Files);
impl_string_collection!(Dirs);

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Extensions(Vec<String>);

impl<T> FromIterator<T> for Extensions
where
    T: Into<String>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(
            iter.into_iter()
                .map(|s| {
                    let ext = s.into();
                    ext.trim_start_matches('.').to_string()
                })
                .collect(),
        )
    }
}

impl Deref for Extensions {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
