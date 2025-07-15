/* SPDX-FileCopyrightText: © 2023-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPCODE_CATEGORY_EXT(R3000GTE, NORMAL, EncodedFieldMask::opcode, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R3000GTE, SPECIAL, EncodedFieldMask::function, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R3000GTE, REGIMM, EncodedFieldMask::rt, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R3000GTE, COP0, EncodedFieldMask::fmt, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R3000GTE, COP1, EncodedFieldMask::fmt, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R3000GTE, COP2, EncodedFieldMask::fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R3000GTE, COP2_GTE, EncodedFieldMask::function, 0x0, )
