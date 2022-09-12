// function SimpleFunction.test 2
(SimpleFunction.test)
@LCL
A=M
M=0
A=A+1
M=0
A=A+1
@SP
D=M
@2
D=D+A
@SP
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
// add
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) += D
M=M+D
// not
@SP
A=M-1 // *(sp-1) = !*(sp-1)
M=!M
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
// add
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) += D
M=M+D
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
// sub
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) - D
M=M-D
// return (from SimpleFunction.test)
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