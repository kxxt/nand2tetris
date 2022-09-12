// function Class1.set 0
(Class1.set)
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
// pop static 0
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@Class1.0
M=D
// push Argument i
// D = *(segment + i)
@ARG
D=M
@1
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// pop static 1
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@Class1.1
M=D
// push constant i
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
// return (from Class1.set)
// R14(return addr) = LCL - 5
@LCL
D=M
@5
D=D-A
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
// function Class1.get 0
(Class1.get)
@LCL
A=M
@SP
D=M
@0
D=D+A
@SP
M=D
// push static i
@Class1.0
D=M
@SP
A=M
M=D
@SP
M=M+1
// push static i
@Class1.1
D=M
@SP
A=M
M=D
@SP
M=M+1
// sub
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) - D
M=M-D
// return (from Class1.get)
// R14(return addr) = LCL - 5
@LCL
D=M
@5
D=D-A
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
// function Class2.set 0
(Class2.set)
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
// pop static 0
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@Class2.0
M=D
// push Argument i
// D = *(segment + i)
@ARG
D=M
@1
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// pop static 1
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@Class2.1
M=D
// push constant i
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
// return (from Class2.set)
// R14(return addr) = LCL - 5
@LCL
D=M
@5
D=D-A
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
// function Class2.get 0
(Class2.get)
@LCL
A=M
@SP
D=M
@0
D=D+A
@SP
M=D
// push static i
@Class2.0
D=M
@SP
A=M
M=D
@SP
M=M+1
// push static i
@Class2.1
D=M
@SP
A=M
M=D
@SP
M=M+1
// sub
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) - D
M=M-D
// return (from Class2.get)
// R14(return addr) = LCL - 5
@LCL
D=M
@5
D=D-A
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
@6
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@8
D=A
@SP
A=M
M=D
@SP
M=M+1
// call Class1.set 2
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
@2
D=D-A
@ARG
M=D
// LCL = SP
@SP
D=M
@LCL
M=D
// goto Class1.set
@Class1.set
0;JMP
// (return addr)
(Sys.init$ret.1)

// pop temp 0
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@5
M=D
// push constant i
@23
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@15
D=A
@SP
A=M
M=D
@SP
M=M+1
// call Class2.set 2
@Sys.init$ret.2
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
@2
D=D-A
@ARG
M=D
// LCL = SP
@SP
D=M
@LCL
M=D
// goto Class2.set
@Class2.set
0;JMP
// (return addr)
(Sys.init$ret.2)

// pop temp 0
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@5
M=D
// call Class1.get 0
@Sys.init$ret.3
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
@0
D=D-A
@ARG
M=D
// LCL = SP
@SP
D=M
@LCL
M=D
// goto Class1.get
@Class1.get
0;JMP
// (return addr)
(Sys.init$ret.3)

// call Class2.get 0
@Sys.init$ret.4
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
@0
D=D-A
@ARG
M=D
// LCL = SP
@SP
D=M
@LCL
M=D
// goto Class2.get
@Class2.get
0;JMP
// (return addr)
(Sys.init$ret.4)

(Sys.init$WHILE)
// goto Sys.init$WHILE
@Sys.init$WHILE
0;JMP