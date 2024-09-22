/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Abi, RegisterDescriptor, RegisterRspCop2, RSP_COP2_REGISTERS};

impl RegisterRspCop2 {
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }

    #[must_use]
    pub fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &RSP_COP2_REGISTERS[*self]
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

impl Default for RegisterRspCop2 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterRspCop2> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterRspCop2) -> &Self::Output {
        &self[index as usize]
    }
}
