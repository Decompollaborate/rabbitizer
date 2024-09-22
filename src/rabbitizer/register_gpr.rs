/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Abi, traits::Register, RegisterDescriptor};
use crate::registers::{RegisterGpr, GPR_REGISTERS};

impl RegisterGpr {
    #[must_use]
    pub const fn default() -> Self {
        Self::zero
    }
}

impl Register for RegisterGpr {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &GPR_REGISTERS[*self]
    }

    #[must_use]
    fn is_temp(&self, abi: Abi) -> bool {
        let descriptor = self.get_descriptor();

        if descriptor.either_arg_or_temp() {
            match abi {
                Abi::NUMERIC | Abi::O32 => descriptor.is_temp(),
                Abi::N32 | Abi::N64 => false,
            }
        } else {
            descriptor.is_temp()
        }
    }
    #[must_use]
    fn is_arg(&self, abi: Abi) -> bool {
        let descriptor = self.get_descriptor();

        if descriptor.either_arg_or_temp() {
            match abi {
                Abi::NUMERIC | Abi::O32 => false,
                Abi::N32 | Abi::N64 => descriptor.is_arg(),
            }
        } else {
            descriptor.is_arg()
        }
    }
}

impl Default for RegisterGpr {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterGpr> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterGpr) -> &Self::Output {
        &self[index as usize]
    }
}
