/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Abi, RegisterDescriptor, RegisterRspVector, RSP_VECTOR_REGISTERS};

impl RegisterRspVector {
    #[must_use]
    pub const fn default() -> Self {
        Self::v0
    }

    #[must_use]
    pub fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &RSP_VECTOR_REGISTERS[*self]
    }

    #[must_use]
    pub fn numeric_reg(&self) -> &'static str {
        self.get_descriptor().numeric_reg()
    }

    #[must_use]
    pub fn named_reg(&self, abi: Abi) -> &'static str {
        self.get_descriptor().named_reg(abi)
    }
}

impl Default for RegisterRspVector {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterRspVector> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterRspVector) -> &Self::Output {
        &self[index as usize]
    }
}
