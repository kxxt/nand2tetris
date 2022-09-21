use std::error::Error;

use crate::{
    emitter::Emitter,
    parser::Parser,
    tokenizer::{Source, Tokenizer},
};

pub struct Compiler;

pub type VMCode = String;

impl Compiler {
    pub fn compile(source: Source) -> Result<VMCode, Box<dyn Error>> {
        let token_stream = Tokenizer::stream(&source);
        let ast = Parser::new(token_stream, source.name.to_string()).parse()?;
        Ok(Emitter::new(ast).emit()?)
    }
}
