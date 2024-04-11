/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 ----------------------------------------0
    |  = COP3   |   fmt   |                                         |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | MFC3  | ---   | CFC3  | MFHC3 | MTC3  | ---   | CTC3  | MTHC3 |
 01 | *1    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
       *1=BC See BC3 list
*/

    // OP rt, cop3cd
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, mfc3,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_r4000allegrex_cop3cd},
        .modifiesRt=true
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x04, mtc3,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_r4000allegrex_cop3cd},
        .readsRt=true
    )

    // OP rt, cop3cd
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x02, cfc3,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_r4000allegrex_cop3cd},
        .modifiesRt=true
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x06, ctc3,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_r4000allegrex_cop3cd},
        .readsRt=true
    )

/*
TODO:
- MFHC3
- MTHC3
*/
