/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::opcodes::OpcodeCategory;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct OpcodeCategoryDescriptor {
    pub(crate) name: &'static str,
    pub(crate) handwritten_category: bool,
}

impl OpcodeCategoryDescriptor {
    pub(crate) const fn default() -> Self {
        Self {
            name: "",
            handwritten_category: false,
        }
    }

    pub(crate) const fn new(name: &'static str) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }
}

impl OpcodeCategoryDescriptor {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn handwritten_category(&self) -> bool {
        self.handwritten_category
    }
}

impl Index<OpcodeCategory> for [OpcodeCategoryDescriptor] {
    type Output = OpcodeCategoryDescriptor;

    fn index(&self, index: OpcodeCategory) -> &Self::Output {
        &self[index as usize]
    }
}
