/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DefaultLabelDisplay {
    FullExpression,
    Computed,
    Absolute,
}

impl DefaultLabelDisplay {
    #[must_use]
    pub const fn default() -> Self {
        Self::Absolute
    }
}

impl Default for DefaultLabelDisplay {
    fn default() -> Self {
        Self::default()
    }
}
