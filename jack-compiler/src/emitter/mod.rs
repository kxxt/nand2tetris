use std::{collections::HashMap, fmt::Write};

use crate::{ast::*, compiler::VMCode, errors::EmitterError};
use anyhow::Result;

use self::{segment::Segment, variable::VariableInfo};

mod segment;
mod variable;

pub struct Emitter {
    class_name: Option<String>,
    root_table: HashMap<String, VariableInfo>,
    static_counter: u16,
    field_counter: u16,
    subroutine_table: Option<HashMap<String, VariableInfo>>,
    subroutines: HashMap<String, SubroutineKind>,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            root_table: HashMap::new(),
            static_counter: 0,
            field_counter: 0,
            subroutine_table: None,
            subroutines: HashMap::new(),
            class_name: None,
        }
    }

    pub fn emit(&mut self, ast: AST) -> Result<VMCode> {
        let mut code = String::new();
        for ele in ast.variables {
            self.handle_class_var(ele);
        }
        Ok(code)
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
            let info = VariableInfo {
                segment: match class_var.kind {
                    ClassVariableKind::Field => Segment::This,
                    ClassVariableKind::Static => Segment::Static,
                },
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

    fn handle_var(&mut self, var: VariableDeclarationNode, mut cnt: u16) -> Result<u16> {
        if self.subroutine_table.is_none() {
            return Err(EmitterError::NotInASubroutine.into());
        }
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
        Ok(cnt)
    }

    fn lookup_var(&self, name: &str) -> Result<&VariableInfo> {
        self.subroutine_table
            .as_ref()
            .and_then(|map| map.get(name))
            .or_else(|| self.root_table.get(name))
            .ok_or_else(|| EmitterError::VariableNotFound(name.to_string()).into())
    }

    fn emit_subroutine(&mut self, routine: SubroutineDeclarationNode) -> Result<VMCode> {
        todo!()
    }

    fn emit_term(&self, term: &TermNode) -> VMCode {
        return match term {
            TermNode::IntegerConstant(i) => format!("\npush {i}"),
            TermNode::StringConstant(s) => Self::emit_string(s.to_string()),
            TermNode::KeywordConstant(v) => {
                format!(
                    "\npush {}",
                    match v {
                        KeywordConstant::False => "false",
                        KeywordConstant::True => "true",
                        KeywordConstant::Null => "constant 0",
                        KeywordConstant::This => "pointer 0",
                    }
                )
            }
            TermNode::Variable(v) => todo!(),
            TermNode::Parentheses(expr) => self.emit_expr(&expr),
            TermNode::SubroutineCall(call) => todo!(),
            TermNode::UnaryOperation(op) => format!(
                r"
{}
{}",
                self.emit_term(&op.subject),
                match op.operator {
                    UnaryOperator::ArthemiticNegation => "neg",
                    UnaryOperator::LogicalNegation => "not",
                }
            ),
            TermNode::ArrayElement(ArrayElementNode { name, index }) => todo!(),
        };
    }

    fn emit_call(&self, call: &SubroutineCallNode) -> Result<VMCode> {
        let SubroutineCallNode {
            this,
            name,
            arguments,
        } = call;
        let mut code = String::new();
        let mut r#type = self.class_name.as_ref().unwrap();
        let mut arg_len = arguments.len();
        if let Some(this) = this {
            arg_len += 1;
            let info = self.lookup_var(&this.0)?;
            write!(code, "\npush {} {}", info.segment, info.index)?;
            if let TypeNode::Class(c) = &info.r#type {
                r#type = &c.0;
            } else {
                return Err(EmitterError::UnexpectedPrimitiveType(info.r#type.clone()).into());
            }
        }
        for arg in arguments {
            write!(code, "{}", self.emit_expr(arg))?;
        }
        write!(code, "\ncall {}.{} {arg_len}", r#type, &name.0)?;
        Ok(code)
    }

    fn emit_expr(&self, expr: &ExpressionNode) -> VMCode {
        if expr.parts.len() == 0 {
            return self.emit_term(&expr.term);
        }
        let mut code = self.emit_term(&expr.term);
        let mut iter = expr.parts.iter();
        while let Some(ExpressionPart { operator, term }) = iter.next() {
            code += &self.emit_term(term);
            code += match operator {
                BinaryOperator::Plus => "add",
                BinaryOperator::Minus => "sub",
                BinaryOperator::Multiply => todo!(),
                BinaryOperator::Divide => todo!(),
                BinaryOperator::And => "and",
                BinaryOperator::Or => "or",
                BinaryOperator::LessThan => "lt",
                BinaryOperator::GreaterThan => "gt",
                BinaryOperator::Equal => "eq",
            }
        }
        code
    }

    fn emit_return(&mut self, node: &ReturnNode) -> VMCode {
        let code = node
            .value
            .as_ref()
            .map(|expr| self.emit_expr(&expr))
            .unwrap_or(String::new());
        format!("{code}\nreturn")
    }

    fn emit_statement(&mut self, statement: StatementNode) -> VMCode {
        todo!()
    }

    fn emit_string(string: String) -> VMCode {
        let push_chars: String = string
            .chars()
            .map(|x| {
                format!(
                    r"
push temp 5
push constant {}
call String.appendChar 2
pop temp 3",
                    x as u16
                )
            })
            .collect();
        format!(
            r"
call String.new 0
pop temp 5
{push_chars}
push temp 5"
        )
    }
}
