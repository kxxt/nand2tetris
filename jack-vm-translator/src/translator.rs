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
}
