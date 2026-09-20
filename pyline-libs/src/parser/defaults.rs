//! Default collection filters and extensions per language.

use crate::{
    CodeLanguage,
    collector::types::{Dirs, Extensions, Files},
};

/// Default filters and extensions used when autoconfiguration is enabled.
#[derive(Copy, Clone)]
pub(crate) struct LangDefaults {
    /// Directory names to exclude.
    pub(crate) exclude_dirs: &'static [&'static str],

    /// File names to exclude.
    pub(crate) exclude_filenames: &'static [&'static str],

    /// Marker files whose presence excludes the containing directory.
    pub(crate) marker_files: &'static [&'static str],

    /// File extensions to collect.
    pub(crate) valid_extensions: &'static [&'static str],
}

impl LangDefaults {
    pub(crate) const PYTHON: Self = Self {
        exclude_dirs: &["venv", "env", "__pycache__", "mypy_cache"],
        exclude_filenames: &[],
        marker_files: &["pyvenv.cfg"],
        valid_extensions: &["py"],
    };

    pub(crate) const RUST: Self = Self {
        exclude_dirs: &["target"],
        exclude_filenames: &[],
        marker_files: &[],
        valid_extensions: &["rs"],
    };

    /// Default excluded directories.
    pub(crate) fn exclude_dirs(&self) -> Dirs {
        self.exclude_dirs.into()
    }

    /// Default excluded filenames.
    pub(crate) fn exclude_filenames(&self) -> Files {
        self.exclude_filenames.into()
    }

    /// Default marker files.
    pub(crate) fn marker_files(&self) -> Files {
        self.marker_files.into()
    }

    /// Default valid extensions.
    pub(crate) fn valid_extensions(&self) -> Extensions {
        self.valid_extensions.into()
    }
}

impl CodeLanguage {
    /// Returns the default configuration for this language.
    pub(crate) fn defaults(&self) -> LangDefaults {
        match self {
            CodeLanguage::Python => LangDefaults::PYTHON,
            CodeLanguage::Rust => LangDefaults::RUST,
        }
    }
}
