/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26----------20-------16------------------------------0
    | = REGIMM  |          |   rt    |                              |
    ------6---------------------5------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | BLTZ  | BGEZ  | ---   | ---   | ---   | ---   | ---   | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | BLTZAL| BGEZAL| ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x02, core, bltzl, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x03, core, bgezl, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x08, core, tgei, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x09, core, tgeiu, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x0A, core, tlti, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x0B, core, tltiu, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x0C, core, teqi, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x0E, core, tnei, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x12, core, bltzall, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x13, core, bgezall, RSP)

    RAB_DEF_OPCODE_REDIRECT(_, none, regimm)
