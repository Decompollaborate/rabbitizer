/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::AccessType;

impl AccessType {
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
