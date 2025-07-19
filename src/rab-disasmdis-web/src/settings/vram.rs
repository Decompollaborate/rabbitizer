/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use web_sys::HtmlSelectElement;
use yew::events::Event;
use yew::html::Scope;
use yew::{html, Component, Html, TargetCast};

pub use rabbitizer::Vram;

use crate::settings::{InputStruct, LabelPosition, Storagable};

const KEY: &str = "decompollaborate.disasmdis-web.state.vram";

impl Storagable for Vram {
    fn storage_key() -> &'static str {
        KEY
    }
}

impl InputStruct for Vram {
    fn label_text() -> &'static str {
        "Vram:"
    }
    fn input_id() -> &'static str {
        "vram"
    }

    fn gen_input<F, M, S>(&self, link: &Scope<S>, label_position: LabelPosition, msgfier: F) -> Html
    where
        F: Fn(Self) -> M + 'static,
        S: Component<Message = M>,
    {
        let label_text = Self::label_text();
        let input_id = Self::input_id();

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

        let input = html! {
          <div class="settings-label-container-centerer" >
            <span class="settings-label-centered"> {"0x"} </span>
            <input class="settings-dropdown" type="text" id={input_id} size=8 maxlength=8 {value} {onchange} />
          </div>
        };

        match label_position {
            LabelPosition::Upper => html! {
              <label for={input_id}> { label_text }
                { input }
              </label>
            },
            LabelPosition::Left => html! {
              <>
                <label for={input_id}> { label_text }</label>
                { input }
              </>
            },
        }
    }
}
