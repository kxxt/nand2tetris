use super::{IdentifierNode, NodeBox, NodeCollection};

#[derive(Debug)]
pub struct ExpressionNode {
    pub(crate) term: NodeBox<TermNode>,
    pub(crate) next: NodeCollection<ExpressionPart>,
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
    pub(crate) parameters: NodeCollection<ExpressionNode>,
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
    pub(crate) name: IdentifierNode,
    pub(crate) index: ExpressionNode,
}

#[derive(Debug)]
pub struct UnaryOperationNode {
    pub(crate) operator: UnaryOperator,
    pub(crate) subject: NodeBox<TermNode>,
}
