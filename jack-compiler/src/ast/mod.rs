mod exprs;
mod kinds;
mod nodes;
mod statements;
use std::rc::Rc;

pub use exprs::*;
pub use kinds::*;
pub use nodes::*;
pub use statements::*;

type NodeCollection<T> = Vec<T>;
type NodeBox<T> = Rc<T>;
