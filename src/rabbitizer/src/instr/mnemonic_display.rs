/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::display_flags::InstructionDisplayFlags;

use super::Instruction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MnemonicDisplay<'ins, 'flg> {
    instr: &'ins Instruction,
    display_flags: &'flg InstructionDisplayFlags,
}

impl<'ins, 'flg> MnemonicDisplay<'ins, 'flg> {
    pub(crate) const fn new(
        instr: &'ins Instruction,
        display_flags: &'flg InstructionDisplayFlags,
    ) -> Self {
        Self {
            instr,
            display_flags,
        }
    }
}

impl fmt::Display for MnemonicDisplay<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opcode = self.instr.opcode();
        let opcode_descriptor = opcode.get_descriptor();

        write!(f, "{}", opcode_descriptor.name())?;

        if let Some(suffix) = opcode_descriptor.instr_suffix() {
            let suffix_display = suffix.display(self.instr, self.display_flags);
            write!(f, "{}", suffix_display)?;
        }

        Ok(())
    }
}
