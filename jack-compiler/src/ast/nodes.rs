use super::kinds::*;
use super::statements::*;
use super::NodeCollection;

#[derive(Debug)]
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
    r#type: TypeNode,
    name: IdentifierNode,
}

#[derive(Debug)]
pub struct VariableDeclarationNode {
    r#type: TypeNode,
    names: NodeCollection<IdentifierNode>,
}

#[derive(Debug)]
pub struct SubroutineBody {
    variables: NodeCollection<VariableDeclarationNode>,
    statements: NodeCollection<StatementNode>,
}
