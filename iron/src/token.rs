use crate::debug::SourceLocation;

pub enum PrimitiveType {
    _16b,
    _32b,
}

impl PrimitiveType {
    pub fn as_empty_bytes(&self) -> &[u8] {
        match self {
            Self::_16b => &[0, 0],
            Self::_32b => &[0, 0, 0, 0],
        }
    }
}

pub enum TokenKind {
    /// Identifier.
    /// `[A-Za-z_]+[A-Za-z_@0-9]*`
    Identifier(String),

    PrimitiveType(PrimitiveType),

    /// `ret` keyword
    Return,

    /// `extern` keyword.
    Extern,

    /// Comparison kind.
    /// %eq, %gt, %lt, %gte, %lte
    ComparisonKind,

    /// `emp` keyword
    /// Empty variable declaration.
    Emp,
}

impl TokenKind {
    pub fn from_identifier(identifier: String) -> Self {
        match identifier.as_ref() {
            "ret" => Self::Return,
            "emp" => Self::Emp,
            "extern" => Self::Extern,
            _ => Self::Identifier(identifier),
        }
    }
}

pub struct Token {
    kind: TokenKind,
    source_location: SourceLocation,
}

impl Token {
    pub fn get_kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn new(kind: TokenKind, source_location: SourceLocation) -> Self {
        Self {
            kind,
            source_location,
        }
    }
}
