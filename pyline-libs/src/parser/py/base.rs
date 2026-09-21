//! Default filters and keywords for the Python parser.
//!
//! Provides directory/file exclusions, valid file extensions, and the
//! set of Python keywords used for statistics.

use phf::phf_map;

/// Python language keywords.
///
/// Each variant corresponds to a keyword; the lowercase spelling used
/// in source code is returned by [`Display`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PyKeywords {
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
        f.write_str(s)
    }
}

/// Static map from lowercased keyword strings to [`PyKeywords`].
///
/// The parser lowercases each token before lookup, so `True`, `False`,
/// and `None` are keyed as `"true"`, `"false"`, and `"none"`.
pub(crate) static KEYWORDS: phf::Map<&'static str, PyKeywords> = phf_map! {
    "False" => PyKeywords::False,
    "None" => PyKeywords::None,
    "True" => PyKeywords::True,
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
