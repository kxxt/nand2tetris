use super::{IdentifierNode, NodeBox, NodeCollection};

pub struct ExpressionNode {
    term: NodeBox<TermNode>,
    next: NodeCollection<ExpressionPart>,
}

pub struct ExpressionPart {
    operator: BinaryOperator,
    term: NodeBox<TermNode>,
}

pub struct SubroutineCallNode {
    this: Option<IdentifierNode>,
    name: IdentifierNode,
    parameters: NodeCollection<ExpressionNode>,
}

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

pub enum UnaryOperator {
    LogicalNegation,
    ArthemiticNegation,
}

pub enum KeywordConstant {
    True,
    False,
    Null,
    This,
}

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

pub struct ArrayElementNode {
    name: IdentifierNode,
    index: ExpressionNode,
}

pub struct UnaryOperationNode {
    operator: UnaryOperator,
    subject: NodeBox<TermNode>,
}
