/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26---------------------------------------------------0
    |  opcode   |                                                   |
    ------6----------------------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | *1    | *2    | J     | JAL   | BEQ   | BNE   | BLEZ  | BGTZ  |
001 | ADDI  | ADDIU | SLTI  | SLTIU | ANDI  | ORI   | XORI  | LUI   |
010 | *3    | *4    | *5    | *6    | BEQL  | BNEL  | BLEZL | BGTZL |
011 | *7    | *8    | ---   | *9    | *10   | ---   | ---   | *11   |
100 | LB    | LH    | LWL   | LW    | LBU   | LHU   | LWR   | ---   |
101 | SB    | SH    | SWL   | SW    | ---   | ---   | SWR   | CACHE |
110 | LL    | LWC1  | LWC2  | ---   | *12   | ---   | ---   | *13   |
111 | SC    | SWC1  | SWC2  | ---   | *14   | ---   | ---   | *15   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = SPECIAL, see SPECIAL list
     *2 = REGIMM, see REGIMM list
     *3 = COP0
     *4 = COP1
     *5 = COP2
     *6 = COP3
     *7 = VFPU0
     *8 = VFPU1
     *9 = VFPU3
     *10 = SPECIAL2
     *11 = SPECIAL3
     *12 = VFPU4
     *13 = VFPU5
     *14 = VFPU6
     *15 = VFPU7
*/

    // The other instructions are implemented using the main CPU table
