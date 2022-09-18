macro_rules! unexpected_token {
    ($token:expr, $($t:tt)*) => {
        return Err(ParserError::UnexpectedToken($token, format!($($t)*)).into())
    };
}

pub(super) use unexpected_token;

macro_rules! n_class {
    ($name:expr) => {
        n_class!($name, vec![], vec![])
    };
    ($name:expr, $subroutines:expr) => {
        n_class!($name, $subroutines, vec![])
    };
    ($name:expr, $subroutines:expr, $variables: expr) => {
        ClassNode {
            name: $name.to_string().into(),
            variables: $variables,
            subroutines: $subroutines,
        }
    };
}

pub(super) use n_class;

macro_rules! n_class_vars {
    ($($kind:ident $type:ident $($val:ident),+);* $(;)?) => {
        vec![
           $(ClassVariableDeclarationNode {
                kind: ClassVariableKind::$kind,
                variables: VariableDeclarationNode {
                r#type: n_type!($type),
                names: vec![
                    $(stringify!($val).to_string().into()),+
                ]
            }
           }),*
        ]
    };
}
pub(super) use n_class_vars;

macro_rules! n_type {
    (Int) => {
        TypeNode::Int
    };
    (Char) => {
        TypeNode::Char
    };
    (Boolean) => {
        TypeNode::Boolean
    };
    ($t:ident) => {
        TypeNode::Class(stringify!($t).to_string().into())
    };
}

pub(super) use n_type;

macro_rules! n_ret_type {
    (Int) => {
        Some(TypeNode::Int)
    };
    (Char) => {
        Some(TypeNode::Char)
    };
    (Boolean) => {
        Some(TypeNode::Boolean)
    };
    (Void) => {
        None
    };
    ($t:ident) => {
        Some(TypeNode::Class(stringify!($t).to_string().into()))
    };
}

pub(super) use n_ret_type;

macro_rules! n_vars {
    ($($type:ident $($val:ident),+);+ $(;)?) => {
        vec![
            $(VariableDeclarationNode {
                r#type: n_type!($type),
                names: vec![
                    $(stringify!($val).to_string().into()),+
                ]
            }),+
        ]
    };
}

pub(super) use n_vars;

macro_rules! n_subroutine {
    ($kind:ident $) => {};
}
