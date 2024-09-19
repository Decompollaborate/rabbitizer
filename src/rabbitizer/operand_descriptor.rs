/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{EncodedFieldMask, Operand};

// OperandDescriptor

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash, Default)]
pub struct OperandDescriptor<'a> {
    pub(crate) name: &'a str,
    pub(crate) mask: EncodedFieldMask,
}

impl<'a> OperandDescriptor<'a> {
    pub(crate) const fn default() -> Self {
        Self {
            name: "",
            mask: EncodedFieldMask::default(),
        }
    }

    pub(crate) const fn new(name: &'a str, mask: EncodedFieldMask) -> Self {
        Self { name, mask }
    }
}

impl<'a> OperandDescriptor<'a> {
    #[must_use]
    pub const fn name(&self) -> &'a str {
        &self.name
    }

    #[must_use]
    pub const fn mask(&self) -> &EncodedFieldMask {
        &self.mask
    }
}

impl Index<Operand> for [OperandDescriptor<'static>] {
    type Output = OperandDescriptor<'static>;

    fn index(&self, index: Operand) -> &Self::Output {
        &self[index as usize]
    }
}
