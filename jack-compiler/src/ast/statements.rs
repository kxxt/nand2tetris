use super::exprs::*;
use super::nodes::*;

#[derive(Debug, derive_more::From)]
pub enum StatementNode {
    IfElse(IfElseNode),
    Do(DoNode),
    Let(LetNode),
    While(WhileNode),
    SubroutineCall(SubroutineCallNode),
    Return(ReturnNode),
}

#[derive(Debug)]
pub struct IfElseNode {
    pub(crate) condition: ExpressionNode,
    pub(crate) statements: Vec<StatementNode>,
    pub(crate) else_node: Option<Vec<StatementNode>>,
}

#[derive(Debug)]
pub struct DoNode {
    pub(crate) call: SubroutineCallNode,
}

#[derive(Debug)]
pub struct LetNode {
    pub(crate) name: IdentifierNode,
    pub(crate) index: Option<ExpressionNode>,
    pub(crate) value: ExpressionNode,
}

#[derive(Debug)]
pub struct WhileNode {
    pub(crate) condition: ExpressionNode,
    pub(crate) statements: Vec<StatementNode>,
}

#[derive(Debug)]
pub struct ReturnNode {
    pub(crate) value: Option<ExpressionNode>,
}
