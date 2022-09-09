use crate::command::Command;
use crate::translation_state::TranslationState;

pub struct Translator;

pub struct TranslationError {
    message: String,
}

impl Translator {
    fn translate(commands: impl Iterator<Item = Command>) -> Result<String, TranslationError> {
        let mut state = TranslationState::new();
        Ok(commands
            .map(|command| command.to_asm(&mut state))
            .collect::<Vec<_>>()
            .join("\n"))
    }
}
