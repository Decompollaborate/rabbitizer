/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use serde::{Deserialize, Serialize};

use crate::settings::{DropdownEnum, Storagable};

const KEY: &str = "decompollaborate.disasmdis-web.state.coding_mode";

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
/// for (en)coding and (de)coding
pub enum CodingMode {
    #[default]
    Decoder,
    Encoder,
}

impl Storagable for CodingMode {
    fn storage_key() -> &'static str {
        KEY
    }
}

impl DropdownEnum for CodingMode {
    fn from_id(id: &str) -> Self {
        match id {
            "decoder" => Self::Decoder,
            "encoder" => Self::Encoder,
            _ => Self::default(),
        }
    }

    fn id(&self) -> &'static str {
        match self {
            Self::Decoder => "decoder",
            Self::Encoder => "encoder",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Decoder => "Hex decoder (disassembler)",
            Self::Encoder => "Instruction encoder (pseudo-assembler)",
        }
    }

    fn array() -> &'static [Self] {
        &ARR
    }
}

static ARR: [CodingMode; 2] = [CodingMode::Decoder, CodingMode::Encoder];
