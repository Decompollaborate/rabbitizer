/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::encoded_field_mask::EncodedFieldMask;
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
    R5900EE_COP2_NI,
    #[cfg(feature = "R5900EE")]
    R5900EE_COP2_I,
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
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "NORMAL"),
            EncodedFieldMask::opcode,
            0x0,
        )
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_SPECIAL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "SPECIAL"),
            EncodedFieldMask::function,
            0x0,
        )
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_REGIMM as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "REGIMM"),
            EncodedFieldMask::rt,
            0x04000000,
        )
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP0 as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "COP0"),
            EncodedFieldMask::fmt,
            0x40000000,
        )
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP0_BC0 as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "COP0_BC0"),
            EncodedFieldMask::bc_fmt,
            0x41000000,
        )
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP0_TLB as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "COP0_TLB"),
            EncodedFieldMask::function,
            0x42000000,
        )
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "COP1"),
            EncodedFieldMask::fmt,
            0x44000000,
        )
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP1_BC1 as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "COP1_BC1"),
            EncodedFieldMask::bc_fmt,
            0x45000000,
        )
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP1_FPUS as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "COP1_FPUS"),
            EncodedFieldMask::function,
            0x46000000,
        )
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP1_FPUD as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "COP1_FPUD"),
            EncodedFieldMask::function,
            0x46200000,
        )
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP1_FPUW as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "COP1_FPUW"),
            EncodedFieldMask::function,
            0x46800000,
        )
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP1_FPUL as usize] = OpcodeCategoryDescriptor {
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "COP1_FPUL"),
            EncodedFieldMask::function,
            0x46A00000,
        )
    }
    .check_panic_chain();
    table[OpcodeCategory::CORE_COP2 as usize] = OpcodeCategoryDescriptor {
        handwritten_category: true,
        ..OpcodeCategoryDescriptor::new(
            concat!("CORE", "_", "COP2"),
            EncodedFieldMask::fmt,
            0x48000000,
        )
    }
    .check_panic_chain();
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_NORMAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("RSP", "_", "NORMAL"),
                EncodedFieldMask::opcode,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_NORMAL_LWC2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("RSP", "_", "NORMAL_LWC2"),
                EncodedFieldMask::rd,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_NORMAL_SWC2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("RSP", "_", "NORMAL_SWC2"),
                EncodedFieldMask::rd,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_SPECIAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("RSP", "_", "SPECIAL"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_REGIMM as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("RSP", "_", "REGIMM"),
                EncodedFieldMask::rt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_COP0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "COP0"), EncodedFieldMask::fmt, 0x0)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_COP1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "COP1"), EncodedFieldMask::fmt, 0x0)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_COP2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(concat!("RSP", "_", "COP2"), EncodedFieldMask::fmt, 0x0)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[OpcodeCategory::RSP_COP2_VU as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("RSP", "_", "COP2_VU"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_NORMAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R3000GTE", "_", "NORMAL"),
                EncodedFieldMask::opcode,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_SPECIAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R3000GTE", "_", "SPECIAL"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_REGIMM as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R3000GTE", "_", "REGIMM"),
                EncodedFieldMask::rt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_COP0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R3000GTE", "_", "COP0"),
                EncodedFieldMask::fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_COP1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R3000GTE", "_", "COP1"),
                EncodedFieldMask::fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_COP2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R3000GTE", "_", "COP2"),
                EncodedFieldMask::fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[OpcodeCategory::R3000GTE_COP2_GTE as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R3000GTE", "_", "COP2_GTE"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_NORMAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "NORMAL"),
                EncodedFieldMask::opcode,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_SPECIAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "SPECIAL"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_SPECIAL_RS as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "SPECIAL_RS"),
                EncodedFieldMask::rs,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_SPECIAL_SA as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "SPECIAL_SA"),
                EncodedFieldMask::sa,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_REGIMM as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "REGIMM"),
                EncodedFieldMask::rt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_SPECIAL2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "SPECIAL2"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_SPECIAL3 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "SPECIAL3"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_SPECIAL3_BSHFL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "SPECIAL3_BSHFL"),
                EncodedFieldMask::sa,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP0 as usize] = OpcodeCategoryDescriptor {
            handwritten_category: true,
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP0"),
                EncodedFieldMask::fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP0_BC0 as usize] = OpcodeCategoryDescriptor {
            handwritten_category: true,
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP0_BC0"),
                EncodedFieldMask::bc_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP0_TLB as usize] = OpcodeCategoryDescriptor {
            handwritten_category: true,
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP0_TLB"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP1"),
                EncodedFieldMask::fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP1_BC1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP1_BC1"),
                EncodedFieldMask::bc_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP1_FPUS as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP1_FPUS"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP1_FPUW as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP1_FPUW"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP2"),
                EncodedFieldMask::fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP2_BC2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP2_BC2"),
                EncodedFieldMask::r4000allegrex_bc2_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP2_MFHC2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP2_MFHC2"),
                EncodedFieldMask::r4000allegrex_mxhc2,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP2_MFHC2_P as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP2_MFHC2_P"),
                EncodedFieldMask::r4000allegrex_mfhc2_p_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP2_MFHC2_P_S as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP2_MFHC2_P_S"),
                EncodedFieldMask::r4000allegrex_mfhc2_p_s_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_COP2_MTHC2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "COP2_MTHC2"),
                EncodedFieldMask::r4000allegrex_mxhc2,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU0"),
                EncodedFieldMask::r4000allegrex_vfpu0_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU1"),
                EncodedFieldMask::r4000allegrex_vfpu0_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU3 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU3"),
                EncodedFieldMask::r4000allegrex_vfpu0_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU4"),
                EncodedFieldMask::r4000allegrex_vfpu4_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU4_FMT0"),
                EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT0"),
                EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT2"),
                EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT3"),
                EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_RND"),
                EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTFLT as usize] =
            OpcodeCategoryDescriptor {
                ..OpcodeCategoryDescriptor::new(
                    concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_CVTFLT"),
                    EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt,
                    0x0,
                )
            }
            .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT as usize] =
            OpcodeCategoryDescriptor {
                ..OpcodeCategoryDescriptor::new(
                    concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_CVTINT"),
                    EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt,
                    0x0,
                )
            }
            .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT8"),
                EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT9 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_FMT9"),
                EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CONTROL as usize] =
            OpcodeCategoryDescriptor {
                ..OpcodeCategoryDescriptor::new(
                    concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_CONTROL"),
                    EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt,
                    0x0,
                )
            }
            .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_COLOR as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_COLOR"),
                EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CST as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU4_FMT0_CST"),
                EncodedFieldMask::r4000allegrex_tp,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU4_FMT2"),
                EncodedFieldMask::r4000allegrex_vfpu4_fmt2_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2_CNDMOVE as usize] =
            OpcodeCategoryDescriptor {
                ..OpcodeCategoryDescriptor::new(
                    concat!("R4000ALLEGREX", "_", "VFPU4_FMT2_CNDMOVE"),
                    EncodedFieldMask::r4000allegrex_vfpu4_fmt2_cndmove_fmt,
                    0x0,
                )
            }
            .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU5 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU5"),
                EncodedFieldMask::r4000allegrex_vfpu5_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU6 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU6"),
                EncodedFieldMask::r4000allegrex_vfpu6_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU6_FMT7"),
                EncodedFieldMask::r4000allegrex_vfpu6_fmt7_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU6_FMT7_FMT0"),
                EncodedFieldMask::r4000allegrex_vfpu6_fmt7_fmt0_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_VFPU7 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "VFPU7"),
                EncodedFieldMask::r4000allegrex_vfpu7_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[OpcodeCategory::R4000ALLEGREX_QUADLR as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R4000ALLEGREX", "_", "QUADLR"),
                EncodedFieldMask::r4000allegrex_wb,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_NORMAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "NORMAL"),
                EncodedFieldMask::opcode,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_SPECIAL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "SPECIAL"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_REGIMM as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "REGIMM"),
                EncodedFieldMask::rt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP0 as usize] = OpcodeCategoryDescriptor {
            handwritten_category: true,
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "COP0"),
                EncodedFieldMask::fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP0_TLB as usize] = OpcodeCategoryDescriptor {
            handwritten_category: true,
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "COP0_TLB"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "COP1"),
                EncodedFieldMask::fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP1_FPUS as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "COP1_FPUS"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "COP2"),
                EncodedFieldMask::r5900ee_cop2_discriminant,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2_NI as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "COP2_NI"),
                EncodedFieldMask::r5900ee_cop2_ini_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2_I as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "COP2_I"),
                EncodedFieldMask::r5900ee_cop2_ini_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2_BC2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "COP2_BC2"),
                EncodedFieldMask::bc_fmt,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2_SPECIAL1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "COP2_SPECIAL1"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2_SPECIAL2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "COP2_SPECIAL2"),
                EncodedFieldMask::r5900ee_fhi_flo,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_COP2_VIWR as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "COP2_VIWR"),
                EncodedFieldMask::r5900ee_viwr_fhilo,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "MMI"),
                EncodedFieldMask::function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI_0 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "MMI_0"),
                EncodedFieldMask::r5900ee_mmi_function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI_1 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "MMI_1"),
                EncodedFieldMask::r5900ee_mmi_function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI_2 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "MMI_2"),
                EncodedFieldMask::r5900ee_mmi_function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI_3 as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "MMI_3"),
                EncodedFieldMask::r5900ee_mmi_function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI_PMFHL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "MMI_PMFHL"),
                EncodedFieldMask::r5900ee_mmi_function,
                0x0,
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[OpcodeCategory::R5900EE_MMI_PMTHL as usize] = OpcodeCategoryDescriptor {
            ..OpcodeCategoryDescriptor::new(
                concat!("R5900EE", "_", "MMI_PMTHL"),
                EncodedFieldMask::r5900ee_mmi_function,
                0x0,
            )
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
