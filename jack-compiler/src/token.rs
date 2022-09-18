use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

pub struct TokenRef<'a> {
    pub kind: &'a TokenKind,
    pub value: &'a str,
}

impl Token {
    pub fn as_ref<'a>(&'a self) -> TokenRef<'a> {
        TokenRef {
            kind: &self.kind,
            value: &self.value,
        }
    }
}

impl<'a> PartialEq<TokenRef<'a>> for Token {
    fn eq(&self, other: &TokenRef<'a>) -> bool {
        &self.kind == other.kind && self.value == other.value
    }
}

impl<'a> Display for TokenRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \"{}\"", self.kind, self.value)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \"{}\"", self.kind, self.value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

pub(crate) use token;
