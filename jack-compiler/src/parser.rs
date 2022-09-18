use crate::ast::*;
use crate::errors::TokenizerError;

use super::errors::ParserError;
use super::token::{Token, TokenKind, TokenRef};
use super::{ast::AST, tokenizer::TokenStream};
use anyhow::{Ok, Result};

pub struct Parser<'a> {
    token_stream: TokenStream<'a>,
    token_buffer: Option<Token>,
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
            self.token_buffer.take().unwrap()
        } else {
            self.token_stream
                .next()
                .ok_or(ParserError::UnexpectedEndOfStream)??
        })
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
}
