/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors::COP0_CONTROL;
use crate::registers::Cop0Control;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl Cop0Control {
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for Cop0Control {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &COP0_CONTROL[*self]
    }
}
