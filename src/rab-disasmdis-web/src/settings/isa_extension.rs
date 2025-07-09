/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub use rabbitizer::IsaExtension;

use crate::settings::DropdownEnum;

const KEY: &str = "decompollaborate.disasmdis-web.state.isa_extension";

impl DropdownEnum for Option<IsaExtension> {
    fn from_id(id: &str) -> Self {
        match id {
            "None" => None,
            "RSP" => Some(IsaExtension::RSP),
            "R3000 GTE" => Some(IsaExtension::R3000GTE),
            "R4000 ALLEGREX" => Some(IsaExtension::R4000ALLEGREX),
            "R5900 EE" => Some(IsaExtension::R5900EE),
            _ => Self::default(),
        }
    }

    fn id(&self) -> &'static str {
        match self {
            None => "None",
            Some(IsaExtension::RSP) => "RSP",
            Some(IsaExtension::R3000GTE) => "R3000 GTE",
            Some(IsaExtension::R4000ALLEGREX) => "R4000 ALLEGREX",
            Some(IsaExtension::R5900EE) => "R5900 EE",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            None => "None",
            Some(IsaExtension::RSP) => "RSP",
            Some(IsaExtension::R3000GTE) => "R3000 GTE",
            Some(IsaExtension::R4000ALLEGREX) => "R4000 ALLEGREX",
            Some(IsaExtension::R5900EE) => "R5900 EE",
        }
    }

    fn array() -> &'static [Self] {
        &ARR
    }

    fn storage_key() -> &'static str {
        KEY
    }
}

static ARR: [Option<IsaExtension>; 5] = [
    None,
    Some(IsaExtension::RSP),
    Some(IsaExtension::R3000GTE),
    Some(IsaExtension::R4000ALLEGREX),
    Some(IsaExtension::R5900EE),
];
