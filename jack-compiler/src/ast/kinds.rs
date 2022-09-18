#[derive(Debug, PartialEq)]
pub enum ClassVariableKind {
    Static,
    Field,
}

#[derive(Debug, PartialEq)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}
