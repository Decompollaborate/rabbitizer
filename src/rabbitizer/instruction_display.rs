/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::{DisplayFlags, Instruction};

pub struct InstructionDisplay<'ins, 'imm, 'flg> {
    instr: &'ins Instruction,
    imm_override: Option<&'imm str>,
    display_flags: &'flg DisplayFlags,
}

impl<'ins, 'imm, 'flg> InstructionDisplay<'ins, 'imm, 'flg> {
    pub(crate) const fn new(
        instr: &'ins Instruction,
        imm_override: Option<&'imm str>,
        display_flags: &'flg DisplayFlags,
    ) -> Self {
        Self {
            instr,
            imm_override,
            display_flags,
        }
    }

    pub(crate) fn must_disasm_as_data(&self) -> bool {
        if !self.instr.is_valid() {
            return true;
        }

        // TODO

        false
    }
}

impl<'ins, 'imm, 'flg> InstructionDisplay<'ins, 'imm, 'flg> {
    pub(crate) fn display_as_instruction_disassembly(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let opcode = self.instr.opcode();
        let opcode_name = opcode.name();
        let mut written_chars = 0;

        write!(f, "{}", opcode_name)?;
        written_chars += opcode_name.len();

        // TODO: instruction suffix
        // written_chars += suffix.len();

        if !opcode.has_any_operands() {
            // We do an early return to avoid generating empty space after the
            // opcode name and before the non-existing operands
            return Ok(());
        }

        let padding_len = self.display_flags.opcode_ljust() as i32 - written_chars as i32;
        if padding_len > 0 {
            write!(f, "{:>width$}", ' ', width = padding_len as usize)?;
        }
        // We uncoditionally write a single space after the ljust padding to
        // ensure the opcode and the first operand are not glued together
        write!(f, " ")?;

        for (i, operand) in self.instr.operands_iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(
                f,
                "{}",
                operand.display(self.instr, self.imm_override, self.display_flags)
            )?;
        }

        Ok(())
    }
}

impl<'ins, 'imm, 'flg> fmt::Display for InstructionDisplay<'ins, 'imm, 'flg> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.must_disasm_as_data() {
            todo!()
        }
        self.display_as_instruction_disassembly(f)
    }
}
