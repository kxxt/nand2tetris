use std::error::Error;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct ParserError {
    pub line_number: u32,
    pub inner_error: ParseCommandError,
}

impl ParserError {
    pub fn new(err: ParseCommandError, line_number: u32) -> Self {
        Self {
            line_number,
            inner_error: err,
        }
    }
}

impl Error for ParserError {}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        std::fmt::Display::fmt(&self.inner_error, f)?;
        write!(f, " on line {}", self.line_number)
    }
}

#[derive(Debug)]
pub struct TranslationError {
    pub message: String,
}

impl Error for TranslationError {}
impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}

#[derive(Debug)]
pub enum ParseCommandError {
    InvalidCommandName(String),
    NotEnoughArguments,
    TooManyArguments,
    InvalidArgument(String),
    NoCommand,
    ParseSegmentError(String),
}

impl Error for ParseCommandError {}
impl fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidArgument(arg) => {
                write!(f, "Invalid argument \"{arg}\"")
            }
            Self::NotEnoughArguments => write!(f, "Not enough arguments"),
            Self::TooManyArguments => write!(f, "Too many arguments"),
            Self::NoCommand => write!(f, "No command"),
            Self::ParseSegmentError(segment) => {
                write!(f, "Failed to parse segment \"{segment}\"")
            }
            Self::InvalidCommandName(command) => {
                write!(f, "Invalid command \"{command}\"")
            }
        }
    }
}
