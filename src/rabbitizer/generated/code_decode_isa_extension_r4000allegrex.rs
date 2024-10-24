/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::isa::IsaVersion;
use crate::opcodes::{Opcode, OpcodeCategory, OpcodeDecoder};
use crate::{DecodingFlags, EncodedFieldMask};
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_normal(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_NORMAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x32 => Opcode::r4000allegrex_lv_s,
            0x3A => Opcode::r4000allegrex_sv_s,
            0x36 => Opcode::r4000allegrex_lv_q,
            0x3E => Opcode::r4000allegrex_sv_q,
            0x00 => {
                return Self::decode_isa_extension_r4000allegrex_special(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x01 => {
                return Self::decode_isa_extension_r4000allegrex_regimm(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x1C => {
                return Self::decode_isa_extension_r4000allegrex_special2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x1F => {
                return Self::decode_isa_extension_r4000allegrex_special3(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x10 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x11 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x12 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x18 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x19 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x1B => {
                return Self::decode_isa_extension_r4000allegrex_vfpu3(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x34 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x37 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu5(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3C => {
                return Self::decode_isa_extension_r4000allegrex_vfpu6(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3D => {
                return Self::decode_isa_extension_r4000allegrex_quadlr(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3F => {
                return Self::decode_isa_extension_r4000allegrex_vfpu7(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => {
                return Self::decode_isa_extension_none_normal(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_special(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_SPECIAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x16 => Opcode::r4000allegrex_clz,
            0x17 => Opcode::r4000allegrex_clo,
            0x1C => Opcode::r4000allegrex_madd,
            0x1D => Opcode::r4000allegrex_maddu,
            0x2E => Opcode::r4000allegrex_msub,
            0x2F => Opcode::r4000allegrex_msubu,
            0x0A => Opcode::r4000allegrex_movz,
            0x0B => Opcode::r4000allegrex_movn,
            0x2C => Opcode::r4000allegrex_max,
            0x2D => Opcode::r4000allegrex_min,
            0x02 => {
                return Self::decode_isa_extension_r4000allegrex_special_rs(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x06 => {
                return Self::decode_isa_extension_r4000allegrex_special_sa(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => {
                return Self::decode_isa_extension_none_special(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_special_rs(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rs;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_SPECIAL_RS;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r4000allegrex_srl,
            0x01 => Opcode::r4000allegrex_rotr,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_special_sa(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::sa;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_SPECIAL_SA;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r4000allegrex_srlv,
            0x01 => Opcode::r4000allegrex_rotrv,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_regimm(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rt;
        let _opcode_category = OpcodeCategory::R4000ALLEGREX_REGIMM;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let _opcode = match mask.get_shifted(word) {
            _ => {
                return Self::decode_isa_extension_none_regimm(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_special2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_SPECIAL2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r4000allegrex_sleep,
            0x24 => Opcode::r4000allegrex_mfie,
            0x26 => Opcode::r4000allegrex_mtie,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_special3(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_SPECIAL3;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r4000allegrex_ext,
            0x04 => Opcode::r4000allegrex_ins,
            0x20 => {
                return Self::decode_isa_extension_r4000allegrex_special3_bshfl(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_special3_bshfl(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::sa;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_SPECIAL3_BSHFL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x02 => Opcode::r4000allegrex_wsbh,
            0x03 => Opcode::r4000allegrex_wsbw,
            0x10 => Opcode::r4000allegrex_seb,
            0x18 => Opcode::r4000allegrex_seh,
            0x14 => Opcode::r4000allegrex_bitrev,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x01 => Opcode::ALL_INVALID,
            0x02 => Opcode::ALL_INVALID,
            0x05 => Opcode::ALL_INVALID,
            0x06 => Opcode::ALL_INVALID,
            0x08 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor0_bc0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x10 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor0_tlb(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => {
                return Self::decode_isa_extension_none_coprocessor0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor0_bc0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::bc_fmt;
        let _opcode_category = OpcodeCategory::R4000ALLEGREX_COP0_BC0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let _opcode = match mask.get_shifted(word) {
            _ => {
                return Self::decode_isa_extension_none_coprocessor0_bc0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor0_tlb(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let _opcode_category = OpcodeCategory::R4000ALLEGREX_COP0_TLB;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let _opcode = match mask.get_shifted(word) {
            _ => {
                return Self::decode_isa_extension_none_coprocessor0_tlb(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor1(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x08 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor1_bc1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x10 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor1_fpu_s(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x14 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor1_fpu_w(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x11 => Opcode::ALL_INVALID,
            0x15 => Opcode::ALL_INVALID,
            _ => {
                return Self::decode_isa_extension_none_coprocessor1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor1_bc1(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::bc_fmt;
        let _opcode_category = OpcodeCategory::R4000ALLEGREX_COP1_BC1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let _opcode = match mask.get_shifted(word) {
            _ => {
                return Self::decode_isa_extension_none_coprocessor1_bc1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor1_fpu_s(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP1_FPUS;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x08 => Opcode::ALL_INVALID,
            0x09 => Opcode::ALL_INVALID,
            0x0A => Opcode::ALL_INVALID,
            0x0B => Opcode::ALL_INVALID,
            0x21 => Opcode::ALL_INVALID,
            0x25 => Opcode::ALL_INVALID,
            _ => {
                return Self::decode_isa_extension_none_coprocessor1_fpu_s(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor1_fpu_w(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP1_FPUW;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x21 => Opcode::ALL_INVALID,
            _ => {
                return Self::decode_isa_extension_none_coprocessor1_fpu_w(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let _opcode_category = OpcodeCategory::R4000ALLEGREX_COP2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let _opcode = match mask.get_shifted(word) {
            0x08 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor2_bc2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x03 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor2_mfhc2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x07 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor2_mthc2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => {
                return Self::decode_isa_extension_none_coprocessor2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor2_bc2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_bc2_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP2_BC2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r4000allegrex_bvf,
            0x01 => Opcode::r4000allegrex_bvt,
            0x02 => Opcode::r4000allegrex_bvfl,
            0x03 => Opcode::r4000allegrex_bvtl,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor2_mfhc2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_mxhc2;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP2_MFHC2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0 => Opcode::r4000allegrex_mfv,
            0x01 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor2_mfhc2_p(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor2_mfhc2_p(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_mfhc2_p_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP2_MFHC2_P;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0 => Opcode::r4000allegrex_mfvc,
            0x07 => {
                return Self::decode_isa_extension_r4000allegrex_coprocessor2_mfhc2_p_s(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor2_mfhc2_p_s(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_mfhc2_p_s_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP2_MFHC2_P_S;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0xF => Opcode::r4000allegrex_vsync2,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor2_mthc2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_mxhc2;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP2_MTHC2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0 => Opcode::r4000allegrex_mtv,
            0x1 => Opcode::r4000allegrex_mtvc,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00000 => Opcode::r4000allegrex_vadd_s,
            0x00001 => Opcode::r4000allegrex_vadd_p,
            0x00100 => Opcode::r4000allegrex_vadd_t,
            0x00101 => Opcode::r4000allegrex_vadd_q,
            0x10000 => Opcode::r4000allegrex_vsub_s,
            0x10001 => Opcode::r4000allegrex_vsub_p,
            0x10100 => Opcode::r4000allegrex_vsub_t,
            0x10101 => Opcode::r4000allegrex_vsub_q,
            0x20000 => Opcode::r4000allegrex_vsbn_s,
            0x70000 => Opcode::r4000allegrex_vdiv_s,
            0x70001 => Opcode::r4000allegrex_vdiv_p,
            0x70100 => Opcode::r4000allegrex_vdiv_t,
            0x70101 => Opcode::r4000allegrex_vdiv_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu1(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00000 => Opcode::r4000allegrex_vmul_s,
            0x00001 => Opcode::r4000allegrex_vmul_p,
            0x00100 => Opcode::r4000allegrex_vmul_t,
            0x00101 => Opcode::r4000allegrex_vmul_q,
            0x10001 => Opcode::r4000allegrex_vdot_p,
            0x10100 => Opcode::r4000allegrex_vdot_t,
            0x10101 => Opcode::r4000allegrex_vdot_q,
            0x20001 => Opcode::r4000allegrex_vscl_p,
            0x20100 => Opcode::r4000allegrex_vscl_t,
            0x20101 => Opcode::r4000allegrex_vscl_q,
            0x40001 => Opcode::r4000allegrex_vhdp_p,
            0x40100 => Opcode::r4000allegrex_vhdp_t,
            0x40101 => Opcode::r4000allegrex_vhdp_q,
            0x50100 => Opcode::r4000allegrex_vcrs_t,
            0x60001 => Opcode::r4000allegrex_vdet_p,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu3(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU3;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00000 => Opcode::r4000allegrex_vcmp_s,
            0x00001 => Opcode::r4000allegrex_vcmp_p,
            0x00100 => Opcode::r4000allegrex_vcmp_t,
            0x00101 => Opcode::r4000allegrex_vcmp_q,
            0x20000 => Opcode::r4000allegrex_vmin_s,
            0x20001 => Opcode::r4000allegrex_vmin_p,
            0x20100 => Opcode::r4000allegrex_vmin_t,
            0x20101 => Opcode::r4000allegrex_vmin_q,
            0x30000 => Opcode::r4000allegrex_vmax_s,
            0x30001 => Opcode::r4000allegrex_vmax_p,
            0x30100 => Opcode::r4000allegrex_vmax_t,
            0x30101 => Opcode::r4000allegrex_vmax_q,
            0x50000 => Opcode::r4000allegrex_vscmp_s,
            0x50001 => Opcode::r4000allegrex_vscmp_p,
            0x50100 => Opcode::r4000allegrex_vscmp_t,
            0x50101 => Opcode::r4000allegrex_vscmp_q,
            0x60000 => Opcode::r4000allegrex_vsge_s,
            0x60001 => Opcode::r4000allegrex_vsge_p,
            0x60100 => Opcode::r4000allegrex_vsge_t,
            0x60101 => Opcode::r4000allegrex_vsge_q,
            0x70000 => Opcode::r4000allegrex_vslt_s,
            0x70001 => Opcode::r4000allegrex_vslt_p,
            0x70100 => Opcode::r4000allegrex_vslt_t,
            0x70101 => Opcode::r4000allegrex_vslt_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x60000 => Opcode::r4000allegrex_vwbn_s,
            0x00000 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x00001 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x00100 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x00101 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x40000 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x40001 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x40100 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x40101 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_fmt0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x2 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_fmt2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_fmt3(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x4 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_rnd(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x6 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_cvtflt(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x7 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_cvtint(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x8 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_fmt8(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x9 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_fmt9(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0xA => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_control(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0xB => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_color(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0xC => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_cst(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0xD => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_cst(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0xE => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_cst(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0xF => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt0_cst(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt0_fmt0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0000 => Opcode::r4000allegrex_vmov_s,
            0x0001 => Opcode::r4000allegrex_vmov_p,
            0x0100 => Opcode::r4000allegrex_vmov_t,
            0x0101 => Opcode::r4000allegrex_vmov_q,
            0x0200 => Opcode::r4000allegrex_vabs_s,
            0x0201 => Opcode::r4000allegrex_vabs_p,
            0x0300 => Opcode::r4000allegrex_vabs_t,
            0x0301 => Opcode::r4000allegrex_vabs_q,
            0x0400 => Opcode::r4000allegrex_vneg_s,
            0x0401 => Opcode::r4000allegrex_vneg_p,
            0x0500 => Opcode::r4000allegrex_vneg_t,
            0x0501 => Opcode::r4000allegrex_vneg_q,
            0x0601 => Opcode::r4000allegrex_vidt_p,
            0x0701 => Opcode::r4000allegrex_vidt_q,
            0x0800 => Opcode::r4000allegrex_vsat0_s,
            0x0801 => Opcode::r4000allegrex_vsat0_p,
            0x0900 => Opcode::r4000allegrex_vsat0_t,
            0x0901 => Opcode::r4000allegrex_vsat0_q,
            0x0A00 => Opcode::r4000allegrex_vsat1_s,
            0x0A01 => Opcode::r4000allegrex_vsat1_p,
            0x0B00 => Opcode::r4000allegrex_vsat1_t,
            0x0B01 => Opcode::r4000allegrex_vsat1_q,
            0x0C00 => Opcode::r4000allegrex_vzero_s,
            0x0C01 => Opcode::r4000allegrex_vzero_p,
            0x0D00 => Opcode::r4000allegrex_vzero_t,
            0x0D01 => Opcode::r4000allegrex_vzero_q,
            0x0E00 => Opcode::r4000allegrex_vone_s,
            0x0E01 => Opcode::r4000allegrex_vone_p,
            0x0F00 => Opcode::r4000allegrex_vone_t,
            0x0F01 => Opcode::r4000allegrex_vone_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt0_fmt2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0000 => Opcode::r4000allegrex_vrcp_s,
            0x0001 => Opcode::r4000allegrex_vrcp_p,
            0x0100 => Opcode::r4000allegrex_vrcp_t,
            0x0101 => Opcode::r4000allegrex_vrcp_q,
            0x0200 => Opcode::r4000allegrex_vrsq_s,
            0x0201 => Opcode::r4000allegrex_vrsq_p,
            0x0300 => Opcode::r4000allegrex_vrsq_t,
            0x0301 => Opcode::r4000allegrex_vrsq_q,
            0x0400 => Opcode::r4000allegrex_vsin_s,
            0x0401 => Opcode::r4000allegrex_vsin_p,
            0x0500 => Opcode::r4000allegrex_vsin_t,
            0x0501 => Opcode::r4000allegrex_vsin_q,
            0x0600 => Opcode::r4000allegrex_vcos_s,
            0x0601 => Opcode::r4000allegrex_vcos_p,
            0x0700 => Opcode::r4000allegrex_vcos_t,
            0x0701 => Opcode::r4000allegrex_vcos_q,
            0x0800 => Opcode::r4000allegrex_vexp2_s,
            0x0801 => Opcode::r4000allegrex_vexp2_p,
            0x0900 => Opcode::r4000allegrex_vexp2_t,
            0x0901 => Opcode::r4000allegrex_vexp2_q,
            0x0A00 => Opcode::r4000allegrex_vlog2_s,
            0x0A01 => Opcode::r4000allegrex_vlog2_p,
            0x0B00 => Opcode::r4000allegrex_vlog2_t,
            0x0B01 => Opcode::r4000allegrex_vlog2_q,
            0x0C00 => Opcode::r4000allegrex_vsqrt_s,
            0x0C01 => Opcode::r4000allegrex_vsqrt_p,
            0x0D00 => Opcode::r4000allegrex_vsqrt_t,
            0x0D01 => Opcode::r4000allegrex_vsqrt_q,
            0x0E00 => Opcode::r4000allegrex_vasin_s,
            0x0E01 => Opcode::r4000allegrex_vasin_p,
            0x0F00 => Opcode::r4000allegrex_vasin_t,
            0x0F01 => Opcode::r4000allegrex_vasin_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt0_fmt3(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0000 => Opcode::r4000allegrex_vnrcp_s,
            0x0001 => Opcode::r4000allegrex_vnrcp_p,
            0x0100 => Opcode::r4000allegrex_vnrcp_t,
            0x0101 => Opcode::r4000allegrex_vnrcp_q,
            0x0400 => Opcode::r4000allegrex_vnsin_s,
            0x0401 => Opcode::r4000allegrex_vnsin_p,
            0x0500 => Opcode::r4000allegrex_vnsin_t,
            0x0501 => Opcode::r4000allegrex_vnsin_q,
            0x0800 => Opcode::r4000allegrex_vrexp2_s,
            0x0801 => Opcode::r4000allegrex_vrexp2_p,
            0x0900 => Opcode::r4000allegrex_vrexp2_t,
            0x0901 => Opcode::r4000allegrex_vrexp2_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt0_rnd(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0000 => Opcode::r4000allegrex_vrnds_s,
            0x0200 => Opcode::r4000allegrex_vrndi_s,
            0x0201 => Opcode::r4000allegrex_vrndi_p,
            0x0300 => Opcode::r4000allegrex_vrndi_t,
            0x0301 => Opcode::r4000allegrex_vrndi_q,
            0x0400 => Opcode::r4000allegrex_vrndf1_s,
            0x0401 => Opcode::r4000allegrex_vrndf1_p,
            0x0500 => Opcode::r4000allegrex_vrndf1_t,
            0x0501 => Opcode::r4000allegrex_vrndf1_q,
            0x0600 => Opcode::r4000allegrex_vrndf2_s,
            0x0601 => Opcode::r4000allegrex_vrndf2_p,
            0x0700 => Opcode::r4000allegrex_vrndf2_t,
            0x0701 => Opcode::r4000allegrex_vrndf2_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt0_cvtflt(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTFLT;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0401 => Opcode::r4000allegrex_vf2h_p,
            0x0501 => Opcode::r4000allegrex_vf2h_q,
            0x0600 => Opcode::r4000allegrex_vh2f_s,
            0x0601 => Opcode::r4000allegrex_vh2f_p,
            0x0C00 => Opcode::r4000allegrex_vsbz_s,
            0x0E00 => Opcode::r4000allegrex_vlgb_s,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt0_cvtint(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0000 => Opcode::r4000allegrex_vuc2ifs_s,
            0x0200 => Opcode::r4000allegrex_vc2i_s,
            0x0400 => Opcode::r4000allegrex_vus2i_s,
            0x0401 => Opcode::r4000allegrex_vus2i_p,
            0x0600 => Opcode::r4000allegrex_vs2i_s,
            0x0601 => Opcode::r4000allegrex_vs2i_p,
            0x0901 => Opcode::r4000allegrex_vi2uc_q,
            0x0B01 => Opcode::r4000allegrex_vi2c_q,
            0x0C01 => Opcode::r4000allegrex_vi2us_p,
            0x0D01 => Opcode::r4000allegrex_vi2us_q,
            0x0E01 => Opcode::r4000allegrex_vi2s_p,
            0x0F01 => Opcode::r4000allegrex_vi2s_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt0_fmt8(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0101 => Opcode::r4000allegrex_vsrt1_q,
            0x0301 => Opcode::r4000allegrex_vsrt2_q,
            0x0401 => Opcode::r4000allegrex_vbfy1_p,
            0x0501 => Opcode::r4000allegrex_vbfy1_q,
            0x0701 => Opcode::r4000allegrex_vbfy2_q,
            0x0800 => Opcode::r4000allegrex_vocp_s,
            0x0801 => Opcode::r4000allegrex_vocp_p,
            0x0900 => Opcode::r4000allegrex_vocp_t,
            0x0901 => Opcode::r4000allegrex_vocp_q,
            0x0A00 => Opcode::r4000allegrex_vsocp_s,
            0x0A01 => Opcode::r4000allegrex_vsocp_p,
            0x0C01 => Opcode::r4000allegrex_vfad_p,
            0x0D00 => Opcode::r4000allegrex_vfad_t,
            0x0D01 => Opcode::r4000allegrex_vfad_q,
            0x0E01 => Opcode::r4000allegrex_vavg_p,
            0x0F00 => Opcode::r4000allegrex_vavg_t,
            0x0F01 => Opcode::r4000allegrex_vavg_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt0_fmt9(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT9;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0101 => Opcode::r4000allegrex_vsrt3_q,
            0x0301 => Opcode::r4000allegrex_vsrt4_q,
            0x0400 => Opcode::r4000allegrex_vsgn_s,
            0x0401 => Opcode::r4000allegrex_vsgn_p,
            0x0500 => Opcode::r4000allegrex_vsgn_t,
            0x0501 => Opcode::r4000allegrex_vsgn_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt0_control(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CONTROL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0100 => Opcode::r4000allegrex_vmfvc,
            0x0201 => Opcode::r4000allegrex_vmtvc,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt0_color(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt0_fmt0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_COLOR;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0301 => Opcode::r4000allegrex_vt4444_q,
            0x0501 => Opcode::r4000allegrex_vt5551_q,
            0x0701 => Opcode::r4000allegrex_vt5650_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt0_cst(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_tp;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CST;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0 => Opcode::r4000allegrex_vcst_s,
            0x1 => Opcode::r4000allegrex_vcst_p,
            0x100 => Opcode::r4000allegrex_vcst_t,
            0x101 => Opcode::r4000allegrex_vcst_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt2_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00000 => Opcode::r4000allegrex_vf2in_s,
            0x00001 => Opcode::r4000allegrex_vf2in_p,
            0x00100 => Opcode::r4000allegrex_vf2in_t,
            0x00101 => Opcode::r4000allegrex_vf2in_q,
            0x04000 => Opcode::r4000allegrex_vf2iz_s,
            0x04001 => Opcode::r4000allegrex_vf2iz_p,
            0x04100 => Opcode::r4000allegrex_vf2iz_t,
            0x04101 => Opcode::r4000allegrex_vf2iz_q,
            0x08000 => Opcode::r4000allegrex_vf2iu_s,
            0x08001 => Opcode::r4000allegrex_vf2iu_p,
            0x08100 => Opcode::r4000allegrex_vf2iu_t,
            0x08101 => Opcode::r4000allegrex_vf2iu_q,
            0x0C000 => Opcode::r4000allegrex_vf2id_s,
            0x0C001 => Opcode::r4000allegrex_vf2id_p,
            0x0C100 => Opcode::r4000allegrex_vf2id_t,
            0x0C101 => Opcode::r4000allegrex_vf2id_q,
            0x10000 => Opcode::r4000allegrex_vi2f_s,
            0x10001 => Opcode::r4000allegrex_vi2f_p,
            0x10100 => Opcode::r4000allegrex_vi2f_t,
            0x10101 => Opcode::r4000allegrex_vi2f_q,
            0x14000 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt2_cndmove(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x14001 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt2_cndmove(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x14100 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt2_cndmove(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x14101 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu4_fmt2_cndmove(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu4_fmt2_cndmove(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu4_fmt2_cndmove_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2_CNDMOVE;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00000 => Opcode::r4000allegrex_vcmovt_s,
            0x00001 => Opcode::r4000allegrex_vcmovt_p,
            0x00100 => Opcode::r4000allegrex_vcmovt_t,
            0x00101 => Opcode::r4000allegrex_vcmovt_q,
            0x01000 => Opcode::r4000allegrex_vcmovf_s,
            0x01001 => Opcode::r4000allegrex_vcmovf_p,
            0x01100 => Opcode::r4000allegrex_vcmovf_t,
            0x01101 => Opcode::r4000allegrex_vcmovf_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu5(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu5_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU5;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r4000allegrex_vpfxs,
            0x02 => Opcode::r4000allegrex_vpfxt,
            0x04 => Opcode::r4000allegrex_vpfxd,
            0x6 => Opcode::r4000allegrex_viim_s,
            0x7 => Opcode::r4000allegrex_vfim_s,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu6(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu6_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU6;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00001 => Opcode::r4000allegrex_vmmul_p,
            0x00100 => Opcode::r4000allegrex_vmmul_t,
            0x00101 => Opcode::r4000allegrex_vmmul_q,
            0x10000 => Opcode::r4000allegrex_vhtfm2_p,
            0x10001 => Opcode::r4000allegrex_vtfm2_p,
            0x20001 => Opcode::r4000allegrex_vhtfm3_t,
            0x20100 => Opcode::r4000allegrex_vtfm3_t,
            0x30100 => Opcode::r4000allegrex_vhtfm4_q,
            0x30101 => Opcode::r4000allegrex_vtfm4_q,
            0x40001 => Opcode::r4000allegrex_vmscl_p,
            0x40100 => Opcode::r4000allegrex_vmscl_t,
            0x40101 => Opcode::r4000allegrex_vmscl_q,
            0x50100 => Opcode::r4000allegrex_vcrsp_t,
            0x50101 => Opcode::r4000allegrex_vqmul_q,
            0x70000 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu6_fmt7(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x70001 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu6_fmt7(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x70100 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu6_fmt7(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x70101 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu6_fmt7(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu6_fmt7(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu6_fmt7_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x04001 => Opcode::r4000allegrex_vrot_p,
            0x04100 => Opcode::r4000allegrex_vrot_t,
            0x04101 => Opcode::r4000allegrex_vrot_q,
            0x000 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu6_fmt7_fmt0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x001 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu6_fmt7_fmt0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x100 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu6_fmt7_fmt0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x101 => {
                return Self::decode_isa_extension_r4000allegrex_vfpu6_fmt7_fmt0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu6_fmt7_fmt0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu6_fmt7_fmt0_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0001 => Opcode::r4000allegrex_vmmov_p,
            0x0100 => Opcode::r4000allegrex_vmmov_t,
            0x0101 => Opcode::r4000allegrex_vmmov_q,
            0x0601 => Opcode::r4000allegrex_vmidt_p,
            0x0700 => Opcode::r4000allegrex_vmidt_t,
            0x0701 => Opcode::r4000allegrex_vmidt_q,
            0x0C01 => Opcode::r4000allegrex_vmzero_p,
            0x0D00 => Opcode::r4000allegrex_vmzero_t,
            0x0D01 => Opcode::r4000allegrex_vmzero_q,
            0x0E01 => Opcode::r4000allegrex_vmone_p,
            0x0F00 => Opcode::r4000allegrex_vmone_t,
            0x0F01 => Opcode::r4000allegrex_vmone_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_quadlr(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_wb;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_QUADLR;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0 => Opcode::r4000allegrex_svl_q,
            0x1 => Opcode::r4000allegrex_svr_q,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_vfpu7(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r4000allegrex_vfpu7_fmt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_VFPU7;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x3FF0000 => Opcode::r4000allegrex_vnop,
            0x3FF0320 => Opcode::r4000allegrex_vsync,
            0x3FF040D => Opcode::r4000allegrex_vflush,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
}
