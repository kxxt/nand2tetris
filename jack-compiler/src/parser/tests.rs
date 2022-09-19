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
    test_array_test,
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
            variables: {
                Array a;
                Int length;
                Int i, sum;
            },
            statements: [
                {Let length = n_call!(Keyboard.readInt(n_string!("HOW MANY NUMBERS? ")))},
                {Let a = n_call!(Array.new(n_var!(length)))},
                {Let i = n_int!(0)},
                {While (n_binop!(
                    n_var_t!(i), LessThan, n_var_t!(length)
                )) {
                    {Let a[n_var!(i)] = n_call!(Keyboard.readInt(n_string!("ENTER THE NEXT NUMBER: ")))},
                    {Let i = n_binop!(n_var_t!(i), Plus, n_int_t!(1))}
                }},
                {Let i = n_int!(0)},
                {Let sum = n_int!(0)},
                {While (n_binop!(
                    n_var_t!(i), LessThan, n_var_t!(length)
                )) {
                    {Let sum = n_binop!(n_var_t!(sum), Plus, n_var_t!(a[n_var!(i)]))},
                    {Let i = n_binop!(n_var_t!(i), Plus, n_int_t!(1))}
                }},
                {Do Output.printString(n_string!("THE AVERAGE IS: "))},
                {Do Output.printInt(n_binop!(n_var_t!(sum), Divide, n_var_t!(length)))},
                {Do Output.println()},
                {Return}
            ]
        })]
    )
);


test_parser!(
    test_square_main,
    "Main",
    r###"// This file is part of www.nand2tetris.org
    // and the book "The Elements of Computing Systems"
    // by Nisan and Schocken, MIT Press.
    // File name: projects/10/Square/Main.jack
    
    // (derived from projects/09/Square/Main.jack, with testing additions)
    
    /** Initializes a new Square Dance game and starts running it. */
    class Main {
        static boolean test;    // Added for testing -- there is no static keyword
                                // in the Square files.
        function void main() {
          var SquareGame game;
          let game = SquareGame.new();
          do game.run();
          do game.dispose();
          return;
        }
    
        function void more() {  // Added to test Jack syntax that is not used in
            var int i, j;       // the Square files.
            var String s;
            var Array a;
            if (false) {
                let s = "string constant";
                let s = null;
                let a[1] = a[2];
            }
            else {              // There is no else keyword in the Square files.
                let i = i * (-j);
                let j = j / (-2);   // note: unary negate constant 2
                let i = i | j;
            }
            return;
        }
    }
    "###,
)