// init SP = 256
@256
D=A
@SP
M=D
// // init LCL to SP
// @LCL
// M=D
// return address
@0
D=A
@SP
A=M
M=D
// push LCL,...etc
@LCL
D=M
@SP
AM=M+1
M=D
@ARG
D=M
@SP
AM=M+1
M=D
@THIS
D=M
@SP
AM=M+1
M=D
@THAT
D=M
@SP
AM=M+1
M=D
@SP
M=M+1
// new ARG
@SP
D=M
@5
D=D-A
@ARG
M=D
// LCL = SP
@SP
D=M
@LCL
M=D
@Sys.init
0;JMP
// function Main.fibonacci 0
(Main.fibonacci)
@LCL
A=M
@SP
D=M
@0
D=D+A
@SP
M=D
// push Argument i
// D = *(segment + i)
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@2
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
AM=M-1 // D = *(--sp)
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.Main.TRUE.1
D;JLT
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.Main.END.1
0;JMP
(CMPR.Main.TRUE.1)
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.Main.END.1)

// if-goto Main.fibonacci$IF_TRUE
@SP
AM=M-1
D=M
@Main.fibonacci$IF_TRUE
D;JNE
// goto Main.fibonacci$IF_FALSE
@Main.fibonacci$IF_FALSE
0;JMP
(Main.fibonacci$IF_TRUE)
// push Argument i
// D = *(segment + i)
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// return (from Main.fibonacci)
// R14(return addr) = *(LCL - 5)
@LCL
D=M
@5
A=D-A
D=M
@R14
M=D
// *ARG = top()
@SP
A=M-1
D=M
@ARG
A=M
M=D
// SP = ARG + 1
@ARG
D=M+1
@SP
M=D
// THAT = *(LCL - 1),...etc
@LCL
AM=M-1
D=M
@THAT
M=D
@LCL
AM=M-1
D=M
@THIS
M=D
@LCL
AM=M-1
D=M
@ARG
M=D
@LCL
A=M-1
D=M
@LCL
M=D
// goto *R14
@R14
A=M
0;JMP
(Main.fibonacci$IF_FALSE)
// push Argument i
// D = *(segment + i)
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@2
D=A
@SP
A=M
M=D
@SP
M=M+1
// sub
@SP
AM=M-1 // D = *(--sp)
D=M
A=A-1 // *(sp-1) = *(sp-1) - D
M=M-D
// call Main.fibonacci 1
@Main.fibonacci$ret.1
D=A
@SP
A=M
M=D
// push LCL,...etc
@LCL
D=M
@SP
AM=M+1
M=D
@ARG
D=M
@SP
AM=M+1
M=D
@THIS
D=M
@SP
AM=M+1
M=D
@THAT
D=M
@SP
AM=M+1
M=D
@SP
M=M+1
// new ARG
@SP
D=M
@5
D=D-A
@1
D=D-A
@ARG
M=D
// LCL = SP
@SP
D=M
@LCL
M=D
// goto Main.fibonacci
@Main.fibonacci
0;JMP
// (return addr)
(Main.fibonacci$ret.1)

// push Argument i
// D = *(segment + i)
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
// sub
@SP
AM=M-1 // D = *(--sp)
D=M
A=A-1 // *(sp-1) = *(sp-1) - D
M=M-D
// call Main.fibonacci 1
@Main.fibonacci$ret.2
D=A
@SP
A=M
M=D
// push LCL,...etc
@LCL
D=M
@SP
AM=M+1
M=D
@ARG
D=M
@SP
AM=M+1
M=D
@THIS
D=M
@SP
AM=M+1
M=D
@THAT
D=M
@SP
AM=M+1
M=D
@SP
M=M+1
// new ARG
@SP
D=M
@5
D=D-A
@1
D=D-A
@ARG
M=D
// LCL = SP
@SP
D=M
@LCL
M=D
// goto Main.fibonacci
@Main.fibonacci
0;JMP
// (return addr)
(Main.fibonacci$ret.2)

// add
@SP
AM=M-1 // D = *(--sp)
D=M
A=A-1 // *(sp-1) += D
M=M+D
// return (from Main.fibonacci)
// R14(return addr) = *(LCL - 5)
@LCL
D=M
@5
A=D-A
D=M
@R14
M=D
// *ARG = top()
@SP
A=M-1
D=M
@ARG
A=M
M=D
// SP = ARG + 1
@ARG
D=M+1
@SP
M=D
// THAT = *(LCL - 1),...etc
@LCL
AM=M-1
D=M
@THAT
M=D
@LCL
AM=M-1
D=M
@THIS
M=D
@LCL
AM=M-1
D=M
@ARG
M=D
@LCL
A=M-1
D=M
@LCL
M=D
// goto *R14
@R14
A=M
0;JMP
// function Sys.init 0
(Sys.init)
@LCL
A=M
@SP
D=M
@0
D=D+A
@SP
M=D
// push constant i
@4
D=A
@SP
A=M
M=D
@SP
M=M+1
// call Main.fibonacci 1
@Sys.init$ret.1
D=A
@SP
A=M
M=D
// push LCL,...etc
@LCL
D=M
@SP
AM=M+1
M=D
@ARG
D=M
@SP
AM=M+1
M=D
@THIS
D=M
@SP
AM=M+1
M=D
@THAT
D=M
@SP
AM=M+1
M=D
@SP
M=M+1
// new ARG
@SP
D=M
@5
D=D-A
@1
D=D-A
@ARG
M=D
// LCL = SP
@SP
D=M
@LCL
M=D
// goto Main.fibonacci
@Main.fibonacci
0;JMP
// (return addr)
(Sys.init$ret.1)

(Sys.init$WHILE)
// goto Sys.init$WHILE
@Sys.init$WHILE
0;JMP