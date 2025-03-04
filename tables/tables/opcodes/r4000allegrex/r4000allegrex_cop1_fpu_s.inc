/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31----------26-25--------21 -----------------------------------------5----------0
    |   = COP1    |   = S     |                                         | function  |
    -------6------------5-----------------------------------------------------6------
    |---000---|---001---|---010---|---011---|---100---|---101---|---110---|---111---| lo
000 | ADD.S   | SUB.S   | MUL.S   | DIV.S   | SQRT.S  | ABS.S   | MOV.S   | NEG.S   |
001 | ---     | ---     | ---     | ---     |ROUND.W.S|TRUNC.W.S| CEIL.W.S|FLOOR.W.S|
010 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
011 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
100 | ---     | ---     | ---     | ---     | CVT.W.S | ---     | ---     | ---     |
101 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
110 | C.F.S   | C.UN.S  | C.EQ.S  | C.UEQ.S | C.OLT.S | C.ULT.S | C.OLE.S | C.ULE.S |
111 | C.SF.S  | C.NGLE.S| C.SEQ.S | C.NGL.S | C.LT.S  | C.NGE.S | C.LE.S  | C.NGT.S |
 hi |---------|---------|---------|---------|---------|---------|---------|---------|
*/

    // The other instructions are implemented using the main CORE table

    RAB_DEF_OPCODE_EXT_REMOVED(0x08, core, round_l_s, R4000ALLEGREX)
    RAB_DEF_OPCODE_EXT_REMOVED(0x09, core, trunc_l_s, R4000ALLEGREX)
    RAB_DEF_OPCODE_EXT_REMOVED(0x0A, core, ceil_l_s, R4000ALLEGREX)
    RAB_DEF_OPCODE_EXT_REMOVED(0x0B, core, floor_l_s, R4000ALLEGREX)
    RAB_DEF_OPCODE_EXT_REMOVED(0x21, core, cvt_d_s, R4000ALLEGREX)
    RAB_DEF_OPCODE_EXT_REMOVED(0x25, core, cvt_l_s, R4000ALLEGREX)

    RAB_DEF_OPCODE_REDIRECT(_, none, coprocessor1_fpu_s)
