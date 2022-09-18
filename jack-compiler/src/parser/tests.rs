#![cfg(test)]

use super::*;
use crate::tokenizer::{Source, Tokenizer};

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
    ClassNode {
        name: "Main".to_string().into(),
        subroutines: vec![],
        variables: vec![],
    }
);
