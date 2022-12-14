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
    label_counter: u16,
    subroutine_table: Option<HashMap<String, VariableInfo>>,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            root_table: HashMap::new(),
            static_counter: 0,
            field_counter: 0,
            label_counter: 0,
            subroutine_table: None,
            class_name: None,
        }
    }

    pub fn emit(&mut self, ast: &AST) -> Result<VMCode> {
        let mut code = String::new();
        self.class_name = Some(ast.name.0.to_string());
        for ele in &ast.variables {
            self.handle_class_var(ele);
        }
        for subroutine in &ast.subroutines {
            code += &self.emit_subroutine(subroutine)?;
        }
        Ok(code)
    }

    fn advance_static_counter(&mut self) -> u16 {
        let temp = self.static_counter;
        self.static_counter += 1;
        temp
    }

    fn advance_field_counter(&mut self) -> u16 {
        let temp = self.field_counter;
        self.field_counter += 1;
        temp
    }

    fn next_label(&mut self) -> String {
        let label = format!(
            "{}.LABEL.{}",
            self.class_name.as_ref().unwrap(),
            self.label_counter
        );
        self.label_counter += 1;
        label
    }

    fn handle_class_var(&mut self, class_var: &ClassVariableDeclarationNode) {
        for name in &class_var.variables.names {
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
            self.root_table.insert(name.0.to_string(), info);
        }
    }

    fn handle_var(&mut self, var: &VariableDeclarationNode, mut cnt: u16) -> Result<u16> {
        if self.subroutine_table.is_none() {
            return Err(EmitterError::NotInASubroutine.into());
        }
        for name in &var.names {
            let info = VariableInfo {
                r#type: var.r#type.clone(),
                segment: Segment::Local,
                index: {
                    let temp = cnt;
                    cnt += 1;
                    temp
                },
            };
            self.subroutine_table
                .as_mut()
                .unwrap()
                .insert(name.0.clone(), info);
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

    fn emit_constructor(
        &mut self,
        ctor: &SubroutineDeclarationNode,
        var_cnt: u16,
    ) -> Result<VMCode> {
        let SubroutineDeclarationNode {
            kind,
            return_type,
            name,
            body,
            ..
        } = ctor;
        // checks
        assert_eq!(kind, &SubroutineKind::Constructor);
        let class_name = self.class_name.as_ref().unwrap();
        let expected = Some(TypeNode::Class(class_name.clone().into()));
        if return_type != &expected {
            return Err(EmitterError::MismatchedType(expected, return_type.clone()).into());
        }
        // format VMCode
        let name = &name.0;
        let fields_cnt = self.field_counter;
        let mut code = format!(
            r#"
function {class_name}.{name} {var_cnt}
push constant {fields_cnt}
call Memory.alloc 1
pop pointer 0"#
        );
        write!(code, "{}", self.emit_statements(&body.statements)?)?;
        Ok(code)
    }

    fn emit_function(&mut self, func: &SubroutineDeclarationNode, var_cnt: u16) -> Result<VMCode> {
        let SubroutineDeclarationNode {
            kind, name, body, ..
        } = func;
        // checks
        assert_eq!(kind, &SubroutineKind::Function);
        // format VMCode
        let name = &name.0;
        let class_name = self.class_name.as_deref().unwrap();
        let mut code = format!("\nfunction {class_name}.{name} {var_cnt}");
        write!(code, "{}", self.emit_statements(&body.statements)?)?;
        Ok(code)
    }

    fn emit_method(&mut self, func: &SubroutineDeclarationNode, var_cnt: u16) -> Result<VMCode> {
        let SubroutineDeclarationNode {
            kind, name, body, ..
        } = func;
        // checks
        assert_eq!(kind, &SubroutineKind::Method);
        // format VMCode
        let name = &name.0;
        let class_name = self.class_name.as_deref().unwrap();
        let mut code = format!(
            r#"
function {class_name}.{name} {var_cnt}
push argument 0
pop pointer 0"#
        );
        write!(code, "{}", self.emit_statements(&body.statements)?)?;
        Ok(code)
    }

    fn emit_subroutine(&mut self, subroutine: &SubroutineDeclarationNode) -> Result<VMCode> {
        // handle variables
        self.subroutine_table = Some(HashMap::new());
        let mut cnt = 0;
        for var in &subroutine.body.variables {
            cnt = self.handle_var(var, cnt)?;
        }
        // handle parameters
        let mut param_cnt = if subroutine.kind == SubroutineKind::Method {
            1
        } else {
            0
        };
        for param in &subroutine.parameters {
            self.subroutine_table.as_mut().unwrap().insert(
                param.name.0.to_string(),
                VariableInfo {
                    r#type: param.r#type.clone(),
                    segment: Segment::Argument,
                    index: {
                        let temp = param_cnt;
                        param_cnt += 1;
                        temp
                    },
                },
            );
        }
        match subroutine.kind {
            SubroutineKind::Constructor => self.emit_constructor(subroutine, cnt),
            SubroutineKind::Function => self.emit_function(subroutine, cnt),
            SubroutineKind::Method => self.emit_method(subroutine, cnt),
        }
    }

    fn emit_term(&self, term: &TermNode) -> Result<VMCode> {
        Ok(match term {
            TermNode::IntegerConstant(i) => format!("\npush constant {i}"),
            TermNode::StringConstant(s) => Self::emit_string(s.to_string()),
            TermNode::KeywordConstant(v) => {
                format!(
                    "\n{}",
                    match v {
                        KeywordConstant::False | KeywordConstant::Null => "push constant 0",
                        KeywordConstant::True => "push constant 0\nnot",
                        KeywordConstant::This => "push pointer 0",
                    }
                )
            }
            TermNode::Variable(v) => {
                let VariableInfo { segment, index, .. } = self.lookup_var(&v.0)?;
                format!("\npush {} {}", segment, index)
            }
            TermNode::Parentheses(expr) => self.emit_expr(&expr)?,
            TermNode::SubroutineCall(call) => self.emit_call(call)?,
            TermNode::UnaryOperation(op) => format!(
                r"
{}
{}",
                self.emit_term(&op.subject)?,
                match op.operator {
                    UnaryOperator::ArthemiticNegation => "neg",
                    UnaryOperator::LogicalNegation => "not",
                }
            ),
            TermNode::ArrayElement(ArrayElementNode { name, index }) => {
                let VariableInfo {
                    segment,
                    index: var_index,
                    ..
                } = self.lookup_var(&name.0)?;
                format!(
                    r"
push {segment} {var_index}{}
add
pop pointer 1
push that 0",
                    self.emit_expr(index)?
                )
            }
        })
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
            if let Ok(info) = self.lookup_var(&this.0) {
                // look up for variable
                arg_len += 1;
                write!(code, "\npush {} {}", info.segment, info.index)?;
                if let TypeNode::Class(c) = &info.r#type {
                    r#type = &c.0;
                } else {
                    return Err(EmitterError::UnexpectedPrimitiveType(info.r#type.clone()).into());
                }
            } else {
                // variable not found. pretend it is a type
                // (function call)
                r#type = &this.0;
            }
        } else {
            // method call
            arg_len += 1;
            write!(code, "\npush pointer 0")?;
        }
        for arg in arguments {
            write!(code, "{}", self.emit_expr(arg)?)?;
        }
        write!(code, "\ncall {}.{} {arg_len}", r#type, &name.0)?;
        Ok(code)
    }

    fn emit_expr(&self, expr: &ExpressionNode) -> Result<VMCode> {
        if expr.parts.len() == 0 {
            return Ok(self.emit_term(&expr.term)?);
        }
        let mut code = self.emit_term(&expr.term)?;
        let mut iter = expr.parts.iter();
        while let Some(ExpressionPart { operator, term }) = iter.next() {
            write!(code, "{}\n", self.emit_term(term)?)?;
            code += match operator {
                BinaryOperator::Plus => "add",
                BinaryOperator::Minus => "sub",
                BinaryOperator::Multiply => "call Math.multiply 2",
                BinaryOperator::Divide => "call Math.divide 2",
                BinaryOperator::And => "and",
                BinaryOperator::Or => "or",
                BinaryOperator::LessThan => "lt",
                BinaryOperator::GreaterThan => "gt",
                BinaryOperator::Equal => "eq",
            }
        }
        Ok(code)
    }

    fn emit_return(&self, node: &ReturnNode) -> Result<VMCode> {
        let code = node
            .value
            .as_ref()
            .map(|expr| self.emit_expr(&expr))
            .transpose()?
            .unwrap_or("\npush constant 0".to_string());
        Ok(format!("{code}\nreturn"))
    }

    fn emit_let(&self, node: &LetNode) -> Result<VMCode> {
        let LetNode { name, index, value } = node;
        if index.is_none() {
            let VariableInfo { segment, index, .. } = self.lookup_var(&name.0)?;
            let mut code = self.emit_expr(value)?;
            write!(code, "\npop {} {}", segment, index)?;
            Ok(code)
        } else {
            let LetNode { name, index, value } = node;
            let VariableInfo {
                segment,
                index: var_index,
                ..
            } = self.lookup_var(&name.0)?;
            let index = self.emit_expr(index.as_ref().unwrap())?;
            let value = self.emit_expr(value)?;
            Ok(format!(
                r"
push {segment} {var_index}{index}
add{value}
pop temp 4
pop pointer 1
push temp 4
pop that 0"
            ))
        }
    }

    fn emit_do(&self, node: &DoNode) -> Result<VMCode> {
        let mut code = self.emit_call(&node.call)?;
        code += "\npop temp 3";
        Ok(code)
    }

    fn emit_if(&mut self, node: &IfElseNode) -> Result<VMCode> {
        let IfElseNode {
            condition,
            statements,
            else_node,
        } = node;
        let mut code = self.emit_expr(condition)?;
        let label_else = self.next_label();
        let label_end = self.next_label();
        let code_if = self.emit_statements(statements)?;
        let code_else = if let Some(else_statements) = else_node {
            self.emit_statements(else_statements)?
        } else {
            String::new()
        };
        write!(
            code,
            r#"
not
if-goto {label_else}{code_if}
goto {label_end}
label {label_else}{code_else}
label {label_end}"#
        )?;
        Ok(code)
    }

    fn emit_while(&mut self, node: &WhileNode) -> Result<VMCode> {
        let WhileNode {
            condition,
            statements,
        } = node;
        let label_cond = self.next_label();
        let label_end = self.next_label();
        let cond = self.emit_expr(condition)?;
        let body = self.emit_statements(statements)?;
        Ok(format!(
            r"
label {label_cond}{cond}
not
if-goto {label_end}{body}
goto {label_cond}
label {label_end}"
        ))
    }

    fn emit_statement(&mut self, statement: &StatementNode) -> Result<VMCode> {
        match statement {
            StatementNode::IfElse(node) => self.emit_if(node),
            StatementNode::Do(node) => self.emit_do(node),
            StatementNode::Let(node) => self.emit_let(node),
            StatementNode::While(node) => self.emit_while(node),
            StatementNode::Return(node) => self.emit_return(node),
        }
    }

    fn emit_string(string: String) -> VMCode {
        let len = string.len();
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
push constant {len}
call String.new 1
pop temp 5{push_chars}
push temp 5"
        )
    }

    fn emit_statements(&mut self, statements: &[StatementNode]) -> Result<VMCode> {
        let codes: Result<Vec<_>> = statements.iter().map(|s| self.emit_statement(s)).collect();
        Ok(codes?.join(""))
    }
}
