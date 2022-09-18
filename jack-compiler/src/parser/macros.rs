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
    ($($type:ident $($val:ident),+;)*) => {
        vec![
            $(VariableDeclarationNode {
                r#type: n_type!($type),
                names: vec![
                    $(stringify!($val).to_string().into()),+
                ]
            }),*
        ]
    };
}

pub(super) use n_vars;

macro_rules! n_expr {
    ($e:expr) => {
        ExpressionNode {
            term: Rc::new(0u16.into()),
            parts: vec![],
        }
    };
}

pub(super) use n_expr;

macro_rules! n_call {
    ($name:ident ($e:expr)) => {
        ExpressionNode {
            term: Rc::new(SubroutineCallNode {
                this: None,
                name: stringify!($name).to_string().into(),
                arguments: $e,
            }),
            parts: vec![],
        }
    };
    ($name:ident.$method:ident ($e:expr)) => {
        ExpressionNode {
            term: Rc::new(
                SubroutineCallNode {
                    this: Some(stringify!(name).to_string().into()),
                    name: stringify!($method).to_string().into(),
                    arguments: $e,
                }
                .into(),
            ),
            parts: vec![],
        }
    };
}

pub(super) use n_call;

macro_rules! n_string {
    ($e:expr) => {
        ExpressionNode {
            term: Rc::new($e.to_string().into()),
            parts: vec![],
        }
    };
}

pub(super) use n_string;

macro_rules! n_int {
    ($e:expr) => {
        ExpressionNode {
            term: Rc::new(($e as u16).into()),
            parts: vec![],
        }
    };
}

pub(super) use n_int;

macro_rules! n_var {
    ($e:ident) => {
        ExpressionNode {
            term: Rc::new(IdentifierNode::from(stringify!($e).to_string()).into()),
            parts: vec![],
        }
    };
}

pub(super) use n_var;

macro_rules! n_constant {
    (This) => {
        ExpressionNode {
            term: KeywordConstant::This.into(),
            parts: vec![],
        }
    };
    (Null) => {
        ExpressionNode {
            term: KeywordConstant::Null.into(),
            parts: vec![],
        }
    };
    (True) => {
        ExpressionNode {
            term: KeywordConstant::True.into(),
            parts: vec![],
        }
    };
    (False) => {
        ExpressionNode {
            term: KeywordConstant::False.into(),
            parts: vec![],
        }
    };
}

pub(super) use n_constant;

macro_rules! n_cmd {
    (Let $name:ident = $e:expr) => {
        LetNode {
            name: stringify!($name).to_string().into(),
            index: None,
            value: $e,
        }
        .into()
    };
    (Let $name:ident [$k:expr] = $t:expr ) => {
        LetNode {
            name: stringify!($name).to_string().into(),
            index: $k,
            value: $t,
        }
        .into()
    };
    (While ($c:expr) { $b:expr } ) => {

    }
}

pub(super) use n_cmd;

macro_rules! n_subroutine {
    ($kind:ident $ret:ident $name:ident ($($type:ident $param:ident),*) {
        $($body:tt)*
    }) => {
        SubroutineDeclarationNode {
            kind: SubroutineKind::$kind,
            return_type: n_ret_type!($ret),
            name: stringify!($name).to_string().into(),
            parameters: vec![
                $(ParameterNode {
                    r#type: n_type!($type),
                    name: stringify!($param).to_string().into()
                }),*
            ],
            body: SubroutineBody {
                $($body)*
            }
        }
    };
}

pub(super) use n_subroutine;
