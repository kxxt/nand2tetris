use std::str::FromStr;

use crate::command::Command;
use crate::errors::{ParserError, ParseCommandError};

pub struct Parser;

impl Parser {
    pub fn parse(source: &str) -> Result<Vec<Command>, ParserError> {
        source
            .lines()
            .enumerate()
            .filter_map(|(id, line)| {
                let result = Command::from_str(line);
                if let Err(ParseCommandError::NoCommand) = result {
                    None
                } else {
                    Some(result.map_err(|err| ParserError::new(err, id as u32 + 1)))
                }
            })
            .collect()
    }
}
