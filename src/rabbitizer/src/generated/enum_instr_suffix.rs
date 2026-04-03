/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::instr_suffixes::{InstrSuffixDescriptor, InstrSuffixDisplay, INSTR_SUFFIX_COUNT};
use core::fmt;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum InstrSuffix {
    #[cfg(feature = "R5900EE")]
    r5900ee_xyzw,
}
pub static INSTR_SUFFIXES: [InstrSuffixDescriptor; INSTR_SUFFIX_COUNT] = {
    #[allow(unused_mut)]
    let mut table = [InstrSuffixDescriptor::default(); INSTR_SUFFIX_COUNT];
    #[cfg(feature = "R5900EE")]
    {
        table[InstrSuffix::r5900ee_xyzw as usize] = InstrSuffixDescriptor {
            ..InstrSuffixDescriptor::new("xyzw")
        };
    }
    table
};
impl InstrSuffixDisplay<'_, '_> {
    #[cfg_attr(not(any(feature = "R5900EE",)), expect(unreachable_code))]
    pub(crate) fn display_this_suffix(
        &self,
        _f: &mut fmt::Formatter<'_>, // '
    ) -> fmt::Result {
        match self.instr_suffix() {
            #[cfg(feature = "R5900EE")]
            InstrSuffix::r5900ee_xyzw => InstrSuffixDisplay::display_r5900ee_xyzw(self, _f),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_descriptor_valid() {
        for x in INSTR_SUFFIXES {
            x.check_valid_entry();
        }
    }
}
