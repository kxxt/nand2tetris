#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Keyword,
    Symbol,
    Identifier,
    IntegerConstant,
    StringConstant,
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
