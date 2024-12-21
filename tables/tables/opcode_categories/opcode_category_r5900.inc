/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPCODE_CATEGORY(R5900, NORMAL, )

    RAB_DEF_OPCODE_CATEGORY(R5900, SPECIAL, )

    RAB_DEF_OPCODE_CATEGORY(R5900, REGIMM, )

    RAB_DEF_OPCODE_CATEGORY(R5900, COP0, handwritten_category: true, )
    RAB_DEF_OPCODE_CATEGORY(R5900, COP0_TLB, handwritten_category: true, )

    RAB_DEF_OPCODE_CATEGORY(R5900, COP1, )
    RAB_DEF_OPCODE_CATEGORY(R5900, COP1_FPUS, )

    RAB_DEF_OPCODE_CATEGORY(R5900, COP2, )
    RAB_DEF_OPCODE_CATEGORY(R5900, COP2_NOHIGHBIT, )
    RAB_DEF_OPCODE_CATEGORY(R5900, COP2_BC2, )
    RAB_DEF_OPCODE_CATEGORY(R5900, COP2_SPECIAL1, )
    RAB_DEF_OPCODE_CATEGORY(R5900, COP2_SPECIAL2, )
    RAB_DEF_OPCODE_CATEGORY(R5900, COP2_VIWR, )

    RAB_DEF_OPCODE_CATEGORY(R5900, MMI, )
    RAB_DEF_OPCODE_CATEGORY(R5900, MMI_0, )
    RAB_DEF_OPCODE_CATEGORY(R5900, MMI_1, )
    RAB_DEF_OPCODE_CATEGORY(R5900, MMI_2, )
    RAB_DEF_OPCODE_CATEGORY(R5900, MMI_3, )
    RAB_DEF_OPCODE_CATEGORY(R5900, MMI_PMFHL, )
    RAB_DEF_OPCODE_CATEGORY(R5900, MMI_PMTHL, )