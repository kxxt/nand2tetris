#![cfg(test)]

use super::*;

macro_rules! test_tokenizer {
    ($name:ident, $source:expr, $output:expr) => {
        #[test]
        fn $name() {
            let source = Source {
                name: "Test".to_string(),
                content: $source.to_string(),
            };
            let tokens: Vec<Token> = Tokenizer::stream(&source)
                .collect::<Result<Vec<Token>, _>>()
                .unwrap();
            assert_eq!(tokens, $output);
        }
    };
}

#[test]
fn remove_comments() {
    let source = Source {
        name: "Test".to_string(),
        content: r##"
/** implements particle
system simulation.*/
class ParticleSystem {
    field int count;
    field Array particles;
    constructor ParticleSystem new() {
        let count = 0;
        return this;
    }

    /** Update the simulation by one step. */
    method void update() {
        var int i;
        var Particle cur;
        let i = 0;
        while (i < count) {
            let cur = particles[i]; 
            do cur.update();
            let i = i + 1;
        }
        return;
    }

    /** Randomly generate all particles. */
    method void generate() {
        var int i, xsign, ysign;
        var Particle p;
        let i = 0;
        let xsign = 1;
        let ysign = 1;
        while (i < count) {
            if (Random.rand() > 16383) {
                let xsign = -xsign;
            }
            if (Random.rand() > 16383) {
                let ysign = -ysign;
            }
            let p = Particle.new(Random.randRange(508-3)+3, Random.randRange(252-3)+3, 
                        xsign * Random.randRange(3), ysign * Random.randRange(3));
            let particles[i] = p;
            let i = i + 1;
        }
        return;
    }

    /** Runs the simulation: quit by pressing q */
    method void run() {
        var char key;  // the key currently pressed by the user
        var boolean exit;
        do Random.setSeed(2333);
        do Output.printString("Welcome to particle system!");
        do Output.println();
        let count = Keyboard.readInt("How many particles do you want to render?> ");
        while ((count < 0) | (count = 0)) {
            let count = Keyboard.readInt("Please enter a positive number!> ");
        }
        do Output.printString("Generating particles, please wait...");
        do Output.println();
        let particles = Array.new(count);
        do generate();
        do Output.printString("Done generation! Press any key to start simulation.");
        do Output.println();
        do Keyboard.readChar();
        do Screen.clearScreen();
        let exit = false;
        
        while (~exit) {
            let key = Keyboard.keyPressed();
            do update();
            if (key = 81)  { let exit = true; }     // q key
        }
        return;
    }

    method void dispose() {
        do particles.dispose();
        do Memory.deAlloc(this);
        return;
    }
}"##
        .to_string(),
    };
    let stream = Tokenizer::stream(&source);
    assert_eq!(
        stream.source.to_string(),
        r##"   class ParticleSystem { field int count; field Array particles; constructor ParticleSystem new() { let count = 0; return this; }   method void update() { var int i; var Particle cur; let i = 0; while (i < count) { let cur = particles[i]; do cur.update(); let i = i + 1; } return; }   method void generate() { var int i, xsign, ysign; var Particle p; let i = 0; let xsign = 1; let ysign = 1; while (i < count) { if (Random.rand() > 16383) { let xsign = -xsign; } if (Random.rand() > 16383) { let ysign = -ysign; } let p = Particle.new(Random.randRange(508-3)+3, Random.randRange(252-3)+3, xsign * Random.randRange(3), ysign * Random.randRange(3)); let particles[i] = p; let i = i + 1; } return; }   method void run() { var char key;   var boolean exit; do Random.setSeed(2333); do Output.printString("Welcome to particle system!"); do Output.println(); let count = Keyboard.readInt("How many particles do you want to render?> "); while ((count < 0) | (count = 0)) { let count = Keyboard.readInt("Please enter a positive number!> "); } do Output.printString("Generating particles, please wait..."); do Output.println(); let particles = Array.new(count); do generate(); do Output.printString("Done generation! Press any key to start simulation."); do Output.println(); do Keyboard.readChar(); do Screen.clearScreen(); let exit = false; while (~exit) { let key = Keyboard.keyPressed(); do update(); if (key = 81) { let exit = true; }   } return; } method void dispose() { do particles.dispose(); do Memory.deAlloc(this); return; } }"##
    );
}

