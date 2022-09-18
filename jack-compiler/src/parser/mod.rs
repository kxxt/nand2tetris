mod classes;
mod expr;
mod macros;
mod statements;
use self::macros::unexpected_token;
use super::errors::ParserError;
use super::token::{Token, TokenKind, TokenRef};
use super::{ast::AST, tokenizer::TokenResult};
use crate::ast::*;
use anyhow::{Ok, Result};

pub struct Parser<I: Iterator<Item = TokenResult>> {
    token_stream: I,
    token_buffer: Option<Token>,
    token_storage: Option<Token>,
    source_name: String,
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

impl<I: Iterator<Item = TokenResult>> Parser<I> {
    pub fn new(token_stream: I, source_name: String) -> Self {
        Self {
            token_stream,
            token_buffer: None,
            token_storage: None,
            source_name,
        }
    }

    pub fn parse(&mut self) -> Result<AST> {
        self.parse_class()
    }

    pub fn source_name(&self) -> &str {
        self.source_name.as_str()
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
}
