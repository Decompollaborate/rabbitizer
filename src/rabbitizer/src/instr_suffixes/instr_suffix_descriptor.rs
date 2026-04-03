/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use super::InstrSuffix;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstrSuffixDescriptor {
    pub(crate) name: &'static str,
}

impl InstrSuffixDescriptor {
    pub(crate) const fn default() -> Self {
        Self { name: "" }
    }

    #[cfg(feature = "R5900EE")]
    pub(crate) const fn new(name: &'static str) -> Self {
        Self { name }
    }

    #[cfg(test)]
    pub(crate) const fn check_valid_entry(&self) {}
}

impl InstrSuffixDescriptor {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }
}

impl Index<InstrSuffix> for [InstrSuffixDescriptor] {
    type Output = InstrSuffixDescriptor;

    fn index(&self, index: InstrSuffix) -> &Self::Output {
        &self[index as usize]
    }
}
