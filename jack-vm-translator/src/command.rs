use std::str::FromStr;

use crate::translation_state::TranslationState;

#[derive(Debug)]
pub enum Segment {
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
pub enum Command {
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
    fn parse_push_pop_args<'a>(
        mut it: impl Iterator<Item = &'a str>,
    ) -> Result<(Segment, u16), ParseCommandError> {
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

    fn to_asm(self, state: &mut TranslationState) -> String {
        match self {
            Self::Add => r"// add
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) += D
M=M+D"
                .to_owned(),
            Self::Sub => r"// sub
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) - D
M=M-D"
                .to_owned(),
            Self::Neg => r"// neg
@SP
A=M-1 // *(sp-1) = -*(sp-1)
M=-M"
                .to_owned(),
            Self::Eq => {
                let cnt = state.advance_comparison_counter();
                format!(
                    r"// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.TRUE.{cnt}
D;JEQ
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.END.{cnt}
0;JMP
(CMPR.TRUE.{cnt})
@65535
D=A
@SP
A=M-1 // *(sp-1) = 0xffff
M=D
(CMPR.END.{cnt})
"
                )
            }
            Self::Lt => {
                let cnt = state.advance_comparison_counter();
                format!(
                    r"// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.TRUE.{cnt}
D;JLT
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.END.{cnt}
0;JMP
(CMPR.TRUE.{cnt})
@65535
D=A
@SP
A=M-1 // *(sp-1) = 0xffff
M=D
(EQ.END.{cnt})
"
                )
            }
            Self::Gt => {
                let cnt = state.advance_comparison_counter();
                format!(
                    r"// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.TRUE.{cnt}
D;JGT
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.END.{cnt}
0;JMP
(CMPR.TRUE.{cnt})
@65535
D=A
@SP
A=M-1 // *(sp-1) = 0xffff
M=D
(EQ.END.{cnt})
"
                )
            }
            Self::And => r"// and
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) & D
M=M&D"
                .to_owned(),
            Self::Or => r"// or
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) | D
M=M&D"
                .to_owned(),
            Self::Not => r"// not
@SP
A=M-1 // *(sp-1) = !*(sp-1)
M=!M"
                .to_owned(),
            _ => todo!(),
        }
    }
}
