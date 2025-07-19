/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub use rabbitizer::IsaVersion;

use crate::settings::{DropdownEnum, Storagable};

const KEY: &str = "decompollaborate.disasmdis-web.state.isa_version";

impl Storagable for IsaVersion {
    fn storage_key() -> &'static str {
        KEY
    }
}

impl DropdownEnum for IsaVersion {
    fn from_id(id: &str) -> Self {
        match id {
            "mips1" => Self::MIPS_I,
            "mips2" => Self::MIPS_II,
            "mips3" => Self::MIPS_III,
            "mips4" => Self::MIPS_IV,
            _ => Self::default(),
        }
    }

    fn id(&self) -> &'static str {
        match self {
            Self::MIPS_I => "mips1",
            Self::MIPS_II => "mips2",
            Self::MIPS_III => "mips3",
            Self::MIPS_IV => "mips4",
            Self::EXTENSION => "??",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::MIPS_I => "MIPS I",
            Self::MIPS_II => "MIPS II",
            Self::MIPS_III => "MIPS III",
            Self::MIPS_IV => "MIPS IV",
            Self::EXTENSION => "??",
        }
    }

    fn array() -> &'static [Self] {
        &ARR
    }

    fn label_text() -> &'static str {
        "ISA version:"
    }
    fn dropdown_id() -> &'static str {
        "isa_version"
    }
}

static ARR: [IsaVersion; 4] = [
    IsaVersion::MIPS_I,
    IsaVersion::MIPS_II,
    IsaVersion::MIPS_III,
    IsaVersion::MIPS_IV,
];
