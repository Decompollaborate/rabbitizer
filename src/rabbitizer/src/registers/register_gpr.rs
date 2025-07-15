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
    fn as_index(&self) -> usize {
        *self as usize
    }

    fn count() -> usize {
        Self::count()
    }

    fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
        Self::try_from_u32(value)
    }

    fn descriptor_array() -> &'static [RegisterDescriptor] {
        &register_descriptors::GPR
    }

    fn is_temp(&self, abi: Abi) -> bool {
        let descriptor = self.get_descriptor();

        if descriptor.either_arg_or_temp() {
            match abi {
                Abi::O32 | Abi::O64 => descriptor.is_temp(),
                Abi::N32 | Abi::N64 | Abi::EABI32 | Abi::EABI64 => false,
            }
        } else {
            descriptor.is_temp()
        }
    }
    fn is_arg(&self, abi: Abi) -> bool {
        let descriptor = self.get_descriptor();

        if descriptor.either_arg_or_temp() {
            match abi {
                Abi::O32 | Abi::O64 => false,
                Abi::N32 | Abi::N64 | Abi::EABI32 | Abi::EABI64 => descriptor.is_arg(),
            }
        } else {
            descriptor.is_arg()
        }
    }
}
