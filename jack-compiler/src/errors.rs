use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {}

#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("unexpected EOF")]
    UnexpectedEOF,
    #[error("invalid identifier {0}")]
    InvalidIdentifier(String),
}

#[derive(Error, Debug)]
pub enum EmitterError {}
