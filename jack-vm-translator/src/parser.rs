use std::str::FromStr;

use crate::command::{Command, ParseCommandError};

pub struct Parser;

pub struct ParserError {
    line_number: u32,
    message: String,
    inner_error: ParseCommandError,
}

impl ParserError {
    fn new(err: ParseCommandError, line_number: u32) -> Self {
        let message = err.get_message(line_number);
        Self {
            line_number,
            message,
            inner_error: err,
        }
    }
}

impl Parser {
    fn parse(source: &str) -> Result<Vec<Command>, ParserError> {
        source
            .lines()
            .enumerate()
            .map(|(id, line)| {
                Command::from_str(line).map_err(|err| ParserError::new(err, id as u32 + 1))
            })
            .collect()
    }
}
