use super::{unexpected_token, Parser};
use crate::ast::*;
use crate::errors::ParserError;
use crate::token::*;
use anyhow::Result;

impl<'a> Parser<'a> {
    pub(super) fn parse_expression(&mut self) -> Result<ExpressionNode> {
        let term = self.parse_term()?;
        let mut parts = NodeCollection::new();
        while let Some(operator) = self.look_ahead_for_op()? {
            self.eat()?;
            let term = self.parse_term()?;
            parts.push(ExpressionPart { operator, term })
        }
        Ok(ExpressionNode { term, parts })
    }

    pub(super) fn parse_expression_list(&mut self) -> Result<NodeCollection<ExpressionNode>> {
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

    pub(super) fn parse_term(&mut self) -> Result<NodeBox<TermNode>> {
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

    pub(super) fn look_ahead_for_op(&mut self) -> Result<Option<BinaryOperator>> {
        Ok(match self.peek()?.map(|t| t.as_ref()) {
            Some(TokenRef {
                kind: TokenKind::Symbol,
                value,
            }) => value.try_into().ok(),
            _ => None,
        })
    }

    pub(super) fn parse_subroutine_call(&mut self) -> Result<SubroutineCallNode> {
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
