//! Default settings for parsing a Python codebase.
//!
//! Include information for correctly building the required files, excluding,
//! for example, environment directories. Plus a set of keywords that the
//! parser will use to parse code lines.
use phf::phf_map;

/// Directories to be excluded from the build process.
///
/// This list contains common directories that are not part of the actual
/// source code, such as virtual environments and cache folders. Dot-prefixed
/// directories (e.g., `.git`, `.venv`) are handled separately by
/// [`EXCLUDE_DOT_DIRS`] and should not be included here.
pub const EXCLUDE_DIRS: &[&str] = &["venv", "env", "__pycache__", "mypy_cache"];

/// Directories with a dot prefix (hidden in Unix-like systems) to be excluded
/// from the build.
///
/// These are typically configuration, cache, or IDE-specific directories.
/// This constant works in conjunction with [`EXCLUDE_DIRS`] to provide
/// comprehensive filtering.
pub const EXCLUDE_DOT_DIRS: &[&str] = &[
    ".pytest_cache",
    ".venv",
    ".env",
    ".git",
    ".idea",
    ".vscode",
    ".eggs",
    ".cache",
];

/// File names (without paths) that should be excluded from processing.
pub const EXCLUDE_FILENAMES: &[&str] = &[];

/// Special marker files whose presence identifies certain directory types.
///
/// For example, `pyvenv.cfg` indicates a Python virtual environment directory.
/// These files are checked to validate or exclude entire directory subtrees.
pub const MARKER_FILE: &[&str] = &["pyvenv.cfg"];

/// File extensions that are considered valid for source code parsing.
///
/// Only files with these extensions will be processed by the parser.
/// Other files will be ignored even if they pass directory and filename
/// filtering.
pub const VALID_EXTENSIONS: &[&str] = &["py"];

/// Python keywords for parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum PyKeywords {
    False,
    None,
    True,
    And,
    As,
    Assert,
    Async,
    Await,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    Nonlocal,
    Not,
    Or,
    Pass,
    Raise,
    Return,
    Try,
    While,
    With,
    Yield,
}

impl std::fmt::Display for PyKeywords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::False => "False",
            Self::None => "None",
            Self::True => "True",
            Self::And => "and",
            Self::As => "as",
            Self::Assert => "assert",
            Self::Async => "async",
            Self::Await => "await",
            Self::Break => "break",
            Self::Class => "class",
            Self::Continue => "continue",
            Self::Def => "def",
            Self::Del => "del",
            Self::Elif => "elif",
            Self::Else => "else",
            Self::Except => "except",
            Self::Finally => "finally",
            Self::For => "for",
            Self::From => "from",
            Self::Global => "global",
            Self::If => "if",
            Self::Import => "import",
            Self::In => "in",
            Self::Is => "is",
            Self::Lambda => "lambda",
            Self::Nonlocal => "nonlocal",
            Self::Not => "not",
            Self::Or => "or",
            Self::Pass => "pass",
            Self::Raise => "raise",
            Self::Return => "return",
            Self::Try => "try",
            Self::While => "while",
            Self::With => "with",
            Self::Yield => "yield",
        };
        write!(f, "{}", s)
    }
}

/// Case-sensitive static hash map for O(1) keyword lookup.
///
/// Maps lowercase Python keyword strings to [`PyKeywords`] enum variants.
/// Note: Python keywords are case-sensitive (`True` vs `true`).
pub(crate) static KEYWORDS: phf::Map<&'static str, PyKeywords> = phf_map! {
    "false" => PyKeywords::False,
    "none" => PyKeywords::None,
    "true" => PyKeywords::True,
    "and" => PyKeywords::And,
    "as" => PyKeywords::As,
    "assert" => PyKeywords::Assert,
    "async" => PyKeywords::Async,
    "await" => PyKeywords::Await,
    "break" => PyKeywords::Break,
    "class" => PyKeywords::Class,
    "continue" => PyKeywords::Continue,
    "def" => PyKeywords::Def,
    "del" => PyKeywords::Del,
    "elif" => PyKeywords::Elif,
    "else" => PyKeywords::Else,
    "except" => PyKeywords::Except,
    "finally" => PyKeywords::Finally,
    "for" => PyKeywords::For,
    "from" => PyKeywords::From,
    "global" => PyKeywords::Global,
    "if" => PyKeywords::If,
    "import" => PyKeywords::Import,
    "in" => PyKeywords::In,
    "is" => PyKeywords::Is,
    "lambda" => PyKeywords::Lambda,
    "nonlocal" => PyKeywords::Nonlocal,
    "not" => PyKeywords::Not,
    "or" => PyKeywords::Or,
    "pass" => PyKeywords::Pass,
    "raise" => PyKeywords::Raise,
    "return" => PyKeywords::Return,
    "try" => PyKeywords::Try,
    "while" => PyKeywords::While,
    "with" => PyKeywords::With,
    "yield" => PyKeywords::Yield,
};
