use crate::ast::TypeNode;

use super::segment::Segment;

pub struct VariableInfo {
    pub r#type: TypeNode,
    pub segment: Segment,
    pub index: u16,
}
