/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::HtmlSelectElement;
use yew::events::Event;
use yew::html::Scope;
use yew::{html, Component, Html, TargetCast};

mod endian;
mod isa_extension;
mod isa_version;
mod theme;

pub use endian::Endian;
pub use isa_extension::IsaExtension;
pub use isa_version::IsaVersion;
pub use theme::Theme;

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
