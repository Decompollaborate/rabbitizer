/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::encoded_field_mask::EncodedFieldMask;
use crate::isa::IsaVersion;
use crate::opcodes::{DecodingFlags, Opcode, OpcodeCategory, OpcodeDecoder};
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900_normal(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let opcode_category = OpcodeCategory::R5900_NORMAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x1E => Opcode::r5900_lq,
            0x1F => Opcode::r5900_sq,
            0x36 => Opcode::r5900_lqc2,
            0x3E => Opcode::r5900_sqc2,
            0x00 => {
                return Self::decode_isa_extension_r5900_special(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x01 => {
                return Self::decode_isa_extension_r5900_regimm(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x10 => {
                return Self::decode_isa_extension_r5900_coprocessor0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x11 => {
                return Self::decode_isa_extension_r5900_coprocessor1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x12 => {
                return Self::decode_isa_extension_r5900_coprocessor2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x1C => {
                return Self::decode_isa_extension_r5900_mmi(
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
    pub(crate) const fn decode_isa_extension_r5900_special(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R5900_SPECIAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0F => Opcode::r5900_sync_p,
            0x18 => Opcode::r5900_mult,
            0x28 => Opcode::r5900_mfsa,
            0x29 => Opcode::r5900_mtsa,
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
        .fixups_decode_isa_extension_r5900_special(word, flags, isa_version)
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900_regimm(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rt;
        let opcode_category = OpcodeCategory::R5900_REGIMM;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x18 => Opcode::r5900_mtsab,
            0x19 => Opcode::r5900_mtsah,
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
    pub(crate) const fn decode_isa_extension_r5900_coprocessor0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let _opcode_category = OpcodeCategory::R5900_COP0;
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
                return Self::decode_isa_extension_r5900_coprocessor0_tlb(
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
    pub(crate) const fn decode_isa_extension_r5900_coprocessor0_tlb(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R5900_COP0_TLB;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x38 => Opcode::r5900_ei,
            0x39 => Opcode::r5900_di,
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
    pub(crate) const fn decode_isa_extension_r5900_coprocessor1(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let _opcode_category = OpcodeCategory::R5900_COP1;
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
                return Self::decode_isa_extension_r5900_coprocessor1_fpu_s(
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
    pub(crate) const fn decode_isa_extension_r5900_coprocessor1_fpu_s(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R5900_COP1_FPUS;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x04 => Opcode::r5900_c1__sqrt_s,
            0x16 => Opcode::r5900_rsqrt_s,
            0x18 => Opcode::r5900_adda_s,
            0x19 => Opcode::r5900_suba_s,
            0x1A => Opcode::r5900_mula_s,
            0x1C => Opcode::r5900_madd_s,
            0x1D => Opcode::r5900_msub_s,
            0x1E => Opcode::r5900_madda_s,
            0x1F => Opcode::r5900_msuba_s,
            0x28 => Opcode::r5900_max_s,
            0x29 => Opcode::r5900_min_s,
            0x34 => Opcode::r5900_c_lt_s,
            0x36 => Opcode::r5900_c_le_s,
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
    pub(crate) const fn decode_isa_extension_r5900_coprocessor2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900_cop2_highbit;
        let opcode_category = OpcodeCategory::R5900_COP2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0 => {
                return Self::decode_isa_extension_r5900_coprocessor2_nohighbit(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x1 => {
                return Self::decode_isa_extension_r5900_coprocessor2_special1(
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
    pub(crate) const fn decode_isa_extension_r5900_coprocessor2_nohighbit(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900_cop2_nohighbit_fmt;
        let opcode_category = OpcodeCategory::R5900_COP2_NOHIGHBIT;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x01 => Opcode::r5900_qmfc2,
            0x02 => Opcode::r5900_cfc2,
            0x05 => Opcode::r5900_qmtc2,
            0x06 => Opcode::r5900_ctc2,
            0x08 => {
                return Self::decode_isa_extension_r5900_coprocessor2_bc2(
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
    pub(crate) const fn decode_isa_extension_r5900_coprocessor2_bc2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::bc_fmt;
        let opcode_category = OpcodeCategory::R5900_COP2_BC2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900_bc2f,
            0x01 => Opcode::r5900_bc2t,
            0x02 => Opcode::r5900_bc2fl,
            0x03 => Opcode::r5900_bc2tl,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900_coprocessor2_special1(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R5900_COP2_SPECIAL1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900_vaddx,
            0x01 => Opcode::r5900_vaddy,
            0x02 => Opcode::r5900_vaddz,
            0x03 => Opcode::r5900_vaddw,
            0x04 => Opcode::r5900_vsubx,
            0x05 => Opcode::r5900_vsuby,
            0x06 => Opcode::r5900_vsubz,
            0x07 => Opcode::r5900_vsubw,
            0x08 => Opcode::r5900_vmaddx,
            0x09 => Opcode::r5900_vmaddy,
            0x0A => Opcode::r5900_vmaddz,
            0x0B => Opcode::r5900_vmaddw,
            0x0C => Opcode::r5900_vmsubx,
            0x0D => Opcode::r5900_vmsuby,
            0x0E => Opcode::r5900_vmsubz,
            0x0F => Opcode::r5900_vmsubw,
            0x10 => Opcode::r5900_vmaxx,
            0x11 => Opcode::r5900_vmaxy,
            0x12 => Opcode::r5900_vmaxz,
            0x13 => Opcode::r5900_vmaxw,
            0x14 => Opcode::r5900_vminix,
            0x15 => Opcode::r5900_vminiy,
            0x16 => Opcode::r5900_vminiz,
            0x17 => Opcode::r5900_vminiw,
            0x18 => Opcode::r5900_vmulx,
            0x19 => Opcode::r5900_vmuly,
            0x1A => Opcode::r5900_vmulz,
            0x1B => Opcode::r5900_vmulw,
            0x1C => Opcode::r5900_vmulq,
            0x1D => Opcode::r5900_vmaxi,
            0x1E => Opcode::r5900_vmuli,
            0x1F => Opcode::r5900_vminii,
            0x20 => Opcode::r5900_vaddq,
            0x21 => Opcode::r5900_vmaddq,
            0x22 => Opcode::r5900_vaddi,
            0x23 => Opcode::r5900_vmaddi,
            0x24 => Opcode::r5900_vsubq,
            0x25 => Opcode::r5900_vmsubq,
            0x26 => Opcode::r5900_vsubi,
            0x27 => Opcode::r5900_vmsubi,
            0x28 => Opcode::r5900_vadd,
            0x29 => Opcode::r5900_vmadd,
            0x2A => Opcode::r5900_vmul,
            0x2B => Opcode::r5900_vmax,
            0x2C => Opcode::r5900_vsub,
            0x2D => Opcode::r5900_vmsub,
            0x2E => Opcode::r5900_vopmsub,
            0x2F => Opcode::r5900_vmini,
            0x30 => Opcode::r5900_viadd,
            0x31 => Opcode::r5900_visub,
            0x32 => Opcode::r5900_viaddi,
            0x34 => Opcode::r5900_viand,
            0x35 => Opcode::r5900_vior,
            0x38 => Opcode::r5900_vcallms,
            0x39 => Opcode::r5900_vcallmsr,
            0x3C => {
                return Self::decode_isa_extension_r5900_coprocessor2_special2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3D => {
                return Self::decode_isa_extension_r5900_coprocessor2_special2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3E => {
                return Self::decode_isa_extension_r5900_coprocessor2_special2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3F => {
                return Self::decode_isa_extension_r5900_coprocessor2_special2(
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
    pub(crate) const fn decode_isa_extension_r5900_coprocessor2_special2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900_fhi_flo;
        let opcode_category = OpcodeCategory::R5900_COP2_SPECIAL2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900_vaddax,
            0x01 => Opcode::r5900_vadday,
            0x02 => Opcode::r5900_vaddaz,
            0x03 => Opcode::r5900_vaddaw,
            0x40 => Opcode::r5900_vsubax,
            0x41 => Opcode::r5900_vsubay,
            0x42 => Opcode::r5900_vsubaz,
            0x43 => Opcode::r5900_vsubaw,
            0x80 => Opcode::r5900_vmaddax,
            0x81 => Opcode::r5900_vmadday,
            0x82 => Opcode::r5900_vmaddaz,
            0x83 => Opcode::r5900_vmaddaw,
            0xC0 => Opcode::r5900_vmsubax,
            0xC1 => Opcode::r5900_vmsubay,
            0xC2 => Opcode::r5900_vmsubaz,
            0xC3 => Opcode::r5900_vmsubaw,
            0x100 => Opcode::r5900_vitof0,
            0x101 => Opcode::r5900_vitof4,
            0x102 => Opcode::r5900_vitof12,
            0x103 => Opcode::r5900_vitof15,
            0x140 => Opcode::r5900_vftoi0,
            0x141 => Opcode::r5900_vftoi4,
            0x142 => Opcode::r5900_vftoi12,
            0x143 => Opcode::r5900_vftoi15,
            0x180 => Opcode::r5900_vmulax,
            0x181 => Opcode::r5900_vmulay,
            0x182 => Opcode::r5900_vmulaz,
            0x183 => Opcode::r5900_vmulaw,
            0x1C0 => Opcode::r5900_vmulaq,
            0x1C1 => Opcode::r5900_vabs,
            0x1C2 => Opcode::r5900_vmulai,
            0x1C3 => Opcode::r5900_vclipw,
            0x200 => Opcode::r5900_vaddaq,
            0x201 => Opcode::r5900_vmaddaq,
            0x202 => Opcode::r5900_vaddai,
            0x203 => Opcode::r5900_vmaddai,
            0x240 => Opcode::r5900_vsubaq,
            0x241 => Opcode::r5900_vmsubaq,
            0x242 => Opcode::r5900_vsubai,
            0x243 => Opcode::r5900_vmsubai,
            0x280 => Opcode::r5900_vadda,
            0x281 => Opcode::r5900_vmadda,
            0x282 => Opcode::r5900_vmula,
            0x2C0 => Opcode::r5900_vsuba,
            0x2C1 => Opcode::r5900_vmsuba,
            0x2C2 => Opcode::r5900_vopmula,
            0x2C3 => Opcode::r5900_vnop,
            0x300 => Opcode::r5900_vmove,
            0x301 => Opcode::r5900_vmr32,
            0x340 => Opcode::r5900_vlqi,
            0x341 => Opcode::r5900_vsqi,
            0x342 => Opcode::r5900_vlqd,
            0x343 => Opcode::r5900_vsqd,
            0x380 => Opcode::r5900_vdiv,
            0x381 => Opcode::r5900_vsqrt,
            0x382 => Opcode::r5900_vrsqrt,
            0x383 => Opcode::r5900_vwaitq,
            0x3C0 => Opcode::r5900_vmtir,
            0x3C1 => Opcode::r5900_vmfir,
            0x400 => Opcode::r5900_vrnext,
            0x401 => Opcode::r5900_vrget,
            0x402 => Opcode::r5900_vrinit,
            0x403 => Opcode::r5900_vrxor,
            0x3C2 => {
                return Self::decode_isa_extension_r5900_coprocessor2_viwr(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3C3 => {
                return Self::decode_isa_extension_r5900_coprocessor2_viwr(
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
    pub(crate) const fn decode_isa_extension_r5900_coprocessor2_viwr(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900_viwr_fhilo;
        let opcode_category = OpcodeCategory::R5900_COP2_VIWR;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x0200002 => Opcode::r5900_vilwr_w,
            0x0400002 => Opcode::r5900_vilwr_z,
            0x0800002 => Opcode::r5900_vilwr_y,
            0x1000002 => Opcode::r5900_vilwr_x,
            0x0200003 => Opcode::r5900_viswr_w,
            0x0400003 => Opcode::r5900_viswr_z,
            0x0800003 => Opcode::r5900_viswr_y,
            0x1000003 => Opcode::r5900_viswr_x,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900_mmi(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R5900_MMI;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900_madd,
            0x01 => Opcode::r5900_maddu,
            0x04 => Opcode::r5900_plzcw,
            0x10 => Opcode::r5900_mfhi1,
            0x11 => Opcode::r5900_mthi1,
            0x12 => Opcode::r5900_mflo1,
            0x13 => Opcode::r5900_mtlo1,
            0x18 => Opcode::r5900_mult1,
            0x19 => Opcode::r5900_multu1,
            0x1A => Opcode::r5900_div1,
            0x1B => Opcode::r5900_divu1,
            0x20 => Opcode::r5900_madd1,
            0x21 => Opcode::r5900_maddu1,
            0x34 => Opcode::r5900_psllh,
            0x36 => Opcode::r5900_psrlh,
            0x37 => Opcode::r5900_psrah,
            0x3C => Opcode::r5900_psllw,
            0x3E => Opcode::r5900_psrlw,
            0x3F => Opcode::r5900_psraw,
            0x08 => {
                return Self::decode_isa_extension_r5900_mmi_0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x09 => {
                return Self::decode_isa_extension_r5900_mmi_2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x28 => {
                return Self::decode_isa_extension_r5900_mmi_1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x29 => {
                return Self::decode_isa_extension_r5900_mmi_3(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x30 => {
                return Self::decode_isa_extension_r5900_mmi_pmfhl(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x31 => {
                return Self::decode_isa_extension_r5900_mmi_pmthl(
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
    pub(crate) const fn decode_isa_extension_r5900_mmi_0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900_mmi_function;
        let opcode_category = OpcodeCategory::R5900_MMI_0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900_paddw,
            0x01 => Opcode::r5900_psubw,
            0x02 => Opcode::r5900_pcgtw,
            0x03 => Opcode::r5900_pmaxw,
            0x04 => Opcode::r5900_paddh,
            0x05 => Opcode::r5900_psubh,
            0x06 => Opcode::r5900_pcgth,
            0x07 => Opcode::r5900_pmaxh,
            0x08 => Opcode::r5900_paddb,
            0x09 => Opcode::r5900_psubb,
            0x0A => Opcode::r5900_pcgtb,
            0x10 => Opcode::r5900_paddsw,
            0x11 => Opcode::r5900_psubsw,
            0x12 => Opcode::r5900_pextlw,
            0x13 => Opcode::r5900_ppacw,
            0x14 => Opcode::r5900_paddsh,
            0x15 => Opcode::r5900_psubsh,
            0x16 => Opcode::r5900_pextlh,
            0x17 => Opcode::r5900_ppach,
            0x18 => Opcode::r5900_paddsb,
            0x19 => Opcode::r5900_psubsb,
            0x1A => Opcode::r5900_pextlb,
            0x1B => Opcode::r5900_ppacb,
            0x1E => Opcode::r5900_pext5,
            0x1F => Opcode::r5900_ppac5,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900_mmi_1(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900_mmi_function;
        let opcode_category = OpcodeCategory::R5900_MMI_1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x01 => Opcode::r5900_pabsw,
            0x02 => Opcode::r5900_pceqw,
            0x03 => Opcode::r5900_pminw,
            0x04 => Opcode::r5900_padsbh,
            0x05 => Opcode::r5900_pabsh,
            0x06 => Opcode::r5900_pceqh,
            0x07 => Opcode::r5900_pminh,
            0x0A => Opcode::r5900_pceqb,
            0x10 => Opcode::r5900_padduw,
            0x11 => Opcode::r5900_psubuw,
            0x12 => Opcode::r5900_pextuw,
            0x14 => Opcode::r5900_padduh,
            0x15 => Opcode::r5900_psubuh,
            0x16 => Opcode::r5900_pextuh,
            0x18 => Opcode::r5900_paddub,
            0x19 => Opcode::r5900_psubub,
            0x1A => Opcode::r5900_pextub,
            0x1B => Opcode::r5900_qfsrv,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900_mmi_2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900_mmi_function;
        let opcode_category = OpcodeCategory::R5900_MMI_2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900_pmaddw,
            0x02 => Opcode::r5900_psllvw,
            0x03 => Opcode::r5900_psrlvw,
            0x04 => Opcode::r5900_pmsubw,
            0x08 => Opcode::r5900_pmfhi,
            0x09 => Opcode::r5900_pmflo,
            0x0A => Opcode::r5900_pinth,
            0x0C => Opcode::r5900_pmultw,
            0x0D => Opcode::r5900_pdivw,
            0x0E => Opcode::r5900_pcpyld,
            0x10 => Opcode::r5900_pmaddh,
            0x11 => Opcode::r5900_phmadh,
            0x12 => Opcode::r5900_pand,
            0x13 => Opcode::r5900_pxor,
            0x14 => Opcode::r5900_pmsubh,
            0x15 => Opcode::r5900_phmsbh,
            0x1A => Opcode::r5900_pexeh,
            0x1B => Opcode::r5900_prevh,
            0x1C => Opcode::r5900_pmulth,
            0x1D => Opcode::r5900_pdivbw,
            0x1E => Opcode::r5900_pexew,
            0x1F => Opcode::r5900_prot3w,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900_mmi_3(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900_mmi_function;
        let opcode_category = OpcodeCategory::R5900_MMI_3;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900_pmadduw,
            0x03 => Opcode::r5900_psravw,
            0x08 => Opcode::r5900_pmthi,
            0x09 => Opcode::r5900_pmtlo,
            0x0A => Opcode::r5900_pinteh,
            0x0C => Opcode::r5900_pmultuw,
            0x0D => Opcode::r5900_pdivuw,
            0x0E => Opcode::r5900_pcpyud,
            0x12 => Opcode::r5900_por,
            0x13 => Opcode::r5900_pnor,
            0x1A => Opcode::r5900_pexch,
            0x1B => Opcode::r5900_pcpyh,
            0x1E => Opcode::r5900_pexcw,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900_mmi_pmfhl(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900_mmi_function;
        let opcode_category = OpcodeCategory::R5900_MMI_PMFHL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900_pmfhl_lw,
            0x01 => Opcode::r5900_pmfhl_uw,
            0x02 => Opcode::r5900_pmfhl_slw,
            0x03 => Opcode::r5900_pmfhl_lh,
            0x04 => Opcode::r5900_pmfhl_sh,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900_mmi_pmthl(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::r5900_mmi_function;
        let opcode_category = OpcodeCategory::R5900_MMI_PMTHL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r5900_pmthl_lw,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
}
