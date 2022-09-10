use std::fmt::Debug;

pub trait Error: Debug {
    fn get_message(&self) -> String;
}

#[derive(Debug)]
pub struct ParserError {
    pub line_number: u32,
    pub message: String,
    pub inner_error: ParseCommandError,
}

impl ParserError {
    pub fn new(err: ParseCommandError, line_number: u32) -> Self {
        let message = err.get_message(line_number);
        Self {
            line_number,
            message,
            inner_error: err,
        }
    }
}

impl Error for ParserError {
    fn get_message(&self) -> String {
        self.inner_error.get_message(self.line_number)
    }
}

#[derive(Debug)]
pub struct TranslationError {
    pub message: String,
}

impl Error for TranslationError {
    fn get_message(&self) -> String {
        self.message.to_owned()
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

impl ParseCommandError {
    pub fn get_message(&self, line_number: u32) -> String {
        match self {
            Self::InvalidArgument(arg) => {
                format!("Invalid argument \"{arg}\" on line {line_number}.")
            }
            Self::NotEnoughArguments => format!("Not enough arguments on line {line_number}."),
            Self::TooManyArguments => format!("Too many arguments on line {line_number}"),
            Self::NoCommand => format!("No command on line {line_number}"),
            Self::ParseSegmentError(segment) => {
                format!("Failed to parse segment \"{segment}\" on line {line_number}")
            }
            Self::InvalidCommandName(command) => {
                format!("Invalid command \"{command}\" on line {line_number}")
            }
        }
    }
}
