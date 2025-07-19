/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use rabbitizer::{
    encoder::EncoderIterator, vram::VramOffset, Instruction, InstructionDisplayFlags,
    InstructionFlags, Vram,
};
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
    ChangeCodingMode(CodingMode),
    ChangeTheme(Theme),
    ChangeEndian(Endian),
    ChangeIsaVersion(IsaVersion),
    ChangeIsaExtension(Option<IsaExtension>),
    ChangeBranchLabel(DefaultLabelDisplay),
    ChangeVram(Vram),
}

pub struct App {
    /// Used with CodingMode::Decoder
    hex_input: String,
    /// Used with CodingMode::Encoder
    instr_input: String,
    state: PersistentState,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            hex_input: "3C028020\n24 42 12 34\n03E00008 00000000".to_string(),
            instr_input: "lui $v0, 0x8020\naddiu $v0, $v0, 0x1234\njr $ra; nop".to_string(),
            state: PersistentState::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputData(input) => match self.state.coding_mode {
                CodingMode::Decoder => self.hex_input = input,
                CodingMode::Encoder => self.instr_input = input,
            },
            Msg::ChangeCodingMode(coding_mode) => {
                self.state.coding_mode = coding_mode;
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
            Msg::ChangeBranchLabel(branch_label) => {
                self.state.branch_label = branch_label;
            }
            Msg::ChangeVram(vram) => {
                self.state.vram = vram;
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
        let link = ctx.link();
        let label_position = LabelPosition::Left;

        let dropdown_coding_mode =
            self.state
                .coding_mode
                .gen_dropdown(link, label_position, Msg::ChangeCodingMode);

        let dropdown = self
            .state
            .theme
            .gen_dropdown(link, label_position, Msg::ChangeTheme);

        html! {
          <header>
            <h1> { "🧩 disasmdis-web" } <h6> { built_info::PKG_VERSION } </h6> </h1>

            <div class="theme-selector">
              { dropdown_coding_mode }
            </div>

            <div class="theme-selector">
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
        let placeholder = match self.state.coding_mode {
            CodingMode::Decoder => "Enter hex code...",
            CodingMode::Encoder => "Enter assembly instructions...",
        };
        let value = match self.state.coding_mode {
            CodingMode::Decoder => &self.hex_input,
            CodingMode::Encoder => &self.instr_input,
        }
        .clone();

        html! {
          <div class="input-box">
            <h2 for="bytes-input"> { "Input" } </h2>
            <textarea
              id="bytes-input"
              rows="8"
              cols="80"
              {placeholder}
              {oninput}
              {value}
            />
          </div>
        }
    }

    fn view_disassemble_box(&self) -> Html {
        let result = match self.state.coding_mode {
            CodingMode::Decoder => self.disassemble_input(),
            CodingMode::Encoder => self.encode_input(),
        };
        let label = match self.state.coding_mode {
            CodingMode::Decoder => "Disassembled output",
            CodingMode::Encoder => "Encoded output",
        };

        html! {
          <div class="output-box">
            <h2> { label } </h2>
            <div class="scrollable-container">
              <pre><code /*class="language-mipsasm"*/>
                <table> { result } </table>
              </code></pre>
            </div>
          </div>
        }
    }

    fn disassemble_input(&self) -> Vec<Html> {
        let mut result = Vec::new();
        let mut vram = self.state.vram;
        let mut byte_offset = 0;

        let flags = InstructionFlags::new_isa(self.state.isa_version, self.state.isa_extension);
        let display_flags = InstructionDisplayFlags::new_gnu_as()
            .with_branch_default_label_display(self.state.branch_label);

        for x in BytesTextParser::new(&self.hex_input) {
            match x {
                ParsedTextResult::Bytes(b) => {
                    let word = self.state.endian.word_from_bytes(b);
                    let instr = Instruction::new(word, vram, flags);
                    let disassembled = instr.display::<&str>(&display_flags, None, -4).to_string();
                    let comment = format!("/* {byte_offset:04X} {vram} {word:08X} */");
                    result.push(html! {
                      <tr>
                        <td class="cod"> { comment } { " " } { disassembled } </td>
                      </tr>
                    });
                    vram += VramOffset::new(4);
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

        result
    }

    fn encode_input(&self) -> Vec<Html> {
        let mut result = Vec::new();

        let flags = InstructionFlags::new_isa(self.state.isa_version, self.state.isa_extension);

        for x in EncoderIterator::new(&self.instr_input, self.state.vram, flags) {
            match x {
                Err(e) => {
                    result.push(html! {
                      <tr>
                        <td class="cod">{ "/* Error: " } {e} { " */" } </td>
                      </tr>
                    });
                }
                Ok(instr) => {
                    let bytes = self.state.endian.bytes_from_word(instr.word());
                    let formatted = format!(
                        "{:02X}{:02X}{:02X}{:02X}",
                        bytes[0], bytes[1], bytes[2], bytes[3]
                    );

                    result.push(html! {
                      <tr>
                        <td class="cod">{ formatted } </td>
                      </tr>
                    });
                }
            }
        }

        result
    }

    fn view_config(&self, link: &Scope<Self>) -> Html {
        let label_position = LabelPosition::Upper;

        let dropdown_endian =
            self.state
                .endian
                .gen_dropdown(link, label_position, Msg::ChangeEndian);

        let dropdown_isa_version =
            self.state
                .isa_version
                .gen_dropdown(link, label_position, Msg::ChangeIsaVersion);

        let dropdown_isa_extension =
            self.state
                .isa_extension
                .gen_dropdown(link, label_position, Msg::ChangeIsaExtension);

        let dropdown_branch_label =
            self.state
                .branch_label
                .gen_dropdown(link, label_position, Msg::ChangeBranchLabel);

        let dropdown_vram = self
            .state
            .vram
            .gen_input(link, label_position, Msg::ChangeVram);

        html! {
          <>
            <h3> { "⚙️ Configuration" } </h3>
            <div class="settings">
              { dropdown_endian }
              { dropdown_isa_version }
              { dropdown_isa_extension }
              { dropdown_branch_label }
              { dropdown_vram }
            </div>
          </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
