use super::Parser;
use crate::ast::*;
use crate::token::*;
use crate::tokenizer::TokenResult;
use anyhow::Result;

impl<I: Iterator<Item = TokenResult>> Parser<I> {
    pub(super) fn parse_statements(&mut self) -> Result<NodeCollection<StatementNode>> {
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

    pub(super) fn parse_let_statement(&mut self) -> Result<LetNode> {
        // The parse_statements method guarantees this token is "let"
        self.eat()?;
        let name = self.eat_identifier()?.into();
        let index = self.parse_array_index()?;
        self.eat_symbol("=")?;
        let value = self.parse_expression()?;
        self.eat_symbol(";")?;
        return Ok(LetNode { name, index, value });
    }

    pub(super) fn parse_if_statement(&mut self) -> Result<IfElseNode> {
        // The parse_statements method guarantees this token is "if"
        self.eat()?;
        self.eat_symbol("(")?;
        let condition = self.parse_expression()?;
        self.eat_symbol(")")?;
        self.eat_symbol("{")?;
        let statements = self.parse_statements()?;
        self.eat_symbol("}")?;
        let else_node = if self.peek()?.is_some_and(
            |t| t.kind == TokenKind::Keyword && t.value == "else"
        ) {
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

    pub(super) fn parse_while_statement(&mut self) -> Result<WhileNode> {
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

    pub(super) fn parse_do_statement(&mut self) -> Result<DoNode> {
        // The parse_statements method guarantees this token is "do"
        self.eat()?;
        let call = self.parse_subroutine_call()?;
        self.eat_symbol(";")?;
        Ok(DoNode { call })
    }

    pub(super) fn parse_return_statement(&mut self) -> Result<ReturnNode> {
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

    pub(super) fn parse_array_index(&mut self) -> Result<Option<ExpressionNode>> {
        if self.look_ahead_for_symbol("[")? {
            self.eat()?;
            let expr = self.parse_expression()?;
            self.eat_symbol("]")?;
            Ok(Some(expr))
        } else {
            Ok(None)
        }
    }
}
