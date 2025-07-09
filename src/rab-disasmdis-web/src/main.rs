/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use rabbitizer::{Instruction, InstructionDisplayFlags, InstructionFlags, Vram};
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::events::InputEvent;
use yew::functional::use_effect;
use yew::html::Scope;
use yew::{html, Component, Context, Html, TargetCast};

mod bytes_text_parser;
mod persistent_state;
mod settings;

use crate::bytes_text_parser::{BytesTextParser, ParsedTextResult};
use crate::persistent_state::PersistentState;
use crate::settings::*;

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
    ChangeIsaVersion(IsaVersion),
    ChangeIsaExtension(Option<IsaExtension>),
}

pub struct App {
    input: String,
    state: PersistentState,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: "".to_string(),
            state: PersistentState::new(),
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
            Msg::ChangeIsaVersion(isa_version) => {
                self.state.isa_version = isa_version;
            }
            Msg::ChangeIsaExtension(isa_extension) => {
                self.state.isa_extension = isa_extension;
                if let Some(isa_extension) = isa_extension {
                    self.state.isa_version = isa_extension.isa_version();
                }
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
        let flags = InstructionFlags::new_isa(self.state.isa_version, self.state.isa_extension);
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
                    let comment = format!("/* {byte_offset:04X} {word:08X} */");
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
        let dropdown_id_endian = "endian";
        let dropdown_endian =
            self.state
                .endian
                .gen_dropdown(link, dropdown_id_endian, Msg::ChangeEndian);

        let dropdown_id_isa_version = "isa_version";
        let dropdown_isa_version = self.state.isa_version.gen_dropdown(
            link,
            dropdown_id_isa_version,
            Msg::ChangeIsaVersion,
        );

        let dropdown_id_isa_extension = "isa_extension";
        let dropdown_isa_extension = self.state.isa_extension.gen_dropdown(
            link,
            dropdown_id_isa_extension,
            Msg::ChangeIsaExtension,
        );

        html! {
          <>
            <h3> { "⚙️ Configuration" }</h3>
            <div class="settings">
              <label for={dropdown_id_endian}> { "Endianness:" }
                { dropdown_endian }
              </label>
              <label for={dropdown_id_isa_version}> { "ISA version:" }
                { dropdown_isa_version }
              </label>
              <label for={dropdown_id_isa_extension}> { "ISA extension:" }
                { dropdown_isa_extension }
              </label>
            </div>
          </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
