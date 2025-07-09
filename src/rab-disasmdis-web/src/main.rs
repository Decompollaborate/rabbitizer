/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use rabbitizer::{
    Instruction, InstructionDisplayFlags, InstructionFlags, IsaExtension, IsaVersion, Vram,
};
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::events::InputEvent;
use yew::functional::use_effect;
use yew::html::Scope;
use yew::{html, Component, Context, Html, TargetCast};

mod bytes_text_parser;
mod persistent_state;

use bytes_text_parser::{BytesTextParser, ParsedTextResult};
use persistent_state::{DropdownEnum, Endian, PersistentState, Theme};

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightAll();
}

pub enum Msg {
    InputData(String),
    ChangeTheme(Theme),
    ChangeEndian(Endian),
}

pub struct App {
    input: String,
    state: PersistentState,
    isa_version: IsaVersion,
    _isa_extension: Option<IsaExtension>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: "".to_string(),
            state: PersistentState::new(),
            isa_version: IsaVersion::default(),
            _isa_extension: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputData(input) => {
                self.input = input;
            }
            Msg::ChangeTheme(theme) => {
                self.state.theme = theme;
            }
            Msg::ChangeEndian(endian) => {
                self.state.endian = endian;
            }
        }
        self.state.save();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let header = self.view_header(ctx);
        let main = self.view_main(ctx);
        let footer = self.view_footer(ctx);

        let root_class = format!("{} view-root", self.state.theme.id());

        use_effect(move || {
            // asm syntax highlight.
            // TODO: breaks the site due to lack of auto update
            // highlightAll();
        });

        html! {
          <div class={root_class}>
            { header }
            { main }
            { footer }
          </div>
        }
    }
}

impl App {
    fn view_header(&self, ctx: &Context<Self>) -> Html {
        let dropdown_id = "theme";
        let dropdown = self
            .state
            .theme
            .gen_dropdown(ctx.link(), dropdown_id, Msg::ChangeTheme);

        html! {
          <header>
            <h1> { "🧩 disasmdis-web" } <h6> { built_info::PKG_VERSION } </h6> </h1>

            <div class="theme-selector">
              <label for={dropdown_id}> { "Theme:" } </label>
              { dropdown }
            </div>
          </header>
        }
    }

    fn view_main(&self, ctx: &Context<Self>) -> Html {
        html! {
          <main>
            <section class="editor">
              { self.view_input(ctx.link()) }
              { self.view_disassemble_box() }
            </section>

            <section class="config">
              { self.view_config(ctx.link()) }
            </section>
          </main>
        }
    }

    fn view_footer(&self, _ctx: &Context<Self>) -> Html {
        let git_info = if let Some(info) = built_info::GIT_COMMIT_HASH_SHORT {
            format!("Git hash: {info}")
        } else {
            String::new()
        };

        html! {
          <footer>
            <p> { "© 2025 Decompollaborate" } </p>
            <p> { "Powered by " } <a target="_blank" href={ built_info::PKG_REPOSITORY }>{ "rabbitizer" }</a> </p>
            <p> { git_info } </p>
          </footer>
        }
    }
}

impl App {
    fn view_input(&self, link: &Scope<Self>) -> Html {
        let oninput = link.batch_callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Some(Msg::InputData(input.value()))
        });

        html! {
          <div class="input-box">
            <h2 for="bytes-input"> { "Input" } </h2>
            <textarea
              id="bytes-input"
              rows="8"
              cols="80"
              placeholder="Enter hex code..."
              {oninput}
            />
          </div>
        }
    }

    fn view_disassemble_box(&self) -> Html {
        // TODO: configurable flags
        /*
        let flags = InstructionFlags::new(args.isa_version.into())
            .with_isa_extension(args.isa_extension.map(|x| x.into()))
            .with_all_pseudos(args.pseudos);
        */
        let flags = InstructionFlags::new(self.isa_version);
        let vram = Vram::new(0x8000_0000);
        let display_flags = InstructionDisplayFlags::new_gnu_as();

        let mut result = Vec::new();
        let mut byte_offset = 0;

        for x in BytesTextParser::new(&self.input) {
            match x {
                ParsedTextResult::Bytes(b) => {
                    // TODO: endian
                    let word = self.state.endian.word_from_bytes(b);
                    let instr = Instruction::new(word, vram, flags);
                    let disassembled = instr.display::<&str>(&display_flags, None, 0).to_string();
                    let comment = format!("/* {byte_offset:06X} {word:08X} */");
                    result.push(html! {
                      <tr>
                        <td class="cod"> { comment } { " " } { disassembled } </td>
                      </tr>
                    });
                    byte_offset += 4;
                }
                ParsedTextResult::InvalidCharacter(c, index) => {
                    result.push(html! {
                      <tr>
                        <td class="cod">{ "/* Invalid character '" } {c} { "' at index " } {index} { " */" } </td>
                      </tr>
                    });
                }
            }
        }

        html! {
          <div class="output-box">
            <h2> { "Disassembled Output" } </h2>
            <div class="scrollable-container">
              <pre><code /*class="language-mipsasm"*/>
                <table> { result } </table>
              </code></pre>
            </div>
          </div>
        }
    }

    fn view_config(&self, link: &Scope<Self>) -> Html {
        let dropdown_id = "endian";
        let dropdown = self
            .state
            .endian
            .gen_dropdown(link, dropdown_id, Msg::ChangeEndian);

        html! {
          <>
            <h3> { "⚙️ Configuration" }</h3>
            <div class="settings">
              <label for={dropdown_id}> { "Endianness:" }
                { dropdown }
              </label>
            </div>
          </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
