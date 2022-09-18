use std::{collections::HashMap, convert::TryFrom};

use anyhow::anyhow;
use lazy_static::lazy_static;

use super::{IdentifierNode, NodeBox, NodeCollection};

#[derive(Debug)]
pub struct ExpressionNode {
    pub(crate) term: NodeBox<TermNode>,
    pub(crate) parts: NodeCollection<ExpressionPart>,
}

#[derive(Debug)]
pub struct ExpressionPart {
    pub(crate) operator: BinaryOperator,
    pub(crate) term: NodeBox<TermNode>,
}

#[derive(Debug)]
pub struct SubroutineCallNode {
    pub(crate) this: Option<IdentifierNode>,
    pub(crate) name: IdentifierNode,
    pub(crate) arguments: NodeCollection<ExpressionNode>,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    And,
    Or,
    LessThan,
    GreaterThan,
    Equal,
}

impl<'a> TryFrom<&'a str> for BinaryOperator {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref MAP: HashMap<&'static str, BinaryOperator> = HashMap::from([
                ("+", BinaryOperator::Plus),
                ("-", BinaryOperator::Minus),
                ("*", BinaryOperator::Multiply),
                ("/", BinaryOperator::Divide),
                ("&", BinaryOperator::And),
                ("|", BinaryOperator::Or),
                ("<", BinaryOperator::LessThan),
                (">", BinaryOperator::GreaterThan),
                ("=", BinaryOperator::Equal)
            ]);
        }
        MAP.get(value)
            .copied()
            .ok_or(anyhow!("Invalid binary operator {}", value))
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    LogicalNegation,
    ArthemiticNegation,
}

#[derive(Debug, Clone, Copy)]
pub enum KeywordConstant {
    True,
    False,
    Null,
    This,
}

impl TryFrom<&str> for KeywordConstant {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref MAP: HashMap<&'static str, KeywordConstant> = HashMap::from([
                ("this", KeywordConstant::This),
                ("null", KeywordConstant::Null),
                ("true", KeywordConstant::True),
                ("false", KeywordConstant::False)
            ]);
        }
        MAP.get(value)
            .copied()
            .ok_or(anyhow!("Invalid keyword constant {}", value))
    }
}

#[derive(Debug, derive_more::From)]
pub enum TermNode {
    IntegerConstant(u16),
    StringConstant(String),
    KeywordConstant(KeywordConstant),
    Variable(IdentifierNode),
    ArrayElement(ArrayElementNode),
    SubroutineCall(SubroutineCallNode),
    UnaryOperation(UnaryOperationNode),
    Parentheses(NodeBox<ExpressionNode>),
}

#[derive(Debug)]
pub struct ArrayElementNode {
    pub(crate) name: IdentifierNode,
    pub(crate) index: ExpressionNode,
}

#[derive(Debug)]
pub struct UnaryOperationNode {
    pub(crate) operator: UnaryOperator,
    pub(crate) subject: NodeBox<TermNode>,
}
