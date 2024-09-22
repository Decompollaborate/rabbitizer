/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Abi, RegisterDescriptor, RegisterRspGpr, RSP_GPR_REGISTERS};

impl RegisterRspGpr {
    #[must_use]
    pub const fn default() -> Self {
        Self::zero
    }

    #[must_use]
    pub fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &RSP_GPR_REGISTERS[*self]
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

impl Default for RegisterRspGpr {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterRspGpr> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterRspGpr) -> &Self::Output {
        &self[index as usize]
    }
}
