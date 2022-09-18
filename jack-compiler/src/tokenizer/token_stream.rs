use super::*;
use std::borrow::Cow;

pub struct TokenStream<'a> {
    pub(super) source: Cow<'a, str>,
    pub(super) source_name: &'a str,
    pub(super) offset: usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(source: Cow<'a, str>, source_name: &'a str) -> Self {
        Self {
            source,
            source_name,
            offset: 0,
        }
    }

    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    fn eat_whitespace(&mut self) {
        let count = self
            .source()
            .chars()
            .take_while(|s| s.is_whitespace())
            .count();
        self.eat(count);
    }

    fn eat(&mut self, offset: usize) {
        self.offset += offset;
    }

    fn source(&self) -> &str {
        &self.source[self.offset..]
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.eat_whitespace();
        if self.source().is_empty() {
            None
        } else {
            Some(
                Tokenizer::tokenize_one_token(self.source()).map(|(token, offset)| {
                    self.eat(offset);
                    token
                }),
            )
        }
    }
}
