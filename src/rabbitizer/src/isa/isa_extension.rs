/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::isa::{IsaExtension, IsaVersion};

impl IsaExtension {
    #[must_use]
    pub const fn isa_version(&self) -> IsaVersion {
        match *self {
            #[cfg(feature = "RSP")]
            Self::RSP => IsaVersion::MIPS_III,
            #[cfg(feature = "R3000GTE")]
            Self::R3000GTE => IsaVersion::MIPS_I,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::R4000ALLEGREX => IsaVersion::MIPS_III,
            #[cfg(feature = "R5900EE")]
            Self::R5900EE => IsaVersion::MIPS_IV,
        }
    }
}
