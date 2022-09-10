// push constant i
@7
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
// add
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) += D
M=M+D