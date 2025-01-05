/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;
use core::iter::FusedIterator;

use crate::display_flags::InstructionDisplayFlags;
use crate::instr::Instruction;
use crate::operands::{Operand, OperandDescriptor, OperandDisplay, ValuedOperand, OPERANDS};

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `Opcode::MAX` value, so instead we manually maintain
// this constant.
pub(crate) const OPERAND_COUNT: usize = 121;

pub const OPERAND_COUNT_MAX: usize = 5;

impl Operand {
    #[must_use]
    pub fn get_descriptor(&self) -> &'static OperandDescriptor {
        &OPERANDS[*self]
    }
}

impl Operand {
    pub const fn display<'ins, 'flg, T>(
        &self,
        instr: &'ins Instruction,
        display_flags: &'flg InstructionDisplayFlags,
        imm_override: Option<T>,
    ) -> OperandDisplay<'ins, 'flg, T>
    where
        T: fmt::Display,
    {
        OperandDisplay::new(*self, instr, display_flags, imm_override)
    }
}

impl Operand {
    #[must_use]
    pub(crate) fn to_valued_operand(self, instr: &Instruction) -> ValuedOperand {
        ValuedOperand::from_operand(self, instr)
    }
}

impl Operand {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::ALL_EMPTY
    }

    #[must_use]
    pub(crate) const fn arr0() -> [Self; OPERAND_COUNT_MAX] {
        [Self::default(); OPERAND_COUNT_MAX]
    }
    #[must_use]
    pub(crate) const fn arr1(op0: Self) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr
    }
    #[must_use]
    pub(crate) const fn arr2(op0: Self, op1: Self) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr[1] = op1;
        arr
    }
    #[must_use]
    pub(crate) const fn arr3(op0: Self, op1: Self, op2: Self) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr[1] = op1;
        arr[2] = op2;
        arr
    }
    #[must_use]
    pub(crate) const fn arr4(
        op0: Self,
        op1: Self,
        op2: Self,
        op3: Self,
    ) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr[1] = op1;
        arr[2] = op2;
        arr[3] = op3;
        arr
    }
    #[must_use]
    pub(crate) const fn arr5(
        op0: Self,
        op1: Self,
        op2: Self,
        op3: Self,
        op4: Self,
    ) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr[1] = op1;
        arr[2] = op2;
        arr[3] = op3;
        arr[4] = op4;
        arr
    }
}

impl Default for Operand {
    fn default() -> Self {
        Self::default()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperandIterator<'ins> {
    operands: &'ins [Operand; OPERAND_COUNT_MAX],
    index: usize,
}

impl<'ins> OperandIterator<'ins> {
    pub(crate) const fn new(operands: &'ins [Operand; OPERAND_COUNT_MAX]) -> Self {
        Self { operands, index: 0 }
    }
}

impl<'ins> Iterator for OperandIterator<'ins> {
    type Item = &'ins Operand;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.operands.len() {
            return None;
        }

        let val = &self.operands[self.index];
        if *val == Operand::default() {
            return None;
        }

        self.index += 1;
        Some(val)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.operands.len() - self.index;

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for OperandIterator<'_> {}
impl FusedIterator for OperandIterator<'_> {}

#[cfg(test)]
mod tests {
    use crate::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_addiu_operands() {
        let mut operands = Opcode::core_addiu.get_descriptor().operands_iter();

        assert_eq!(operands.next(), Some(Operand::core_rt).as_ref());
        assert_eq!(operands.next(), Some(Operand::core_rs).as_ref());
        assert_eq!(operands.next(), Some(Operand::core_immediate).as_ref());
        assert_eq!(operands.next(), None);
    }
}
