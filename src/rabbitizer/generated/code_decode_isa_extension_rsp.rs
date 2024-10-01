/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{DecodingFlags, EncodedFieldMask, IsaVersion, Opcode, OpcodeCategory, OpcodeDecoder};
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_rsp_normal(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::RSP_NORMAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x02 => opcode = Opcode::rsp_j,
            0x03 => opcode = Opcode::rsp_jal,
            0x04 => opcode = Opcode::rsp_beq,
            0x05 => opcode = Opcode::rsp_bne,
            0x06 => opcode = Opcode::rsp_blez,
            0x07 => opcode = Opcode::rsp_bgtz,
            0x08 => opcode = Opcode::rsp_addi,
            0x09 => opcode = Opcode::rsp_addiu,
            0x0A => opcode = Opcode::rsp_slti,
            0x0B => opcode = Opcode::rsp_sltiu,
            0x0C => opcode = Opcode::rsp_andi,
            0x0D => opcode = Opcode::rsp_ori,
            0x0E => opcode = Opcode::rsp_xori,
            0x0F => opcode = Opcode::rsp_lui,
            0x20 => opcode = Opcode::rsp_lb,
            0x21 => opcode = Opcode::rsp_lh,
            0x23 => opcode = Opcode::rsp_lw,
            0x24 => opcode = Opcode::rsp_lbu,
            0x25 => opcode = Opcode::rsp_lhu,
            0x28 => opcode = Opcode::rsp_sb,
            0x29 => opcode = Opcode::rsp_sh,
            0x2B => opcode = Opcode::rsp_sw,
            0x33 => opcode = Opcode::rsp_pref,

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
            _ => {}
        }
        Self {
            opcode: Self::fixups_decode_isa_extension_rsp_normal(word, opcode, flags, isa_version),
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
        let opcode;
        let opcode_category = OpcodeCategory::RSP_NORMAL_LWC2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 => opcode = Opcode::rsp_lbv,
            0x01 => opcode = Opcode::rsp_lsv,
            0x02 => opcode = Opcode::rsp_llv,
            0x03 => opcode = Opcode::rsp_ldv,
            0x04 => opcode = Opcode::rsp_lqv,
            0x05 => opcode = Opcode::rsp_lrv,
            0x06 => opcode = Opcode::rsp_lpv,
            0x07 => opcode = Opcode::rsp_luv,
            0x08 => opcode = Opcode::rsp_lhv,
            0x09 => opcode = Opcode::rsp_lfv,
            0x0B => opcode = Opcode::rsp_ltv,
            _ => opcode = Opcode::ALL_INVALID,
        }
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
        let opcode;
        let opcode_category = OpcodeCategory::RSP_NORMAL_SWC2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 => opcode = Opcode::rsp_sbv,
            0x01 => opcode = Opcode::rsp_ssv,
            0x02 => opcode = Opcode::rsp_slv,
            0x03 => opcode = Opcode::rsp_sdv,
            0x04 => opcode = Opcode::rsp_sqv,
            0x05 => opcode = Opcode::rsp_srv,
            0x06 => opcode = Opcode::rsp_spv,
            0x07 => opcode = Opcode::rsp_suv,
            0x08 => opcode = Opcode::rsp_shv,
            0x09 => opcode = Opcode::rsp_sfv,
            0x0B => opcode = Opcode::rsp_stv,
            0xFFFFFF07 => opcode = Opcode::rsp_swv,
            _ => opcode = Opcode::ALL_INVALID,
        }
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
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::RSP_SPECIAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 => opcode = Opcode::rsp_sll,
            0x02 => opcode = Opcode::rsp_srl,
            0x03 => opcode = Opcode::rsp_sra,
            0x04 => opcode = Opcode::rsp_sllv,
            0x06 => opcode = Opcode::rsp_srlv,
            0x07 => opcode = Opcode::rsp_srav,
            0x08 => opcode = Opcode::rsp_jr,
            0x09 => opcode = Opcode::rsp_jalr,
            0x0A => opcode = Opcode::rsp_movz,
            0x0B => opcode = Opcode::rsp_movn,
            0x20 => opcode = Opcode::rsp_add,
            0x21 => opcode = Opcode::rsp_addu,
            0x22 => opcode = Opcode::rsp_sub,
            0x23 => opcode = Opcode::rsp_subu,
            0x24 => opcode = Opcode::rsp_and,
            0x25 => opcode = Opcode::rsp_or,
            0x26 => opcode = Opcode::rsp_xor,
            0x27 => opcode = Opcode::rsp_nor,
            0x2A => opcode = Opcode::rsp_slt,
            0x2B => opcode = Opcode::rsp_sltu,
            0x0D => opcode = Opcode::rsp_break,

            _ => {}
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
        .fixups_decode_isa_extension_rsp_special(word, flags, isa_version)
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_rsp_regimm(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rt;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::RSP_REGIMM;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 => opcode = Opcode::rsp_bltz,
            0x01 => opcode = Opcode::rsp_bgez,
            0x10 => opcode = Opcode::rsp_bltzal,
            0x11 => opcode = Opcode::rsp_bgezal,

            _ => {}
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
        .fixups_decode_isa_extension_rsp_regimm(word, flags, isa_version)
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_rsp_coprocessor0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let opcode;
        let opcode_category = OpcodeCategory::RSP_COP0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 => opcode = Opcode::rsp_mfc0,
            0x04 => opcode = Opcode::rsp_mtc0,
            _ => opcode = Opcode::ALL_INVALID,
        }
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
        let opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::RSP_COP1;
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
        let opcode;
        let opcode_category = OpcodeCategory::RSP_COP2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 => opcode = Opcode::rsp_mfc2,
            0x04 => opcode = Opcode::rsp_mtc2,
            0x02 => opcode = Opcode::rsp_cfc2,
            0x06 => opcode = Opcode::rsp_ctc2,
            _ => {
                return Self::decode_isa_extension_rsp_coprocessor2_vu(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        }
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
        let opcode;
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
        match mask.get_shifted(word) {
            0x00 => opcode = Opcode::rsp_vmulf,
            0x01 => opcode = Opcode::rsp_vmulu,
            0x02 => opcode = Opcode::rsp_vrndp,
            0x03 => opcode = Opcode::rsp_vmulq,
            0x04 => opcode = Opcode::rsp_vmudl,
            0x05 => opcode = Opcode::rsp_vmudm,
            0x06 => opcode = Opcode::rsp_vmudn,
            0x07 => opcode = Opcode::rsp_vmudh,
            0x08 => opcode = Opcode::rsp_vmacf,
            0x09 => opcode = Opcode::rsp_vmacu,
            0x0A => opcode = Opcode::rsp_vrndn,
            0x0B => opcode = Opcode::rsp_vmacq,
            0x0C => opcode = Opcode::rsp_vmadl,
            0x0D => opcode = Opcode::rsp_vmadm,
            0x0E => opcode = Opcode::rsp_vmadn,
            0x0F => opcode = Opcode::rsp_vmadh,
            0x10 => opcode = Opcode::rsp_vadd,
            0x11 => opcode = Opcode::rsp_vsub,
            0x13 => opcode = Opcode::rsp_vabs,
            0x14 => opcode = Opcode::rsp_vaddc,
            0x15 => opcode = Opcode::rsp_vsubc,
            0x1D => opcode = Opcode::rsp_vsar,
            0x28 => opcode = Opcode::rsp_vand,
            0x29 => opcode = Opcode::rsp_vnand,
            0x2A => opcode = Opcode::rsp_vor,
            0x2B => opcode = Opcode::rsp_vnor,
            0x2C => opcode = Opcode::rsp_vxor,
            0x2D => opcode = Opcode::rsp_vnxor,
            0x20 => opcode = Opcode::rsp_vlt,
            0x21 => opcode = Opcode::rsp_veq,
            0x22 => opcode = Opcode::rsp_vne,
            0x23 => opcode = Opcode::rsp_vge,
            0x24 => opcode = Opcode::rsp_vcl,
            0x25 => opcode = Opcode::rsp_vch,
            0x26 => opcode = Opcode::rsp_vcr,
            0x27 => opcode = Opcode::rsp_vmrg,
            0x30 => opcode = Opcode::rsp_vrcp,
            0x31 => opcode = Opcode::rsp_vrcpl,
            0x32 => opcode = Opcode::rsp_vrcph,
            0x33 => opcode = Opcode::rsp_vmov,
            0x34 => opcode = Opcode::rsp_vrsq,
            0x35 => opcode = Opcode::rsp_vrsql,
            0x36 => opcode = Opcode::rsp_vrsqh,
            0x37 => opcode = Opcode::rsp_vnop,
            _ => opcode = Opcode::ALL_INVALID,
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
}
