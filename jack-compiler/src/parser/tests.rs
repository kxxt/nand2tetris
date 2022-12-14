#![cfg(test)]

use super::*;
use crate::{
    parser::macros::builder::*,
    tokenizer::{Source, Tokenizer},
};
use pretty_assertions::assert_eq;

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
                                    Box::new(n_e!(-n_t!(j)))
                                )
                            )},
                            {let j = n_binop!(
                                n_t!(j), Divide, TermNode::Parentheses(
                                    Box::new(n_e!(-n_int_t!(2)))
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

test_parser!(
    test_square_square_game,
    "SquareGame",
    r##"// This file is part of www.nand2tetris.org
    // and the book "The Elements of Computing Systems"
    // by Nisan and Schocken, MIT Press.
    // File name: projects/10/Square/SquareGame.jack
    
    // (same as projects/09/Square/SquareGame.jack)
    
    /**
     * Implements the Square Dance game.
     * This simple game allows the user to move a black square around
     * the screen, and change the square's size during the movement.
     * When the game starts, a square of 30 by 30 pixels is shown at the
     * top-left corner of the screen. The user controls the square as follows.
     * The 4 arrow keys are used to move the square up, down, left, and right.
     * The 'z' and 'x' keys are used, respectively, to decrement and increment
     * the square's size. The 'q' key is used to quit the game.
     */
    
    class SquareGame {
       field Square square; // the square of this game
       field int direction; // the square's current direction: 
                            // 0=none, 1=up, 2=down, 3=left, 4=right
    
       /** Constructs a new Square Game. */
       constructor SquareGame new() {
          // Creates a 30 by 30 pixels square and positions it at the top-left
          // of the screen.
          let square = Square.new(0, 0, 30);
          let direction = 0;  // initial state is no movement
          return this;
       }
    
       /** Disposes this game. */
       method void dispose() {
          do square.dispose();
          do Memory.deAlloc(this);
          return;
       }
    
       /** Moves the square in the current direction. */
       method void moveSquare() {
          if (direction = 1) { do square.moveUp(); }
          if (direction = 2) { do square.moveDown(); }
          if (direction = 3) { do square.moveLeft(); }
          if (direction = 4) { do square.moveRight(); }
          do Sys.wait(5);  // delays the next movement
          return;
       }
    
       /** Runs the game: handles the user's inputs and moves the square accordingly */
       method void run() {
          var char key;  // the key currently pressed by the user
          var boolean exit;
          let exit = false;
          
          while (~exit) {
             // waits for a key to be pressed
             while (key = 0) {
                let key = Keyboard.keyPressed();
                do moveSquare();
             }
             if (key = 81)  { let exit = true; }     // q key
             if (key = 90)  { do square.decSize(); } // z key
             if (key = 88)  { do square.incSize(); } // x key
             if (key = 131) { let direction = 1; }   // up arrow
             if (key = 133) { let direction = 2; }   // down arrow
             if (key = 130) { let direction = 3; }   // left arrow
             if (key = 132) { let direction = 4; }   // right arrow
    
             // waits for the key to be released
             while (~(key = 0)) {
                let key = Keyboard.keyPressed();
                do moveSquare();
             }
         } // while
         return;
       }
    }"##,
    n_class!(
        "SquareGame",
        vec![
            n_subroutine! {
                Constructor SquareGame new() {
                    {let square = n_e!(Square.new(n_int!(0), n_int!(0), n_int!(30)))},
                    {let direction = n_int!(0)},
                    {return n_e!(this)}
                }
            },
            n_subroutine! {
                Method void dispose() {
                    {do square.dispose()},
                    {do Memory.deAlloc(n_e!(this))},
                    {return}
                }
            },
            n_subroutine! {
                Method void moveSquare() {
                    {if (n_binop!(n_t!(direction), Equal, n_int_t!(1))) {
                        {do square.moveUp()}
                    }},
                    {if (n_binop!(n_t!(direction), Equal, n_int_t!(2))) {
                        {do square.moveDown()}
                    }},
                    {if (n_binop!(n_t!(direction), Equal, n_int_t!(3))) {
                        {do square.moveLeft()}
                    }},
                    {if (n_binop!(n_t!(direction), Equal, n_int_t!(4))) {
                        {do square.moveRight()}
                    }},
                    {do Sys.wait(n_int!(5))},
                    {return}
                }
            },
            n_subroutine! {
                Method void run() {
                    variables: {
                        char key;
                        boolean exit;
                    },
                    statements: [
                        {let exit = n_e!(false)},
                        {while (n_e!(~n_t!(exit))) {
                            {while (n_binop!(n_t!(key), Equal, n_int_t!(0))) {
                                {let key = n_e!(Keyboard.keyPressed())},
                                {do moveSquare()}
                            }},
                            {if (n_binop!(n_t!(key), Equal, n_int_t!(81))) {
                                {let exit = n_e!(true)}
                            }},
                            {if (n_binop!(n_t!(key), Equal, n_int_t!(90))) {
                                {do square.decSize()}
                            }},
                            {if (n_binop!(n_t!(key), Equal, n_int_t!(88))) {
                                {do square.incSize()}
                            }},
                            {if (n_binop!(n_t!(key), Equal, n_int_t!(131))) {
                                {let direction = n_int!(1)}
                            }},
                            {if (n_binop!(n_t!(key), Equal, n_int_t!(133))) {
                                {let direction = n_int!(2)}
                            }},
                            {if (n_binop!(n_t!(key), Equal, n_int_t!(130))) {
                                {let direction = n_int!(3)}
                            }},
                            {if (n_binop!(n_t!(key), Equal, n_int_t!(132))) {
                                {let direction = n_int!(4)}
                            }},
                            {while (n_e!(~TermNode::Parentheses(
                                Box::new(n_binop!(n_t!(key), Equal, n_int_t!(0)))
                            ))) {
                                {let key = n_e!(Keyboard.keyPressed())},
                                {do moveSquare()}
                            }}
                        }},
                        {return}
                    ]
                }
            }
        ],
        n_class_vars! {
            Field Square square;
            Field int direction;
        }
    )
);

test_parser!(
    test_square_square,
    "Square",
    r##"// This file is part of www.nand2tetris.org
    // and the book "The Elements of Computing Systems"
    // by Nisan and Schocken, MIT Press.
    // File name: projects/10/Square/Square.jack
    
    // (same as projects/09/Square/Square.jack)
    
    /** Implements a graphical square. */
    class Square {
    
       field int x, y; // screen location of the square's top-left corner
       field int size; // length of this square, in pixels
    
       /** Constructs a new square with a given location and size. */
       constructor Square new(int Ax, int Ay, int Asize) {
          let x = Ax;
          let y = Ay;
          let size = Asize;
          do draw();
          return this;
       }
    
       /** Disposes this square. */
       method void dispose() {
          do Memory.deAlloc(this);
          return;
       }
    
       /** Draws the square on the screen. */
       method void draw() {
          do Screen.setColor(true);
          do Screen.drawRectangle(x, y, x + size, y + size);
          return;
       }
    
       /** Erases the square from the screen. */
       method void erase() {
          do Screen.setColor(false);
          do Screen.drawRectangle(x, y, x + size, y + size);
          return;
       }
    
        /** Increments the square size by 2 pixels. */
       method void incSize() {
          if (((y + size) < 254) & ((x + size) < 510)) {
             do erase();
             let size = size + 2;
             do draw();
          }
          return;
       }
    
       /** Decrements the square size by 2 pixels. */
       method void decSize() {
          if (size > 2) {
             do erase();
             let size = size - 2;
             do draw();
          }
          return;
       }
    
       /** Moves the square up by 2 pixels. */
       method void moveUp() {
          if (y > 1) {
             do Screen.setColor(false);
             do Screen.drawRectangle(x, (y + size) - 1, x + size, y + size);
             let y = y - 2;
             do Screen.setColor(true);
             do Screen.drawRectangle(x, y, x + size, y + 1);
          }
          return;
       }
    
       /** Moves the square down by 2 pixels. */
       method void moveDown() {
          if ((y + size) < 254) {
             do Screen.setColor(false);
             do Screen.drawRectangle(x, y, x + size, y + 1);
             let y = y + 2;
             do Screen.setColor(true);
             do Screen.drawRectangle(x, (y + size) - 1, x + size, y + size);
          }
          return;
       }
    
       /** Moves the square left by 2 pixels. */
       method void moveLeft() {
          if (x > 1) {
             do Screen.setColor(false);
             do Screen.drawRectangle((x + size) - 1, y, x + size, y + size);
             let x = x - 2;
             do Screen.setColor(true);
             do Screen.drawRectangle(x, y, x + 1, y + size);
          }
          return;
       }
    
       /** Moves the square right by 2 pixels. */
       method void moveRight() {
          if ((x + size) < 510) {
             do Screen.setColor(false);
             do Screen.drawRectangle(x, y, x + 1, y + size);
             let x = x + 2;
             do Screen.setColor(true);
             do Screen.drawRectangle((x + size) - 1, y, x + size, y + size);
          }
          return;
       }
    }
    "##,
    n_class!(
        "Square",
        vec![
            n_subroutine! {
                Constructor Square new(int Ax, int Ay, int Asize) {
                    {let x = n_e!(Ax)},
                    {let y = n_e!(Ay)},
                    {let size = n_e!(Asize)},
                    {do draw()},
                    {return n_e!(this)}
                }
            },
            n_subroutine! {
                Method void dispose() {
                    {do Memory.deAlloc(n_e!(this))},
                    {return}
                }
            },
            n_subroutine! {
                Method void draw() {
                    {do Screen.setColor(n_e!(true))},
                    {do Screen.drawRectangle(n_e!(x),n_e!(y), n_binop!(n_t!(x), Plus, n_t!(size)), n_binop!(n_t!(y), Plus, n_t!(size)))},
                    {return}
                }
            },
            n_subroutine! {
                Method void erase() {
                    {do Screen.setColor(n_e!(false))},
                    {do Screen.drawRectangle(n_e!(x),n_e!(y), n_binop!(n_t!(x), Plus, n_t!(size)), n_binop!(n_t!(y), Plus, n_t!(size)))},
                    {return}
                }
            },
            n_subroutine! {
                Method void incSize() {
                    {if (
                        n_binop!(
                            TermNode::Parentheses(Box::new(n_binop!(
                                TermNode::Parentheses(
                                    Box::new(
                                        n_binop!(n_t!(y), Plus, n_t!(size))
                                    )
                                ),LessThan,n_int_t!(254)
                            ))),
                            And,
                            TermNode::Parentheses(Box::new(n_binop!(
                                TermNode::Parentheses(
                                    Box::new(
                                        n_binop!(n_t!(x), Plus, n_t!(size))
                                    )
                                ),LessThan,n_int_t!(510)
                            )))
                        )
                    ) {
                        {do erase()},
                        {let size = n_binop!(n_t!(size), Plus, n_int_t!(2))},
                        {do draw()}
                    }},
                    {return}
                }
            },
            n_subroutine! {
                Method void decSize() {
                    {if (
                        n_binop!(
                            n_t!(size), GreaterThan, n_int_t!(2)
                        )
                    ) {
                        {do erase()},
                        {let size = n_binop!(n_t!(size), Minus, n_int_t!(2))},
                        {do draw()}
                    }},
                    {return}
                }
            },
            n_subroutine! {
                Method void moveUp() {
                    {if (n_binop!(
                        n_t!(y), GreaterThan, n_int_t!(1)
                    )) {
                        {do Screen.setColor(n_e!(false))},
                        {do Screen.drawRectangle(
                            n_e!(x),
                            n_binop!(
                                TermNode::Parentheses(
                                    Box::new(n_binop!(
                                        n_t!(y),Plus,n_t!(size)
                                    ))
                                ),
                                Minus,
                                n_int_t!(1)
                            ),
                            n_binop!(n_t!(x), Plus, n_t!(size)),
                            n_binop!(n_t!(y), Plus, n_t!(size))
                        )},
                        {let y = n_binop!(n_t!(y), Minus, n_int_t!(2))},
                        {do Screen.setColor(n_e!(true))},
                        {do Screen.drawRectangle(n_e!(x), n_e!(y),
                            n_binop!(n_t!(x),Plus,n_t!(size)),
                            n_binop!(n_t!(y),Plus,n_int_t!(1)),
                        )}
                    }},
                    {return}
                }
            },
            n_subroutine! {
                Method void moveDown() {
                    {if (n_binop!(
                        TermNode::Parentheses(
                            Box::new(n_binop!(
                                n_t!(y), Plus, n_t!(size)
                            ))
                        ), LessThan, n_int_t!(254)
                    )) {
                        {do Screen.setColor(n_e!(false))},
                        {do Screen.drawRectangle(n_e!(x), n_e!(y),
                            n_binop!(n_t!(x),Plus,n_t!(size)),
                            n_binop!(n_t!(y),Plus,n_int_t!(1)),
                        )},
                        {let y = n_binop!(n_t!(y), Plus, n_int_t!(2))},
                        {do Screen.setColor(n_e!(true))},
                        {do Screen.drawRectangle(
                            n_e!(x),
                            n_binop!(
                                TermNode::Parentheses(
                                    Box::new(n_binop!(
                                        n_t!(y), Plus, n_t!(size)
                                    ))
                                ),
                                Minus,
                                n_int_t!(1)
                            ),
                            n_binop!(n_t!(x), Plus, n_t!(size)),
                            n_binop!(n_t!(y), Plus, n_t!(size))
                        )}
                    }},
                    {return}
                }
            },
            n_subroutine! {
                Method void moveLeft() {
                    {if (n_binop!(
                        n_t!(x), GreaterThan, n_int_t!(1)
                    )) {
                        {do Screen.setColor(n_e!(false))},
                        {do Screen.drawRectangle(
                            n_binop!(
                                TermNode::Parentheses(
                                    Box::new(n_binop!(
                                        n_t!(x),Plus,n_t!(size)
                                    ))
                                ),
                                Minus,
                                n_int_t!(1)
                            ),
                            n_e!(y),
                            n_binop!(n_t!(x), Plus, n_t!(size)),
                            n_binop!(n_t!(y), Plus, n_t!(size))
                        )},
                        {let x = n_binop!(n_t!(x), Minus, n_int_t!(2))},
                        {do Screen.setColor(n_e!(true))},
                        {do Screen.drawRectangle(n_e!(x), n_e!(y),
                            n_binop!(n_t!(x),Plus,n_int_t!(1)),
                            n_binop!(n_t!(y),Plus,n_t!(size)),
                        )}
                    }},
                    {return}
                }
            },
            n_subroutine! {
                Method void moveRight() {
                    {if (n_binop!(
                        TermNode::Parentheses(
                            Box::new(n_binop!(
                                n_t!(x), Plus, n_t!(size)
                            ))
                        ), LessThan, n_int_t!(510)
                    )) {
                        {do Screen.setColor(n_e!(false))},
                        {do Screen.drawRectangle(n_e!(x), n_e!(y),
                            n_binop!(n_t!(x),Plus,n_int_t!(1)),
                            n_binop!(n_t!(y),Plus,n_t!(size)),
                        )},
                        {let x = n_binop!(n_t!(x), Plus, n_int_t!(2))},
                        {do Screen.setColor(n_e!(true))},
                        {do Screen.drawRectangle(
                            n_binop!(
                                TermNode::Parentheses(
                                    Box::new(n_binop!(
                                        n_t!(x), Plus, n_t!(size)
                                    ))
                                ),
                                Minus,
                                n_int_t!(1)
                            ),
                            n_e!(y),
                            n_binop!(n_t!(x), Plus, n_t!(size)),
                            n_binop!(n_t!(y), Plus, n_t!(size))
                        )}
                    }},
                    {return}
                }
            },
        ],
        n_class_vars! {
            Field int x, y;
            Field int size;
        }
    )
);
