/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use web_sys::HtmlSelectElement;
use yew::events::Event;
use yew::html::Scope;
use yew::{html, Component, Html, TargetCast};

pub use rabbitizer::Vram;

use crate::settings::{InputStruct, Storagable};

const KEY: &str = "decompollaborate.disasmdis-web.state.vram";

impl Storagable for Vram {
    fn storage_key() -> &'static str {
        KEY
    }
}

impl InputStruct for Vram {
    fn gen_input<F, T, S>(&self, link: &Scope<S>, input_id: &'static str, msgfier: F) -> Html
    where
        F: Fn(Self) -> T + 'static,
        S: Component<Message = T>,
    {
        let onchange = link.batch_callback(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let filtered: String = select
                .value()
                .chars()
                .filter(|x| x.is_ascii_hexdigit())
                .collect();
            let number = u32::from_str_radix(&filtered, 16).unwrap();
            Some(msgfier(Self::new(number)))
        });

        let value = format!("{:08X}", self.inner());

        html! {
          <div class="settings-label-container-centerer" >
            <span class="settings-label-centered"> {"0x"} </span>
            <input type="text" id={input_id} size=8 maxlength=8 {value} {onchange} />
          </div>
        }
    }
}
