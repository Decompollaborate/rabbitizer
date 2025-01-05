/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, clap::ValueEnum)]
pub enum Endian {
    #[clap(aliases=&["b", "B"])]
    #[default]
    Big,
    #[clap(aliases=&["l", "L"])]
    Little,
}

impl Endian {
    pub const fn word_from_bytes(self, bytes: [u8; 4]) -> u32 {
        match self {
            Endian::Big => u32::from_be_bytes(bytes),
            Endian::Little => u32::from_le_bytes(bytes),
        }
    }
}
