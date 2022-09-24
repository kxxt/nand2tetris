use std::collections::HashMap;

use crate::{ast::*, compiler::VMCode, errors::EmitterError};
use anyhow::Result;

use self::{
    segment::Segment,
    variable::{ClassVariableInfo, VariableInfo},
};

mod segment;
mod variable;

pub struct Emitter {
    root_table: HashMap<String, ClassVariableInfo>,
    static_counter: u16,
    field_counter: u16,
    subroutine_table: Option<HashMap<String, VariableInfo>>,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            root_table: HashMap::new(),
            static_counter: 0,
            field_counter: 0,
            subroutine_table: None,
        }
    }

    pub fn emit(&mut self, ast: AST) -> Result<VMCode> {
        let mut code = String::new();
        for ele in ast.variables {
            self.handle_class_var(ele);
        }
        Ok(code)
    }

    fn emit_subroutine(&mut self, routine: SubroutineDeclarationNode) -> VMCode {
        todo!()
    }

    fn emit_expr(&mut self, routine: ExpressionNode) -> VMCode {
        todo!()
    }

    fn advance_static_counter(&mut self) -> u16 {
        let temp = self.static_counter;
        self.static_counter += 1;
        self.static_counter
    }

    fn advance_field_counter(&mut self) -> u16 {
        let temp = self.field_counter;
        self.field_counter += 1;
        self.field_counter
    }

    fn handle_class_var(&mut self, class_var: ClassVariableDeclarationNode) {
        for name in class_var.variables.names {
            let info = ClassVariableInfo {
                kind: class_var.kind,
                r#type: class_var.variables.r#type.clone(),
                index: if class_var.kind == ClassVariableKind::Static {
                    self.advance_static_counter()
                } else {
                    self.advance_field_counter()
                },
            };
            self.root_table.insert(name.0, info);
        }
    }

    fn handle_var(&mut self, var: VariableDeclarationNode) -> Result<()> {
        if self.subroutine_table.is_none() {
            return Err(EmitterError::NotInASubroutine.into());
        }
        let mut cnt = 0u16;
        for name in var.names {
            let info = VariableInfo {
                r#type: var.r#type.clone(),
                segment: Segment::Local,
                index: {
                    let temp = cnt;
                    cnt += 1;
                    temp
                },
            };
            self.subroutine_table.as_mut().unwrap().insert(name.0, info);
        }
        Ok(())
    }

    fn emit_statement(&mut self, statement: StatementNode) -> VMCode {
        todo!()
    }
}
