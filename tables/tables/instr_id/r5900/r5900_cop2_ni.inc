/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 --------------------------------------1-0
    |  = COP2   |   fmt   |                                       |0|
    ------6----------5---------------------------------------------1-
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | ---   |QMFC2.NI|CFC2.NI| ---  | ---   |QMTC2.NI|CTC2.NI| ---  |
 01 | *1    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | *2    | *2    | *2    | *2    | *2    | *2    | *2    | *2    |
 11 | *2    | *2    | *2    | *2    | *2    | *2    | *2    | *2    |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = BC instructions, see BC2 list   *2 =see special1 table
*/

    // TODO: properties
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, 0x01, qmfc2_ni, qmfc2.ni,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_r5900_vfs},
        .modifiesRt=true
    )
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, 0x02, cfc2_ni, cfc2.ni,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_r5900_vis},
        .modifiesRt=true
    )
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, 0x05, qmtc2_ni, qmtc2.ni,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_r5900_vfs},
        .readsRt=true
    )
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, 0x06, ctc2_ni, ctc2.ni,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_r5900_vis},
        .readsRt=true
    )
