/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use serde::{Deserialize, Serialize};

use crate::settings::{DropdownEnum, Storagable};

const KEY: &str = "decompollaborate.disasmdis-web.state.endian";

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
pub enum Endian {
    #[default]
    Big,
    Little,
}

impl Storagable for Endian {
    fn storage_key() -> &'static str {
        KEY
    }
}

impl DropdownEnum for Endian {
    fn from_id(id: &str) -> Self {
        match id {
            "big" => Self::Big,
            "little" => Self::Little,
            _ => Self::default(),
        }
    }

    fn id(&self) -> &'static str {
        match self {
            Self::Big => "big",
            Self::Little => "little",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Big => "Big endian",
            Self::Little => "Little endian",
        }
    }

    fn array() -> &'static [Self] {
        &ARR
    }

    fn label_text() -> &'static str {
        "Endianness:"
    }
    fn dropdown_id() -> &'static str {
        "endian"
    }
}

impl Endian {
    pub fn word_from_bytes(self, bytes: [u8; 4]) -> u32 {
        match self {
            Self::Big => u32::from_be_bytes(bytes),
            Self::Little => u32::from_le_bytes(bytes),
        }
    }
    pub fn bytes_from_word(self, word: u32) -> [u8; 4] {
        match self {
            Self::Big => word.to_be_bytes(),
            Self::Little => word.to_le_bytes(),
        }
    }
}

static ARR: [Endian; 2] = [Endian::Big, Endian::Little];
