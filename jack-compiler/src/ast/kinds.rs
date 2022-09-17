#[derive(Debug)]
pub enum ClassVariableKind {
    Static,
    Field,
}

#[derive(Debug)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}
