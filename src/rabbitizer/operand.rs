/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{
    DisplayFlags, Instruction, Operand, OperandDescriptor, OperandDisplay, ValuedOperand, OPERANDS,
};

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `Opcode::MAX` value, so instead we manually maintain
// this constant.
pub(crate) const OPERAND_COUNT: usize = 123;

pub(crate) const OPERAND_COUNT_MAX: usize = 5;

impl Operand {
    #[must_use]
    pub fn get_descriptor(&self) -> &'static OperandDescriptor {
        &OPERANDS[*self]
    }
}

impl Operand {
    #[must_use]
    pub const fn display<'ins, 'imm, 'flg>(
        &self,
        instr: &'ins Instruction,
        imm_override: Option<&'imm str>,
        display_flags: &'flg DisplayFlags,
    ) -> OperandDisplay<'ins, 'imm, 'flg> {
        OperandDisplay::new(*self, instr, imm_override, display_flags)
    }
}

impl Operand {
    #[must_use]
    pub(crate) fn to_valued_operand(self, instr: &Instruction) -> ValuedOperand {
        ValuedOperand::from_operand(self, instr)
    }
}

impl Operand {
    #[must_use]
    pub const fn default() -> Self {
        Self::ALL_EMPTY
    }

    #[must_use]
    pub const fn arr0() -> [Self; OPERAND_COUNT_MAX] {
        [Self::default(); OPERAND_COUNT_MAX]
    }
    #[must_use]
    pub const fn arr1(op0: Self) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr
    }
    #[must_use]
    pub const fn arr2(op0: Self, op1: Self) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr[1] = op1;
        arr
    }
    #[must_use]
    pub const fn arr3(op0: Self, op1: Self, op2: Self) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr[1] = op1;
        arr[2] = op2;
        arr
    }
    #[must_use]
    pub const fn arr4(op0: Self, op1: Self, op2: Self, op3: Self) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr[1] = op1;
        arr[2] = op2;
        arr[3] = op3;
        arr
    }
    #[must_use]
    pub const fn arr5(
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
}

#[cfg(test)]
mod tests {
    use crate::Opcode;

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
