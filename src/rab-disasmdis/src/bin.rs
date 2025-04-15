/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod endian;
mod isas;

use endian::Endian;
use isas::{ArgIsaExtension, ArgIsaVersion};

use clap::Parser;

/// disasmdis (disassemble this): CLI tool to decode words as MIPS instructions
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Hex words to be disassembled. Each word literal must be in groups of 8 characters.
    inputs: Vec<String>,

    /// Endian of the input strings
    #[clap(short = 'E', long, value_enum, default_value_t = Endian::default())]
    endian: Endian,

    /// Enables all pseudo instructions supported by rabbitizer
    #[clap(short, long, default_value_t = false)]
    pseudos: bool,

    #[clap(short = 'i', long, value_enum, default_value_t = ArgIsaVersion::default())]
    isa_version: ArgIsaVersion,
    #[clap(short = 'e', long)]
    isa_extension: Option<ArgIsaExtension>,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Data {
    characters: [char; 8],
    index: usize,
}

impl Data {
    pub fn new() -> Self {
        Self {
            characters: ['\0'; 8],
            index: 0,
        }
    }

    pub const fn done(&self) -> bool {
        self.index >= self.characters.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.index == 0
    }

    pub const fn missing_chars(&self) -> usize {
        self.characters.len() - self.index
    }

    pub fn get_bytes(&mut self) -> Option<[u8; 4]> {
        self.done().then(|| {
            let mut bytes = [0u8; 4];

            for (i, c) in self.characters.iter().enumerate() {
                let byte_index = i / 2;
                let nibble = i % 2 != 0;

                let value = match c {
                    '0'..='9' => *c as u8 - b'0',
                    'A'..='F' => *c as u8 - b'A' + 10,
                    'a'..='f' => *c as u8 - b'a' + 10,
                    _ => panic!(),
                };

                if nibble {
                    bytes[byte_index] |= value;
                } else {
                    bytes[byte_index] = value << 4;
                }
            }

            self.index = 0;
            bytes
        })
    }

    pub fn push(&mut self, c: char) -> Result<(), ()> {
        if self.done() {
            return Err(());
        }

        match c {
            '0'..='9' | 'A'..='F' | 'a'..='f' => {
                assert!(self.index < self.characters.len());
                self.characters[self.index] = c;
                self.index += 1;
                Ok(())
            }
            'x' | 'X' => self.pop(),
            ' ' | ':' | '-' | '+' | ',' => Ok(()), // ignore those
            _ => Err(()),
        }
    }

    pub fn pop(&mut self) -> Result<(), ()> {
        if self.is_empty() {
            return Err(());
        }

        self.characters[self.index - 1] = '\0';
        self.index -= 1;

        Ok(())
    }
}

fn main() {
    let args = Args::parse();

    let endian = args.endian;
    let mut data = Data::new();
    let flags = rabbitizer::instr::InstructionFlags::new(args.isa_version.into())
        .with_isa_extension(args.isa_extension.map(|x| x.into()))
        .with_all_pseudos(args.pseudos);
    let vram = rabbitizer::vram::Vram::new(0x8000_0000);
    let display_flags = rabbitizer::display_flags::InstructionDisplayFlags::new_gnu_as();

    if args.inputs.is_empty() {
        eprintln!("Missing arguments");
        std::process::exit(1);
    }

    for input in args.inputs {
        for c in input.chars() {
            if let Some(bytes) = data.get_bytes() {
                // Display an instruction each time the buffer is full
                let word = endian.word_from_bytes(bytes);
                let instr = rabbitizer::Instruction::new(word, vram, flags);
                println!("{}", instr.display::<&str>(&display_flags, None, 0));
            }

            data.push(c)
                .unwrap_or_else(|_| panic!("Error when parsing character '{}'", c));
        }
    }

    if let Some(bytes) = data.get_bytes() {
        let word = endian.word_from_bytes(bytes);
        let instr = rabbitizer::instr::Instruction::new(word, vram, flags);
        println!("{}", instr.display::<&str>(&display_flags, None, 0));
    } else if data.missing_chars() < 8 {
        eprintln!(
            "Could not fill up a word while parsing the input. Missing characters: '{}'",
            data.missing_chars()
        );
        eprint!("Characters seen: ");
        for (i, c) in data.characters.iter().enumerate() {
            if i < data.index {
                eprint!("{}", c);
            } else {
                eprint!("*");
            }
        }
        eprintln!();
        std::process::exit(1);
    }
}
