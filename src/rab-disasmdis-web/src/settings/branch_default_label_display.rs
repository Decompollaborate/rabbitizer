/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub use rabbitizer::operands::DefaultLabelDisplay;

use crate::settings::{DropdownEnum, Storagable};

const KEY: &str = "decompollaborate.disasmdis-web.state.branch_default_label_display";

impl Storagable for DefaultLabelDisplay {
    fn storage_key() -> &'static str {
        KEY
    }
}

impl DropdownEnum for DefaultLabelDisplay {
    fn from_id(id: &str) -> Self {
        match id {
            "FullExpression" => DefaultLabelDisplay::FullExpression,
            "Computed" => DefaultLabelDisplay::Computed,
            "Absolute" => DefaultLabelDisplay::Absolute,
            _ => Self::default(),
        }
    }

    fn id(&self) -> &'static str {
        match self {
            DefaultLabelDisplay::FullExpression => "FullExpression",
            DefaultLabelDisplay::Computed => "Computed",
            DefaultLabelDisplay::Absolute => "Absolute",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            DefaultLabelDisplay::FullExpression => "Full expression",
            DefaultLabelDisplay::Computed => "Computed",
            DefaultLabelDisplay::Absolute => "Absolute",
        }
    }

    fn array() -> &'static [Self] {
        &ARR
    }

    fn label_text() -> &'static str {
        "Branch label style:"
    }
    fn dropdown_id() -> &'static str {
        "branch_label"
    }
}

static ARR: [DefaultLabelDisplay; 3] = [
    DefaultLabelDisplay::FullExpression,
    DefaultLabelDisplay::Computed,
    DefaultLabelDisplay::Absolute,
];
