#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ClassVariableKind {
    Static,
    Field,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}
