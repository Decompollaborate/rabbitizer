/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::encoded_field_mask::EncodedFieldMask;
use crate::isa::IsaVersion;
use crate::opcodes::{DecodingFlags, Opcode, OpcodeCategory, OpcodeDecoder};
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_rsp_normal(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let opcode_category = OpcodeCategory::RSP_NORMAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x14 => Opcode::ALL_INVALID,
            0x15 => Opcode::ALL_INVALID,
            0x16 => Opcode::ALL_INVALID,
            0x17 => Opcode::ALL_INVALID,
            0x18 => Opcode::ALL_INVALID,
            0x19 => Opcode::ALL_INVALID,
            0x1A => Opcode::ALL_INVALID,
            0x1B => Opcode::ALL_INVALID,
            0x22 => Opcode::ALL_INVALID,
            0x26 => Opcode::ALL_INVALID,
            0x27 => Opcode::ALL_INVALID,
            0x2A => Opcode::ALL_INVALID,
            0x2C => Opcode::ALL_INVALID,
            0x2D => Opcode::ALL_INVALID,
            0x2E => Opcode::ALL_INVALID,
            0x30 => Opcode::ALL_INVALID,
            0x34 => Opcode::ALL_INVALID,
            0x37 => Opcode::ALL_INVALID,
            0x38 => Opcode::ALL_INVALID,
            0x3C => Opcode::ALL_INVALID,
            0x3F => Opcode::ALL_INVALID,
            0x2F => Opcode::ALL_INVALID,
            0x31 => Opcode::ALL_INVALID,
            0x35 => Opcode::ALL_INVALID,
            0x39 => Opcode::ALL_INVALID,
            0x3D => Opcode::ALL_INVALID,
            0x36 => Opcode::ALL_INVALID,
            0x3E => Opcode::ALL_INVALID,
            0x00 => {
                return Self::decode_isa_extension_rsp_special(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x01 => {
                return Self::decode_isa_extension_rsp_regimm(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x10 => {
                return Self::decode_isa_extension_rsp_coprocessor0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x11 => {
                return Self::decode_isa_extension_rsp_coprocessor1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x12 => {
                return Self::decode_isa_extension_rsp_coprocessor2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x32 => {
                return Self::decode_isa_extension_rsp_lwc2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3A => {
                return Self::decode_isa_extension_rsp_swc2(
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
    pub(crate) const fn decode_isa_extension_rsp_lwc2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rd;
        let opcode_category = OpcodeCategory::RSP_NORMAL_LWC2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::rsp_lbv,
            0x01 => Opcode::rsp_lsv,
            0x02 => Opcode::rsp_llv,
            0x03 => Opcode::rsp_ldv,
            0x04 => Opcode::rsp_lqv,
            0x05 => Opcode::rsp_lrv,
            0x06 => Opcode::rsp_lpv,
            0x07 => Opcode::rsp_luv,
            0x08 => Opcode::rsp_lhv,
            0x09 => Opcode::rsp_lfv,
            0x0B => Opcode::rsp_ltv,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_rsp_swc2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rd;
        let opcode_category = OpcodeCategory::RSP_NORMAL_SWC2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::rsp_sbv,
            0x01 => Opcode::rsp_ssv,
            0x02 => Opcode::rsp_slv,
            0x03 => Opcode::rsp_sdv,
            0x04 => Opcode::rsp_sqv,
            0x05 => Opcode::rsp_srv,
            0x06 => Opcode::rsp_spv,
            0x07 => Opcode::rsp_suv,
            0x08 => Opcode::rsp_shv,
            0x09 => Opcode::rsp_sfv,
            0x0B => Opcode::rsp_stv,
            0xFFFFFF07 => Opcode::rsp_swv,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
        .fixups_decode_isa_extension_rsp_swc2(word, flags, isa_version)
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_rsp_special(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::RSP_SPECIAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x38 => Opcode::ALL_INVALID,
            0x3A => Opcode::ALL_INVALID,
            0x3B => Opcode::ALL_INVALID,
            0x3C => Opcode::ALL_INVALID,
            0x3E => Opcode::ALL_INVALID,
            0x3F => Opcode::ALL_INVALID,
            0x14 => Opcode::ALL_INVALID,
            0x16 => Opcode::ALL_INVALID,
            0x17 => Opcode::ALL_INVALID,
            0x11 => Opcode::ALL_INVALID,
            0x13 => Opcode::ALL_INVALID,
            0x10 => Opcode::ALL_INVALID,
            0x12 => Opcode::ALL_INVALID,
            0x1A => Opcode::ALL_INVALID,
            0x1B => Opcode::ALL_INVALID,
            0x1E => Opcode::ALL_INVALID,
            0x1F => Opcode::ALL_INVALID,
            0x2C => Opcode::ALL_INVALID,
            0x2D => Opcode::ALL_INVALID,
            0x2E => Opcode::ALL_INVALID,
            0x2F => Opcode::ALL_INVALID,
            0x0C => Opcode::ALL_INVALID,
            0x0F => Opcode::ALL_INVALID,
            0x18 => Opcode::ALL_INVALID,
            0x19 => Opcode::ALL_INVALID,
            0x1C => Opcode::ALL_INVALID,
            0x1D => Opcode::ALL_INVALID,
            0x30 => Opcode::ALL_INVALID,
            0x31 => Opcode::ALL_INVALID,
            0x32 => Opcode::ALL_INVALID,
            0x33 => Opcode::ALL_INVALID,
            0x34 => Opcode::ALL_INVALID,
            0x36 => Opcode::ALL_INVALID,
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
    pub(crate) const fn decode_isa_extension_rsp_regimm(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rt;
        let opcode_category = OpcodeCategory::RSP_REGIMM;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x02 => Opcode::ALL_INVALID,
            0x03 => Opcode::ALL_INVALID,
            0x08 => Opcode::ALL_INVALID,
            0x09 => Opcode::ALL_INVALID,
            0x0A => Opcode::ALL_INVALID,
            0x0B => Opcode::ALL_INVALID,
            0x0C => Opcode::ALL_INVALID,
            0x0E => Opcode::ALL_INVALID,
            0x12 => Opcode::ALL_INVALID,
            0x13 => Opcode::ALL_INVALID,
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
    pub(crate) const fn decode_isa_extension_rsp_coprocessor0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let opcode_category = OpcodeCategory::RSP_COP0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::rsp_mfc0,
            0x04 => Opcode::rsp_mtc0,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_rsp_coprocessor1(
        _word: u32,
        mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let opcode_category = OpcodeCategory::RSP_COP1;
        let opcode = Opcode::ALL_INVALID;
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_rsp_coprocessor2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let opcode_category = OpcodeCategory::RSP_COP2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::rsp_mfc2,
            0x04 => Opcode::rsp_mtc2,
            0x02 => Opcode::rsp_cfc2,
            0x06 => Opcode::rsp_ctc2,
            _ => {
                return Self::decode_isa_extension_rsp_coprocessor2_vu(
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
    pub(crate) const fn decode_isa_extension_rsp_coprocessor2_vu(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::RSP_COP2_VU;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        mandatory_bits = mandatory_bits.union(EncodedFieldMask::rsp_vu.mask_value(word));
        if EncodedFieldMask::rsp_vu.get_shifted(word) != 1 {
            return Self {
                opcode: Opcode::ALL_INVALID,
                opcode_category,
                mandatory_bits,
            };
        }
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::rsp_vmulf,
            0x01 => Opcode::rsp_vmulu,
            0x02 => Opcode::rsp_vrndp,
            0x03 => Opcode::rsp_vmulq,
            0x04 => Opcode::rsp_vmudl,
            0x05 => Opcode::rsp_vmudm,
            0x06 => Opcode::rsp_vmudn,
            0x07 => Opcode::rsp_vmudh,
            0x08 => Opcode::rsp_vmacf,
            0x09 => Opcode::rsp_vmacu,
            0x0A => Opcode::rsp_vrndn,
            0x0B => Opcode::rsp_vmacq,
            0x0C => Opcode::rsp_vmadl,
            0x0D => Opcode::rsp_vmadm,
            0x0E => Opcode::rsp_vmadn,
            0x0F => Opcode::rsp_vmadh,
            0x10 => Opcode::rsp_vadd,
            0x11 => Opcode::rsp_vsub,
            0x13 => Opcode::rsp_vabs,
            0x14 => Opcode::rsp_vaddc,
            0x15 => Opcode::rsp_vsubc,
            0x1D => Opcode::rsp_vsar,
            0x28 => Opcode::rsp_vand,
            0x29 => Opcode::rsp_vnand,
            0x2A => Opcode::rsp_vor,
            0x2B => Opcode::rsp_vnor,
            0x2C => Opcode::rsp_vxor,
            0x2D => Opcode::rsp_vnxor,
            0x20 => Opcode::rsp_vlt,
            0x21 => Opcode::rsp_veq,
            0x22 => Opcode::rsp_vne,
            0x23 => Opcode::rsp_vge,
            0x24 => Opcode::rsp_vcl,
            0x25 => Opcode::rsp_vch,
            0x26 => Opcode::rsp_vcr,
            0x27 => Opcode::rsp_vmrg,
            0x30 => Opcode::rsp_vrcp,
            0x31 => Opcode::rsp_vrcpl,
            0x32 => Opcode::rsp_vrcph,
            0x33 => Opcode::rsp_vmov,
            0x34 => Opcode::rsp_vrsq,
            0x35 => Opcode::rsp_vrsql,
            0x36 => Opcode::rsp_vrsqh,
            0x37 => Opcode::rsp_vnop,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
}
