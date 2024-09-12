/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{IsaExtension, IsaVersion, Opcode, OpcodeCategory, OpcodeDecoder};

pub struct Instruction {
    word: u32,
    vram: u32, // TODO: maybe make Vram wrapper type?

    isa_version: IsaVersion,
    isa_extension: IsaExtension,

    opcode_decoder: OpcodeDecoder,

    _handwritten_category: bool, // TODO: remove in favour of checking the OpcodeCategory instead
    _in_handwritten_function: bool,// TODO: maybe remove?

    // flags: u32,
}

impl Instruction {
    pub const fn new(
        word: u32,
        vram: u32,
        isa_version: IsaVersion,
        isa_extension: IsaExtension,) -> Self {
        let opcode_decoder = OpcodeDecoder::decode(word, isa_version, isa_extension);

        Self {
            word,
            vram,
            isa_version,
            isa_extension,
            opcode_decoder,
            _handwritten_category: false,
            _in_handwritten_function: false,
        }
    }
}

impl Instruction {
    // getters and setters

    pub const fn word(&self) -> u32 {
        self.word
    }

    pub const fn vram(&self) -> u32 {
        self.vram
    }

    pub const fn isa_version(&self) -> IsaVersion {
        self.isa_version
    }

    pub const fn isa_extension(&self) -> IsaExtension {
        self.isa_extension
    }

    pub const fn opcode(&self) -> Opcode {
        self.opcode_decoder.opcode()
    }

    pub const fn opcode_category(&self) -> OpcodeCategory {
        self.opcode_decoder.opcode_category()
    }
}
