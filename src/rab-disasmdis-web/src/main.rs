/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use rabbitizer::{
    Instruction, InstructionDisplayFlags, InstructionFlags, IsaExtension, IsaVersion, Vram,
};
use web_sys::HtmlInputElement as InputElement;
use yew::events::InputEvent;
use yew::html::Scope;
use yew::{html, Component, Context, Html, TargetCast};

mod bytes_text_parser;
use bytes_text_parser::{BytesTextParser, ParsedTextResult};

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/*
const KEY: &str = "decompollaborate.disasmdis-web.self";
*/

pub enum Msg {
    InputData(String),
}

pub struct App {
    input: String,
    isa_version: IsaVersion,
    _isa_extension: Option<IsaExtension>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: "".to_string(),
            isa_version: IsaVersion::default(),
            _isa_extension: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputData(input) => {
                self.input = input.to_string();
            }
        }
        /*
        LocalStorage::set(KEY, &self.).expect("Failed to save into LocalStorage.");
        */
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let git_info = if let Some(info) = built_info::GIT_COMMIT_HASH_SHORT {
            format!("Git hash: {info}")
        } else {
            String::new()
        };

        html! {
            <div>
                <section>
                    <header class="header">
                        <h1>{ "disasmdis-web " } { built_info::PKG_VERSION } </h1>
                        { self.view_input(ctx.link()) }
                    </header>
                    <section>
                        <ul>
                            { self.disassemble() }
                        </ul>
                        /*
                        <ul>
                            { &self.input }
                        </ul>
                        */
                    </section>
                </section>
                <footer>
                    <div>
                    { git_info }
                    </div>
                    <div>
                    { "Powered by " } <a target="_blank" href={ built_info::PKG_REPOSITORY }>{ "rabbitizer" }</a>
                    </div>
                </footer>
            </div>
        }
    }
}

impl App {
    fn view_input(&self, link: &Scope<Self>) -> Html {
        let oninput = link.batch_callback(|e: InputEvent| {
            let input: InputElement = e.target_unchecked_into();
            Some(Msg::InputData(input.value()))
        });

        html! {
            <div>
                <label
                    for="bytes-input"
                >
                    { "Enter MIPS bytes:" }
                </label>
                <br/>
                <textarea
                    id="bytes-input"
                    rows="8"
                    cols="80"
                    {oninput}
                />
            </div>
        }
    }

    fn disassemble(&self) -> Vec<Html> {
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

        for x in BytesTextParser::new(&self.input) {
            match x {
                ParsedTextResult::Bytes(b) => {
                    // TODO: endian
                    let word = u32::from_be_bytes(b);
                    let instr = Instruction::new(word, vram, flags);
                    let disassembled = instr.display::<&str>(&display_flags, None, 0).to_string();
                    let word_str = format!("{word:08X}");
                    result.push(html! {
                        <li>
                            <div>
                                <code> { word_str } { " => " } { disassembled } </code>
                            </div>
                        </li>
                    });
                }
                ParsedTextResult::InvalidCharacter(c, index) => {
                    result.push(html! {
                        <li>
                            <div>
                                { "Invalid character '" } {c} { "' at index " } {index}
                            </div>
                        </li>
                    });
                }
            }
        }

        result
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
