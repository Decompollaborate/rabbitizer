/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::registers::Gpr;
use crate::traits::Register;
use crate::Abi;
use crate::RegisterDescriptor;

impl Gpr {
    #[must_use]
    pub const fn default() -> Self {
        Self::zero
    }
}

impl Register for Gpr {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::GPR[*self]
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
