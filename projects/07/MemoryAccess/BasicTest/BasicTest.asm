// push constant i
@10
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Local 0
// R13 = segment + i
@LCL
D=M
@0
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
@21
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@22
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Argument 2
// R13 = segment + i
@ARG
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
// pop Argument 1
// R13 = segment + i
@ARG
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
@36
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop This 6
// R13 = segment + i
@THIS
D=M
@6
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
@42
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@45
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop That 5
// R13 = segment + i
@THAT
D=M
@5
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
// pop That 2
// R13 = segment + i
@THAT
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
@510
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop temp 6
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@11
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
// push That i
// D = *(segment + i)
@THAT
D=M
@5
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
// push This i
// D = *(segment + i)
@THIS
D=M
@6
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// push This i
// D = *(segment + i)
@THIS
D=M
@6
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
// sub
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) - D
M=M-D
// push temp i
@11
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