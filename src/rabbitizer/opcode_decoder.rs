/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![allow(clippy::wildcard_enum_match_arm)]

use crate::{DecodingFlags, EncodedFieldMask, IsaExtension, IsaVersion, Opcode, OpcodeCategory};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct OpcodeDecoder {
    pub(crate) opcode: Opcode,
    pub(crate) opcode_category: OpcodeCategory,
    pub(crate) mandatory_bits: EncodedFieldMask,
}

impl OpcodeDecoder {
    #[must_use]
    pub const fn decode(
        word: u32,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
        isa_extension: IsaExtension,
    ) -> Self {
        // TODO
        match isa_extension {
            IsaExtension::NONE => Self::decode_isa_extension_none(word, flags, isa_version),
            IsaExtension::RSP => Self::decode_isa_extension_rsp(word, flags, isa_version),
            IsaExtension::R3000GTE => Self::decode_isa_extension_r3000gte(word, flags, isa_version),
            // IsaExtension::R4000ALLEGREX => Self::decode_isa_extension_r4000allegrex(word, flags, isa_version),
            IsaExtension::R5900 => Self::decode_isa_extension_r5900(word, flags, isa_version),
            _ => Self {
                opcode: Opcode::ALL_INVALID,
                opcode_category: OpcodeCategory::ALL_INVALID,
                mandatory_bits: EncodedFieldMask::empty(),
            },
        }
    }
}

impl OpcodeDecoder {
    // getters and setters

    #[must_use]
    pub const fn opcode(&self) -> Opcode {
        self.opcode
    }

    #[must_use]
    pub const fn opcode_category(&self) -> OpcodeCategory {
        self.opcode_category
    }

    #[must_use]
    pub const fn mandatory_bits(&self) -> EncodedFieldMask {
        self.mandatory_bits
    }
}

impl OpcodeDecoder {
    #[must_use]
    pub const fn is_nop(word: u32) -> bool {
        word == 0
    }
}

// IsaExtension::NONE
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_none(
        word: u32,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let mandatory_bits = mask.mask_value(word);

        match mask.get_shifted(word) {
            0x00 => {
                Self::decode_isa_extension_none_special(word, mandatory_bits, flags, isa_version)
            }
            0x01 => {
                Self::decode_isa_extension_none_regimm(word, mandatory_bits, flags, isa_version)
            }
            0x10 => Self::decode_isa_extension_none_coprocessor0(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            0x11 => Self::decode_isa_extension_none_coprocessor1(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            0x12 => Self::decode_isa_extension_none_coprocessor2(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            _ => Self::decode_isa_extension_none_normal(word, mandatory_bits, flags, isa_version),
        }
    }

    #[must_use]
    pub(crate) const fn fixups_decode_isa_extension_none_normal(
        word: u32,
        mut opcode: Opcode,
        flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Opcode {
        match opcode {
            Opcode::core_beq => {
                if EncodedFieldMask::rt.get_shifted(word) == 0 {
                    if EncodedFieldMask::rs.get_shifted(word) == 0 {
                        if flags
                            .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_b))
                        {
                            opcode = Opcode::core_b;
                        }
                    } else if flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_beqz))
                    {
                        opcode = Opcode::core_beqz;
                    }
                }
            }
            Opcode::core_bne => {
                if EncodedFieldMask::rt.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_bnez))
                {
                    opcode = Opcode::core_bnez;
                }
            }
            _ => {}
        }

        opcode
    }

    #[must_use]
    pub(crate) const fn fixups_decode_isa_extension_none_special(
        word: u32,
        mut opcode: Opcode,
        flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Opcode {
        if Self::is_nop(word) {
            // NOP is special enough, so we don't provide a way to disable it.
            return Opcode::core_nop;
        }

        match opcode {
            Opcode::core_or => {
                if EncodedFieldMask::rt.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_move))
                {
                    opcode = Opcode::core_move;
                }
            }
            Opcode::core_nor => {
                if EncodedFieldMask::rt.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_not))
                {
                    opcode = Opcode::core_not;
                }
            }
            Opcode::core_sub => {
                if EncodedFieldMask::rs.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_neg))
                {
                    opcode = Opcode::core_neg;
                }
            }
            Opcode::core_subu => {
                if EncodedFieldMask::rs.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_negu))
                {
                    opcode = Opcode::core_negu;
                }
            }
            _ => {}
        }

        opcode
    }

    #[must_use]
    pub(crate) const fn fixups_decode_isa_extension_none_regimm(
        word: u32,
        opcode: Opcode,
        flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Opcode {
        match opcode {
            Opcode::core_bgezal => {
                if EncodedFieldMask::rs.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_bal))
                {
                    Opcode::core_bal
                } else {
                    opcode
                }
            }
            _ => opcode,
        }
    }
}

