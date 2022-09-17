use super::{IdentifierNode, NodeBox, NodeCollection};

#[derive(Debug)]
pub struct ExpressionNode {
    term: NodeBox<TermNode>,
    next: NodeCollection<ExpressionPart>,
}

#[derive(Debug)]
pub struct ExpressionPart {
    operator: BinaryOperator,
    term: NodeBox<TermNode>,
}

#[derive(Debug)]
pub struct SubroutineCallNode {
    this: Option<IdentifierNode>,
    name: IdentifierNode,
    parameters: NodeCollection<ExpressionNode>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnaryOperator {
    LogicalNegation,
    ArthemiticNegation,
}

#[derive(Debug)]
pub enum KeywordConstant {
    True,
    False,
    Null,
    This,
}

#[derive(Debug)]
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
    name: IdentifierNode,
    index: ExpressionNode,
}

#[derive(Debug)]
pub struct UnaryOperationNode {
    operator: UnaryOperator,
    subject: NodeBox<TermNode>,
}
