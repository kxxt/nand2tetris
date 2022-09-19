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
        vec![n_subroutine! {Function void main() {
            variables: {
                Array a;
                int length;
                int i, sum;
            },
            statements: [
                {let length = n_e!(Keyboard.readInt(n_string!("HOW MANY NUMBERS? ")))},
                {let a = n_e!(Array.new(n_e!(length)))},
                {let i = n_int!(0)},
                {while (n_binop!(
                    n_t!(i), LessThan, n_t!(length)
                )) {
                    {let a[n_e!(i)] = n_e!(Keyboard.readInt(n_string!("ENTER THE NEXT NUMBER: ")))},
                    {let i = n_binop!(n_t!(i), Plus, n_int_t!(1))}
                }},
                {let i = n_int!(0)},
                {let sum = n_int!(0)},
                {while (n_binop!(
                    n_t!(i), LessThan, n_t!(length)
                )) {
                    {let sum = n_binop!(n_t!(sum), Plus, n_t!(a[n_e!(i)]))},
                    {let i = n_binop!(n_t!(i), Plus, n_int_t!(1))}
                }},
                {do Output.printString(n_string!("THE AVERAGE IS: "))},
                {do Output.printInt(n_binop!(n_t!(sum), Divide, n_t!(length)))},
                {do Output.println()},
                {return}
            ]
        }}]
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
    n_class!(
        "Main",
        vec![
            n_subroutine! {
                Function void main() {
                    variables: {
                        SquareGame game;
                    },
                    statements: [
                        {let game = n_e!(SquareGame.new())},
                        {do game.run()},
                        {do game.dispose()},
                        {return}
                    ]
                }
            },
            n_subroutine! {
                Function void more() {
                    variables: {
                        int i,j;
                        String s;
                        Array a;
                    },
                    statements: [
                        {if (n_e!(false)) {
                            {let s = n_string!("string constant")},
                            {let s = n_e!(null)},
                            {let a[n_int!(1)] = n_e!(a[n_int!(2)])}
                        } else {
                            {let i = n_binop!(
                                n_t!(i),
                                Multiply,
                                TermNode::Parentheses(
                                    Rc::new(n_e!(-Rc::new(n_t!(j))))
                                )
                            )},
                            {let j = n_binop!(
                                n_t!(j), Divide, TermNode::Parentheses(
                                    Rc::new(n_e!(-Rc::new(n_int_t!(2))))
                                )
                            )},
                            {let i = n_binop!(n_t!(i), Or, n_t!(j))}
                        }},
                        {return}
                    ]
                }
            }
        ],
        n_class_vars! {
            Static boolean test;
        }
    )
);
