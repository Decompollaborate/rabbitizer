/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPCODE_REDIRECT(0x00, none, special)
    RAB_DEF_OPCODE_REDIRECT(0x01, none, regimm)
    RAB_DEF_OPCODE_REDIRECT(0x10, none, coprocessor0)
    RAB_DEF_OPCODE_REDIRECT(0x11, none, coprocessor1)
    RAB_DEF_OPCODE_REDIRECT(0x12, r3000gte, coprocessor2)
    RAB_DEF_OPCODE_REDIRECT(_, none, normal)
