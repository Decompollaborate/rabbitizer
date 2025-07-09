/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::HtmlSelectElement;
use yew::events::Event;
use yew::html::Scope;
use yew::{html, Component, Html, TargetCast};

const KEY_THEME: &str = "decompollaborate.disasmdis-web.state.theme";
const KEY_ENDIAN: &str = "decompollaborate.disasmdis-web.state.endian";

pub struct PersistentState {
    pub theme: Theme,
    pub endian: Endian,
}

impl PersistentState {
    pub fn new() -> Self {
        Self {
            theme: Theme::load_storage(),
            endian: Endian::load_storage(),
        }
    }

    pub fn save(&self) {
        let Self { theme, endian } = self;

        theme.save_storage();
        endian.save_storage();
    }
}

pub trait DropdownEnum
where
    Self: Sized + PartialEq + Default + Serialize + for<'de> Deserialize<'de> + 'static,
{
    fn from_id(id: &str) -> Self;
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn array() -> &'static [Self];
    fn storage_key() -> &'static str;

    fn gen_dropdown<F, T, S>(&self, link: &Scope<S>, dropdown_id: &'static str, msgfier: F) -> Html
    where
        F: Fn(Self) -> T + 'static,
        S: Component<Message = T>,
    {
        let onchange = link.batch_callback(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            Some(msgfier(Self::from_id(&select.value())))
        });

        let elements: Vec<Html> = Self::array()
            .iter()
            .map(|x| {
                let selected = x == self;

                html! {
                    <option value={x.id()} {selected}> { {x.name()} } </option>
                }
            })
            .collect();

        html! {
          <select id={dropdown_id} {onchange}>
            { elements }
          </select>
        }
    }

    fn load_storage() -> Self {
        LocalStorage::get(Self::storage_key()).unwrap_or_default()
    }

    fn save_storage(self) {
        LocalStorage::set(Self::storage_key(), self)
            .expect("Failed to save key into LocalStorage.");
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
        &THEMES
    }

    fn storage_key() -> &'static str {
        KEY_THEME
    }
}

static THEMES: [Theme; 6] = [
    Theme::PastelMidnight,
    Theme::Dracula,
    Theme::SolarizedDark,
    Theme::SolarizedLight,
    Theme::Dark,
    Theme::Light,
];

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
pub enum Endian {
    #[default]
    Big,
    Little,
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
        &ENDIANS
    }

    fn storage_key() -> &'static str {
        KEY_ENDIAN
    }
}

impl Endian {
    pub fn word_from_bytes(self, bytes: [u8; 4]) -> u32 {
        match self {
            Self::Big => u32::from_be_bytes(bytes),
            Self::Little => u32::from_le_bytes(bytes),
        }
    }
}

static ENDIANS: [Endian; 2] = [Endian::Big, Endian::Little];
