use crate::{ast::*, compiler::VMCode};
use anyhow::Result;

mod classes;

pub struct Emitter {
    ast: AST,
}

impl Emitter {
    pub fn new(ast: AST) -> Self {
        Self { ast }
    }

    pub fn emit(&mut self) -> Result<VMCode> {
        todo!()
    }

    fn emit_subroutine(&mut self, routine: SubroutineDeclarationNode) -> VMCode {
        todo!()
    }

    fn emit_expr(&mut self, routine: ExpressionNode) -> VMCode {
        todo!()
    }

    fn emit_class_var(&mut self, class_var: ClassVariableDeclarationNode) -> VMCode {
        todo!()
    }

    fn emit_statement(&mut self, statement: StatementNode) -> VMCode {
        todo!()
    }
}
