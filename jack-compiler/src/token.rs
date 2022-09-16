#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    value: String,
}

#[derive(Debug)]
pub enum TokenKind {
    Keyword,
    Symbol,
    Identifier,
    IntegerConstant,
    StringConstant,
}
