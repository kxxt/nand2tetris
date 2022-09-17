use thiserror::Error;

use crate::token::Token;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("unexpected end of token stream")]
    UnexpectedEndOfStream,
    #[error("unexpected token {0:?}, expected {1}")]
    UnexpectedToken(Token, String),
}

#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("unexpected EOF")]
    UnexpectedEOF,
    #[error("identifier should not start with a digit")]
    IdentifierStartsWithDigit,
    #[error("invalid keyword")]
    InvalidKeyword,
    #[error("integer {0} is out of range [0, 32767]")]
    IntegerOutOfRange(String),
    #[error("unexpected character '{0}', which should be {1}")]
    UnexpectedCharacter(char, String),
}

#[derive(Error, Debug)]
pub enum EmitterError {}
