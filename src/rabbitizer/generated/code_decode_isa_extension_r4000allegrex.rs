/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{DecodingFlags, EncodedFieldMask, IsaVersion, Opcode, OpcodeCategory, OpcodeDecoder};
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
        flags: &DecodingFlags,
        isa_version: IsaVersion,
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
        flags: &DecodingFlags,
        isa_version: IsaVersion,
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
        let opcode_category = OpcodeCategory::R4000ALLEGREX_REGIMM;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            _ => {
                return Self::decode_isa_extension_none_regimm(
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
    pub(crate) const fn decode_isa_extension_r4000allegrex_special2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
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
        flags: &DecodingFlags,
        isa_version: IsaVersion,
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
            _ => Opcode::ALL_INVALID,
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
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP0_BC0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            _ => {
                return Self::decode_isa_extension_none_coprocessor0_bc0(
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
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor0_tlb(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP0_TLB;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            _ => {
                return Self::decode_isa_extension_none_coprocessor0_tlb(
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
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP1_BC1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            _ => {
                return Self::decode_isa_extension_none_coprocessor1_bc1(
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
            _ => Opcode::ALL_INVALID,
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
            _ => Opcode::ALL_INVALID,
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
        let opcode_category = OpcodeCategory::R4000ALLEGREX_COP2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
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
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_coprocessor2_bc2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::bc_fmt;
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
        flags: &DecodingFlags,
        isa_version: IsaVersion,
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
        flags: &DecodingFlags,
        isa_version: IsaVersion,
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
        flags: &DecodingFlags,
        isa_version: IsaVersion,
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
        flags: &DecodingFlags,
        isa_version: IsaVersion,
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
        flags: &DecodingFlags,
        isa_version: IsaVersion,
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
}
