use crate::ast::{
    ClassNode, ClassVariableDeclarationNode, ClassVariableKind, IdentifierNode, TypeNode,
    VariableDeclarationNode,
};
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

    fn parse_class(&mut self) -> ClassNode {
        todo!()
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

    fn parse_class_variables(&mut self) -> Result<ClassVariableDeclarationNode> {
        let token = self.next_token()?;
        if token.kind != TokenKind::Keyword {
            unexpected_token!(token, "static or field");
        }
        let kind = match token.value.as_str() {
            "static" => Ok(ClassVariableKind::Static),
            "field" => Ok(ClassVariableKind::Field),
            _ => unexpected_token!(token, "static or field"),
        }?;
        todo!()
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
        let token = self.next_token()?;
        let r#type = match token.as_ref() {
            TokenRef {
                kind: TokenKind::Keyword,
                value: "boolean",
            } => TypeNode::Boolean,
            TokenRef {
                kind: TokenKind::Keyword,
                value: "int",
            } => TypeNode::Int,
            TokenRef {
                kind: TokenKind::Keyword,
                value: "char",
            } => TypeNode::Char,
            TokenRef {
                kind: TokenKind::Identifier,
                ..
            } => TypeNode::Class(token.value.into()),
            _ => unexpected_token!(token, "type"),
        };
        let mut names = Vec::new();
        names.push(self.eat_identifier()?.into());
        while let Some(TokenRef {
            kind: TokenKind::Symbol,
            value: ",",
        }) = self.peek()?.map(|x| x.as_ref())
        {
            self.eat()?;
            names.push(self.eat_identifier()?.into());
        }

        Ok(VariableDeclarationNode { r#type, names })
    }
}
