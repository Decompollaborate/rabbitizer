/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{Operand, OperandDescriptor, OPERANDS};

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `Opcode::MAX` value, so instead we manually maintain
// this constant.
pub(crate) const OPERAND_COUNT: usize = 134;

pub(crate) const OPERAND_COUNT_MAX: usize = 5;

impl<'a> Operand {
    #[must_use]
    pub fn get_descriptor(&self) -> &'a OperandDescriptor {
        &OPERANDS[*self]
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

pub struct OperandIterator<'a> {
    operands: &'a [Operand; OPERAND_COUNT_MAX],
    index: usize,
}

impl<'a> OperandIterator<'a> {
    pub(crate) const fn new(operands: &'a [Operand; OPERAND_COUNT_MAX]) -> Self {
        Self { operands, index: 0 }
    }
}

impl<'a> Iterator for OperandIterator<'a> {
    type Item = &'a Operand;

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
