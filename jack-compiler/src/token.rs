#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \"{}\"", self.kind, self.value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Keyword,
    Symbol,
    Identifier,
    IntegerConstant,
    StringConstant,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyword => write!(f, "keyword"),
            Self::Symbol => write!(f, "symbol"),
            Self::Identifier => write!(f, "identifier"),
            Self::IntegerConstant => write!(f, "integer constant"),
            Self::StringConstant => write!(f, "string constant"),
        }
    }
}

macro_rules! token {
    ($kind:ident, $value:expr) => {
        Token {
            kind: TokenKind::$kind,
            value: $value.to_owned(),
        }
    };
}

use std::fmt::Display;

pub(crate) use token;
