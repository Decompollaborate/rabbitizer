/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::encoded_field_mask::EncodedFieldMask;
use crate::opcodes::OpcodeCategory;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct OpcodeCategoryDescriptor {
    pub(crate) name: &'static str,
    pub(crate) field_mask: EncodedFieldMask,
    pub(crate) trailing_bits: u32,
    pub(crate) handwritten_category: bool,
}

impl OpcodeCategoryDescriptor {
    pub(crate) const fn default() -> Self {
        Self {
            name: "",
            field_mask: EncodedFieldMask::empty(),
            trailing_bits: 0,
            handwritten_category: false,
        }
    }

    pub(crate) const fn new(
        name: &'static str,
        field_mask: EncodedFieldMask,
        trailing_bits: u32,
    ) -> Self {
        Self {
            name,
            field_mask,
            trailing_bits,
            ..Self::default()
        }
    }

    #[cfg(test)]
    pub(crate) const fn check_valid_entry(&self) {
        assert!(
            !self.name.is_empty(),
            "An opcode category must not have an empty name"
        );
        assert!(
            self.field_mask.bits() != self.trailing_bits,
            "The masks from an opcode category must be different"
        );
    }
}

impl OpcodeCategoryDescriptor {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn field_mask(&self) -> EncodedFieldMask {
        self.field_mask
    }
    #[must_use]
    pub const fn trailing_bits(&self) -> u32 {
        self.trailing_bits
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
