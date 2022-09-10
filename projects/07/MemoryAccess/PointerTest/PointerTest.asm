// push constant i
@3030
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
@3040
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
@32
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop This 2
// R13 = segment + i
@THIS
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
@46
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop That 6
// R13 = segment + i
@THAT
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
// push pointer i
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// push pointer i
@THAT
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
// push This i
// D = *(segment + i)
@THIS
D=M
@2
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
// push That i
// D = *(segment + i)
@THAT
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