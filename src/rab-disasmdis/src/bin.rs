/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod endian;

pub use endian::Endian;

use clap::Parser;

/// disasmdis (disassemble this): CLI tool to decode words as MIPS instructions
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Hex words to be disassembled. Each word literal must be in groups of 8 characters.
    inputs: Vec<String>,

    /// Endian of the input strings
    #[clap(short, long, value_enum, default_value_t = Endian::default())]
    endian: Endian,

    /// Enables all pseudo instructions supported by rabbitizer
    #[clap(short, long, default_value_t = false)]
    pseudos: bool,
    // TODO: IsaVersion and IsaExtension
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Data {
    bytes: [u8; 4],
    index: usize,
    nibble: bool,
}

impl Data {
    pub fn new() -> Self {
        Self {
            bytes: [0; 4],
            index: 0,
            nibble: false,
        }
    }

    pub const fn done(&self) -> bool {
        self.index >= self.bytes.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.index == 0 && !self.nibble
    }

    pub const fn missing_chars(&self) -> usize {
        let n = if self.nibble { 1 } else { 0 };

        (self.bytes.len() - self.index) * 2 - n
    }

    pub fn get_bytes(&mut self) -> Option<[u8; 4]> {
        self.done().then(|| {
            let bytes = self.bytes;
            self.index = 0;
            self.nibble = false;
            bytes
        })
    }

    pub fn push(&mut self, c: char) -> Result<(), ()> {
        if self.done() {
            return Err(());
        }

        let value = match c {
            '0'..='9' => c as u8 - b'0',
            'A'..='F' => c as u8 - b'A' + 10,
            'a'..='f' => c as u8 - b'a' + 10,
            'x' | 'X' => return self.pop(),
            ' ' | ':' | '-' | '+' => return Ok(()),
            _ => return Err(()),
        };

        assert!(self.index < self.bytes.len());
        let val = if self.nibble {
            self.bytes[self.index] | value
        } else {
            value << 4
        };
        self.bytes[self.index] = val;
        if self.nibble {
            self.index += 1;
        }
        self.nibble = !self.nibble;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<(), ()> {
        if self.is_empty() {
            return Err(());
        }

        if self.nibble {
            self.bytes[self.index] = 0;
        } else {
            self.bytes[self.index - 1] &= !0xF;
            self.index -= 1;
        }
        self.nibble = !self.nibble;

        Ok(())
    }
}

fn main() {
    let args = Args::parse();

    let endian = args.endian;
    let mut data = Data::new();
    let flags = rabbitizer::instr::InstructionFlags::new().with_all_pseudos(args.pseudos);
    let isa_extension = rabbitizer::isa::IsaExtension::NONE;
    let vram = rabbitizer::vram::Vram::new(0x8000_0000);
    let display_flags = rabbitizer::display_flags::DisplayFlags::new_gnu_as();

    for input in args.inputs {
        for c in input.chars() {
            if let Some(bytes) = data.get_bytes() {
                let word = endian.word_from_bytes(bytes);
                let instr = rabbitizer::instr::Instruction::new(
                    word,
                    vram,
                    flags,
                    isa_extension.isa_version(),
                    isa_extension,
                );
                println!("{}", instr.display(None, &display_flags));
            }

            data.push(c)
                .unwrap_or_else(|_| panic!("Error when parsing character '{}'", c));
        }
    }

    if let Some(bytes) = data.get_bytes() {
        let word = endian.word_from_bytes(bytes);
        let instr = rabbitizer::instr::Instruction::new(
            word,
            vram,
            flags,
            isa_extension.isa_version(),
            isa_extension,
        );
        println!("{}", instr.display(None, &display_flags));
    } else {
        panic!(
            "Could not fill up a word while parsing the input. Missing characters: '{}'",
            data.missing_chars()
        );
    }
}
