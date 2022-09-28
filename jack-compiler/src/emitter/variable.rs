use crate::ast::TypeNode;

use super::segment::Segment;

#[derive(Debug)]
pub struct VariableInfo {
    pub r#type: TypeNode,
    pub segment: Segment,
    pub index: u16,
}
