/* SPDX-FileCopyrightText: © 2023-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPCODE_CATEGORY_EXT(RSP, NORMAL, EncodedFieldMask::opcode, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(RSP, NORMAL_LWC2, EncodedFieldMask::rd, 0xC8000000, )
    RAB_DEF_OPCODE_CATEGORY_EXT(RSP, NORMAL_SWC2, EncodedFieldMask::rd, 0xE8000000, )

    RAB_DEF_OPCODE_CATEGORY_EXT(RSP, SPECIAL, EncodedFieldMask::function, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(RSP, REGIMM, EncodedFieldMask::rt, 0x04000000, )

    RAB_DEF_OPCODE_CATEGORY_EXT(RSP, COP0, EncodedFieldMask::fmt, 0x40000000, )

    RAB_DEF_OPCODE_CATEGORY_EXT(RSP, COP1, EncodedFieldMask::fmt, 0x44000000, )

    RAB_DEF_OPCODE_CATEGORY_EXT(RSP, COP2, EncodedFieldMask::fmt, 0x48000000, )
    RAB_DEF_OPCODE_CATEGORY_EXT(RSP, COP2_VU, EncodedFieldMask::function, 0x4A000000, )
