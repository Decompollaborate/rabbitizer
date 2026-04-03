/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(feature = "R5900EE")]
use crate::encoded_field_mask::EncodedFieldMask;
use crate::instr_suffixes::InstrSuffix;
use crate::opcodes::Opcode;

use super::EncodingError;

impl InstrSuffix {
    pub(crate) fn encode_to_bits<'s>(
        self,
        original_suffix_str: &'s str,
        opcode: Opcode,
    ) -> Result<u32, EncodingError<'s>> {
        // suffix_str should include the starting dot
        let Some(_suffix_str) = original_suffix_str.strip_prefix('.') else {
            return Err(EncodingError::MalformedInstrSuffix(
                opcode,
                original_suffix_str,
            ));
        };

        let _val = match self {
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_xyzw => {
                let mut bits = 0;
                const SUFFIXES_COUNT: usize = 4;
                static XYZW_SUFFIX_DATA: [(char, u32); SUFFIXES_COUNT] = [
                    ('x', EncodedFieldMask::r5900ee_xyzw_x.bits()),
                    ('y', EncodedFieldMask::r5900ee_xyzw_y.bits()),
                    ('z', EncodedFieldMask::r5900ee_xyzw_z.bits()),
                    ('w', EncodedFieldMask::r5900ee_xyzw_w.bits()),
                ];
                let mut seen: [bool; SUFFIXES_COUNT] = [false; SUFFIXES_COUNT];

                // Assemblers seem to accept this suffix in any order.
                // Only constraint seems to be each char must not be duplicated
                for character in _suffix_str.chars() {
                    let mut found = false;
                    for (index, (suffix_char, suffix_bits)) in XYZW_SUFFIX_DATA.iter().enumerate() {
                        if character == *suffix_char {
                            if seen[index] {
                                return Err(EncodingError::InvalidDuplicatedSuffix(
                                    opcode,
                                    self,
                                    _suffix_str,
                                    character,
                                ));
                            }
                            bits |= *suffix_bits;
                            seen[index] = true;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return Err(EncodingError::UnrecognizedInstrSuffix(
                            opcode,
                            self,
                            _suffix_str,
                            character,
                        ));
                    }
                }
                bits
            }
        };

        #[allow(unreachable_code)]
        {
            Ok(_val)
        }
    }
}
