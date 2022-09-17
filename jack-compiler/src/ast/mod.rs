mod exprs;
mod kinds;
mod nodes;
mod statements;
use std::rc::Rc;

pub use exprs::*;
pub use kinds::*;
pub use nodes::*;
pub use statements::*;

pub type NodeCollection<T> = Vec<T>;
pub type NodeBox<T> = Rc<T>;

/// Each compilation unit is a class
pub type AST = ClassNode;
