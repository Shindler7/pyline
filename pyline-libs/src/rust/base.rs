//! Default settings for parsing a Rust codebase.
//!
//! Include information for correctly building the required files, excluding,
//! for example, environment directories. Plus a set of keywords that the
//! parser will use to parse code lines.
use phf::phf_map;

/// Directories to exclude when parsing.
pub const RUST_EXCLUDE_DIRS: &[&str] = &["target", "build", "dist", "__pycache__"];

/// Dot directories (starting with a dot) to exclude.
pub const RUST_EXCLUDE_DOT_DIRS: &[&str] = &[
    ".git",
    ".idea",
    ".vscode",
    ".cargo",
    ".rustup",
    ".cache",
    ".pytest_cache",
    ".venv",
    ".env",
];

/// Filenames to exclude.
pub const RUST_EXCLUDE_FILENAMES: &[&str] = &["Cargo.lock", ".gitignore", ".gitmodules"];

/// Special marker files whose presence identifies certain directory types.
pub const RUST_MARKER_FILE: &[&str] = &["rust-toolchain", "rustfmt.toml"];

/// Valid file extensions for analysis.
pub const RUST_VALID_EXTENSIONS: &[&str] = &["rs"];

/// Rust language keywords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum RustKeywords {
    // Primitive types
    Bool,
    Char,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    F32,
    F64,
    Str,

    // Flow control keywords
    As,
    Async,
    Await,
    Break,
    Const,
    Continue,
    Crate,
    Dyn,
    Else,
    Enum,
    Extern,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Match,
    Mod,
    Move,
    Mut,
    Pub,
    Ref,
    Return,
    SelfValue,
    SelfType,
    Static,
    Struct,
    Super,
    Trait,
    True,
    Type,
    Unsafe,
    Use,
    Where,
    While,

    // Reserved keywords
    Abstract,
    Become,
    Box,
    Do,
    Final,
    Macro,
    Override,
    Priv,
    Typeof,
    Unsized,
    Virtual,
    Yield,

    // Memory-related keywords
    Drop,
    Sizeof,
    Alignof,
    Offsetof,
}

impl std::fmt::Display for RustKeywords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            // Primitive types
            Self::Bool => "bool",
            Self::Char => "char",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::Isize => "isize",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::U128 => "u128",
            Self::Usize => "usize",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::Str => "str",

            // Flow control keywords
            Self::As => "as",
            Self::Async => "async",
            Self::Await => "await",
            Self::Break => "break",
            Self::Const => "const",
            Self::Continue => "continue",
            Self::Crate => "crate",
            Self::Dyn => "dyn",
            Self::Else => "else",
            Self::Enum => "enum",
            Self::Extern => "extern",
            Self::False => "false",
            Self::Fn => "fn",
            Self::For => "for",
            Self::If => "if",
            Self::Impl => "impl",
            Self::In => "in",
            Self::Let => "let",
            Self::Loop => "loop",
            Self::Match => "match",
            Self::Mod => "mod",
            Self::Move => "move",
            Self::Mut => "mut",
            Self::Pub => "pub",
            Self::Ref => "ref",
            Self::Return => "return",
            Self::SelfValue => "self",
            Self::SelfType => "Self",
            Self::Static => "static",
            Self::Struct => "struct",
            Self::Super => "super",
            Self::Trait => "trait",
            Self::True => "true",
            Self::Type => "type",
            Self::Unsafe => "unsafe",
            Self::Use => "use",
            Self::Where => "where",
            Self::While => "while",

            // Reserved keywords
            Self::Abstract => "abstract",
            Self::Become => "become",
            Self::Box => "box",
            Self::Do => "do",
            Self::Final => "final",
            Self::Macro => "macro",
            Self::Override => "override",
            Self::Priv => "priv",
            Self::Typeof => "typeof",
            Self::Unsized => "unsized",
            Self::Virtual => "virtual",
            Self::Yield => "yield",

            // Memory-related keywords
            Self::Drop => "drop",
            Self::Sizeof => "sizeof",
            Self::Alignof => "alignof",
            Self::Offsetof => "offsetof",
        };
        write!(f, "{}", s)
    }
}

/// Rust keywords map for fast lookups.
pub(crate) static RUST_KEYWORDS: phf::Map<&'static str, RustKeywords> = phf_map! {
    // Primitive types
    "bool" => RustKeywords::Bool,
    "char" => RustKeywords::Char,
    "i8" => RustKeywords::I8,
    "i16" => RustKeywords::I16,
    "i32" => RustKeywords::I32,
    "i64" => RustKeywords::I64,
    "i128" => RustKeywords::I128,
    "isize" => RustKeywords::Isize,
    "u8" => RustKeywords::U8,
    "u16" => RustKeywords::U16,
    "u32" => RustKeywords::U32,
    "u64" => RustKeywords::U64,
    "u128" => RustKeywords::U128,
    "usize" => RustKeywords::Usize,
    "f32" => RustKeywords::F32,
    "f64" => RustKeywords::F64,
    "str" => RustKeywords::Str,

    // Flow control keywords
    "as" => RustKeywords::As,
    "async" => RustKeywords::Async,
    "await" => RustKeywords::Await,
    "break" => RustKeywords::Break,
    "const" => RustKeywords::Const,
    "continue" => RustKeywords::Continue,
    "crate" => RustKeywords::Crate,
    "dyn" => RustKeywords::Dyn,
    "else" => RustKeywords::Else,
    "enum" => RustKeywords::Enum,
    "extern" => RustKeywords::Extern,
    "false" => RustKeywords::False,
    "fn" => RustKeywords::Fn,
    "for" => RustKeywords::For,
    "if" => RustKeywords::If,
    "impl" => RustKeywords::Impl,
    "in" => RustKeywords::In,
    "let" => RustKeywords::Let,
    "loop" => RustKeywords::Loop,
    "match" => RustKeywords::Match,
    "mod" => RustKeywords::Mod,
    "move" => RustKeywords::Move,
    "mut" => RustKeywords::Mut,
    "pub" => RustKeywords::Pub,
    "ref" => RustKeywords::Ref,
    "return" => RustKeywords::Return,
    "self" => RustKeywords::SelfValue,
    "Self" => RustKeywords::SelfType,
    "static" => RustKeywords::Static,
    "struct" => RustKeywords::Struct,
    "super" => RustKeywords::Super,
    "trait" => RustKeywords::Trait,
    "true" => RustKeywords::True,
    "type" => RustKeywords::Type,
    "unsafe" => RustKeywords::Unsafe,
    "use" => RustKeywords::Use,
    "where" => RustKeywords::Where,
    "while" => RustKeywords::While,

    // Reserved keywords
    "abstract" => RustKeywords::Abstract,
    "become" => RustKeywords::Become,
    "box" => RustKeywords::Box,
    "do" => RustKeywords::Do,
    "final" => RustKeywords::Final,
    "macro" => RustKeywords::Macro,
    "override" => RustKeywords::Override,
    "priv" => RustKeywords::Priv,
    "typeof" => RustKeywords::Typeof,
    "unsized" => RustKeywords::Unsized,
    "virtual" => RustKeywords::Virtual,
    "yield" => RustKeywords::Yield,

    // Memory-related keywords
    "drop" => RustKeywords::Drop,
    "sizeof" => RustKeywords::Sizeof,
    "alignof" => RustKeywords::Alignof,
    "offsetof" => RustKeywords::Offsetof,
};
