/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const KEY_THEME: &str = "decompollaborate.disasmdis-web.state.theme";

pub struct PersistentState {
    pub theme: Theme,
}

impl PersistentState {
    pub fn new() -> Self {
        Self {
            theme: LocalStorage::get(KEY_THEME).unwrap_or_else(|_| Theme::default()),
        }
    }

    pub fn save(&self) {
        LocalStorage::set(KEY_THEME, self.theme).expect("Failed to save theme into LocalStorage.");
    }
}

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

impl Theme {
    pub fn from_id(id: &str) -> Self {
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

    pub fn id(self) -> &'static str {
        match self {
            Self::PastelMidnight => "theme-pastel-midnight",
            Self::Dracula => "theme-dracula",
            Self::SolarizedDark => "theme-solarized-dark",
            Self::SolarizedLight => "theme-solarized-light",
            Self::Dark => "theme-dark",
            Self::Light => "theme-light",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::PastelMidnight => "Pastel Midnight",
            Self::Dracula => "Dracula",
            Self::SolarizedDark => "Solarized Dark",
            Self::SolarizedLight => "Solarized Light",
            Self::Dark => "Dark",
            Self::Light => "Light",
        }
    }
}

pub static THEMES: [Theme; 6] = [
    Theme::PastelMidnight,
    Theme::Dracula,
    Theme::SolarizedDark,
    Theme::SolarizedLight,
    Theme::Dark,
    Theme::Light,
];
