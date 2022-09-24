use crate::ast::{ClassVariableDeclarationNode, ClassVariableKind, TypeNode};

use super::segment::Segment;

pub struct ClassVariableInfo {
    pub r#type: TypeNode,
    pub kind: ClassVariableKind,
    pub index: u16,
}

pub struct VariableInfo {
    pub r#type: TypeNode,
    pub segment: Segment,
    pub index: u16,
}
