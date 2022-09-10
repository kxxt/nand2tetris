// push constant i
@111
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@333
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@888
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop static 8
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@STATIC.8
M=D
// pop static 3
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@STATIC.3
M=D
// pop static 1
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
@STATIC.1
M=D
// push static i
@STATIC.3
D=M
@SP
A=M
M=D
@SP
M=M+1
// push static i
@STATIC.1
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
// push static i
@STATIC.8
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