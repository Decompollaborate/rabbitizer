/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::opcodes::{OpcodeCategoryDescriptor, OPCODE_CATEGORY_COUNT};
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
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
    #[cfg(feature = "RSP")]
    RSP_NORMAL,
    #[cfg(feature = "RSP")]
    RSP_NORMAL_LWC2,
    #[cfg(feature = "RSP")]
    RSP_NORMAL_SWC2,
    #[cfg(feature = "RSP")]
    RSP_SPECIAL,
    #[cfg(feature = "RSP")]
    RSP_REGIMM,
    #[cfg(feature = "RSP")]
    RSP_COP0,
    #[cfg(feature = "RSP")]
    RSP_COP1,
    #[cfg(feature = "RSP")]
    RSP_COP2,
    #[cfg(feature = "RSP")]
    RSP_COP2_VU,
    #[cfg(feature = "R3000GTE")]
    R3000GTE_NORMAL,
    #[cfg(feature = "R3000GTE")]
    R3000GTE_SPECIAL,
    #[cfg(feature = "R3000GTE")]
    R3000GTE_REGIMM,
    #[cfg(feature = "R3000GTE")]
    R3000GTE_COP0,
    #[cfg(feature = "R3000GTE")]
    R3000GTE_COP1,
    #[cfg(feature = "R3000GTE")]
    R3000GTE_COP2,
    #[cfg(feature = "R3000GTE")]
    R3000GTE_COP2_GTE,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_NORMAL,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_SPECIAL,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_SPECIAL_RS,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_SPECIAL_SA,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_REGIMM,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_SPECIAL2,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_SPECIAL3,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_SPECIAL3_BSHFL,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP0,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP0_BC0,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP0_TLB,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP1,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP1_BC1,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP1_FPUS,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP1_FPUW,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP2,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP2_BC2,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP2_MFHC2,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP2_MFHC2_P,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP2_MFHC2_P_S,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_COP2_MTHC2,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU0,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU1,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU3,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT0,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT0_FMT0,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT0_FMT2,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT0_FMT3,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT0_RND,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT0_CVTFLT,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT0_CVTINT,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT0_FMT8,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT0_FMT9,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT0_CONTROL,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT0_COLOR,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT0_CST,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT2,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU4_FMT2_CNDMOVE,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU5,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU6,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU6_FMT7,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU6_FMT7_FMT0,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_VFPU7,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX_QUADLR,
    #[cfg(feature = "R5900EE")]
    R5900EE_NORMAL,
    #[cfg(feature = "R5900EE")]
    R5900EE_SPECIAL,
    #[cfg(feature = "R5900EE")]
    R5900EE_REGIMM,
    #[cfg(feature = "R5900EE")]
    R5900EE_COP0,
    #[cfg(feature = "R5900EE")]
    R5900EE_COP0_TLB,
    #[cfg(feature = "R5900EE")]
    R5900EE_COP1,
    #[cfg(feature = "R5900EE")]
    R5900EE_COP1_FPUS,
    #[cfg(feature = "R5900EE")]
    R5900EE_COP2,
    #[cfg(feature = "R5900EE")]
    R5900EE_COP2_NOHIGHBIT,
    #[cfg(feature = "R5900EE")]
    R5900EE_COP2_BC2,
    #[cfg(feature = "R5900EE")]
    R5900EE_COP2_SPECIAL1,
    #[cfg(feature = "R5900EE")]
    R5900EE_COP2_SPECIAL2,
    #[cfg(feature = "R5900EE")]
    R5900EE_COP2_VIWR,
    #[cfg(feature = "R5900EE")]
    R5900EE_MMI,
    #[cfg(feature = "R5900EE")]
    R5900EE_MMI_0,
    #[cfg(feature = "R5900EE")]
    R5900EE_MMI_1,
    #[cfg(feature = "R5900EE")]
    R5900EE_MMI_2,
    #[cfg(feature = "R5900EE")]
    R5900EE_MMI_3,
    #[cfg(feature = "R5900EE")]
    R5900EE_MMI_PMFHL,
    #[cfg(feature = "R5900EE")]
    R5900EE_MMI_PMTHL,
}
pub static OPCODE_CATEGORIES: [OpcodeCategoryDescriptor; OPCODE_CATEGORY_COUNT] = {
    let mut table = [OpcodeCategoryDescriptor::default(); OPCODE_CATEGORY_COUNT];
    table[OpcodeCategory::CORE_NORMAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "NORMAL"))
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_SPECIAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "SPECIAL"))
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_REGIMM as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "REGIMM"))
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP0 as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP0"))
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP0_BC0 as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP0_BC0"))
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP0_TLB as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP0_TLB"))
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP1"))
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP1_BC1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP1_BC1"))
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP1_FPUS as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP1_FPUS"))
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP1_FPUD as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP1_FPUD"))
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP1_FPUW as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP1_FPUW"))
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP1_FPUL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP1_FPUL"))
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP2 as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(concat!("CORE", "_", "COP2"))
    }
    .check_panic_chain();
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_NORMAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "NORMAL"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_NORMAL_LWC2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "NORMAL_LWC2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_NORMAL_SWC2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "NORMAL_SWC2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_SPECIAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "SPECIAL"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_REGIMM as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "REGIMM"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_COP0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "COP0"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_COP1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "COP1"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_COP2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "COP2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_COP2_VU as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "COP2_VU"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_NORMAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "NORMAL"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_SPECIAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "SPECIAL"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_REGIMM as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "REGIMM"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_COP0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "COP0"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_COP1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "COP1"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_COP2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "COP2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_COP2_GTE as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R3000GTE", "_", "COP2_GTE"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_NORMAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "NORMAL"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_SPECIAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "SPECIAL"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_SPECIAL_RS as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "SPECIAL_RS"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_SPECIAL_SA as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "SPECIAL_SA"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_REGIMM as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "REGIMM"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_SPECIAL2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "SPECIAL2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_SPECIAL3 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "SPECIAL3"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_SPECIAL3_BSHFL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "SPECIAL3_BSHFL"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP0 as usize] = OpcodeCategoryDescriptor {
            handwritten_category: true,
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP0"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP0_BC0 as usize] = OpcodeCategoryDescriptor {
            handwritten_category: true,
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP0_BC0"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP0_TLB as usize] = OpcodeCategoryDescriptor {
            handwritten_category: true,
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP0_TLB"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP1"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP1_BC1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP1_BC1"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP1_FPUS as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP1_FPUS"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP1_FPUW as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP1_FPUW"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP2_BC2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP2_BC2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP2_MFHC2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP2_MFHC2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP2_MFHC2_P as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP2_MFHC2_P"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP2_MFHC2_P_S as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP2_MFHC2_P_S"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP2_MTHC2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "COP2_MTHC2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU0"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU1"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU3 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU3"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT0"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT3"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_RND"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTFLT as usize] =
            OpcodeCategoryDescriptor {
                ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_CVTFLT"))
            }
            .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT as usize] =
            OpcodeCategoryDescriptor {
                ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_CVTINT"))
            }
            .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT8"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT9 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT9"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CONTROL as usize] =
            OpcodeCategoryDescriptor {
                ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_CONTROL"))
            }
            .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_COLOR as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_COLOR"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CST as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_CST"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2_CNDMOVE as usize] =
            OpcodeCategoryDescriptor {
                ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU4_FMT2_CNDMOVE"))
            }
            .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU5 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU5"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU6 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU6"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU6_FMT7"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU6_FMT7_FMT0"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU7 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "VFPU7"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_QUADLR as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R4000ALLEGREX", "_", "QUADLR"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_NORMAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "NORMAL"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_SPECIAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "SPECIAL"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_REGIMM as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "REGIMM"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP0 as usize] = OpcodeCategoryDescriptor {
            handwritten_category: true,
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "COP0"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP0_TLB as usize] = OpcodeCategoryDescriptor {
            handwritten_category: true,
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "COP0_TLB"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "COP1"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP1_FPUS as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "COP1_FPUS"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "COP2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2_NOHIGHBIT as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "COP2_NOHIGHBIT"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2_BC2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "COP2_BC2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2_SPECIAL1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "COP2_SPECIAL1"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2_SPECIAL2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "COP2_SPECIAL2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2_VIWR as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "COP2_VIWR"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "MMI"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI_0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "MMI_0"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI_1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "MMI_1"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI_2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "MMI_2"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI_3 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "MMI_3"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI_PMFHL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "MMI_PMFHL"))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI_PMTHL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("R5900EE", "_", "MMI_PMTHL"))
        }
        .check_panic_chain();
    }
    let mut i = 0;
    while i < OPCODE_CATEGORY_COUNT {
        table[i].check_panic();
        i += 1;
    }
    table
};
