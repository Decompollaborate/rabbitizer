/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::encoded_field_mask::EncodedFieldMask;
use crate::encoder::token::{Token, Tokenize};
use crate::instr::{Instruction, InstructionFlags};
use crate::opcodes::{Opcode, OpcodeDecoder, OPCODES};
use crate::utils::DoubleOptIterator;
use crate::vram::{Vram, VramOffset};

#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EncoderIterator<'s> {
    tokenizer: Tokenize<'s>,
    vram: Vram,
    flags: InstructionFlags,
}

impl<'s> EncoderIterator<'s> {
    pub fn new(text: &'s str, vram: Vram, flags: InstructionFlags) -> Self {
        Self {
            tokenizer: Tokenize::new(text),
            vram,
            flags,
        }
    }
}

impl Iterator for EncoderIterator<'_> {
    type Item = Result<Instruction, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        let opcode = match self.tokenizer.next() {
            None => return None,
            Some(Token::Comma | Token::End) => return Some(Err(())),
            Some(Token::Text(text)) => {
                if let Some(opcode) = find_opcode(text, &self.flags) {
                    opcode
                } else {
                    return Some(Err(()));
                }
            }
        };

        let mut word = opcode.opcode_bits();

        let operands_iter = opcode.operands_iter();
        let mut reamining_operands = operands_iter.len();

        if reamining_operands > 0 {
            let tokenizer_iter = DoubleOptIterator::new(self.tokenizer.by_ref());
            for (operand, (token, next_token)) in opcode.operands_iter().zip(tokenizer_iter) {
                assert!(
                    reamining_operands > 0,
                    "Logic error while encoding operands?"
                );
                reamining_operands -= 1;

                match token {
                    Token::End => return Some(Err(())),
                    Token::Comma => return Some(Err(())),
                    Token::Text(text) => {
                        if let Some(bits) = operand.encode_to_bits(text, self.flags.abi(), false) {
                            word |= bits;
                        }
                    }
                }

                match next_token {
                    None | Some(Token::End) => {
                        if reamining_operands == 0 {
                            break;
                        } else {
                            return Some(Err(()));
                        }
                    }
                    Some(Token::Text(_)) => return Some(Err(())),
                    Some(Token::Comma) => {}
                }
            }
        }

        if reamining_operands != 0 {
            return Some(Err(()));
        }

        let vram = self.vram;
        self.vram += VramOffset::new(4);

        let gated_behind = None;

        let opcode_decoder = OpcodeDecoder::from_raw_parts(
            opcode,
            opcode.opcode_category(),
            EncodedFieldMask::from_bits_retain(opcode.opcode_bits()),
            gated_behind,
        );

        //println!("word = 0x{word:08X}");
        let instr = Instruction::from_raw_parts(word, vram, opcode_decoder, self.flags);

        Some(Ok(instr))
    }
}

fn find_opcode(name: &str, flags: &InstructionFlags) -> Option<Opcode> {
    // TODO: implement more checks like checking pseudos.

    if let Some(isa_extension) = flags.isa_extension() {
        for opc in &OPCODES {
            if opc.isa_extension() == Some(isa_extension) && opc.name() == name {
                return Some(opc.opcode());
            }
        }
    }

    for opc in &OPCODES {
        if opc.isa_extension().is_none()
            && opc.isa_version() <= flags.isa_version()
            && opc.name() == name
        {
            return Some(opc.opcode());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_addiu() {
        let word = 0x27BDF8C0;
        let s = "addiu       $sp, $sp, -0x740";
        let vram = Vram::new(0x80000000);
        let flags = InstructionFlags::default();
        let mut encoder = EncoderIterator::new(s, vram, flags);

        let instr = encoder.next().unwrap().unwrap();
        assert!(instr.is_valid());
        assert_eq!(instr.word(), word);

        assert_eq!(encoder.next(), None);
    }

    #[cfg(feature = "MIPS_II")]
    #[test]
    fn test_encoder_tgei() {
        let word = 0x04A80010;
        let s = "tgei        $a1, 0x10";
        let vram = Vram::new(0x80000000);
        let flags = InstructionFlags::default();
        let mut encoder = EncoderIterator::new(s, vram, flags);

        let instr = encoder.next().unwrap().unwrap();
        assert!(instr.is_valid());
        assert_eq!(instr.word(), word);

        assert_eq!(encoder.next(), None);
    }
}
