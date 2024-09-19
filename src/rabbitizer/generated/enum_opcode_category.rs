/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum OpcodeCategory {
    ALL_INVALID,
    CPU_INVALID,
    CPU_NORMAL,
    CPU_SPECIAL,
    CPU_REGIMM,
    CPU_COP0,
    CPU_COP0_BC0,
    CPU_COP0_TLB,
    CPU_COP1,
    CPU_COP1_BC1,
    CPU_COP1_FPUS,
    CPU_COP1_FPUD,
    CPU_COP1_FPUW,
    CPU_COP1_FPUL,
    CPU_COP2,
    RSP_INVALID,
    RSP_NORMAL,
    RSP_NORMAL_LWC2,
    RSP_NORMAL_SWC2,
    RSP_SPECIAL,
    RSP_REGIMM,
    RSP_COP0,
    RSP_COP1,
    RSP_COP2,
    RSP_COP2_VU,
    R3000GTE_INVALID,
    R3000GTE_NORMAL,
    R3000GTE_SPECIAL,
    R3000GTE_REGIMM,
    R3000GTE_COP0,
    R3000GTE_COP1,
    R3000GTE_COP2,
    R3000GTE_COP2_GTE,
    R4000ALLEGREX_INVALID,
    R4000ALLEGREX_NORMAL,
    R4000ALLEGREX_SPECIAL,
    R4000ALLEGREX_SPECIAL_RS,
    R4000ALLEGREX_SPECIAL_SA,
    R4000ALLEGREX_REGIMM,
    R4000ALLEGREX_SPECIAL2,
    R4000ALLEGREX_SPECIAL3,
    R4000ALLEGREX_SPECIAL3_BSHFL,
    R4000ALLEGREX_COP0,
    R4000ALLEGREX_COP0_BC0,
    R4000ALLEGREX_COP0_TLB,
    R4000ALLEGREX_COP1,
    R4000ALLEGREX_COP1_BC1,
    R4000ALLEGREX_COP1_FPUS,
    R4000ALLEGREX_COP1_FPUW,
    R4000ALLEGREX_COP2,
    R4000ALLEGREX_COP2_BC2,
    R4000ALLEGREX_COP2_MFHC2,
    R4000ALLEGREX_COP2_MFHC2_P,
    R4000ALLEGREX_COP2_MFHC2_P_S,
    R4000ALLEGREX_COP2_MTHC2,
    R4000ALLEGREX_VFPU0,
    R4000ALLEGREX_VFPU1,
    R4000ALLEGREX_VFPU3,
    R4000ALLEGREX_VFPU4,
    R4000ALLEGREX_VFPU4_FMT0,
    R4000ALLEGREX_VFPU4_FMT0_FMT0,
    R4000ALLEGREX_VFPU4_FMT0_FMT2,
    R4000ALLEGREX_VFPU4_FMT0_FMT3,
    R4000ALLEGREX_VFPU4_FMT0_RND,
    R4000ALLEGREX_VFPU4_FMT0_CVTFLT,
    R4000ALLEGREX_VFPU4_FMT0_CVTINT,
    R4000ALLEGREX_VFPU4_FMT0_FMT8,
    R4000ALLEGREX_VFPU4_FMT0_FMT9,
    R4000ALLEGREX_VFPU4_FMT0_CONTROL,
    R4000ALLEGREX_VFPU4_FMT0_COLOR,
    R4000ALLEGREX_VFPU4_FMT0_CST,
    R4000ALLEGREX_VFPU4_FMT2,
    R4000ALLEGREX_VFPU4_FMT2_CNDMOVE,
    R4000ALLEGREX_VFPU5,
    R4000ALLEGREX_VFPU6,
    R4000ALLEGREX_VFPU6_FMT7,
    R4000ALLEGREX_VFPU6_FMT7_FMT0,
    R4000ALLEGREX_VFPU7,
    R4000ALLEGREX_QUADLR,
    R5900_INVALID,
    R5900_NORMAL,
    R5900_SPECIAL,
    R5900_REGIMM,
    R5900_COP0,
    R5900_COP0_TLB,
    R5900_COP1,
    R5900_COP1_FPUS,
    R5900_COP2,
    R5900_COP2_NOHIGHBIT,
    R5900_COP2_BC2,
    R5900_COP2_SPECIAL1,
    R5900_COP2_SPECIAL2,
    R5900_COP2_VIWR,
    R5900_MMI,
    R5900_MMI_0,
    R5900_MMI_1,
    R5900_MMI_2,
    R5900_MMI_3,
    R5900_MMI_PMFHL,
    R5900_MMI_PMTHL,
}