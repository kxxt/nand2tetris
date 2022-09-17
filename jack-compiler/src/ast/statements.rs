use super::exprs::*;
use super::nodes::*;

pub enum StatementNode {
    IfElse(IfElseNode),
    Do(DoNode),
    Let(LetNode),
    While(WhileNode),
    SubroutineCall(SubroutineCallNode),
}

pub struct IfElseNode {
    condition: ExpressionNode,
    statements: Vec<StatementNode>,
    else_node: Option<Vec<StatementNode>>,
}

pub struct DoNode {
    call: SubroutineCallNode,
}

pub struct LetNode {
    name: IdentifierNode,
    index: Option<ExpressionNode>,
    value: ExpressionNode,
}

pub struct WhileNode {
    condition: ExpressionNode,
    statements: Vec<StatementNode>,
}

pub struct ReturnNode {
    value: Option<ExpressionNode>,
}
