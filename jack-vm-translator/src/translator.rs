use crate::command::Command;
use crate::errors::TranslationError;
use crate::translation_state::TranslationState;

pub struct Translator;

impl Translator {
    pub fn translate(commands: impl Iterator<Item = Command>) -> Result<String, TranslationError> {
        let mut state = TranslationState::new();
        commands
            .map(|command| command.to_asm(&mut state))
            .collect::<Result<Vec<_>, _>>()
            .map(|v| v.join("\n"))
    }
}
