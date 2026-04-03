/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use super::InstrSuffix;
use crate::display_flags::InstructionDisplayFlags;
use crate::instr::Instruction;

#[cfg(feature = "R5900EE")]
pub(crate) mod instr_suffix_display_r5900ee;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub struct InstrSuffixDisplay<'ins, 'flg> {
    suffix: InstrSuffix,
    instr: &'ins Instruction,
    display_flags: &'flg InstructionDisplayFlags,
}

impl<'ins, 'flg> InstrSuffixDisplay<'ins, 'flg> {
    pub(crate) const fn new(
        suffix: InstrSuffix,
        instr: &'ins Instruction,
        display_flags: &'flg InstructionDisplayFlags,
    ) -> Self {
        Self {
            suffix,
            instr,
            display_flags,
        }
    }

    pub(crate) const fn instr_suffix(&self) -> InstrSuffix {
        self.suffix
    }
}

impl fmt::Display for InstrSuffixDisplay<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_this_suffix(f)
    }
}
