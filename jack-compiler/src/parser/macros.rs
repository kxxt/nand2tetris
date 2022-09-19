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
    (int) => {
        TypeNode::Int
    };
    (char) => {
        TypeNode::Char
    };
    (boolean) => {
        TypeNode::Boolean
    };
    ($t:ident) => {
        TypeNode::Class(stringify!($t).to_string().into())
    };
}

pub(super) use n_type;

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
            term: Rc::new($e),
            parts: vec![],
        }
    };
}

pub(super) use n_expr;

macro_rules! n_string {
    ($e:expr) => {
        n_expr!(n_string_t!($e))
    };
}

pub(super) use n_string;

macro_rules! n_string_t {
    ($e:expr) => {
        TermNode::StringConstant($e.to_string())
    };
}

pub(super) use n_string_t;

macro_rules! n_int {
    ($e:expr) => {
        n_expr!(n_int_t!($e))
    };
}

pub(super) use n_int;

macro_rules! n_int_t {
    ($e:expr) => {
        TermNode::IntegerConstant($e as u16)
    };
}

pub(super) use n_int_t;

macro_rules! n_t {
    (this) => {
        KeywordConstant::This.into()
    };
    (null) => {
        KeywordConstant::Null.into()
    };
    (true) => {
        KeywordConstant::True.into()
    };
    (false) => {
        KeywordConstant::False.into()
    };
    (- $e:expr) => {
        UnaryOperationNode {
            operator: UnaryOperator::ArthemiticNegation,
            subject: $e
        }.into()
    };
    (~ $e:expr) => {
        UnaryOperationNode {
            operator: UnaryOperator::LogicalNegation,
            subject: $e
        }.into()
    };
    ($name:ident ($($t:tt)*)) => {
        SubroutineCallNode {
            this: None,
            name: stringify!($name).to_string().into(),
            arguments: vec![$($t)*],
        }
        .into()
    };
    ($name:ident.$method:ident ($($t:tt)*)) => {
        SubroutineCallNode {
            this: Some(stringify!($name).to_string().into()),
            name: stringify!($method).to_string().into(),
            arguments: vec![$($t)*],
        }
        .into()
    };
    ($e:ident) => {
        IdentifierNode::from(stringify!($e).to_string()).into()
    };
    ($e:ident[$k:expr]) => {
        ArrayElementNode {
            name: stringify!($e).to_string().into(),
            index: $k,
        }
        .into()
    };
}

pub(super) use n_t;

macro_rules! n_e {
    ($($t:tt)*) => {
        n_expr!(n_t!($($t)*))
    }
}

pub(super) use n_e;

macro_rules! n_binop {
    ($a:expr, $b:ident, $c:expr) => {
        ExpressionNode {
            term: Rc::new($a),
            parts: vec![ExpressionPart {
                operator: BinaryOperator::$b,
                term: Rc::new($c),
            }],
        }
    };
}

pub(super) use n_binop;

macro_rules! n_statements {
    (@cmd {let $name:ident = $e:expr}) => {
        LetNode {
            name: stringify!($name).to_string().into(),
            index: None,
            value: $e,
        }
        .into()
    };
    (@cmd {let $name:ident [$k:expr] = $t:expr }) => {
        LetNode {
            name: stringify!($name).to_string().into(),
            index: Some($k),
            value: $t,
        }
        .into()
    };
    (@cmd {while ($c:expr) { $($t:tt)* } }) => {
        WhileNode {
            condition: $c,
            statements: n_statements!($($t)*)
        }
        .into()
    };
    (@cmd {do $($t:tt)*}) => {
        DoNode {
            call: n_t!($($t)*)
        }.into()
    };
    (@cmd {return}) => {
        ReturnNode{
            value: None,
        }.into()
    };
    (@cmd {return $e:expr}) => {
        ReturnNode {
            value: Some($e),
        }.into()
    };
    (@cmd {if ($e:expr) {$($s:tt)*}}) => {
        IfElseNode {
            condition: $e,
            statements: n_statements!($($s)*),
            else_node: None
        }
    };
    (@cmd {if ($e:expr) {$($s:tt)*} else {$($t:tt)*}}) => {
        IfElseNode {
            condition: $e,
            statements: n_statements!($($s)*),
            else_node: Some(n_statements!($($t)*))
        }.into()
    };
    ($($t:tt),*) => {
        vec![
            $(n_statements!(@cmd $t)),*
        ]
    }
}

pub(super) use n_statements;

macro_rules! n_subroutine {
    (@ret_type int) => {
        Some(TypeNode::Int)
    };
    (@ret_type char) => {
        Some(TypeNode::Char)
    };
    (@ret_type boolean) => {
        Some(TypeNode::Boolean)
    };
    (@ret_type void) => {
        None
    };
    (@ret_type $t:ident) => {
        Some(TypeNode::Class(stringify!($t).to_string().into()))
    };
    ($kind:ident $ret:ident $name:ident ($($type:ident $param:ident),*) {
        variables: { $($v:tt)* },
        statements: [ $($s:tt)* ]
    }) => {
        SubroutineDeclarationNode {
            kind: SubroutineKind::$kind,
            return_type: n_subroutine!(@ret_type $ret),
            name: stringify!($name).to_string().into(),
            parameters: vec![
                $(ParameterNode {
                    r#type: n_type!($type),
                    name: stringify!($param).to_string().into()
                }),*
            ],
            body: SubroutineBody {
                variables: n_vars!{ $($v)* },
                statements: n_statements!($($s)*)

            }
        }
    };
}

pub(super) use n_subroutine;
