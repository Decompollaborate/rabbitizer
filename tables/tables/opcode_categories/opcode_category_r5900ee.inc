/* SPDX-FileCopyrightText: © 2023-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, NORMAL, EncodedFieldMask::opcode, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, SPECIAL, EncodedFieldMask::function, 0x0, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, REGIMM, EncodedFieldMask::rt, 0x04000000, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, COP0, EncodedFieldMask::fmt, 0x40000000, handwritten_category: true, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, COP0_TLB, EncodedFieldMask::function, 0x42000000, handwritten_category: true, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, COP1, EncodedFieldMask::fmt, 0x44000000, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, COP1_FPUS, EncodedFieldMask::function, 0x46000000, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, COP2, EncodedFieldMask::r5900ee_cop2_discriminant, 0x48000000, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, COP2_NI, EncodedFieldMask::r5900ee_cop2_ini_fmt, 0x48000000, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, COP2_I, EncodedFieldMask::r5900ee_cop2_ini_fmt, 0x48000001, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, COP2_BC2, EncodedFieldMask::bc_fmt, 0x49000000, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, COP2_SPECIAL1, EncodedFieldMask::function, 0x4A000000, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, COP2_SPECIAL2, EncodedFieldMask::r5900ee_fhi_flo, 0x4A00003C, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, COP2_VIWR, EncodedFieldMask::r5900ee_viwr_fhilo, 0x4A0003FC, )

    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, MMI, EncodedFieldMask::function, 0x70000000, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, MMI_0, EncodedFieldMask::r5900ee_mmi_function, 0x70000008, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, MMI_1, EncodedFieldMask::r5900ee_mmi_function, 0x70000028, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, MMI_2, EncodedFieldMask::r5900ee_mmi_function, 0x70000009, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, MMI_3, EncodedFieldMask::r5900ee_mmi_function, 0x70000029, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, MMI_PMFHL, EncodedFieldMask::r5900ee_mmi_function, 0x70000030, )
    RAB_DEF_OPCODE_CATEGORY_EXT(R5900EE, MMI_PMTHL, EncodedFieldMask::r5900ee_mmi_function, 0x70000031, )