test_tokenizer!(
    test_file_array_test,
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
    vec![
        token!(Keyword, "class"),
        token!(Identifier, "Main"),
        token!(Symbol, "{"),
        token!(Keyword, "function"),
        token!(Keyword, "void"),
        token!(Identifier, "main"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "var"),
        token!(Identifier, "Array"),
        token!(Identifier, "a"),
        token!(Symbol, ";"),
        token!(Keyword, "var"),
        token!(Keyword, "int"),
        token!(Identifier, "length"),
        token!(Symbol, ";"),
        token!(Keyword, "var"),
        token!(Keyword, "int"),
        token!(Identifier, "i"),
        token!(Symbol, ","),
        token!(Identifier, "sum"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "length"),
        token!(Symbol, "="),
        token!(Identifier, "Keyboard"),
        token!(Symbol, "."),
        token!(Identifier, "readInt"),
        token!(Symbol, "("),
        token!(StringConstant, "HOW MANY NUMBERS? "),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "a"),
        token!(Symbol, "="),
        token!(Identifier, "Array"),
        token!(Symbol, "."),
        token!(Identifier, "new"),
        token!(Symbol, "("),
        token!(Identifier, "length"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "i"),
        token!(Symbol, "="),
        token!(IntegerConstant, "0"),
        token!(Symbol, ";"),
        token!(Keyword, "while"),
        token!(Symbol, "("),
        token!(Identifier, "i"),
        token!(Symbol, "<"),
        token!(Identifier, "length"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "a"),
        token!(Symbol, "["),
        token!(Identifier, "i"),
        token!(Symbol, "]"),
        token!(Symbol, "="),
        token!(Identifier, "Keyboard"),
        token!(Symbol, "."),
        token!(Identifier, "readInt"),
        token!(Symbol, "("),
        token!(StringConstant, "ENTER THE NEXT NUMBER: "),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "i"),
        token!(Symbol, "="),
        token!(Identifier, "i"),
        token!(Symbol, "+"),
        token!(IntegerConstant, "1"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "let"),
        token!(Identifier, "i"),
        token!(Symbol, "="),
        token!(IntegerConstant, "0"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "sum"),
        token!(Symbol, "="),
        token!(IntegerConstant, "0"),
        token!(Symbol, ";"),
        token!(Keyword, "while"),
        token!(Symbol, "("),
        token!(Identifier, "i"),
        token!(Symbol, "<"),
        token!(Identifier, "length"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "sum"),
        token!(Symbol, "="),
        token!(Identifier, "sum"),
        token!(Symbol, "+"),
        token!(Identifier, "a"),
        token!(Symbol, "["),
        token!(Identifier, "i"),
        token!(Symbol, "]"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "i"),
        token!(Symbol, "="),
        token!(Identifier, "i"),
        token!(Symbol, "+"),
        token!(IntegerConstant, "1"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "do"),
        token!(Identifier, "Output"),
        token!(Symbol, "."),
        token!(Identifier, "printString"),
        token!(Symbol, "("),
        token!(StringConstant, "THE AVERAGE IS: "),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Output"),
        token!(Symbol, "."),
        token!(Identifier, "printInt"),
        token!(Symbol, "("),
        token!(Identifier, "sum"),
        token!(Symbol, "/"),
        token!(Identifier, "length"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Output"),
        token!(Symbol, "."),
        token!(Identifier, "println"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Symbol, "}"),
    ]
);

