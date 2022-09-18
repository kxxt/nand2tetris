use super::kinds::*;
use super::statements::*;
use super::NodeCollection;

#[derive(Debug, derive_more::From, PartialEq)]
pub struct IdentifierNode(pub(crate) String);
#[derive(Debug, PartialEq)]

pub struct ClassNode {
    pub(crate) name: IdentifierNode,
    pub(crate) variables: NodeCollection<ClassVariableDeclarationNode>,
    pub(crate) subroutines: NodeCollection<SubroutineDeclarationNode>,
}

#[derive(Debug, PartialEq)]
pub struct ClassVariableDeclarationNode {
    pub(crate) kind: ClassVariableKind,
    pub(crate) variables: VariableDeclarationNode,
}

#[derive(Debug, PartialEq)]
pub struct SubroutineDeclarationNode {
    pub(crate) kind: SubroutineKind,
    pub(crate) return_type: Option<TypeNode>,
    pub(crate) name: IdentifierNode,
    pub(crate) parameters: NodeCollection<ParameterNode>,
    pub(crate) body: SubroutineBody,
}

#[derive(Debug, PartialEq)]
pub enum TypeNode {
    Int,
    Char,
    Boolean,
    Class(IdentifierNode),
}

#[derive(Debug, PartialEq)]
pub struct ParameterNode {
    pub(crate) r#type: TypeNode,
    pub(crate) name: IdentifierNode,
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclarationNode {
    pub(crate) r#type: TypeNode,
    pub(crate) names: NodeCollection<IdentifierNode>,
}

#[derive(Debug, PartialEq)]
pub struct SubroutineBody {
    pub(crate) variables: NodeCollection<VariableDeclarationNode>,
    pub(crate) statements: NodeCollection<StatementNode>,
}
