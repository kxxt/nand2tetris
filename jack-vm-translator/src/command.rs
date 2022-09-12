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
    Function { name: String, n_vars: u16 },
    Call { name: String, n_vars: u16 },
    Return,
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
            "return" => Some(Self::Return),
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
            "function" => Self::parse_func_related_args(components)
                .map(|(name, n_vars)| Self::Function { name, n_vars }),
            "call" => Self::parse_func_related_args(components)
                .map(|(name, n_vars)| Self::Call { name, n_vars }),
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

    fn parse_func_related_args<'a>(
        mut it: impl Iterator<Item = &'a str>,
    ) -> Result<(String, u16), ParseCommandError> {
        let name = it.next().ok_or(ParseCommandError::NotEnoughArguments)?;
        let n_vars = it
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
            Ok((name.to_string(), n_vars))
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
AM=M-1 // D = *(--sp)
D=M
A=A-1 // *(sp-1) += D
M=M+D"
                .to_owned()),
            Self::Sub => Ok(r"// sub
@SP
AM=M-1 // D = *(--sp)
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
AM=M-1 // D = *(--sp)
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
AM=M-1 // D = *(--sp)
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
AM=M-1 // D = *(--sp)
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
AM=M-1 // D = *(--sp)
D=M
A=A-1 // *(sp-1) = *(sp-1) & D
M=M&D"
                .to_owned()),
            Self::Or => Ok(r"// or
@SP
AM=M-1 // D = *(--sp)
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
AM=M-1 // D = *(--sp)
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
AM=M-1 // D = *(--sp)
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
AM=M-1 // D = *(--sp)
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
AM=M-1 // *R13=*(--sp)
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
            Self::Label(label) => Ok(if let Some(func) = state.func() {
                format!("({func}${label})")
            } else {
                format!("({label})")
            }),
            Self::GoTo(label) => Ok(if let Some(func) = state.func() {
                format!("// goto {func}${label}\n@{func}${label}\n0;JMP")
            } else {
                format!("// goto {label}\n@{label}\n0;JMP")
            }),
            Self::IfGoTo(label) => Ok(if let Some(func) = state.func() {
                format!(
                    r"// if-goto {func}${label}
@SP
AM=M-1
D=M
@{func}${label}
D;JNE"
                )
            } else {
                format!(
                    r"// if-goto {label}
@SP
AM=M-1
D=M
@{label}
D;JGT"
                )
            }),
            Self::Function { name, n_vars } => {
                state.enter_func(name);
                Ok(format!(
                    r"// function {func} {n_vars}
({func})
@LCL
A=M{init}
@SP
D=M
@{n_vars}
D=D+A
@SP
M=D",
                    func = name,
                    init = "\nM=0\nA=A+1".repeat(*n_vars as usize)
                ))
            }
            Self::Return => Ok(format!(
                r"// return (from {func})
// R14(return addr) = *(LCL - 5)
@LCL
D=M
@5
A=D-A
D=M
@R14
M=D
// *ARG = top()
@SP
A=M-1
D=M
@ARG
A=M
M=D
// SP = ARG + 1
@ARG
D=M+1
@SP
M=D
// THAT = *(LCL - 1),...etc
@LCL
AM=M-1
D=M
@THAT
M=D
@LCL
AM=M-1
D=M
@THIS
M=D
@LCL
AM=M-1
D=M
@ARG
M=D
@LCL
A=M-1
D=M
@LCL
M=D
// goto *R14
@R14
A=M
0;JMP",
                func = state.func().ok_or(TranslationError {
                    message: format!("Not in a function"),
                })?
            )),
            Self::Call { name, n_vars } => Ok(format!(
                r"// call {name} {n_vars}
@{scope}$ret.{i}
D=A
@SP
A=M
M=D
// push LCL,...etc
@LCL
D=M
@SP
AM=M+1
M=D
@ARG
D=M
@SP
AM=M+1
M=D
@THIS
D=M
@SP
AM=M+1
M=D
@THAT
D=M
@SP
AM=M+1
M=D
@SP
M=M+1
// new ARG
@SP
D=M
@5
D=D-A
@{n_vars}
D=D-A
@ARG
M=D
// LCL = SP
@SP
D=M
@LCL
M=D
// goto {name}
@{name}
0;JMP
// (return addr)
({scope}$ret.{i})
",
                i = state.advance_ret_counter(),
                scope = state.func().ok_or(TranslationError {
                    message: "Not in a function ".to_string()
                })?
            )),
        }
    }
}
