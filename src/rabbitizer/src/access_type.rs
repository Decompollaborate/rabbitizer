/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub use crate::generated::AccessType;

impl AccessType {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::NONE
    }
}

impl Default for AccessType {
    fn default() -> Self {
        Self::default()
    }
}
