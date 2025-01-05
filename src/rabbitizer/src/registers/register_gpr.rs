/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::abi::Abi;
use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::Gpr;
use crate::registers_meta::Register;

impl Gpr {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::zero
    }
}

impl Register for Gpr {
    #[must_use]
    fn as_index(&self) -> usize {
        *self as usize
    }

    #[must_use]
    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::GPR[*self]
    }

    #[must_use]
    fn is_temp(&self, abi: Abi) -> bool {
        let descriptor = self.get_descriptor();

        if descriptor.either_arg_or_temp() {
            match abi {
                Abi::O32 => descriptor.is_temp(),
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
                Abi::O32 => false,
                Abi::N32 | Abi::N64 => descriptor.is_arg(),
            }
        } else {
            descriptor.is_arg()
        }
    }
}
