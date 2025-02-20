/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::encoded_field_mask::EncodedFieldMask;
use crate::isa::IsaVersion;
use crate::opcodes::{DecodingFlags, Opcode, OpcodeCategory, OpcodeDecoder};
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900ee_normal(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let opcode_category = OpcodeCategory::R5900EE_NORMAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x1E => Opcode::r5900ee_lq,
            0x1F => Opcode::r5900ee_sq,
            0x36 => Opcode::r5900ee_lqc2,
            0x3E => Opcode::r5900ee_sqc2,
            0x00 => {
                return Self::decode_isa_extension_r5900ee_special(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x01 => {
                return Self::decode_isa_extension_r5900ee_regimm(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x10 => {
                return Self::decode_isa_extension_r5900ee_coprocessor0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x11 => {
                return Self::decode_isa_extension_r5900ee_coprocessor1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x12 => {
                return Self::decode_isa_extension_r5900ee_coprocessor2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x1C => {
                return Self::decode_isa_extension_r5900ee_mmi(
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
    pub(crate) const fn decode_isa_extension_r5900ee_special(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R5900EE_SPECIAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0F => Opcode::r5900ee_sync_p,
            0x18 => Opcode::r5900ee_mult,
            0x28 => Opcode::r5900ee_mfsa,
            0x29 => Opcode::r5900ee_mtsa,
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
        .fixups_decode_isa_extension_r5900ee_special(word, flags, isa_version)
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900ee_regimm(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rt;
        let opcode_category = OpcodeCategory::R5900EE_REGIMM;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x18 => Opcode::r5900ee_mtsab,
            0x19 => Opcode::r5900ee_mtsah,
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
    pub(crate) const fn decode_isa_extension_r5900ee_coprocessor0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let _opcode_category = OpcodeCategory::R5900EE_COP0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x8 => {
                return Self::decode_isa_extension_none_coprocessor0_bc0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x10 => {
                return Self::decode_isa_extension_r5900ee_coprocessor0_tlb(
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
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900ee_coprocessor0_tlb(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R5900EE_COP0_TLB;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x38 => Opcode::r5900ee_ei,
            0x39 => Opcode::r5900ee_di,
            _ => {
                return Self::decode_isa_extension_none_coprocessor0_tlb(
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
    pub(crate) const fn decode_isa_extension_r5900ee_coprocessor1(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let _opcode_category = OpcodeCategory::R5900EE_COP1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x8 => {
                return Self::decode_isa_extension_none_coprocessor1_bc1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x10 => {
                return Self::decode_isa_extension_r5900ee_coprocessor1_fpu_s(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x11 => {
                return Self::decode_isa_extension_none_coprocessor1_fpu_d(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x14 => {
                return Self::decode_isa_extension_none_coprocessor1_fpu_w(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x15 => {
                return Self::decode_isa_extension_none_coprocessor1_fpu_l(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => {
                return Self::decode_isa_extension_none_coprocessor1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900ee_coprocessor1_fpu_s(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R5900EE_COP1_FPUS;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x04 => Opcode::r5900ee_c1__sqrt_s,
            0x16 => Opcode::r5900ee_rsqrt_s,
            0x18 => Opcode::r5900ee_adda_s,
            0x19 => Opcode::r5900ee_suba_s,
            0x1A => Opcode::r5900ee_mula_s,
            0x1C => Opcode::r5900ee_madd_s,
            0x1D => Opcode::r5900ee_msub_s,
            0x1E => Opcode::r5900ee_madda_s,
            0x1F => Opcode::r5900ee_msuba_s,
            0x28 => Opcode::r5900ee_max_s,
            0x29 => Opcode::r5900ee_min_s,
            0x34 => Opcode::r5900ee_c_lt_s,
            0x36 => Opcode::r5900ee_c_le_s,
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
    pub(crate) const fn decode_isa_extension_r5900ee_coprocessor2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900ee_cop2_highbit;
        let opcode_category = OpcodeCategory::R5900EE_COP2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0 => {
                return Self::decode_isa_extension_r5900ee_coprocessor2_nohighbit(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x1 => {
                return Self::decode_isa_extension_r5900ee_coprocessor2_special1(
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
    pub(crate) const fn decode_isa_extension_r5900ee_coprocessor2_nohighbit(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900ee_cop2_nohighbit_fmt;
        let opcode_category = OpcodeCategory::R5900EE_COP2_NOHIGHBIT;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x01 => Opcode::r5900ee_qmfc2,
            0x02 => Opcode::r5900ee_cfc2,
            0x05 => Opcode::r5900ee_qmtc2,
            0x06 => Opcode::r5900ee_ctc2,
            0x08 => {
                return Self::decode_isa_extension_r5900ee_coprocessor2_bc2(
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
    pub(crate) const fn decode_isa_extension_r5900ee_coprocessor2_bc2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::bc_fmt;
        let opcode_category = OpcodeCategory::R5900EE_COP2_BC2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900ee_bc2f,
            0x01 => Opcode::r5900ee_bc2t,
            0x02 => Opcode::r5900ee_bc2fl,
            0x03 => Opcode::r5900ee_bc2tl,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900ee_coprocessor2_special1(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R5900EE_COP2_SPECIAL1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900ee_vaddx,
            0x01 => Opcode::r5900ee_vaddy,
            0x02 => Opcode::r5900ee_vaddz,
            0x03 => Opcode::r5900ee_vaddw,
            0x04 => Opcode::r5900ee_vsubx,
            0x05 => Opcode::r5900ee_vsuby,
            0x06 => Opcode::r5900ee_vsubz,
            0x07 => Opcode::r5900ee_vsubw,
            0x08 => Opcode::r5900ee_vmaddx,
            0x09 => Opcode::r5900ee_vmaddy,
            0x0A => Opcode::r5900ee_vmaddz,
            0x0B => Opcode::r5900ee_vmaddw,
            0x0C => Opcode::r5900ee_vmsubx,
            0x0D => Opcode::r5900ee_vmsuby,
            0x0E => Opcode::r5900ee_vmsubz,
            0x0F => Opcode::r5900ee_vmsubw,
            0x10 => Opcode::r5900ee_vmaxx,
            0x11 => Opcode::r5900ee_vmaxy,
            0x12 => Opcode::r5900ee_vmaxz,
            0x13 => Opcode::r5900ee_vmaxw,
            0x14 => Opcode::r5900ee_vminix,
            0x15 => Opcode::r5900ee_vminiy,
            0x16 => Opcode::r5900ee_vminiz,
            0x17 => Opcode::r5900ee_vminiw,
            0x18 => Opcode::r5900ee_vmulx,
            0x19 => Opcode::r5900ee_vmuly,
            0x1A => Opcode::r5900ee_vmulz,
            0x1B => Opcode::r5900ee_vmulw,
            0x1C => Opcode::r5900ee_vmulq,
            0x1D => Opcode::r5900ee_vmaxi,
            0x1E => Opcode::r5900ee_vmuli,
            0x1F => Opcode::r5900ee_vminii,
            0x20 => Opcode::r5900ee_vaddq,
            0x21 => Opcode::r5900ee_vmaddq,
            0x22 => Opcode::r5900ee_vaddi,
            0x23 => Opcode::r5900ee_vmaddi,
            0x24 => Opcode::r5900ee_vsubq,
            0x25 => Opcode::r5900ee_vmsubq,
            0x26 => Opcode::r5900ee_vsubi,
            0x27 => Opcode::r5900ee_vmsubi,
            0x28 => Opcode::r5900ee_vadd,
            0x29 => Opcode::r5900ee_vmadd,
            0x2A => Opcode::r5900ee_vmul,
            0x2B => Opcode::r5900ee_vmax,
            0x2C => Opcode::r5900ee_vsub,
            0x2D => Opcode::r5900ee_vmsub,
            0x2E => Opcode::r5900ee_vopmsub,
            0x2F => Opcode::r5900ee_vmini,
            0x30 => Opcode::r5900ee_viadd,
            0x31 => Opcode::r5900ee_visub,
            0x32 => Opcode::r5900ee_viaddi,
            0x34 => Opcode::r5900ee_viand,
            0x35 => Opcode::r5900ee_vior,
            0x38 => Opcode::r5900ee_vcallms,
            0x39 => Opcode::r5900ee_vcallmsr,
            0x3C => {
                return Self::decode_isa_extension_r5900ee_coprocessor2_special2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3D => {
                return Self::decode_isa_extension_r5900ee_coprocessor2_special2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3E => {
                return Self::decode_isa_extension_r5900ee_coprocessor2_special2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3F => {
                return Self::decode_isa_extension_r5900ee_coprocessor2_special2(
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
    pub(crate) const fn decode_isa_extension_r5900ee_coprocessor2_special2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900ee_fhi_flo;
        let opcode_category = OpcodeCategory::R5900EE_COP2_SPECIAL2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900ee_vaddax,
            0x01 => Opcode::r5900ee_vadday,
            0x02 => Opcode::r5900ee_vaddaz,
            0x03 => Opcode::r5900ee_vaddaw,
            0x40 => Opcode::r5900ee_vsubax,
            0x41 => Opcode::r5900ee_vsubay,
            0x42 => Opcode::r5900ee_vsubaz,
            0x43 => Opcode::r5900ee_vsubaw,
            0x80 => Opcode::r5900ee_vmaddax,
            0x81 => Opcode::r5900ee_vmadday,
            0x82 => Opcode::r5900ee_vmaddaz,
            0x83 => Opcode::r5900ee_vmaddaw,
            0xC0 => Opcode::r5900ee_vmsubax,
            0xC1 => Opcode::r5900ee_vmsubay,
            0xC2 => Opcode::r5900ee_vmsubaz,
            0xC3 => Opcode::r5900ee_vmsubaw,
            0x100 => Opcode::r5900ee_vitof0,
            0x101 => Opcode::r5900ee_vitof4,
            0x102 => Opcode::r5900ee_vitof12,
            0x103 => Opcode::r5900ee_vitof15,
            0x140 => Opcode::r5900ee_vftoi0,
            0x141 => Opcode::r5900ee_vftoi4,
            0x142 => Opcode::r5900ee_vftoi12,
            0x143 => Opcode::r5900ee_vftoi15,
            0x180 => Opcode::r5900ee_vmulax,
            0x181 => Opcode::r5900ee_vmulay,
            0x182 => Opcode::r5900ee_vmulaz,
            0x183 => Opcode::r5900ee_vmulaw,
            0x1C0 => Opcode::r5900ee_vmulaq,
            0x1C1 => Opcode::r5900ee_vabs,
            0x1C2 => Opcode::r5900ee_vmulai,
            0x1C3 => Opcode::r5900ee_vclipw,
            0x200 => Opcode::r5900ee_vaddaq,
            0x201 => Opcode::r5900ee_vmaddaq,
            0x202 => Opcode::r5900ee_vaddai,
            0x203 => Opcode::r5900ee_vmaddai,
            0x240 => Opcode::r5900ee_vsubaq,
            0x241 => Opcode::r5900ee_vmsubaq,
            0x242 => Opcode::r5900ee_vsubai,
            0x243 => Opcode::r5900ee_vmsubai,
            0x280 => Opcode::r5900ee_vadda,
            0x281 => Opcode::r5900ee_vmadda,
            0x282 => Opcode::r5900ee_vmula,
            0x2C0 => Opcode::r5900ee_vsuba,
            0x2C1 => Opcode::r5900ee_vmsuba,
            0x2C2 => Opcode::r5900ee_vopmula,
            0x2C3 => Opcode::r5900ee_vnop,
            0x300 => Opcode::r5900ee_vmove,
            0x301 => Opcode::r5900ee_vmr32,
            0x340 => Opcode::r5900ee_vlqi,
            0x341 => Opcode::r5900ee_vsqi,
            0x342 => Opcode::r5900ee_vlqd,
            0x343 => Opcode::r5900ee_vsqd,
            0x380 => Opcode::r5900ee_vdiv,
            0x381 => Opcode::r5900ee_vsqrt,
            0x382 => Opcode::r5900ee_vrsqrt,
            0x383 => Opcode::r5900ee_vwaitq,
            0x3C0 => Opcode::r5900ee_vmtir,
            0x3C1 => Opcode::r5900ee_vmfir,
            0x400 => Opcode::r5900ee_vrnext,
            0x401 => Opcode::r5900ee_vrget,
            0x402 => Opcode::r5900ee_vrinit,
            0x403 => Opcode::r5900ee_vrxor,
            0x3C2 => {
                return Self::decode_isa_extension_r5900ee_coprocessor2_viwr(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3C3 => {
                return Self::decode_isa_extension_r5900ee_coprocessor2_viwr(
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
    pub(crate) const fn decode_isa_extension_r5900ee_coprocessor2_viwr(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900ee_viwr_fhilo;
        let opcode_category = OpcodeCategory::R5900EE_COP2_VIWR;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0200002 => Opcode::r5900ee_vilwr_w,
            0x0400002 => Opcode::r5900ee_vilwr_z,
            0x0800002 => Opcode::r5900ee_vilwr_y,
            0x1000002 => Opcode::r5900ee_vilwr_x,
            0x0200003 => Opcode::r5900ee_viswr_w,
            0x0400003 => Opcode::r5900ee_viswr_z,
            0x0800003 => Opcode::r5900ee_viswr_y,
            0x1000003 => Opcode::r5900ee_viswr_x,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900ee_mmi(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R5900EE_MMI;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900ee_madd,
            0x01 => Opcode::r5900ee_maddu,
            0x04 => Opcode::r5900ee_plzcw,
            0x10 => Opcode::r5900ee_mfhi1,
            0x11 => Opcode::r5900ee_mthi1,
            0x12 => Opcode::r5900ee_mflo1,
            0x13 => Opcode::r5900ee_mtlo1,
            0x18 => Opcode::r5900ee_mult1,
            0x19 => Opcode::r5900ee_multu1,
            0x1A => Opcode::r5900ee_div1,
            0x1B => Opcode::r5900ee_divu1,
            0x20 => Opcode::r5900ee_madd1,
            0x21 => Opcode::r5900ee_maddu1,
            0x34 => Opcode::r5900ee_psllh,
            0x36 => Opcode::r5900ee_psrlh,
            0x37 => Opcode::r5900ee_psrah,
            0x3C => Opcode::r5900ee_psllw,
            0x3E => Opcode::r5900ee_psrlw,
            0x3F => Opcode::r5900ee_psraw,
            0x08 => {
                return Self::decode_isa_extension_r5900ee_mmi_0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x09 => {
                return Self::decode_isa_extension_r5900ee_mmi_2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x28 => {
                return Self::decode_isa_extension_r5900ee_mmi_1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x29 => {
                return Self::decode_isa_extension_r5900ee_mmi_3(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x30 => {
                return Self::decode_isa_extension_r5900ee_mmi_pmfhl(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x31 => {
                return Self::decode_isa_extension_r5900ee_mmi_pmthl(
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
    pub(crate) const fn decode_isa_extension_r5900ee_mmi_0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900ee_mmi_function;
        let opcode_category = OpcodeCategory::R5900EE_MMI_0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900ee_paddw,
            0x01 => Opcode::r5900ee_psubw,
            0x02 => Opcode::r5900ee_pcgtw,
            0x03 => Opcode::r5900ee_pmaxw,
            0x04 => Opcode::r5900ee_paddh,
            0x05 => Opcode::r5900ee_psubh,
            0x06 => Opcode::r5900ee_pcgth,
            0x07 => Opcode::r5900ee_pmaxh,
            0x08 => Opcode::r5900ee_paddb,
            0x09 => Opcode::r5900ee_psubb,
            0x0A => Opcode::r5900ee_pcgtb,
            0x10 => Opcode::r5900ee_paddsw,
            0x11 => Opcode::r5900ee_psubsw,
            0x12 => Opcode::r5900ee_pextlw,
            0x13 => Opcode::r5900ee_ppacw,
            0x14 => Opcode::r5900ee_paddsh,
            0x15 => Opcode::r5900ee_psubsh,
            0x16 => Opcode::r5900ee_pextlh,
            0x17 => Opcode::r5900ee_ppach,
            0x18 => Opcode::r5900ee_paddsb,
            0x19 => Opcode::r5900ee_psubsb,
            0x1A => Opcode::r5900ee_pextlb,
            0x1B => Opcode::r5900ee_ppacb,
            0x1E => Opcode::r5900ee_pext5,
            0x1F => Opcode::r5900ee_ppac5,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900ee_mmi_1(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900ee_mmi_function;
        let opcode_category = OpcodeCategory::R5900EE_MMI_1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x01 => Opcode::r5900ee_pabsw,
            0x02 => Opcode::r5900ee_pceqw,
            0x03 => Opcode::r5900ee_pminw,
            0x04 => Opcode::r5900ee_padsbh,
            0x05 => Opcode::r5900ee_pabsh,
            0x06 => Opcode::r5900ee_pceqh,
            0x07 => Opcode::r5900ee_pminh,
            0x0A => Opcode::r5900ee_pceqb,
            0x10 => Opcode::r5900ee_padduw,
            0x11 => Opcode::r5900ee_psubuw,
            0x12 => Opcode::r5900ee_pextuw,
            0x14 => Opcode::r5900ee_padduh,
            0x15 => Opcode::r5900ee_psubuh,
            0x16 => Opcode::r5900ee_pextuh,
            0x18 => Opcode::r5900ee_paddub,
            0x19 => Opcode::r5900ee_psubub,
            0x1A => Opcode::r5900ee_pextub,
            0x1B => Opcode::r5900ee_qfsrv,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900ee_mmi_2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900ee_mmi_function;
        let opcode_category = OpcodeCategory::R5900EE_MMI_2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900ee_pmaddw,
            0x02 => Opcode::r5900ee_psllvw,
            0x03 => Opcode::r5900ee_psrlvw,
            0x04 => Opcode::r5900ee_pmsubw,
            0x08 => Opcode::r5900ee_pmfhi,
            0x09 => Opcode::r5900ee_pmflo,
            0x0A => Opcode::r5900ee_pinth,
            0x0C => Opcode::r5900ee_pmultw,
            0x0D => Opcode::r5900ee_pdivw,
            0x0E => Opcode::r5900ee_pcpyld,
            0x10 => Opcode::r5900ee_pmaddh,
            0x11 => Opcode::r5900ee_phmadh,
            0x12 => Opcode::r5900ee_pand,
            0x13 => Opcode::r5900ee_pxor,
            0x14 => Opcode::r5900ee_pmsubh,
            0x15 => Opcode::r5900ee_phmsbh,
            0x1A => Opcode::r5900ee_pexeh,
            0x1B => Opcode::r5900ee_prevh,
            0x1C => Opcode::r5900ee_pmulth,
            0x1D => Opcode::r5900ee_pdivbw,
            0x1E => Opcode::r5900ee_pexew,
            0x1F => Opcode::r5900ee_prot3w,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900ee_mmi_3(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900ee_mmi_function;
        let opcode_category = OpcodeCategory::R5900EE_MMI_3;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900ee_pmadduw,
            0x03 => Opcode::r5900ee_psravw,
            0x08 => Opcode::r5900ee_pmthi,
            0x09 => Opcode::r5900ee_pmtlo,
            0x0A => Opcode::r5900ee_pinteh,
            0x0C => Opcode::r5900ee_pmultuw,
            0x0D => Opcode::r5900ee_pdivuw,
            0x0E => Opcode::r5900ee_pcpyud,
            0x12 => Opcode::r5900ee_por,
            0x13 => Opcode::r5900ee_pnor,
            0x1A => Opcode::r5900ee_pexch,
            0x1B => Opcode::r5900ee_pcpyh,
            0x1E => Opcode::r5900ee_pexcw,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900ee_mmi_pmfhl(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900ee_mmi_function;
        let opcode_category = OpcodeCategory::R5900EE_MMI_PMFHL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900ee_pmfhl_lw,
            0x01 => Opcode::r5900ee_pmfhl_uw,
            0x02 => Opcode::r5900ee_pmfhl_slw,
            0x03 => Opcode::r5900ee_pmfhl_lh,
            0x04 => Opcode::r5900ee_pmfhl_sh,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900ee_mmi_pmthl(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900ee_mmi_function;
        let opcode_category = OpcodeCategory::R5900EE_MMI_PMTHL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900ee_pmthl_lw,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
}
