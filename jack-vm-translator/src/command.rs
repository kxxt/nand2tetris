use std::str::FromStr;

#[derive(Debug)]
enum Segment {
    Static,
    Local,
    Argument,
    This,
    That,
    Temp,
}

impl FromStr for Segment {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "static" => Ok(Self::Static),
            "local" => Ok(Self::Local),
            "argument" => Ok(Self::Argument),
            "this" => Ok(Self::This),
            "that" => Ok(Self::That),
            "temp" => Ok(Self::Temp),
            _ => Err(ParseCommandError::ParseSegmentError(format!(
                "Invalid segment {s}!"
            ))),
        }
    }
}

#[derive(Debug)]
enum Command {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    Pop { segment: Segment, i: u16 },
    Push { segment: Segment, i: u16 },
    // Label(u16),
    // GoTo(u16),
    // IfGoTo(u16),
    // Function { name: String, nVars: u8 },
    // Call { name: String, nVars: u8 },
    // Return,
}

enum ParseCommandError {
    InvalidCommandName(String),
    NotEnoughArguments,
    TooManyArguments,
    InvalidArgument(String),
    NoCommand,
    ParseSegmentError(String),
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split_whitespace();
        let command_name = components.next().ok_or(ParseCommandError::NoCommand)?;

        if let Some(no_arg_cmd) = match command_name {
            "add" => Some(Self::Add),
            "sub" => Some(Self::Sub),
            "neg" => Some(Self::Neg),
            "eq" => Some(Self::Eq),
            "lt" => Some(Self::Lt),
            "gt" => Some(Self::Gt),
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            "not" => Some(Self::Not),
            _ => None,
        } {
            return if components.next().is_some() {
                Err(ParseCommandError::TooManyArguments)
            } else {
                Ok(no_arg_cmd)
            };
        } else {
            if command_name == "pop" {
                return Self::parse_push_pop_args(components).map(|(segment, index)| Self::Pop {
                    segment: segment,
                    i: index,
                });
            } else if command_name == "push" {
                return Self::parse_push_pop_args(components).map(|(segment, index)| Self::Push {
                    segment: segment,
                    i: index,
                });
            } else {
                return Err(ParseCommandError::InvalidCommandName(
                    command_name.to_string(),
                ));
            }
        }
    }
}

impl Command {
    fn parse_push_pop_args<'a, I>(mut it: I) -> Result<(Segment, u16), ParseCommandError>
    where
        I: Iterator<Item = &'a str>,
    {
        let segment = it
            .next()
            .ok_or(ParseCommandError::NotEnoughArguments)
            .and_then(Segment::from_str)?;
        let index = it
            .next()
            .ok_or(ParseCommandError::NotEnoughArguments)
            .and_then(|i| {
                i.parse::<u16>()
                    .map_err(|err| ParseCommandError::InvalidArgument(err.to_string()))
            })?;
        let too_many_args = it.next().is_some();
        return if too_many_args {
            Err(ParseCommandError::TooManyArguments)
        } else {
            Ok((segment, index))
        };
    }
}
