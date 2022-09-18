use super::{unexpected_token, Parser};
use crate::{ast::*, tokenizer::TokenResult};
use crate::errors::ParserError;
use crate::token::*;
use anyhow::{Ok, Result};

impl<I: Iterator<Item = TokenResult>> Parser<I> {
    pub(super) fn parse_class(&mut self) -> Result<ClassNode> {
        let token = self.next_token()?;
        if token.kind != TokenKind::Keyword || token.value != "class" {
            unexpected_token!(token, "keyword \"class\"");
        }
        let name = self.eat_identifier()?;
        if name != self.source_name() {
            return Err(ParserError::ClassNameMismatch(
                name.clone(),
                self.source_name().to_string(),
            )
            .into());
        }
        self.eat_symbol("{")?;
        let variables = self.parse_class_variable_declarations()?;
        let mut subroutines = NodeCollection::new();
        while self.look_ahead_for_subroutine_kind()?.is_some() {
            subroutines.push(self.parse_subroutine_declaration()?);
        }
        Ok(ClassNode {
            subroutines,
            variables,
            name: name.into(),
        })
    }

    pub(super) fn parse_class_variable_declaration(
        &mut self,
    ) -> Result<ClassVariableDeclarationNode> {
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

    pub(super) fn parse_class_variable_declarations(
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

    
    pub(super) fn parse_variable_declaration(
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

    pub(super) fn look_ahead_for_subroutine_kind(&mut self) -> Result<Option<SubroutineKind>> {
        Ok(match self.peek()?.map(|r| r.as_ref()) {
            Some(TokenRef {
                kind: TokenKind::Keyword,
                value: "constructor",
            }) => Some(SubroutineKind::Constructor),
            Some(TokenRef {
                kind: TokenKind::Keyword,
                value: "function",
            }) => Some(SubroutineKind::Function),
            Some(TokenRef {
                kind: TokenKind::Keyword,
                value: "method",
            }) => Some(SubroutineKind::Method),
            _ => None,
        })
    }

    pub(super) fn parse_subroutine_declaration(&mut self) -> Result<SubroutineDeclarationNode> {
        // parse (constructor|function|method)
        let kind = self
            .look_ahead_for_subroutine_kind()?
            .ok_or(ParserError::UnexpectedToken(
                // The next line will always be evaluated. This is intended.
                // If an error was raised, it will provide the token.
                // If it's ok, it will also be evaluated, to skip this now useless token.
                self.next_token()?,
                r#""constructor", "function" or "method""#.to_string(),
            ))?;
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

    pub(super) fn parse_parameter(&mut self) -> Result<ParameterNode> {
        let r#type = self.parse_type(false)?.unwrap();
        let name = self.eat_identifier()?.into();
        Ok(ParameterNode { r#type, name })
    }

    pub(super) fn parse_parameter_list(&mut self) -> Result<NodeCollection<ParameterNode>> {
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

    pub(super) fn parse_subroutine_body(&mut self) -> Result<SubroutineBody> {
        self.eat_symbol("{")?;

        let mut variables = NodeCollection::new();
        while let Some(TokenRef {
            kind: TokenKind::Keyword,
            value: "var",
        }) = self.peek()?.map(|x| x.as_ref())
        {
            variables.push(self.parse_variable_declaration(false)?);
        }
        let statements = self.parse_statements()?;
        self.eat_symbol("}")?;
        Ok(SubroutineBody {
            statements,
            variables,
        })
    }
}
