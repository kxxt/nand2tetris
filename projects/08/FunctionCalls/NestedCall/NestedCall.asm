// init SP = 256
@256
D=A
@SP
M=D
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
@4000
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop pointer 0
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@THIS
M=D
// push constant i
@5000
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop pointer 1
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@THAT
M=D
// call Sys.main 0
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
@0
D=D-A
@ARG
M=D
// LCL = SP
@SP
D=M
@LCL
M=D
// goto Sys.main
@Sys.main
0;JMP
// (return addr)
(Sys.init$ret.1)

// pop temp 1
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@6
M=D
(Sys.init$LOOP)
// goto Sys.init$LOOP
@Sys.init$LOOP
0;JMP
// function Sys.main 5
(Sys.main)
@LCL
A=M
M=0
A=A+1
M=0
A=A+1
M=0
A=A+1
M=0
A=A+1
M=0
A=A+1
@SP
D=M
@5
D=D+A
@SP
M=D
// push constant i
@4001
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop pointer 0
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@THIS
M=D
// push constant i
@5001
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop pointer 1
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@THAT
M=D
// push constant i
@200
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Local 1
// R13 = segment + i
@LCL
D=M
@1
D=D+A
@R13
M=D
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@R13
A=M
M=D
// push constant i
@40
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Local 2
// R13 = segment + i
@LCL
D=M
@2
D=D+A
@R13
M=D
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@R13
A=M
M=D
// push constant i
@6
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Local 3
// R13 = segment + i
@LCL
D=M
@3
D=D+A
@R13
M=D
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@R13
A=M
M=D
// push constant i
@123
D=A
@SP
A=M
M=D
@SP
M=M+1
// call Sys.add12 1
@Sys.main$ret.2
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
// goto Sys.add12
@Sys.add12
0;JMP
// (return addr)
(Sys.main$ret.2)

// pop temp 0
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@5
M=D
// push Local i
// D = *(segment + i)
@LCL
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// push Local i
// D = *(segment + i)
@LCL
D=M
@1
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// push Local i
// D = *(segment + i)
@LCL
D=M
@2
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// push Local i
// D = *(segment + i)
@LCL
D=M
@3
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// push Local i
// D = *(segment + i)
@LCL
D=M
@4
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// add
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) += D
M=M+D
// add
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) += D
M=M+D
// add
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) += D
M=M+D
// add
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) += D
M=M+D
// return (from Sys.main)
// R14(return addr) = LCL - 5
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
// function Sys.add12 0
(Sys.add12)
@LCL
A=M
@SP
D=M
@0
D=D+A
@SP
M=D
// push constant i
@4002
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop pointer 0
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@THIS
M=D
// push constant i
@5002
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop pointer 1
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@THAT
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
@12
D=A
@SP
A=M
M=D
@SP
M=M+1
// add
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) += D
M=M+D
// return (from Sys.add12)
// R14(return addr) = LCL - 5
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