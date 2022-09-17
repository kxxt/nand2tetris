use super::exprs::*;
use super::nodes::*;

#[derive(Debug)]
pub enum StatementNode {
    IfElse(IfElseNode),
    Do(DoNode),
    Let(LetNode),
    While(WhileNode),
    SubroutineCall(SubroutineCallNode),
}

#[derive(Debug)]
pub struct IfElseNode {
    condition: ExpressionNode,
    statements: Vec<StatementNode>,
    else_node: Option<Vec<StatementNode>>,
}

#[derive(Debug)]
pub struct DoNode {
    call: SubroutineCallNode,
}

#[derive(Debug)]
pub struct LetNode {
    name: IdentifierNode,
    index: Option<ExpressionNode>,
    value: ExpressionNode,
}

#[derive(Debug)]
pub struct WhileNode {
    condition: ExpressionNode,
    statements: Vec<StatementNode>,
}

#[derive(Debug)]
pub struct ReturnNode {
    value: Option<ExpressionNode>,
}
