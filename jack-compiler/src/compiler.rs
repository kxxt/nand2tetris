use std::error::Error;

use crate::tokenizer::Source;

pub struct Compiler;

pub type VMCode = String;

impl Compiler {
    pub fn compile(source: Source) -> Result<VMCode, Box<dyn Error>> {
        todo!()
    }
}
