use std::str::FromStr;

use crate::command::Command;
use crate::errors::{ParseCommandError, ParserError};
use crate::source::Source;

pub struct ParsedSource {
    commands: Vec<Command>,
    name: String,
}

pub struct Parser;

impl Parser {
    pub fn parse(source: Source) -> Result<ParsedSource, ParserError> {
        Ok(ParsedSource {
            commands: source
                .content
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
                .collect::<Result<Vec<_>, _>>()?,
            name: source.name,
        })
    }
}
