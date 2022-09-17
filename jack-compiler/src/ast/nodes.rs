use super::kinds::*;
use super::statements::*;
use super::NodeCollection;

#[derive(Debug, derive_more::From)]
pub struct IdentifierNode(String);
#[derive(Debug)]

pub struct ClassNode {
    class_name: IdentifierNode,
    class_variable_declarations: NodeCollection<ClassVariableDeclarationNode>,
    subroutine_declarations: NodeCollection<SubroutineDeclarationNode>,
}

#[derive(Debug)]
pub struct ClassVariableDeclarationNode {
    kind: ClassVariableKind,
    variables: NodeCollection<VariableDeclarationNode>,
}

#[derive(Debug)]
pub struct SubroutineDeclarationNode {
    kind: SubroutineKind,
    return_type: Option<TypeNode>,
    name: IdentifierNode,
    parameters: NodeCollection<ParameterNode>,
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
