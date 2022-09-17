use super::kinds::*;
use super::statements::*;
use super::NodeCollection;

pub struct IdentifierNode(String);

pub struct ClassNode {
    class_name: IdentifierNode,
    class_variable_declarations: NodeCollection<ClassVariableDeclarationNode>,
    subroutine_declarations: NodeCollection<SubroutineDeclarationNode>,
}

pub struct ClassVariableDeclarationNode {
    kind: ClassVariableKind,
    variables: NodeCollection<VariableDeclarationNode>,
}

pub struct SubroutineDeclarationNode {
    kind: SubroutineKind,
    return_type: Option<TypeNode>,
    name: IdentifierNode,
    parameters: NodeCollection<ParameterNode>,
}

pub enum TypeNode {
    Int,
    Char,
    Boolean,
    Class(IdentifierNode),
}

pub struct ParameterNode {
    r#type: TypeNode,
    name: IdentifierNode,
}

pub struct VariableDeclarationNode {
    r#type: TypeNode,
    names: NodeCollection<IdentifierNode>,
}

pub struct SubroutineBody {
    variables: NodeCollection<VariableDeclarationNode>,
    statements: NodeCollection<StatementNode>,
}
