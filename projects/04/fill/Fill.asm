// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

// repreat {
//   if *KBD == 0
//      set color white
//   else
//      set color black
//   fill
// }

// Fill screen with a value in @value
// screen: 256 * 512 == 8192 * 16
// The screen memory ends at KBD
// Procedure:
// cursor = screen
// loop
//   if cursor > screen_end_address
//     break
//   *cursor = value
//   cursor++


(LOOP)

// if *KBD == 0
@KBD
D=M
@COLOR_WHITE
D;JEQ

// color black
@value
M=-1
@FILL
0;JMP

(COLOR_WHITE)
@value
M=0

(FILL)
    // init cursor to SCREEN
    @SCREEN
    D=A
    @cursor
    M=D

(FILL_LOOP)
    @FILL_LOOP
    // if cursor > screen_end_address
    //   break
    @cursor
    D=M
    @KBD
    D=D-A
    @LOOP
    D;JGE

    // read value
    @value
    D=M

    // write to *cursor
    @cursor
    A=M
    M=D

    // cursor++ 
    @cursor
    M=M+1
    @FILL_LOOP
    0;JMP
