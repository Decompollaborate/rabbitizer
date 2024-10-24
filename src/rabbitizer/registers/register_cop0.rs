/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::Cop0;
use crate::traits::Register;

impl Cop0 {
    #[must_use]
    pub const fn default() -> Self {
        Self::Index
    }
}

impl Register for Cop0 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::COP0[*self]
    }
}