test_tokenizer!(
    test_file_square_main,
    r##"// This file is part of www.nand2tetris.org
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
     "##,
    vec![
        token!(Keyword, "class"),
        token!(Identifier, "Main"),
        token!(Symbol, "{"),
        token!(Keyword, "static"),
        token!(Keyword, "boolean"),
        token!(Identifier, "test"),
        token!(Symbol, ";"),
        token!(Keyword, "function"),
        token!(Keyword, "void"),
        token!(Identifier, "main"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "var"),
        token!(Identifier, "SquareGame"),
        token!(Identifier, "game"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "game"),
        token!(Symbol, "="),
        token!(Identifier, "SquareGame"),
        token!(Symbol, "."),
        token!(Identifier, "new"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "game"),
        token!(Symbol, "."),
        token!(Identifier, "run"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "game"),
        token!(Symbol, "."),
        token!(Identifier, "dispose"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "function"),
        token!(Keyword, "void"),
        token!(Identifier, "more"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "var"),
        token!(Keyword, "int"),
        token!(Identifier, "i"),
        token!(Symbol, ","),
        token!(Identifier, "j"),
        token!(Symbol, ";"),
        token!(Keyword, "var"),
        token!(Identifier, "String"),
        token!(Identifier, "s"),
        token!(Symbol, ";"),
        token!(Keyword, "var"),
        token!(Identifier, "Array"),
        token!(Identifier, "a"),
        token!(Symbol, ";"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Keyword, "false"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "s"),
        token!(Symbol, "="),
        token!(StringConstant, "string constant"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "s"),
        token!(Symbol, "="),
        token!(Keyword, "null"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "a"),
        token!(Symbol, "["),
        token!(IntegerConstant, "1"),
        token!(Symbol, "]"),
        token!(Symbol, "="),
        token!(Identifier, "a"),
        token!(Symbol, "["),
        token!(IntegerConstant, "2"),
        token!(Symbol, "]"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "else"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "i"),
        token!(Symbol, "="),
        token!(Identifier, "i"),
        token!(Symbol, "*"),
        token!(Symbol, "("),
        token!(Symbol, "-"),
        token!(Identifier, "j"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "j"),
        token!(Symbol, "="),
        token!(Identifier, "j"),
        token!(Symbol, "/"),
        token!(Symbol, "("),
        token!(Symbol, "-"),
        token!(IntegerConstant, "2"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "i"),
        token!(Symbol, "="),
        token!(Identifier, "i"),
        token!(Symbol, "|"),
        token!(Identifier, "j"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Symbol, "}"),
    ]
);

