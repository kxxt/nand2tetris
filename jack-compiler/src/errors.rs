use thiserror::Error;

use crate::{ast::TypeNode, token::Token};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("unexpected end of token stream")]
    UnexpectedEndOfStream,
    #[error("unexpected token {0}, expected {1}")]
    UnexpectedToken(Token, String),
    #[error("the class name \"{0}\" didn't match file name stem \"{1}\"")]
    ClassNameMismatch(String, String),
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
pub enum EmitterError {
    #[error("not in a subroutine")]
    NotInASubroutine,
    #[error("variable \"{0}\" not found")]
    VariableNotFound(String),
    #[error("unexpected primitive type \"{0:?}\"")]
    UnexpectedPrimitiveType(TypeNode),
    #[error("expected type \"{0:?}\", found type \"{1:?}\"")]
    MismatchedType(Option<TypeNode>, Option<TypeNode>),
}
