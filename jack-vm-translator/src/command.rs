use lazy_static::lazy_static;

use std::{collections::HashMap, str::FromStr};

use crate::segment::Segment;
use crate::{
    errors::{ParseCommandError, TranslationError},
    translation_state::TranslationState,
};

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
    Label(String),
    GoTo(String),
    IfGoTo(String),
    // Function { name: String, nVars: u8 },
    // Call { name: String, nVars: u8 },
    // Return,
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let comment_pos = s.find("//");
        if let Some(comment_pos) = comment_pos {
            s = &s[..comment_pos]
        }
        s = s.trim();
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
        }
        match command_name {
            "pop" => Self::parse_push_pop_args(components)
                .map(|(segment, index)| Self::Pop { segment, i: index }),
            "push" => Self::parse_push_pop_args(components)
                .map(|(segment, index)| Self::Push { segment, i: index }),
            "label" => Self::parse_label_related_args(components).map(Self::Label),
            "goto" => Self::parse_label_related_args(components).map(Self::GoTo),
            "if-goto" => Self::parse_label_related_args(components).map(Self::IfGoTo),
            _ => Err(ParseCommandError::InvalidCommandName(
                command_name.to_string(),
            )),
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

    fn parse_label_related_args<'a>(
        mut it: impl Iterator<Item = &'a str>,
    ) -> Result<String, ParseCommandError> {
        let label = it.next().ok_or(ParseCommandError::NotEnoughArguments)?;
        let too_many_args = it.next().is_some();
        return if too_many_args {
            Err(ParseCommandError::TooManyArguments)
        } else {
            Ok(label.to_owned())
        };
    }

    pub fn to_asm(&self, state: &mut TranslationState) -> Result<String, TranslationError> {
        lazy_static! {
            static ref SEGMENT2SYMBOL: HashMap<Segment, &'static str> = HashMap::from([
                (Segment::Local, "LCL"),
                (Segment::Argument, "ARG"),
                (Segment::This, "THIS"),
                (Segment::That, "THAT")
            ]);
        }
        match self {
            Self::Add => Ok(r"// add
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) += D
M=M+D"
                .to_owned()),
            Self::Sub => Ok(r"// sub
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) - D
M=M-D"
                .to_owned()),
            Self::Neg => Ok(r"// neg
@SP
A=M-1 // *(sp-1) = -*(sp-1)
M=-M"
                .to_owned()),
            Self::Eq => {
                let cnt = state.advance_comparison_counter();
                Ok(format!(
                    r"// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.{name}.TRUE.{cnt}
D;JEQ
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.{name}.END.{cnt}
0;JMP
(CMPR.{name}.TRUE.{cnt})
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.{name}.END.{cnt})
",
                    name = state.name()
                ))
            }
            Self::Lt => {
                let cnt = state.advance_comparison_counter();
                Ok(format!(
                    r"// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.{name}.TRUE.{cnt}
D;JLT
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.{name}.END.{cnt}
0;JMP
(CMPR.{name}.TRUE.{cnt})
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.{name}.END.{cnt})
",
                    name = state.name()
                ))
            }
            Self::Gt => {
                let cnt = state.advance_comparison_counter();
                Ok(format!(
                    r"// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.{name}.TRUE.{cnt}
D;JGT
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.{name}.END.{cnt}
0;JMP
(CMPR.{name}.TRUE.{cnt})
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.{name}.END.{cnt})
",
                    name = state.name()
                ))
            }
            Self::And => Ok(r"// and
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) & D
M=M&D"
                .to_owned()),
            Self::Or => Ok(r"// or
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) | D
M=M|D"
                .to_owned()),
            Self::Not => Ok(r"// not
@SP
A=M-1 // *(sp-1) = !*(sp-1)
M=!M"
                .to_owned()),
            Self::Pop {
                segment: Segment::Static,
                i,
            } => Ok(format!(
                r"// pop static {i}
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@{}.{i}
M=D",
                state.name()
            )),
            Self::Pop {
                segment: Segment::Temp,
                i,
            } => {
                if i >= &8 {
                    return Err(TranslationError {
                        message: "Temp segment overflow!".to_string(),
                    });
                }
                let loc = i + 5;
                Ok(format!(
                    r"// pop temp {i}
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@{loc}
M=D"
                ))
            }
            Self::Pop {
                segment: Segment::Pointer,
                i,
            } => {
                if i >= &2 {
                    return Err(TranslationError {
                        message: "Pointer segment overflow!".to_string(),
                    });
                }
                let loc = if i == &0 { "THIS" } else { "THAT" };
                Ok(format!(
                    r"// pop pointer {i}
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@{loc}
M=D"
                ))
            }
            Self::Pop {
                segment: Segment::Constant,
                i,
            } => Err(TranslationError {
                message: format!("Illegal operation: pop constant {i}"),
            }),
            Self::Pop { segment, i } => Ok(format!(
                r"// pop {segment:?} {i}
// R13 = segment + i
@{symbol}
D=M
@{i}
D=D+A
@R13
M=D
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@R13
A=M
M=D",
                symbol = SEGMENT2SYMBOL.get(&segment).unwrap()
            )),
            Self::Push {
                segment: Segment::Constant,
                i,
            } => Ok(format!(
                r"// push constant i
@{i}
D=A
@SP
A=M
M=D
@SP
M=M+1"
            )),
            Self::Push {
                segment: Segment::Static,
                i,
            } => Ok(format!(
                r"// push static i
@{}.{i}
D=M
@SP
A=M
M=D
@SP
M=M+1",
                state.name()
            )),
            Self::Push {
                segment: Segment::Temp,
                i,
            } => {
                if i >= &8 {
                    return Err(TranslationError {
                        message: "Temp segment overflow!".to_string(),
                    });
                }
                let loc = i + 5;
                Ok(format!(
                    r"// push temp i
@{loc}
D=M
@SP
A=M
M=D
@SP
M=M+1"
                ))
            }
            Self::Push {
                segment: Segment::Pointer,
                i,
            } => {
                if i >= &2 {
                    return Err(TranslationError {
                        message: "Pointer segment overflow!".to_string(),
                    });
                }
                let loc = if i == &0 { "THIS" } else { "THAT" };
                Ok(format!(
                    r"// push pointer i
@{loc}
D=M
@SP
A=M
M=D
@SP
M=M+1"
                ))
            }
            Self::Push { segment, i } => Ok(format!(
                r"// push {segment:?} i
// D = *(segment + i)
@{symbol}
D=M
@{i}
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1",
                symbol = SEGMENT2SYMBOL.get(&segment).unwrap()
            )),
            Self::Label(label) => Ok(format!("({label})")),
            Self::GoTo(label) => Ok(format!("// goto {label}\n@{label}\n0;JMP")),
            Self::IfGoTo(label) => Ok(format!(
                r"// if-goto {label}
@SP
M=M-1
A=M
D=M
@{label}
D;JGT"
            )),
        }
    }
}
