/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Abi, RegisterDescriptor, RegisterR5900VI, R5900_VI_REGISTERS};

impl RegisterR5900VI {
    #[must_use]
    pub const fn default() -> Self {
        Self::vi0
    }

    #[must_use]
    pub fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &R5900_VI_REGISTERS[*self]
    }

    #[must_use]
    pub fn numeric_reg(&self) -> &'static str {
        self.get_descriptor().numeric_reg()
    }

    #[must_use]
    pub fn named_reg(&self, abi: Abi) -> &'static str {
        self.get_descriptor().named_reg(abi)
    }

    #[must_use]
    pub fn is_clobbered_by_func_call(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_clobbered_by_func_call()
    }
    #[must_use]
    pub fn is_reserved(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_reserved()
    }
    #[must_use]
    pub fn is_kernel(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_kernel()
    }
    #[must_use]
    pub fn is_zero(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_zero()
    }
    #[must_use]
    pub fn is_assembler_temp(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_assembler_temp()
    }
    #[must_use]
    pub fn holds_return_value(&self, _abi: Abi) -> bool {
        self.get_descriptor().holds_return_value()
    }
    #[must_use]
    pub fn holds_return_address(&self, _abi: Abi) -> bool {
        self.get_descriptor().holds_return_address()
    }
    #[must_use]
    pub fn is_stack_pointer(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_stack_pointer()
    }
    #[must_use]
    pub fn is_global_pointer(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_global_pointer()
    }
    #[must_use]
    pub fn is_saved(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_saved()
    }
    #[must_use]
    pub fn is_temp(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_temp()
    }
    #[must_use]
    pub fn is_arg(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_arg()
    }
}

impl Default for RegisterR5900VI {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterR5900VI> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterR5900VI) -> &Self::Output {
        &self[index as usize]
    }
}
