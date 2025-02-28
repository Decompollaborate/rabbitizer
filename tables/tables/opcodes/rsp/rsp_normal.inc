/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26---------------------------------------------------0
    |  opcode   |                                                   |
    ------6----------------------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | *1    | *2    | J     | JAL   | BEQ   | BNE   | BLEZ  | BGTZ  |
001 | ADDI  | ADDIU | SLTI  | SLTIU | ANDI  | ORI   | XORI  | LUI   |
010 | *3    | *4    |  *5   | ---   | ---   | ---   | ---   | ---   |
011 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
100 | LB    | LH    | ---   | LW    | LBU   | LHU   | ---   | ---   |
101 | SB    | SH    | ---   | SW    | ---   | ---   | ---   | ---   |
110 | ---   | ---   | *6    | PREF  | ---   | ---   | ---   | ---   |
111 | ---   | ---   | *7    | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = SPECIAL
     *2 = REGIMM
     *3 = COP0
     *4 = COP1
     *5 = COP2
     *6 = LWC2
     *7 = SWC2
*/

    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x14, core, beql, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x15, core, bnel, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x16, core, blezl, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x17, core, bgtzl, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x18, core, daddi, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x19, core, daddiu, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x1A, core, ldl, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x1B, core, ldr, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x22, core, lwl, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x26, core, lwr, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x27, core, lwu, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x2A, core, swl, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x2C, core, sdl, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x2D, core, sdr, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x2E, core, swr, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x2F, core, cache, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x30, core, ll, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x31, core, lwc1, RSP)
    // RAB_DEF_OPCODE_EXTENSION_REMOVED(0x32, core, lwc2, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x34, core, lld, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x35, core, ldc1, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x36, core, ldc2, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x37, core, ld, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x38, core, sc, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x39, core, swc1, RSP)
    // RAB_DEF_OPCODE_EXTENSION_REMOVED(0x3A, core, swc2, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x3C, core, scd, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x3D, core, sdc1, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x3E, core, sdc2, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x3F, core, sd, RSP)

    // lwc2 and swc2 are commented out because they interferes with the REDIRECT patterns.

    RAB_DEF_OPCODE_REDIRECT(0x00, rsp, special)
    RAB_DEF_OPCODE_REDIRECT(0x01, rsp, regimm)
    RAB_DEF_OPCODE_REDIRECT(0x10, rsp, coprocessor0)
    RAB_DEF_OPCODE_REDIRECT(0x11, rsp, coprocessor1)
    RAB_DEF_OPCODE_REDIRECT(0x12, rsp, coprocessor2)
    RAB_DEF_OPCODE_REDIRECT(0x32, rsp, lwc2)
    RAB_DEF_OPCODE_REDIRECT(0x3A, rsp, swc2)
    RAB_DEF_OPCODE_REDIRECT(_, none, normal)
