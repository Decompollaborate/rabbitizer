/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use serde::{Deserialize, Serialize};

use crate::settings::{DropdownEnum, Storagable};

const KEY: &str = "decompollaborate.disasmdis-web.state.theme";

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
pub enum Theme {
    #[default]
    PastelMidnight,
    Dracula,
    SolarizedDark,
    SolarizedLight,
    Dark,
    Light,
}

impl Storagable for Theme {
    fn storage_key() -> &'static str {
        KEY
    }
}

impl DropdownEnum for Theme {
    fn from_id(id: &str) -> Self {
        match id {
            "theme-pastel-midnight" => Self::PastelMidnight,
            "theme-dracula" => Self::Dracula,
            "theme-solarized-dark" => Self::SolarizedDark,
            "theme-solarized-light" => Self::SolarizedLight,
            "theme-dark" => Self::Dark,
            "theme-light" => Self::Light,
            _ => Self::default(),
        }
    }

    fn id(&self) -> &'static str {
        match self {
            Self::PastelMidnight => "theme-pastel-midnight",
            Self::Dracula => "theme-dracula",
            Self::SolarizedDark => "theme-solarized-dark",
            Self::SolarizedLight => "theme-solarized-light",
            Self::Dark => "theme-dark",
            Self::Light => "theme-light",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::PastelMidnight => "Pastel Midnight",
            Self::Dracula => "Dracula",
            Self::SolarizedDark => "Solarized Dark",
            Self::SolarizedLight => "Solarized Light",
            Self::Dark => "Dark",
            Self::Light => "Light",
        }
    }

    fn array() -> &'static [Self] {
        &ARR
    }
}

static ARR: [Theme; 6] = [
    Theme::PastelMidnight,
    Theme::Dracula,
    Theme::SolarizedDark,
    Theme::SolarizedLight,
    Theme::Dark,
    Theme::Light,
];
