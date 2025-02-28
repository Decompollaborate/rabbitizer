/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![allow(clippy::wildcard_enum_match_arm)]

use crate::encoded_field_mask::EncodedFieldMask;
use crate::isa::{IsaExtension, IsaVersion};
use crate::opcodes::{DecodingFlags, Opcode, OpcodeCategory};

use super::OpcodeValidityGate;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct OpcodeDecoder {
    pub(crate) opcode: Opcode,
    pub(crate) opcode_category: OpcodeCategory,
    pub(crate) mandatory_bits: EncodedFieldMask,
    pub(crate) gated_behind: Option<OpcodeValidityGate>,
}

impl OpcodeDecoder {
    #[must_use]
    pub const fn decode(
        word: u32,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
        isa_extension: Option<IsaExtension>,
    ) -> Self {
        let mandatory_bits = EncodedFieldMask::empty();

        match isa_extension {
            None => {
                Self::decode_isa_extension_none_normal(word, mandatory_bits, flags, isa_version)
            }
            #[cfg(feature = "RSP")]
            Some(IsaExtension::RSP) => {
                Self::decode_isa_extension_rsp_normal(word, mandatory_bits, flags, isa_version)
            }
            #[cfg(feature = "R3000GTE")]
            Some(IsaExtension::R3000GTE) => {
                Self::decode_isa_extension_r3000gte_normal(word, mandatory_bits, flags, isa_version)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Some(IsaExtension::R4000ALLEGREX) => Self::decode_isa_extension_r4000allegrex_normal(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            #[cfg(feature = "R5900EE")]
            Some(IsaExtension::R5900EE) => {
                Self::decode_isa_extension_r5900ee_normal(word, mandatory_bits, flags, isa_version)
            }
            #[cfg(not(any(
                feature = "RSP",
                feature = "R3000GTE",
                feature = "R4000ALLEGREX",
                feature = "R5900EE",
            )))]
            Some(_) => unreachable!(),
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

    #[must_use]
    pub const fn is_valid(&self, flags: &DecodingFlags) -> bool {
        let _avoid_unused_warning = flags;

        if !self.opcode.is_valid() {
            return false;
        }

        match self.gated_behind {
            #[cfg(feature = "RspViceMsp")]
            Some(OpcodeValidityGate::RspViceMsp) => {
                flags.contains(DecodingFlags::gated_rsp_vice_msp)
            }
            None => true,
            #[cfg(not(any(feature = "RspViceMsp",)))]
            Some(_) => unreachable!(),
        }
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
            #[cfg(feature = "MIPS_II")]
            Opcode::core_beql => {
                if EncodedFieldMask::rt.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_beqzl))
                {
                    opcode = Opcode::core_beqzl;
                }
            }
            #[cfg(feature = "MIPS_II")]
            Opcode::core_bnel => {
                if EncodedFieldMask::rt.get_shifted(word) == 0
                    && flags
                        .contains(DecodingFlags::enable_pseudos.union(DecodingFlags::pseudo_bnezl))
                {
                    opcode = Opcode::core_bnezl;
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

#[cfg(feature = "RSP")]
/// IsaExtension::RSP
impl OpcodeDecoder {}

#[cfg(feature = "R3000GTE")]
// IsaExtension::R3000GTE
impl OpcodeDecoder {}

#[cfg(feature = "R4000ALLEGREX")]
// IsaExtension::R4000ALLEGREX
impl OpcodeDecoder {}

#[cfg(feature = "R5900EE")]
// IsaExtension::R5900EE
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn fixups_decode_isa_extension_r5900ee_special(
        self,
        word: u32,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        // TODO: is it possible to express this check with the table system instead?
        match self.opcode {
            Opcode::core_sync | Opcode::r5900ee_sync_p => {
                let mask = EncodedFieldMask::stype;
                let mandatory_bits = self.mandatory_bits.union(mask.mask_value(word));
                if (mask.get_shifted(word) & 0x10) == 0x10 {
                    Self {
                        opcode: Opcode::r5900ee_sync_p,
                        mandatory_bits,
                        ..self
                    }
                } else {
                    Self {
                        opcode: Opcode::core_sync,
                        mandatory_bits,
                        ..self
                    }
                }
            }
            _ => self,
        }
    }
}
