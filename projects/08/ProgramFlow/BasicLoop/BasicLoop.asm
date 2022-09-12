// push constant i
@0
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
(LOOP_START)
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
// add
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) += D
M=M+D
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
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) - D
M=M-D
// pop Argument 0
// R13 = segment + i
@ARG
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
// if-goto LOOP_START
@SP
M=M-1
A=M
D=M
@LOOP_START
D;JGT
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