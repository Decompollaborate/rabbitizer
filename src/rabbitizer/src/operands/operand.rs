/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;
use core::iter::FusedIterator;

use crate::display_flags::InstructionDisplayFlags;
use crate::encoded_field_mask::EncodedFieldMask;
use crate::instr::Instruction;
use crate::operands::{Operand, OperandDescriptor, OperandDisplay, ValuedOperand, OPERANDS};
use crate::utils;

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `Opcode::MAX` value, so instead we manually maintain
// this constant.
pub(crate) const OPERAND_COUNT: usize = {
    let mut count = 1;
    count += 26;

    if cfg!(feature = "MIPS_II") {
        count += 0;
    }
    if cfg!(feature = "MIPS_III") {
        count += 0;
    }
    if cfg!(feature = "MIPS_IV") {
        count += 0;
    }

    if cfg!(feature = "RSP") {
        count += 13;
    }
    if cfg!(feature = "R3000GTE") {
        count += 6;
    }
    if cfg!(feature = "R4000ALLEGREX") {
        count += 56;
    }
    if cfg!(feature = "R5900EE") {
        count += 24;
    }

    if cfg!(feature = "RspViceMsp") {
        count += 0;
    }

    count
};

pub const OPERAND_COUNT_MAX: usize = {
    5

    // TODO
    /*
    if cfg!(feature = "R3000GTE") {
        5
    } else if cfg!(feature = "R4000ALLEGREX") {
        4
    } else {
        3
    }
    */
};

impl Operand {
    #[must_use]
    pub fn get_descriptor(&self) -> &'static OperandDescriptor {
        &OPERANDS[*self]
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        self.get_descriptor().name()
    }

    #[must_use]
    pub fn mask(&self) -> EncodedFieldMask {
        self.get_descriptor().mask()
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
    #[cfg(feature = "R4000ALLEGREX")]
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
    #[cfg(feature = "R3000GTE")]
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
    end: usize,
}

impl<'ins> OperandIterator<'ins> {
    pub(crate) fn new(operands: &'ins [Operand; OPERAND_COUNT_MAX]) -> Self {
        let end = utils::array_len_non_default(operands);

        Self {
            operands,
            index: 0,
            end,
        }
    }
}

impl Iterator for OperandIterator<'_> {
    type Item = Operand;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            return None;
        }

        let val = self.operands[self.index];
        if val == Operand::default() {
            return None;
        }

        self.index = self.index.saturating_add(1);
        Some(val)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index = self.index.saturating_add(n);
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end - self.index;

        (remaining, Some(remaining))
    }

    fn count(mut self) -> usize {
        // The size_hint is always accurate.
        let count = self.size_hint().0;
        self.index = self.end;
        count
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl DoubleEndedIterator for OperandIterator<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            return None;
        }

        self.end = self.end.saturating_sub(1);
        let val = self.operands[self.end];
        if val == Operand::default() {
            return None;
        }
        Some(val)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.end = self.end.saturating_sub(n);
        self.next_back()
    }
}

impl ExactSizeIterator for OperandIterator<'_> {}
impl FusedIterator for OperandIterator<'_> {}

#[cfg(test)]
mod tests {
    use crate::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_operand_iter_addiu() {
        let mut operands = Opcode::core_addiu.get_descriptor().operands_iter();

        assert_eq!(operands.size_hint(), (3, Some(3)));
        assert_eq!(operands.next(), Some(Operand::core_rt));
        assert_eq!(operands.next(), Some(Operand::core_rs));
        assert_eq!(operands.next(), Some(Operand::core_imm_i16));
        assert_eq!(operands.next(), None);
        assert_eq!(operands.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_operand_iter_addiu_rev() {
        let mut operands = Opcode::core_addiu.get_descriptor().operands_iter().rev();

        assert_eq!(operands.size_hint(), (3, Some(3)));
        assert_eq!(operands.next(), Some(Operand::core_imm_i16));
        assert_eq!(operands.next(), Some(Operand::core_rs));
        assert_eq!(operands.next(), Some(Operand::core_rt));
        assert_eq!(operands.next(), None);
        assert_eq!(operands.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_operand_iter_addiu_forward_back() {
        let mut operands = Opcode::core_addiu.get_descriptor().operands_iter();

        assert_eq!(operands.size_hint(), (3, Some(3)));
        assert_eq!(operands.next(), Some(Operand::core_rt));
        assert_eq!(operands.size_hint(), (2, Some(2)));
        assert_eq!(operands.next_back(), Some(Operand::core_imm_i16));
        assert_eq!(operands.size_hint(), (1, Some(1)));
        assert_eq!(operands.next(), Some(Operand::core_rs));
        assert_eq!(operands.size_hint(), (0, Some(0)));
        assert_eq!(operands.next(), None);
        assert_eq!(operands.size_hint(), (0, Some(0)));
    }
}
