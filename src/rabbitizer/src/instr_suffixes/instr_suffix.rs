/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use super::{InstrSuffix, InstrSuffixDescriptor, InstrSuffixDisplay, INSTR_SUFFIXES};

use crate::{Instruction, InstructionDisplayFlags};

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `InstrSuffix::MAX` value, so instead we manually
// maintain this constant.
pub(crate) const INSTR_SUFFIX_COUNT: usize = {
    let mut count = 0;

    if cfg!(feature = "R5900EE") {
        count += 1;
    }

    count
};

impl InstrSuffix {
    #[must_use]
    pub fn get_descriptor(&self) -> &'static InstrSuffixDescriptor {
        &INSTR_SUFFIXES[*self]
    }
}

impl InstrSuffix {
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.get_descriptor().name()
    }
}

impl InstrSuffix {
    pub const fn display<'ins, 'flg>(
        &self,
        instr: &'ins Instruction,
        display_flags: &'flg InstructionDisplayFlags,
    ) -> InstrSuffixDisplay<'ins, 'flg> {
        InstrSuffixDisplay::new(*self, instr, display_flags)
    }
}
