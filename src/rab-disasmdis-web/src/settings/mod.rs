/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::HtmlSelectElement;
use yew::events::Event;
use yew::html::Scope;
use yew::{html, Component, Html, TargetCast};

mod branch_default_label_display;
mod coding_mode;
mod endian;
mod isa_extension;
mod isa_version;
mod theme;
mod vram;

pub use branch_default_label_display::DefaultLabelDisplay;
pub use coding_mode::CodingMode;
pub use endian::Endian;
pub use isa_extension::IsaExtension;
pub use isa_version::IsaVersion;
pub use theme::Theme;
pub use vram::Vram;

pub trait Storagable
where
    Self: Serialize + for<'de> Deserialize<'de>,
{
    fn storage_key() -> &'static str;

    fn load_storage<F>(default: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        LocalStorage::get(Self::storage_key()).unwrap_or_else(|_| default())
    }

    fn save_storage(self) {
        LocalStorage::set(Self::storage_key(), self)
            .expect("Failed to save key into LocalStorage.");
    }
}

pub trait DropdownEnum
where
    Self: Sized + PartialEq + 'static,
{
    fn from_id(id: &str) -> Self;
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn array() -> &'static [Self];

    fn gen_dropdown<F, M, S>(&self, link: &Scope<S>, dropdown_id: &'static str, msgfier: F) -> Html
    where
        F: Fn(Self) -> M + 'static,
        S: Component<Message = M>,
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
          <select class="settings-dropdown" id={dropdown_id} {onchange}>
            { elements }
          </select>
        }
    }
}

pub trait InputStruct
where
    Self: Sized,
{
    fn gen_input<F, T, S>(&self, link: &Scope<S>, input_id: &'static str, msgfier: F) -> Html
    where
        F: Fn(Self) -> T + 'static,
        S: Component<Message = T>;
}