test_tokenizer!(
    test_file_square_square,
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
    vec![
        token!(Keyword, "class"),
        token!(Identifier, "Square"),
        token!(Symbol, "{"),
        token!(Keyword, "field"),
        token!(Keyword, "int"),
        token!(Identifier, "x"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, ";"),
        token!(Keyword, "field"),
        token!(Keyword, "int"),
        token!(Identifier, "size"),
        token!(Symbol, ";"),
        token!(Keyword, "constructor"),
        token!(Identifier, "Square"),
        token!(Identifier, "new"),
        token!(Symbol, "("),
        token!(Keyword, "int"),
        token!(Identifier, "Ax"),
        token!(Symbol, ","),
        token!(Keyword, "int"),
        token!(Identifier, "Ay"),
        token!(Symbol, ","),
        token!(Keyword, "int"),
        token!(Identifier, "Asize"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "x"),
        token!(Symbol, "="),
        token!(Identifier, "Ax"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "y"),
        token!(Symbol, "="),
        token!(Identifier, "Ay"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "size"),
        token!(Symbol, "="),
        token!(Identifier, "Asize"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "draw"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "return"),
        token!(Keyword, "this"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "method"),
        token!(Keyword, "void"),
        token!(Identifier, "dispose"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "Memory"),
        token!(Symbol, "."),
        token!(Identifier, "deAlloc"),
        token!(Symbol, "("),
        token!(Keyword, "this"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "method"),
        token!(Keyword, "void"),
        token!(Identifier, "draw"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "setColor"),
        token!(Symbol, "("),
        token!(Keyword, "true"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "drawRectangle"),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, ","),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "method"),
        token!(Keyword, "void"),
        token!(Identifier, "erase"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "setColor"),
        token!(Symbol, "("),
        token!(Keyword, "false"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "drawRectangle"),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, ","),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "method"),
        token!(Keyword, "void"),
        token!(Identifier, "incSize"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Symbol, "("),
        token!(Symbol, "("),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, "<"),
        token!(IntegerConstant, "254"),
        token!(Symbol, ")"),
        token!(Symbol, "&"),
        token!(Symbol, "("),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, "<"),
        token!(IntegerConstant, "510"),
        token!(Symbol, ")"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "erase"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "size"),
        token!(Symbol, "="),
        token!(Identifier, "size"),
        token!(Symbol, "+"),
        token!(IntegerConstant, "2"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "draw"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "method"),
        token!(Keyword, "void"),
        token!(Identifier, "decSize"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "size"),
        token!(Symbol, ">"),
        token!(IntegerConstant, "2"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "erase"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "size"),
        token!(Symbol, "="),
        token!(Identifier, "size"),
        token!(Symbol, "-"),
        token!(IntegerConstant, "2"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "draw"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "method"),
        token!(Keyword, "void"),
        token!(Identifier, "moveUp"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "y"),
        token!(Symbol, ">"),
        token!(IntegerConstant, "1"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "setColor"),
        token!(Symbol, "("),
        token!(Keyword, "false"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "drawRectangle"),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, ","),
        token!(Symbol, "("),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, "-"),
        token!(IntegerConstant, "1"),
        token!(Symbol, ","),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "y"),
        token!(Symbol, "="),
        token!(Identifier, "y"),
        token!(Symbol, "-"),
        token!(IntegerConstant, "2"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "setColor"),
        token!(Symbol, "("),
        token!(Keyword, "true"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "drawRectangle"),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, ","),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(IntegerConstant, "1"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "method"),
        token!(Keyword, "void"),
        token!(Identifier, "moveDown"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Symbol, "("),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, "<"),
        token!(IntegerConstant, "254"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "setColor"),
        token!(Symbol, "("),
        token!(Keyword, "false"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "drawRectangle"),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, ","),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(IntegerConstant, "1"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "y"),
        token!(Symbol, "="),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(IntegerConstant, "2"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "setColor"),
        token!(Symbol, "("),
        token!(Keyword, "true"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "drawRectangle"),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, ","),
        token!(Symbol, "("),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, "-"),
        token!(IntegerConstant, "1"),
        token!(Symbol, ","),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "method"),
        token!(Keyword, "void"),
        token!(Identifier, "moveLeft"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, ">"),
        token!(IntegerConstant, "1"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "setColor"),
        token!(Symbol, "("),
        token!(Keyword, "false"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "drawRectangle"),
        token!(Symbol, "("),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, "-"),
        token!(IntegerConstant, "1"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, ","),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "x"),
        token!(Symbol, "="),
        token!(Identifier, "x"),
        token!(Symbol, "-"),
        token!(IntegerConstant, "2"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "setColor"),
        token!(Symbol, "("),
        token!(Keyword, "true"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "drawRectangle"),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, ","),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(IntegerConstant, "1"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "method"),
        token!(Keyword, "void"),
        token!(Identifier, "moveRight"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, "<"),
        token!(IntegerConstant, "510"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "setColor"),
        token!(Symbol, "("),
        token!(Keyword, "false"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "drawRectangle"),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, ","),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(IntegerConstant, "1"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "x"),
        token!(Symbol, "="),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(IntegerConstant, "2"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "setColor"),
        token!(Symbol, "("),
        token!(Keyword, "true"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Screen"),
        token!(Symbol, "."),
        token!(Identifier, "drawRectangle"),
        token!(Symbol, "("),
        token!(Symbol, "("),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, "-"),
        token!(IntegerConstant, "1"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, ","),
        token!(Identifier, "x"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ","),
        token!(Identifier, "y"),
        token!(Symbol, "+"),
        token!(Identifier, "size"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Symbol, "}"),
    ]
);

