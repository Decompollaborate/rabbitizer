/* SPDX-FileCopyrightText: © 2023-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPCODE_CATEGORY(CORE, NORMAL, EncodedFieldMask::opcode, 0x0, )

    RAB_DEF_OPCODE_CATEGORY(CORE, SPECIAL, EncodedFieldMask::function, 0x0, )

    RAB_DEF_OPCODE_CATEGORY(CORE, REGIMM, EncodedFieldMask::rt, 0x04000000, )

    RAB_DEF_OPCODE_CATEGORY(CORE, COP0, EncodedFieldMask::fmt, 0x40000000, handwritten_category: true, )
    RAB_DEF_OPCODE_CATEGORY(CORE, COP0_BC0, EncodedFieldMask::bc_fmt, 0x01000000, handwritten_category: true, )
    RAB_DEF_OPCODE_CATEGORY(CORE, COP0_TLB, EncodedFieldMask::function, 0x02000000, handwritten_category: true, )

    RAB_DEF_OPCODE_CATEGORY(CORE, COP1, EncodedFieldMask::fmt, 0x44000000, )
    RAB_DEF_OPCODE_CATEGORY(CORE, COP1_BC1, EncodedFieldMask::bc_fmt, 0x01000000, )
    RAB_DEF_OPCODE_CATEGORY(CORE, COP1_FPUS, EncodedFieldMask::function, 0x02000000, )
    RAB_DEF_OPCODE_CATEGORY(CORE, COP1_FPUD, EncodedFieldMask::function, 0x02200000, )
    RAB_DEF_OPCODE_CATEGORY(CORE, COP1_FPUW, EncodedFieldMask::function, 0x02800000, )
    RAB_DEF_OPCODE_CATEGORY(CORE, COP1_FPUL, EncodedFieldMask::function, 0x02A00000, )

    RAB_DEF_OPCODE_CATEGORY(CORE, COP2, EncodedFieldMask::fmt, 0x48000000, handwritten_category: true, )
