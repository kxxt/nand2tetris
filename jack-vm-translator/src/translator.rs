use crate::errors::TranslationError;
use crate::parser::ParsedSource;
use crate::translation_state::TranslationState;

pub struct Translator;

impl Translator {
    pub fn translate<'a>(parsed: &ParsedSource) -> Result<String, TranslationError> {
        let ParsedSource { name, commands } = parsed;
        let mut state = TranslationState::new(name);
        commands
            .iter()
            .map(|command| command.to_asm(&mut state))
            .collect::<Result<Vec<_>, _>>()
            .map(|v| v.join("\n"))
    }
    pub const BOOTSTRAP: &str = r"// SP = 256
@256
D=A
@SP
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
@ARG
M=D
// LCL = SP
@SP
D=M
@LCL
M=D
@Sys.init
0;JMP
";
}