test_tokenizer!(
    test_file_square_square_game,
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
    }
    
    
    
    "##,
    vec![
        token!(Keyword, "class"),
        token!(Identifier, "SquareGame"),
        token!(Symbol, "{"),
        token!(Keyword, "field"),
        token!(Identifier, "Square"),
        token!(Identifier, "square"),
        token!(Symbol, ";"),
        token!(Keyword, "field"),
        token!(Keyword, "int"),
        token!(Identifier, "direction"),
        token!(Symbol, ";"),
        token!(Keyword, "constructor"),
        token!(Identifier, "SquareGame"),
        token!(Identifier, "new"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "square"),
        token!(Symbol, "="),
        token!(Identifier, "Square"),
        token!(Symbol, "."),
        token!(Identifier, "new"),
        token!(Symbol, "("),
        token!(IntegerConstant, "0"),
        token!(Symbol, ","),
        token!(IntegerConstant, "0"),
        token!(Symbol, ","),
        token!(IntegerConstant, "30"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "direction"),
        token!(Symbol, "="),
        token!(IntegerConstant, "0"),
        token!(Symbol, ";"),
        token!(Keyword, "return"),
        token!(Keyword, "this"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "method"),
        token!(Keyword, "void"),
        token!(Identifier, "dispose"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "square"),
        token!(Symbol, "."),
        token!(Identifier, "dispose"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "Memory"),
        token!(Symbol, "."),
        token!(Identifier, "deAlloc"),
        token!(Symbol, "("),
        token!(Keyword, "this"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "method"),
        token!(Keyword, "void"),
        token!(Identifier, "moveSquare"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "direction"),
        token!(Symbol, "="),
        token!(IntegerConstant, "1"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "square"),
        token!(Symbol, "."),
        token!(Identifier, "moveUp"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "direction"),
        token!(Symbol, "="),
        token!(IntegerConstant, "2"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "square"),
        token!(Symbol, "."),
        token!(Identifier, "moveDown"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "direction"),
        token!(Symbol, "="),
        token!(IntegerConstant, "3"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "square"),
        token!(Symbol, "."),
        token!(Identifier, "moveLeft"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "direction"),
        token!(Symbol, "="),
        token!(IntegerConstant, "4"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "square"),
        token!(Symbol, "."),
        token!(Identifier, "moveRight"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "do"),
        token!(Identifier, "Sys"),
        token!(Symbol, "."),
        token!(Identifier, "wait"),
        token!(Symbol, "("),
        token!(IntegerConstant, "5"),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "method"),
        token!(Keyword, "void"),
        token!(Identifier, "run"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "var"),
        token!(Keyword, "char"),
        token!(Identifier, "key"),
        token!(Symbol, ";"),
        token!(Keyword, "var"),
        token!(Keyword, "boolean"),
        token!(Identifier, "exit"),
        token!(Symbol, ";"),
        token!(Keyword, "let"),
        token!(Identifier, "exit"),
        token!(Symbol, "="),
        token!(Keyword, "false"),
        token!(Symbol, ";"),
        token!(Keyword, "while"),
        token!(Symbol, "("),
        token!(Symbol, "~"),
        token!(Identifier, "exit"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "while"),
        token!(Symbol, "("),
        token!(Identifier, "key"),
        token!(Symbol, "="),
        token!(IntegerConstant, "0"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "key"),
        token!(Symbol, "="),
        token!(Identifier, "Keyboard"),
        token!(Symbol, "."),
        token!(Identifier, "keyPressed"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "moveSquare"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "key"),
        token!(Symbol, "="),
        token!(IntegerConstant, "81"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "exit"),
        token!(Symbol, "="),
        token!(Keyword, "true"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "key"),
        token!(Symbol, "="),
        token!(IntegerConstant, "90"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "square"),
        token!(Symbol, "."),
        token!(Identifier, "decSize"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "key"),
        token!(Symbol, "="),
        token!(IntegerConstant, "88"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "do"),
        token!(Identifier, "square"),
        token!(Symbol, "."),
        token!(Identifier, "incSize"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "key"),
        token!(Symbol, "="),
        token!(IntegerConstant, "131"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "direction"),
        token!(Symbol, "="),
        token!(IntegerConstant, "1"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "key"),
        token!(Symbol, "="),
        token!(IntegerConstant, "133"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "direction"),
        token!(Symbol, "="),
        token!(IntegerConstant, "2"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "key"),
        token!(Symbol, "="),
        token!(IntegerConstant, "130"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "direction"),
        token!(Symbol, "="),
        token!(IntegerConstant, "3"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "if"),
        token!(Symbol, "("),
        token!(Identifier, "key"),
        token!(Symbol, "="),
        token!(IntegerConstant, "132"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "direction"),
        token!(Symbol, "="),
        token!(IntegerConstant, "4"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Keyword, "while"),
        token!(Symbol, "("),
        token!(Symbol, "~"),
        token!(Symbol, "("),
        token!(Identifier, "key"),
        token!(Symbol, "="),
        token!(IntegerConstant, "0"),
        token!(Symbol, ")"),
        token!(Symbol, ")"),
        token!(Symbol, "{"),
        token!(Keyword, "let"),
        token!(Identifier, "key"),
        token!(Symbol, "="),
        token!(Identifier, "Keyboard"),
        token!(Symbol, "."),
        token!(Identifier, "keyPressed"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Keyword, "do"),
        token!(Identifier, "moveSquare"),
        token!(Symbol, "("),
        token!(Symbol, ")"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Symbol, "}"),
        token!(Keyword, "return"),
        token!(Symbol, ";"),
        token!(Symbol, "}"),
        token!(Symbol, "}"),
    ]
);
