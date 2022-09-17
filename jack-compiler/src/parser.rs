use crate::{ast::AST, tokenizer::Tokenizer};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    fn new(tokenizer: Tokenizer<'a>) -> Self {
        Self { tokenizer }
    }

    fn parse(self) -> AST {
        todo!()
    }
}
