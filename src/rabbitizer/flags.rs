/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct DecodingFlags: u32 {
        /// Produce pseudo instructions (like `move` or `b`) whenever those may match the desired original instruction.
        ///
        /// Turning off this flag disables all the other pseudos.
        const enable_pseudos = 1 << 0;
        const pseudo_move = 1 << 1; // TODO: consider removing
        const pseudo_beqz = 1 << 2;
        const pseudo_bnez = 1 << 3;
        const pseudo_b = 1 << 4;
        const pseudo_bal = 1 << 5;
        const pseudo_not = 1 << 6;
        const pseudo_neg = 1 << 7;
        const pseudo_negu = 1 << 8;

        const sn64_div_fix = 1 << 9;
        const gnu_mode = 1 << 10;
    }
}

impl DecodingFlags {
    pub const fn default() -> Self {
        Self::enable_pseudos
            .union(Self::pseudo_beqz)
            .union(Self::pseudo_bnez)
            .union(Self::pseudo_b)
            .union(Self::pseudo_bal)
            .union(Self::pseudo_not)
            .union(Self::pseudo_neg)
            .union(Self::pseudo_negu)
            .union(Self::gnu_mode)
    }
}

impl Default for DecodingFlags {
    fn default() -> Self {
        Self::default()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstructionFlags {
    decoding_flags: DecodingFlags,
}

impl InstructionFlags {
    pub const fn default() -> Self {
        Self {
            decoding_flags: DecodingFlags::default(),
        }
    }
}

impl InstructionFlags {
    pub const fn decoding_flags(&self) -> &DecodingFlags {
        &self.decoding_flags
    }

    pub fn decoding_flags_mut(&mut self) -> &mut DecodingFlags {
        &mut self.decoding_flags
    }
}

impl Default for InstructionFlags {
    fn default() -> Self {
        Self::default()
    }
}
