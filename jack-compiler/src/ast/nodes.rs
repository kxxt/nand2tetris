use super::kinds::*;
use super::statements::*;
use super::NodeCollection;

#[derive(Debug, derive_more::From)]
pub struct IdentifierNode(String);
#[derive(Debug)]

pub struct ClassNode {
    pub(crate) class_name: IdentifierNode,
    pub(crate) class_variable_declarations: NodeCollection<ClassVariableDeclarationNode>,
    pub(crate) subroutine_declarations: NodeCollection<SubroutineDeclarationNode>,
}

#[derive(Debug)]
pub struct ClassVariableDeclarationNode {
    pub(crate) kind: ClassVariableKind,
    pub(crate) variables: VariableDeclarationNode,
}

#[derive(Debug)]
pub struct SubroutineDeclarationNode {
    pub(crate) kind: SubroutineKind,
    pub(crate) return_type: Option<TypeNode>,
    pub(crate) name: IdentifierNode,
    pub(crate) parameters: NodeCollection<ParameterNode>,
}

#[derive(Debug)]
pub enum TypeNode {
    Int,
    Char,
    Boolean,
    Class(IdentifierNode),
}

#[derive(Debug)]
pub struct ParameterNode {
    pub(crate) r#type: TypeNode,
    pub(crate) name: IdentifierNode,
}

#[derive(Debug)]
pub struct VariableDeclarationNode {
    pub(crate) r#type: TypeNode,
    pub(crate) names: NodeCollection<IdentifierNode>,
}

#[derive(Debug)]
pub struct SubroutineBody {
    pub(crate) variables: NodeCollection<VariableDeclarationNode>,
    pub(crate) statements: NodeCollection<StatementNode>,
}
