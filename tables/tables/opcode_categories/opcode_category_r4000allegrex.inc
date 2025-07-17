/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, NORMAL, EncodedFieldMask::opcode, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, SPECIAL, EncodedFieldMask::function, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, SPECIAL_RS, EncodedFieldMask::rs, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, SPECIAL_SA, EncodedFieldMask::sa, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, REGIMM, EncodedFieldMask::rt, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, SPECIAL2, EncodedFieldMask::function, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, SPECIAL3, EncodedFieldMask::function, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, SPECIAL3_BSHFL, EncodedFieldMask::sa, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP0, EncodedFieldMask::fmt, 0x0, handwritten_category: true, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP0_BC0, EncodedFieldMask::bc_fmt, 0x0, handwritten_category: true, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP0_TLB, EncodedFieldMask::function, 0x0, handwritten_category: true, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP1, EncodedFieldMask::fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP1_BC1, EncodedFieldMask::bc_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP1_FPUS, EncodedFieldMask::function, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP1_FPUW, EncodedFieldMask::function, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP2, EncodedFieldMask::fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP2_BC2, EncodedFieldMask::r4000allegrex_bc2_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP2_MFHC2, EncodedFieldMask::r4000allegrex_mxhc2, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP2_MFHC2_P, EncodedFieldMask::r4000allegrex_mfhc2_p_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP2_MFHC2_P_S, EncodedFieldMask::r4000allegrex_mfhc2_p_s_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, COP2_MTHC2, EncodedFieldMask::r4000allegrex_mxhc2, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU0, EncodedFieldMask::r4000allegrex_vfpu0_fmt, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU1, EncodedFieldMask::r4000allegrex_vfpu0_fmt, 0x0, )

    // VFPU2 doesn't exist?

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU3, EncodedFieldMask::r4000allegrex_vfpu0_fmt, 0x6C000000, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4, EncodedFieldMask::r4000allegrex_vfpu4_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT0, EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT0_FMT0, EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT0_FMT2, EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT0_FMT3, EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT0_RND, EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT0_CVTFLT, EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT0_CVTINT, EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT0_FMT8, EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT0_FMT9, EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT0_CONTROL, EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT0_COLOR, EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT0_CST, EncodedFieldMask::r4000allegrex_tp, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT2, EncodedFieldMask::r4000allegrex_vfpu4_fmt2_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU4_FMT2_CNDMOVE, EncodedFieldMask::r4000allegrex_vfpu4_fmt2_cndmove_fmt, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU5, EncodedFieldMask::r4000allegrex_vfpu5_fmt, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU6, EncodedFieldMask::r4000allegrex_vfpu6_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU6_FMT7, EncodedFieldMask::r4000allegrex_vfpu6_fmt7_fmt, 0x0, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU6_FMT7_FMT0, EncodedFieldMask::r4000allegrex_vfpu6_fmt7_fmt0_fmt, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, VFPU7, EncodedFieldMask::r4000allegrex_vfpu7_fmt, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R4000ALLEGREX, QUADLR, EncodedFieldMask::r4000allegrex_wb, 0x0, )
