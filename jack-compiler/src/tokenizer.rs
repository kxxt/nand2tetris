use super::token::{Token, TokenKind};

pub struct Source {
    pub content: String,
    pub name: String,
}

pub struct Tokenizer {
    source: Source,
}

impl Tokenizer {
    pub fn new(source: Source) -> Self {
        Tokenizer { source }
    }

    fn name(&self) -> &str {
        return &self.source.name;
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
