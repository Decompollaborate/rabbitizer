/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::display_flags::DisplayFlags;
use crate::instr::Instruction;
use crate::isa::IsaExtension;
use crate::opcodes::Opcode;

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

        match self.instr.opcode() {
            Opcode::core_break if self.display_flags.sn64_break_fix() => true,
            Opcode::core_trunc_w_s | Opcode::core_cvt_w_s
                if self.instr.isa_extension() == IsaExtension::R5900 =>
            {
                /*
                 * Due to the R5900's FPU (floating point unit) not being
                 * properly complaint, the instruction `cvt.w.s` always behaves
                 * as `trunc.w.s`, because EE can only do round-to-zero.
                 *
                 * Assemblers like modern GAS implemented a workaround for this
                 * issue by decoding `cvt.w.s` as `trunc.w.s`, but other
                 * assemblers just use `trunc.w.s` and `cvt.w.s` as-is.
                 *
                 * Here's some reading about the binutils rationale:
                 * - https://sourceware.org/legacy-ml/binutils/2012-11/msg00360.html
                 * - https://sourceware.org/pipermail/binutils/2013-January/079863.html
                 *
                 * Because of this, building using GAS with the `-march=r5900`
                 * flag produces:
                 * - `trunc.w.s` is built as the cvt.w.s instruction.
                 * - `cvt.w.s` emits an error, complaining as not being
                 *   supported by the processor.
                 *
                 * To ensure the produced disassembly will still match when it
                 * is built with GAS, we decode thse two instructions as
                 * `.word`s.
                 */
                self.display_flags.r5900_modern_gas_instrs_workarounds()
            }
            Opcode::r5900_vclipw => {
                /*
                 * The `vclipw` instruction has variants that are undocumented
                 * (i.e. `vclipw.xy`, `vclipw.z`) and won't assembly assemble
                 * when using GAS.
                 */
                self.display_flags.r5900_modern_gas_instrs_workarounds()
            }
            Opcode::r5900_vsqrt => {
                /*
                 * The `vsqrt` instruction seems to be representable in
                 * multiple ways, but we only disassemble one of them.
                 */
                self.display_flags.r5900_modern_gas_instrs_workarounds()
            }
            _ => false,
        }
    }
}

impl<'ins, 'imm, 'flg> InstructionDisplay<'ins, 'imm, 'flg> {
    fn display_ljust_padding(
        &self,
        f: &mut fmt::Formatter<'_>,
        ljust: u32,
        written_chars: usize,
    ) -> Result<usize, fmt::Error> {
        let padding_len = ljust.saturating_sub(written_chars as u32) as usize;
        let mut new_written_chars = 0;

        if padding_len > 0 {
            write!(f, "{:>width$}", ' ', width = padding_len)?;
            new_written_chars += padding_len;
        }
        // We uncoditionally write a single space after the ljust padding to
        // ensure the opcode and the first operand are not glued together
        write!(f, " ")?;
        new_written_chars += 1;

        Ok(new_written_chars)
    }

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

        self.display_ljust_padding(f, self.display_flags.opcode_ljust(), written_chars)?;

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

    pub(crate) fn display_as_data(&self, f: &mut fmt::Formatter<'_>) -> Result<usize, fmt::Error> {
        let s = ".word";
        let mut written_chars = 0;

        write!(f, "{}", s)?;
        written_chars += s.len();

        written_chars +=
            self.display_ljust_padding(f, self.display_flags.opcode_ljust(), written_chars)?;

        write!(f, "0x{:08X}", self.instr.word())?;
        written_chars += 10;

        Ok(written_chars)
    }
}

impl<'ins, 'imm, 'flg> fmt::Display for InstructionDisplay<'ins, 'imm, 'flg> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.must_disasm_as_data() {
            let written_chars = self.display_as_data(f)?;

            if self.display_flags.unknown_instr_comment() {
                self.display_ljust_padding(f, 40, written_chars)?;

                write!(f, "/* ")?;
                self.display_as_instruction_disassembly(f)?;

                let valid_bits = self.instr.valid_bits().bits();
                write!(
                    f,
                    " / {:08X} <OpcodeCategory: {}>",
                    ((!valid_bits) & self.instr.word()),
                    self.instr.opcode_category().name(),
                )?;

                write!(f, " */")?;
            }

            Ok(())
        } else {
            self.display_as_instruction_disassembly(f)
        }
    }
}
