// push constant i
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.TRUE.1
D;JEQ
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.END.1
0;JMP
(CMPR.TRUE.1)
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.END.1)

// push constant i
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@16
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.TRUE.2
D;JEQ
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.END.2
0;JMP
(CMPR.TRUE.2)
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.END.2)

// push constant i
@16
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.TRUE.3
D;JEQ
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.END.3
0;JMP
(CMPR.TRUE.3)
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.END.3)

// push constant i
@892
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.TRUE.4
D;JLT
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.END.4
0;JMP
(CMPR.TRUE.4)
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.END.4)

// push constant i
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@892
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.TRUE.5
D;JLT
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.END.5
0;JMP
(CMPR.TRUE.5)
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.END.5)

// push constant i
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.TRUE.6
D;JLT
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.END.6
0;JMP
(CMPR.TRUE.6)
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.END.6)

// push constant i
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.TRUE.7
D;JGT
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.END.7
0;JMP
(CMPR.TRUE.7)
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.END.7)

// push constant i
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.TRUE.8
D;JGT
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.END.8
0;JMP
(CMPR.TRUE.8)
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.END.8)

// push constant i
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // D = *(sp-1) - D
D=M-D
@CMPR.TRUE.9
D;JGT
@0
D=A
@SP
A=M-1 // *(sp-1) = 0
M=D
@CMPR.END.9
0;JMP
(CMPR.TRUE.9)
@0
D=!A
@SP
A=M-1 // *(sp-1) = -1 (true)
M=D
(CMPR.END.9)

// push constant i
@57
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@31
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant i
@53
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
// push constant i
@112
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
// neg
@SP
A=M-1 // *(sp-1) = -*(sp-1)
M=-M
// and
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) & D
M=M&D
// push constant i
@82
D=A
@SP
A=M
M=D
@SP
M=M+1
// or
@SP
M=M-1 // --sp
A=M   // D = *sp
D=M
A=A-1 // *(sp-1) = *(sp-1) | D
M=M|D
// not
@SP
A=M-1 // *(sp-1) = !*(sp-1)
M=!M