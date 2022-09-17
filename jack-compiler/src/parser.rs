use crate::{ast::AST, tokenizer::Tokenizer};

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }

    fn parse(self) -> AST {
        todo!()
    }
}
