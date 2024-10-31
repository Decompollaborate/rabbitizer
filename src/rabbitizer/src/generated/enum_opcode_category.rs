/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::opcodes::{OpcodeCategoryDescriptor, OPCODE_CATEGORY_COUNT};
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum OpcodeCategory {
    CORE_NORMAL,
    CORE_SPECIAL,
    CORE_REGIMM,
    CORE_COP0,
    CORE_COP0_BC0,
    CORE_COP0_TLB,
    CORE_COP1,
    CORE_COP1_BC1,
    CORE_COP1_FPUS,
    CORE_COP1_FPUD,
    CORE_COP1_FPUW,
    CORE_COP1_FPUL,
    CORE_COP2,
    RSP_NORMAL,
    RSP_NORMAL_LWC2,
    RSP_NORMAL_SWC2,
    RSP_SPECIAL,
    RSP_REGIMM,
    RSP_COP0,
    RSP_COP1,
    RSP_COP2,
    RSP_COP2_VU,
    R3000GTE_NORMAL,
    R3000GTE_SPECIAL,
    R3000GTE_REGIMM,
    R3000GTE_COP0,
    R3000GTE_COP1,
    R3000GTE_COP2,
    R3000GTE_COP2_GTE,
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
pub static OPCODE_CATEGORIES: [OpcodeCategoryDescriptor; OPCODE_CATEGORY_COUNT] = {
    let mut table = [OpcodeCategoryDescriptor::default(); OPCODE_CATEGORY_COUNT];
    table[OpcodeCategory::CORE_NORMAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "NORMAL"))
    };
    table[OpcodeCategory::CORE_SPECIAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "SPECIAL"))
    };
    table[OpcodeCategory::CORE_REGIMM as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "REGIMM"))
    };
    table[OpcodeCategory::CORE_COP0 as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP0"))
    };
    table[OpcodeCategory::CORE_COP0_BC0 as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP0_BC0"))
    };
    table[OpcodeCategory::CORE_COP0_TLB as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP0_TLB"))
    };
    table[OpcodeCategory::CORE_COP1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP1"))
    };
    table[OpcodeCategory::CORE_COP1_BC1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP1_BC1"))
    };
    table[OpcodeCategory::CORE_COP1_FPUS as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP1_FPUS"))
    };
    table[OpcodeCategory::CORE_COP1_FPUD as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP1_FPUD"))
    };
    table[OpcodeCategory::CORE_COP1_FPUW as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP1_FPUW"))
    };
    table[OpcodeCategory::CORE_COP1_FPUL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP1_FPUL"))
    };
    table[OpcodeCategory::CORE_COP2 as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP2"))
    };
    table[OpcodeCategory::RSP_NORMAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "NORMAL"))
    };
    table[OpcodeCategory::RSP_NORMAL_LWC2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "NORMAL_LWC2"))
    };
    table[OpcodeCategory::RSP_NORMAL_SWC2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "NORMAL_SWC2"))
    };
    table[OpcodeCategory::RSP_SPECIAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "SPECIAL"))
    };
    table[OpcodeCategory::RSP_REGIMM as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "REGIMM"))
    };
    table[OpcodeCategory::RSP_COP0 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "COP0"))
    };
    table[OpcodeCategory::RSP_COP1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "COP1"))
    };
    table[OpcodeCategory::RSP_COP2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "COP2"))
    };
    table[OpcodeCategory::RSP_COP2_VU as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "COP2_VU"))
    };
    table[OpcodeCategory::R3000GTE_NORMAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "NORMAL"))
    };
    table[OpcodeCategory::R3000GTE_SPECIAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "SPECIAL"))
    };
    table[OpcodeCategory::R3000GTE_REGIMM as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "REGIMM"))
    };
    table[OpcodeCategory::R3000GTE_COP0 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "COP0"))
    };
    table[OpcodeCategory::R3000GTE_COP1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "COP1"))
    };
    table[OpcodeCategory::R3000GTE_COP2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "COP2"))
    };
    table[OpcodeCategory::R3000GTE_COP2_GTE as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "COP2_GTE"))
    };
    table[OpcodeCategory::R4000ALLEGREX_NORMAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "NORMAL"))
    };
    table[OpcodeCategory::R4000ALLEGREX_SPECIAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "SPECIAL"))
    };
    table[OpcodeCategory::R4000ALLEGREX_SPECIAL_RS as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "SPECIAL_RS"))
    };
    table[OpcodeCategory::R4000ALLEGREX_SPECIAL_SA as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "SPECIAL_SA"))
    };
    table[OpcodeCategory::R4000ALLEGREX_REGIMM as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "REGIMM"))
    };
    table[OpcodeCategory::R4000ALLEGREX_SPECIAL2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "SPECIAL2"))
    };
    table[OpcodeCategory::R4000ALLEGREX_SPECIAL3 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "SPECIAL3"))
    };
    table[OpcodeCategory::R4000ALLEGREX_SPECIAL3_BSHFL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "SPECIAL3_BSHFL"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP0 as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP0"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP0_BC0 as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP0_BC0"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP0_TLB as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP0_TLB"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP1"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP1_BC1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP1_BC1"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP1_FPUS as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP1_FPUS"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP1_FPUW as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP1_FPUW"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP2"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP2_BC2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP2_BC2"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP2_MFHC2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP2_MFHC2"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP2_MFHC2_P as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP2_MFHC2_P"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP2_MFHC2_P_S as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP2_MFHC2_P_S"))
    };
    table[OpcodeCategory::R4000ALLEGREX_COP2_MTHC2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP2_MTHC2"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU0 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU0"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU1"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU3 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU3"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT0"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT2"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT3"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_RND"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTFLT as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_CVTFLT"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_CVTINT"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT8"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT9 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT9"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CONTROL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_CONTROL"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_COLOR as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_COLOR"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CST as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_CST"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT2"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2_CNDMOVE as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT2_CNDMOVE"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU5 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU5"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU6 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU6"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU6_FMT7"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU6_FMT7_FMT0"))
    };
    table[OpcodeCategory::R4000ALLEGREX_VFPU7 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU7"))
    };
    table[OpcodeCategory::R4000ALLEGREX_QUADLR as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "QUADLR"))
    };
    table[OpcodeCategory::R5900_NORMAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "NORMAL"))
    };
    table[OpcodeCategory::R5900_SPECIAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "SPECIAL"))
    };
    table[OpcodeCategory::R5900_REGIMM as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "REGIMM"))
    };
    table[OpcodeCategory::R5900_COP0 as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "COP0"))
    };
    table[OpcodeCategory::R5900_COP0_TLB as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "COP0_TLB"))
    };
    table[OpcodeCategory::R5900_COP1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "COP1"))
    };
    table[OpcodeCategory::R5900_COP1_FPUS as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "COP1_FPUS"))
    };
    table[OpcodeCategory::R5900_COP2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "COP2"))
    };
    table[OpcodeCategory::R5900_COP2_NOHIGHBIT as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "COP2_NOHIGHBIT"))
    };
    table[OpcodeCategory::R5900_COP2_BC2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "COP2_BC2"))
    };
    table[OpcodeCategory::R5900_COP2_SPECIAL1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "COP2_SPECIAL1"))
    };
    table[OpcodeCategory::R5900_COP2_SPECIAL2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "COP2_SPECIAL2"))
    };
    table[OpcodeCategory::R5900_COP2_VIWR as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "COP2_VIWR"))
    };
    table[OpcodeCategory::R5900_MMI as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "MMI"))
    };
    table[OpcodeCategory::R5900_MMI_0 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "MMI_0"))
    };
    table[OpcodeCategory::R5900_MMI_1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "MMI_1"))
    };
    table[OpcodeCategory::R5900_MMI_2 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "MMI_2"))
    };
    table[OpcodeCategory::R5900_MMI_3 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "MMI_3"))
    };
    table[OpcodeCategory::R5900_MMI_PMFHL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "MMI_PMFHL"))
    };
    table[OpcodeCategory::R5900_MMI_PMTHL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("R5900", "_", "MMI_PMTHL"))
    };
    table
};
