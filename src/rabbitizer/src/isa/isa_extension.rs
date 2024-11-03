/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::isa::{IsaExtension, IsaVersion};

impl IsaExtension {
    /// Returns a default value.
    ///
    /// This defaults to NO extension.
    #[must_use]
    pub const fn default() -> Self {
        Self::NONE
    }

    #[must_use]
    pub const fn isa_version(&self) -> IsaVersion {
        match *self {
            Self::NONE => IsaVersion::MIPS_III,
            Self::RSP => IsaVersion::MIPS_III,
            Self::R3000GTE => IsaVersion::MIPS_I,
            Self::R4000ALLEGREX => IsaVersion::MIPS_III,
            Self::R5900 => IsaVersion::MIPS_IV,
        }
    }
}

impl Default for IsaExtension {
    fn default() -> Self {
        Self::default()
    }
}
