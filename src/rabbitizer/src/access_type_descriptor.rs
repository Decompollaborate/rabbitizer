/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::access_type::AccessType;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct AccessTypeDescriptor {
    pub(crate) name: &'static str,
    pub(crate) min_size: Option<u8>,
    pub(crate) min_alignment: Option<u8>,
}

impl AccessTypeDescriptor {
    pub(crate) const fn default() -> Self {
        Self {
            name: "",
            min_size: None,
            min_alignment: None,
        }
    }

    pub(crate) const fn new(name: &'static str) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }

    pub(crate) const fn check_panic(&self) {
        assert!(
            !self.name.is_empty(),
            "An access type must not have an empty name"
        );
    }

    pub(crate) const fn check_panic_chain(self) -> Self {
        self.check_panic();
        self
    }
}

impl AccessTypeDescriptor {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn min_size(&self) -> Option<u8> {
        self.min_size
    }

    #[must_use]
    pub const fn min_alignment(&self) -> Option<u8> {
        self.min_alignment
    }
}

impl Index<AccessType> for [AccessTypeDescriptor] {
    type Output = AccessTypeDescriptor;

    fn index(&self, index: AccessType) -> &Self::Output {
        &self[index as usize]
    }
}
