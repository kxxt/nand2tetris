use super::kinds::*;

pub enum ASTNode {
    Class(ClassNode),
}

pub struct IdentifierNode(String);

pub struct ClassNode {
    class_name: IdentifierNode,
    class_variable_declarations: Vec<ClassVariableDeclarationNode>,
    subroutine_declarations: Vec<SubroutineDeclarationNode>,
}

pub struct ClassVariableDeclarationNode {
    kind: ClassVariableKind,
    variables: Vec<VariableDeclarationNode>,
}

pub struct SubroutineDeclarationNode {
    kind: SubroutineKind,
    return_type: Option<TypeNode>,
    name: IdentifierNode,
    parameters: Vec<ParameterNode>,
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
    names: Vec<IdentifierNode>,
}

pub struct SubroutineBody {
    variables: Vec<VariableDeclarationNode>,
    statements: Vec<StatementNode>
}