/// IsaExtension::RSP
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_rsp(
        word: u32,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let mandatory_bits = mask.mask_value(word);

        match mask.get_shifted(word) {
            0x00 => {
                Self::decode_isa_extension_rsp_special(word, mandatory_bits, flags, isa_version)
            }
            0x01 => Self::decode_isa_extension_rsp_regimm(word, mandatory_bits, flags, isa_version),
            0x10 => Self::decode_isa_extension_rsp_coprocessor0(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            0x11 => Self::decode_isa_extension_rsp_coprocessor1(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            0x12 => Self::decode_isa_extension_rsp_coprocessor2(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            _ => Self::decode_isa_extension_rsp_normal(word, mandatory_bits, flags, isa_version),
        }
    }

    #[must_use]
    pub(crate) const fn fixups_decode_isa_extension_rsp_normal(
        word: u32,
        mut opcode: Opcode,
        flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Opcode {
        match opcode {
            Opcode::rsp_beq => {
                if EncodedFieldMask::rt.get_shifted(word) == 0 {
                    if EncodedFieldMask::rs.get_shifted(word) == 0 {
                        if flags
                            .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_b))
                        {
                            opcode = Opcode::rsp_b;
                        }
                    } else if flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_beqz))
                    {
                        opcode = Opcode::rsp_beqz;
                    }
                }
            }
            Opcode::rsp_bne => {
                if EncodedFieldMask::rt.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_bnez))
                {
                    opcode = Opcode::rsp_bnez;
                }
            }
            _ => {}
        }

        opcode
    }

    #[must_use]
    pub(crate) const fn fixups_decode_isa_extension_rsp_swc2(
        self,
        word: u32,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        match self.opcode {
            Opcode::rsp_suv => {
                let mask = EncodedFieldMask::rsp_elementlow;
                let mandatory_bits = self.mandatory_bits.union(mask.mask_value(word));
                if mask.get_shifted(word) != 0x00 {
                    Self {
                        opcode: Opcode::rsp_swv,
                        opcode_category: self.opcode_category,
                        mandatory_bits,
                    }
                } else {
                    Self {
                        opcode: self.opcode,
                        opcode_category: self.opcode_category,
                        mandatory_bits,
                    }
                }
            }
            _ => self,
        }
    }

    #[must_use]
    pub(crate) const fn fixups_decode_isa_extension_rsp_special(
        self,
        word: u32,
        flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        if Self::is_nop(word) {
            // NOP is special enough, so we don't provide a way to disable it.
            return Self {
                opcode: Opcode::rsp_nop,
                opcode_category: self.opcode_category,
                mandatory_bits: self.mandatory_bits,
            };
        }

        match self.opcode {
            Opcode::rsp_or => {
                if EncodedFieldMask::rt.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_move))
                {
                    Self {
                        opcode: Opcode::rsp_move,
                        opcode_category: self.opcode_category,
                        mandatory_bits: self.mandatory_bits,
                    }
                } else {
                    self
                }
            }
            Opcode::rsp_nor => {
                if EncodedFieldMask::rt.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_not))
                {
                    Self {
                        opcode: Opcode::rsp_not,
                        opcode_category: self.opcode_category,
                        mandatory_bits: self.mandatory_bits,
                    }
                } else {
                    self
                }
            }
            Opcode::rsp_sub => {
                if EncodedFieldMask::rs.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_neg))
                {
                    Self {
                        opcode: Opcode::rsp_neg,
                        opcode_category: self.opcode_category,
                        mandatory_bits: self.mandatory_bits,
                    }
                } else {
                    self
                }
            }
            Opcode::rsp_subu => {
                if EncodedFieldMask::rs.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_negu))
                {
                    Self {
                        opcode: Opcode::rsp_negu,
                        opcode_category: self.opcode_category,
                        mandatory_bits: self.mandatory_bits,
                    }
                } else {
                    self
                }
            }
            _ => self,
        }
    }

    #[must_use]
    pub(crate) const fn fixups_decode_isa_extension_rsp_regimm(
        self,
        word: u32,
        flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        match self.opcode {
            Opcode::rsp_bgezal => {
                if EncodedFieldMask::rs.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_bal))
                {
                    Self {
                        opcode: Opcode::rsp_bal,
                        opcode_category: self.opcode_category,
                        mandatory_bits: self.mandatory_bits,
                    }
                } else {
                    self
                }
            }
            _ => self,
        }
    }
}

// IsaExtension::R3000GTE
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_r3000gte(
        word: u32,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let mandatory_bits = mask.mask_value(word);

        match mask.get_shifted(word) {
            0x00 => {
                Self::decode_isa_extension_none_special(word, mandatory_bits, flags, isa_version)
            }
            0x01 => {
                Self::decode_isa_extension_none_regimm(word, mandatory_bits, flags, isa_version)
            }
            0x10 => Self::decode_isa_extension_none_coprocessor0(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            0x11 => Self::decode_isa_extension_none_coprocessor1(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            0x12 => Self::decode_isa_extension_r3000gte_coprocessor2(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            _ => Self::decode_isa_extension_none_normal(word, mandatory_bits, flags, isa_version),
        }
    }
}

// IsaExtension::R5900
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900(
        word: u32,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let mandatory_bits = mask.mask_value(word);

        match mask.get_shifted(word) {
            0x00 => {
                Self::decode_isa_extension_r5900_special(word, mandatory_bits, flags, isa_version)
            }
            0x01 => {
                Self::decode_isa_extension_r5900_regimm(word, mandatory_bits, flags, isa_version)
            }
            0x10 => Self::decode_isa_extension_r5900_coprocessor0(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            0x11 => Self::decode_isa_extension_r5900_coprocessor1(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            0x12 => Self::decode_isa_extension_r5900_coprocessor2(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            0x1C => Self::decode_isa_extension_r5900_mmi(word, mandatory_bits, flags, isa_version),
            _ => Self::decode_isa_extension_r5900_normal(word, mandatory_bits, flags, isa_version),
        }
    }

    #[must_use]
    pub(crate) const fn fixups_decode_isa_extension_r5900_special(
        self,
        word: u32,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        match self.opcode {
            Opcode::core_sync | Opcode::r5900_sync_p => {
                let mask = EncodedFieldMask::stype;
                let mandatory_bits = self.mandatory_bits.union(mask.mask_value(word));
                if (mask.get_shifted(word) & 0x10) == 0x10 {
                    Self {
                        opcode: Opcode::r5900_sync_p,
                        opcode_category: self.opcode_category,
                        mandatory_bits,
                    }
                } else {
                    Self {
                        opcode: Opcode::core_sync,
                        opcode_category: self.opcode_category,
                        mandatory_bits,
                    }
                }
            }
            _ => self,
        }
    }
}
