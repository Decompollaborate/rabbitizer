/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::registers::Cop1Control;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl Cop1Control {
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for Cop1Control {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::COP1CONTROL[*self]
    }
}
