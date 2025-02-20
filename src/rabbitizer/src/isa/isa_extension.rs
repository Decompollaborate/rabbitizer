/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::isa::{IsaExtension, IsaVersion};

impl IsaExtension {
    #[must_use]
    pub const fn isa_version(&self) -> IsaVersion {
        match *self {
            Self::RSP => IsaVersion::MIPS_III,
            Self::R3000GTE => IsaVersion::MIPS_I,
            Self::R4000ALLEGREX => IsaVersion::MIPS_III,
            Self::R5900EE => IsaVersion::MIPS_IV,
        }
    }
}
