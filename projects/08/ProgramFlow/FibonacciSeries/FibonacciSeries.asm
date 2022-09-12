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
// pop pointer 1
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@THAT
M=D
// push constant i
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop That 0
// R13 = segment + i
@THAT
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
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop That 1
// R13 = segment + i
@THAT
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
(MAIN_LOOP_START)
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
// if-goto COMPUTE_ELEMENT
@SP
M=M-1
A=M
D=M
@COMPUTE_ELEMENT
D;JGT
// goto END_PROGRAM
@END_PROGRAM
0;JMP
(COMPUTE_ELEMENT)
// push That i
// D = *(segment + i)
@THAT
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
// push pointer i
@THAT
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
// add
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) += D
M=M+D
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
// goto MAIN_LOOP_START
@MAIN_LOOP_START
0;JMP
(END_PROGRAM)