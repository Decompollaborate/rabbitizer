/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::encoded_field_mask::EncodedFieldMask;
use crate::operands::Operand;

/// Describes properties of a given [`Operand`].
///
/// [`Operand`]: crate::operands::Operand
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct OperandDescriptor {
    pub(crate) name: &'static str,
    pub(crate) mask: EncodedFieldMask,
}

impl OperandDescriptor {
    pub(crate) const fn default() -> Self {
        Self {
            name: "",
            mask: EncodedFieldMask::default(),
        }
    }

    pub(crate) const fn new(name: &'static str, mask: EncodedFieldMask) -> Self {
        Self { name, mask }
    }

    #[cfg(test)]
    pub(crate) const fn check_valid_entry(&self) {
        assert!(
            !self.name.is_empty(),
            "An operand must not have an empty name"
        );
    }
}

impl OperandDescriptor {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn mask(&self) -> EncodedFieldMask {
        self.mask
    }
}

impl Index<Operand> for [OperandDescriptor] {
    type Output = OperandDescriptor;

    fn index(&self, index: Operand) -> &Self::Output {
        &self[index as usize]
    }
}
