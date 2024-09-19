/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{IsaExtension, IsaVersion};

impl IsaExtension {
    #[must_use]
    pub const fn default() -> Self {
        Self::NONE
    }

    #[must_use]
    pub const fn isa_version(&self) -> IsaVersion {
        match self {
            IsaExtension::NONE => IsaVersion::MIPS_III,
            IsaExtension::RSP => IsaVersion::MIPS_III,
            IsaExtension::R3000GTE => IsaVersion::MIPS_I,
            IsaExtension::R4000ALLEGREX => IsaVersion::MIPS_III,
            IsaExtension::R5900 => IsaVersion::MIPS_IV,
        }
    }
}

impl Default for IsaExtension {
    fn default() -> Self {
        Self::default()
    }
}
