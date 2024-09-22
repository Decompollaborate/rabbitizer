/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Abi, RegisterDescriptor, RegisterRspCop0, RSP_COP0_REGISTERS};

impl RegisterRspCop0 {
    #[must_use]
    pub const fn default() -> Self {
        Self::SP_MEM_ADDR
    }

    #[must_use]
    pub fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &RSP_COP0_REGISTERS[*self]
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

impl Default for RegisterRspCop0 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterRspCop0> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterRspCop0) -> &Self::Output {
        &self[index as usize]
    }
}
