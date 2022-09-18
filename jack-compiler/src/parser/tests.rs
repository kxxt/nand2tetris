#![cfg(test)]

use super::*;
use crate::{
    parser::macros::*,
    tokenizer::{Source, Tokenizer},
};
use std::rc::Rc;

macro_rules! test_parser {
    ($name:ident, $source_name:expr, $source:expr, $output:expr) => {
        #[test]
        fn $name() {
            let source = Source {
                name: $source_name.to_string(),
                content: $source.to_string(),
            };
            let tokens = Tokenizer::stream(&source);
            let mut parser = Parser::new(tokens, $source_name.to_string());
            let ast = parser.parse().unwrap();
            assert_eq!(ast, $output);
        }
    };
}

test_parser!(
    parser_array_test,
    "Main",
    r###"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/10/ArrayTest/Main.jack

// (identical to projects/09/Average/Main.jack)

/** Computes the average of a sequence of integers. */
class Main {
    function void main() {
        var Array a;
        var int length;
        var int i, sum;
	
	let length = Keyboard.readInt("HOW MANY NUMBERS? ");
	let a = Array.new(length);
	let i = 0;
	
	while (i < length) {
	    let a[i] = Keyboard.readInt("ENTER THE NEXT NUMBER: ");
	    let i = i + 1;
	}
	
	let i = 0;
	let sum = 0;
	
	while (i < length) {
	    let sum = sum + a[i];
	    let i = i + 1;
	}
	
	do Output.printString("THE AVERAGE IS: ");
	do Output.printInt(sum / length);
	do Output.println();
	
	return;
    }
}
"###,
    n_class!(
        "Main",
        vec![n_subroutine!(Function Void main() {
            variables: n_vars!{
                Array a;
                Int length;
                Int i, sum;
            },
            statements: vec![
                n_cmd!(Let length = n_call!(Keyboard.readInt(n_string!("HOW MANY NUMBERS? ")))),
                n_cmd!(Let a = n_call!(Array.new(n_var!(length)))),
                n_cmd!(Let i = n_int!(0)),
                n_cmd!(While (n_binop!(
                    n_var_t!(i), LessThan, n_var_t!(length)
                )) {
                    n_cmd!(Let a[n_var!(i)] = n_call!(Keyboard.readInt(n_string!("ENTER THE NEXT NUMBER: ")))),
                    n_cmd!(Let i = n_binop!(n_var_t!(i), Plus, n_int_t!(1)))
                }),
                n_cmd!(Let i = n_int!(0)),
                n_cmd!(Let sum = n_int!(0)),
                n_cmd!(While (n_binop!(
                    n_var_t!(i), LessThan, n_var_t!(length)
                )) {
                    n_cmd!(Let sum = n_binop!(n_var_t!(sum), Plus, n_var_t!(a[n_var!(i)]))),
                    n_cmd!(Let i = n_binop!(n_var_t!(i), Plus, n_int_t!(1)))
                }),
                n_cmd!(Do Output.printString(n_string!("THE AVERAGE IS: "))),
                n_cmd!(Do Output.printInt(n_binop!(n_var_t!(sum), Divide, n_var_t!(length)))),
                n_cmd!(Do Output.println()),
                n_cmd!(Return)
            ]
        })]
    )
);
