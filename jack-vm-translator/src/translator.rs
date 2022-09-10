use crate::command::Command;
use crate::translation_state::TranslationState;
use crate::errors::TranslationError;

pub struct Translator;



impl Translator {
    pub fn translate(commands: impl Iterator<Item = Command>) -> Result<String, TranslationError> {
        let mut state = TranslationState::new();
        Ok(commands
            .map(|command| command.to_asm(&mut state))
            .collect::<Vec<_>>()
            .join("\n"))
    }
}
