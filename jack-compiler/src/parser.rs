use std::convert::{TryFrom, TryInto};

use crate::ast::*;

use super::errors::ParserError;
use super::token::{Token, TokenKind, TokenRef};
use super::{ast::AST, tokenizer::TokenStream};
use anyhow::{Ok, Result};

pub struct Parser<'a> {
    token_stream: TokenStream<'a>,
    token_buffer: Option<Token>,
    token_storage: Option<Token>,
}
macro_rules! unexpected_token {
    ($token:expr, $($t:tt)*) => {
        return Err(ParserError::UnexpectedToken($token, format!($($t)*)).into())
    };
}

impl Token {
    pub fn should_eq(self, token: TokenRef) -> Result<Token> {
        if self == token {
            Ok(self)
        } else {
            unexpected_token!(self, "{}", token);
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: TokenStream<'a>) -> Self {
        Self {
            token_stream,
            token_buffer: None,
            token_storage: None,
        }
    }

    fn parse(&mut self) -> AST {
        todo!()
    }

    pub fn source_name(&self) -> &str {
        self.token_stream.source_name()
    }

    /// grab next token with confidence
    fn next_token(&mut self) -> Result<Token> {
        Ok(if self.token_buffer.is_some() {
            let v = self.token_buffer.take().unwrap();
            self.token_buffer = self.token_storage.take();
            v
        } else {
            self.token_stream
                .next()
                .ok_or(ParserError::UnexpectedEndOfStream)??
        })
    }

    /// put back one token into buffer
    fn store_token(&mut self, token: Token) {
        if self.token_buffer.is_none() {
            self.token_buffer = Some(token);
        } else if self.token_storage.is_none() {
            self.token_storage = self.token_buffer.take();
            self.token_buffer = Some(token);
        } else {
            panic!("Invalid parser state! No space to store token!");
        }
    }

    /// peek next token
    fn peek(&mut self) -> Result<Option<&Token>> {
        if self.token_buffer.is_none() {
            self.token_buffer = self.token_stream.next().transpose()?;
        }
        Ok(self.token_buffer.as_ref())
    }

    /// eat one token with confidence
    fn eat(&mut self) -> Result<()> {
        self.next_token()?;
        Ok(())
    }

    fn parse_class(&mut self) -> ClassNode {
        todo!()
    }

    fn parse_class_variable_declaration(&mut self) -> Result<ClassVariableDeclarationNode> {
        let token = self.next_token()?;
        if token.kind != TokenKind::Keyword {
            unexpected_token!(token, "static or field");
        }
        let kind = match token.value.as_str() {
            "static" => Ok(ClassVariableKind::Static),
            "field" => Ok(ClassVariableKind::Field),
            _ => unexpected_token!(token, "static or field"),
        }?;
        let variables = self.parse_variable_declaration(true)?;
        Ok(ClassVariableDeclarationNode { kind, variables })
    }

    fn parse_class_variable_declarations(
        &mut self,
    ) -> Result<NodeCollection<ClassVariableDeclarationNode>> {
        let mut list = NodeCollection::new();
        while let Some(TokenRef {
            kind: TokenKind::Keyword,
            value: "static" | "field",
        }) = self.peek()?.map(|x| x.as_ref())
        {
            list.push(self.parse_class_variable_declaration()?)
        }
        Ok(list)
    }

    fn eat_identifier(&mut self) -> Result<String> {
        let token = self.next_token()?;
        if token.kind == TokenKind::Identifier {
            Ok(token.value)
        } else {
            unexpected_token!(token, "identifier");
        }
    }

    fn eat_symbol(&mut self, symbol: &str) -> Result<String> {
        let token = self.next_token()?;
        if token.kind == TokenKind::Symbol && token.value == symbol {
            Ok(token.value)
        } else {
            unexpected_token!(token, "symbol \"\"");
        }
    }

    fn look_ahead_for_symbol(&mut self, symbol: &str) -> Result<bool> {
        let token = self.peek()?;
        if token.is_none() {
            return Ok(false);
        }
        let token = token.unwrap();
        Ok(token.kind == TokenKind::Symbol && token.value == symbol)
    }

    fn parse_type(&mut self, permit_void: bool) -> Result<Option<TypeNode>> {
        let token = self.next_token()?;
        let err_msg = if permit_void {
            "type or \"void\""
        } else {
            "type"
        };
        if token.kind == TokenKind::Keyword {
            match token.value.as_str() {
                "int" => Ok(Some(TypeNode::Int)),
                "char" => Ok(Some(TypeNode::Char)),
                "boolean" => Ok(Some(TypeNode::Boolean)),
                "void" => {
                    if permit_void {
                        Ok(None)
                    } else {
                        unexpected_token!(token, "type");
                    }
                }
                _ => unexpected_token!(token, "{}", err_msg),
            }
        } else if token.kind == TokenKind::Identifier {
            Ok(Some(TypeNode::Class(token.value.into())))
        } else {
            unexpected_token!(token, "{}", err_msg);
        }
    }

    fn parse_variable_declaration(
        &mut self,
        skip_var_token: bool,
    ) -> Result<VariableDeclarationNode> {
        if !skip_var_token {
            self.next_token()?.should_eq(TokenRef {
                kind: &TokenKind::Keyword,
                value: "var",
            })?;
        }
        let r#type = self.parse_type(false)?.unwrap();
        let mut names = NodeCollection::new();
        names.push(self.eat_identifier()?.into());
        while let Some(TokenRef {
            kind: TokenKind::Symbol,
            value: ",",
        }) = self.peek()?.map(|x| x.as_ref())
        {
            self.eat()?;
            names.push(self.eat_identifier()?.into());
        }
        self.eat_symbol(";")?;
        Ok(VariableDeclarationNode { r#type, names })
    }

    fn parse_subroutine_declaration(&mut self) -> Result<SubroutineDeclarationNode> {
        // parse (constructor|function|method)
        let token = self.next_token()?;
        let kind = match token.as_ref() {
            TokenRef {
                kind: TokenKind::Keyword,
                value: "constructor",
            } => Ok(SubroutineKind::Constructor),
            TokenRef {
                kind: TokenKind::Keyword,
                value: "function",
            } => Ok(SubroutineKind::Function),
            TokenRef {
                kind: TokenKind::Keyword,
                value: "method",
            } => Ok(SubroutineKind::Method),
            _ => unexpected_token!(token, r#""constructor", "function" or "method""#),
        }?;
        // parse (void|type)
        let return_type = self.parse_type(true)?;
        // parse subroutine name
        let name = self.eat_identifier()?.into();
        self.eat_symbol("(")?;
        // parse parameter list
        let parameters = self.parse_parameter_list()?;
        self.eat_symbol(")")?;
        // parse subroutine body
        let body = self.parse_subroutine_body()?;
        Ok(SubroutineDeclarationNode {
            kind,
            return_type,
            name,
            parameters,
            body,
        })
    }

    fn parse_parameter(&mut self) -> Result<ParameterNode> {
        let r#type = self.parse_type(false)?.unwrap();
        let name = self.eat_identifier()?.into();
        Ok(ParameterNode { r#type, name })
    }

    fn parse_parameter_list(&mut self) -> Result<NodeCollection<ParameterNode>> {
        let mut list = NodeCollection::new();
        if self.look_ahead_for_symbol(")")? {
            return Ok(list);
        }
        list.push(self.parse_parameter()?);
        while let Some(TokenRef {
            kind: TokenKind::Symbol,
            value: ",",
        }) = self.peek()?.map(|x| x.as_ref())
        {
            self.eat()?;
            list.push(self.parse_parameter()?);
        }
        Ok(list)
    }

    fn parse_subroutine_body(&mut self) -> Result<SubroutineBody> {
        self.eat_symbol("{")?;

        let mut variables = NodeCollection::new();
        while let Some(TokenRef {
            kind: TokenKind::Keyword,
            value: "var",
        }) = self.peek()?.map(|x| x.as_ref())
        {
            variables.push(self.parse_variable_declaration(false)?);
        }
        todo!();
        self.eat_symbol("}")?;
        todo!()
    }
    fn parse_statements(&mut self) -> Result<NodeCollection<StatementNode>> {
        let mut list: Vec<StatementNode> = NodeCollection::new();
        while let Some(TokenRef {
            kind: TokenKind::Keyword,
            value: val @ ("let" | "if" | "while" | "do" | "return"),
        }) = self.peek()?.map(|t| t.as_ref())
        {
            list.push(match val {
                "let" => self.parse_let_statement()?.into(),
                "if" => self.parse_if_statement()?.into(),
                "while" => self.parse_while_statement()?.into(),
                "do" => self.parse_do_statement()?.into(),
                "return" => self.parse_return_statement()?.into(),
                _ => panic!("Unreachable code"),
            })
        }
        Ok(list)
    }

    fn parse_let_statement(&mut self) -> Result<LetNode> {
        // The parse_statements method guarantees this token is "let"
        self.eat()?;
        let name = self.eat_identifier()?.into();
        let index = self.parse_array_index()?;
        self.eat_symbol("=")?;
        let value = self.parse_expression()?;
        self.eat_symbol(";")?;
        return Ok(LetNode { name, index, value });
    }

    fn parse_if_statement(&mut self) -> Result<IfElseNode> {
        // The parse_statements method guarantees this token is "if"
        self.eat()?;
        self.eat_symbol("(")?;
        let condition = self.parse_expression()?;
        self.eat_symbol(")")?;
        self.eat_symbol("{")?;
        let statements = self.parse_statements()?;
        self.eat_symbol("}")?;
        let else_node = if self.look_ahead_for_symbol("else")? {
            self.eat()?;
            self.eat_symbol("{")?;
            let statements = self.parse_statements()?;
            self.eat_symbol("}")?;
            Some(statements)
        } else {
            None
        };
        Ok(IfElseNode {
            condition,
            statements,
            else_node,
        })
    }

    fn parse_while_statement(&mut self) -> Result<WhileNode> {
        // The parse_statements method guarantees this token is "while"
        self.eat()?;
        self.eat_symbol("(")?;
        let condition = self.parse_expression()?;
        self.eat_symbol(")")?;
        self.eat_symbol("{")?;
        let statements = self.parse_statements()?;
        self.eat_symbol("}")?;
        Ok(WhileNode {
            condition,
            statements,
        })
    }

    fn parse_do_statement(&mut self) -> Result<DoNode> {
        // The parse_statements method guarantees this token is "do"
        self.eat()?;
        let call = self.parse_subroutine_call()?;
        self.eat_symbol(";")?;
        Ok(DoNode { call })
    }

    fn parse_return_statement(&mut self) -> Result<ReturnNode> {
        // The parse_statements method guarantees this token is "return"
        self.eat()?;
        let value = if self.look_ahead_for_symbol(";")? {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.eat_symbol(";")?;
        Ok(ReturnNode { value })
    }

    fn parse_array_index(&mut self) -> Result<Option<ExpressionNode>> {
        if self.look_ahead_for_symbol("[")? {
            self.eat()?;
            let expr = self.parse_expression()?;
            self.eat_symbol("]")?;
            Ok(Some(expr))
        } else {
            Ok(None)
        }
    }

    fn parse_expression(&mut self) -> Result<ExpressionNode> {
        let term = self.parse_term()?;
        let mut parts = NodeCollection::new();
        while let Some(operator) = self.look_ahead_for_op()? {
            self.eat()?;
            let term = self.parse_term()?;
            parts.push(ExpressionPart { operator, term })
        }
        Ok(ExpressionNode { term, parts })
    }

    fn parse_expression_list(&mut self) -> Result<NodeCollection<ExpressionNode>> {
        let mut list = NodeCollection::new();
        if self.look_ahead_for_symbol(")")? {
            return Ok(list);
        }
        while self.look_ahead_for_symbol(",")? {
            self.eat()?;
            list.push(self.parse_expression()?);
        }
        Ok(list)
    }

    fn parse_term(&mut self) -> Result<NodeBox<TermNode>> {
        match self.next_token()? {
            Token {
                kind: TokenKind::IntegerConstant,
                value,
            } => Ok(NodeBox::new(value.parse::<u16>().unwrap().into())),
            Token {
                kind: TokenKind::StringConstant,
                value,
            } => Ok(NodeBox::new(value.into())),
            Token {
                kind: k @ TokenKind::Keyword,
                value,
            } => Ok(NodeBox::new(
                KeywordConstant::try_from(value.as_str())
                    .map_err(|_| {
                        Into::<anyhow::Error>::into(ParserError::UnexpectedToken(
                            Token { kind: k, value },
                            r#""this", "null", "true" or "false""#.to_string(),
                        ))
                    })?
                    .into(),
            )),
            Token {
                kind: TokenKind::Symbol,
                value,
            } if value == "(" => {
                let expr = self.parse_expression()?;
                self.eat_symbol(")")?;
                Ok(NodeBox::new(TermNode::Parentheses(
                    NodeBox::new(expr).into(),
                )))
            }
            Token {
                kind: k @ TokenKind::Symbol,
                value,
            } => {
                let operator = UnaryOperator::try_from(value.as_str()).map_err(|_| {
                    ParserError::UnexpectedToken(
                        Token { kind: k, value },
                        "\"-\" or \"~\"".to_string(),
                    )
                })?;
                let subject = self.parse_term()?;
                Ok(NodeBox::new(
                    UnaryOperationNode { operator, subject }.into(),
                ))
            }
            Token {
                kind: k @ TokenKind::Identifier,
                value,
            } => match self.peek()? {
                Some(Token {
                    kind: TokenKind::Symbol,
                    value: symbol,
                }) => Ok(NodeBox::new(match symbol.as_str() {
                    "." => {
                        self.store_token(Token { kind: k, value });
                        let call = self.parse_subroutine_call()?;
                        call.into()
                    }
                    "[" => {
                        let index = self.parse_array_index()?.unwrap();
                        ArrayElementNode {
                            name: value.into(),
                            index,
                        }
                        .into()
                    }
                    _ => unexpected_token!(self.next_token()?, r#"".", "[" or nothing"#),
                })),
                None => Ok(NodeBox::new(IdentifierNode(value).into())),
                token => unexpected_token!(token.unwrap().clone(), "symbol or nothing"),
            },
        }
    }

    fn look_ahead_for_op(&mut self) -> Result<Option<BinaryOperator>> {
        Ok(match self.peek()?.map(|t| t.as_ref()) {
            Some(TokenRef {
                kind: TokenKind::Symbol,
                value,
            }) => value.try_into().ok(),
            _ => None,
        })
    }

    fn parse_subroutine_call(&mut self) -> Result<SubroutineCallNode> {
        let first = self.eat_identifier()?.into();
        let (this, name) = if self.look_ahead_for_symbol(".")? {
            (Some(first), self.eat_identifier()?.into())
        } else {
            (None, first)
        };
        self.eat_symbol("(")?;
        let arguments = self.parse_expression_list()?;
        self.eat_symbol(")")?;
        Ok(SubroutineCallNode {
            this,
            name,
            arguments,
        })
    }
}
