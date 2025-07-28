/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::encoded_field_mask::EncodedFieldMask;
use crate::encoder::operand_encoder::EncodedOperandBits;
use crate::encoder::token::{Token, Tokenize};
use crate::encoder::EncodingError;
use crate::instr::{Instruction, InstructionFlags};
use crate::opcodes::{Opcode, OpcodeDecoder, OPCODES};
use crate::operands::Operand;
use crate::utils::iter::DoubleOptIterator;
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

    fn find_opcode(&self, name: &str) -> Option<Opcode> {
        // TODO: implement more checks like checking pseudos.

        if let Some(isa_extension) = self.flags.isa_extension() {
            for opc in &OPCODES {
                if opc.isa_extension() == Some(isa_extension) && opc.name() == name {
                    return Some(opc.opcode());
                }
            }
        }

        for opc in &OPCODES {
            if opc.isa_extension().is_none()
                && opc.isa_version() <= self.flags.isa_version()
                && opc.name() == name
            {
                return Some(opc.opcode());
            }
        }

        None
    }

    fn encode_operands(
        &mut self,
        opcode: Opcode,
        allow_dollarless: bool,
    ) -> Result<u32, EncodingError<'s>> {
        let mut word = 0;

        let operands_iter = opcode.operands_iter();
        let mut reamining_operands = operands_iter.len();

        if reamining_operands == 0 {
            return Ok(0);
        }

        let mut tokenizer_iter = DoubleOptIterator::new(self.tokenizer.by_ref());
        for operand in opcode.operands_iter() {
            assert!(
                reamining_operands > 0,
                "Logic error while encoding operands?"
            );
            reamining_operands -= 1;

            match operand.encode_to_bits(
                tokenizer_iter.by_ref(),
                self.flags.abi(),
                allow_dollarless,
                opcode,
            )? {
                EncodedOperandBits::EndBits(bits) => {
                    if reamining_operands == 0 {
                        word = handle_bits(word, bits, operand);
                        break;
                    } else {
                        return Err(EncodingError::EndButMissingOperands(
                            opcode,
                            reamining_operands,
                        ));
                    }
                }
                EncodedOperandBits::ContinueBits(bits) => {
                    word = handle_bits(word, bits, operand);
                }
            }
        }

        if reamining_operands != 0 {
            Err(EncodingError::EndButMissingOperands(
                opcode,
                reamining_operands,
            ))
        } else {
            Ok(word)
        }
    }
}

const fn handle_bits(mut word: u32, bits: u32, operand: Operand) -> u32 {
    match operand {
        #[cfg(feature = "R4000ALLEGREX")]
        Operand::r4000allegrex_size_plus_pos => {
            use crate::utils;

            let s = utils::from_2s_complement::<5>(
                EncodedFieldMask::r4000allegrex_size_plus_pos.get_shifted(bits),
            );
            let b = EncodedFieldMask::r4000allegrex_pos.get_shifted(word) as i32;
            let x = s - 1 + b;

            word |= EncodedFieldMask::r4000allegrex_size_plus_pos.unshift(x as u32);
        }
        _ => word |= bits,
    }
    word
}

impl<'s> Iterator for EncoderIterator<'s> {
    type Item = Result<Instruction, EncodingError<'s>>;

    fn next(&mut self) -> Option<Self::Item> {
        let opcode = loop {
            match self.tokenizer.next() {
                None => return None,
                Some(Token::End) => continue,
                Some(Token::Comma) => return Some(Err(EncodingError::CommaInsteadOfOpcode)),
                Some(Token::Text(text)) => {
                    if let Some(opcode) = self.find_opcode(text) {
                        break opcode;
                    } else {
                        return Some(Err(EncodingError::UnrecognizedOpcode(text)));
                    }
                }
                Some(Token::Bracketed(left, right, bracket_type)) => {
                    return Some(Err(EncodingError::BracketedInsteadOfOpcode(
                        left,
                        right,
                        bracket_type,
                    )))
                }
                Some(Token::BracketSolo(text, bracket_type)) => {
                    return Some(Err(EncodingError::BracketSoloInsteadOfOpcode(
                        text,
                        bracket_type,
                    )))
                }
            }
        };

        let allow_dollarless = true;
        let mut word = opcode.opcode_bits();

        let operand_bits = match self.encode_operands(opcode, allow_dollarless) {
            Ok(w) => w,
            Err(e) => {
                return Some(Err(e));
            }
        };

        word |= operand_bits;

        let vram = self.vram;
        self.vram += VramOffset::new(4);

        let gated_behind = None;

        let opcode_decoder = OpcodeDecoder::from_raw_parts(
            opcode,
            opcode.opcode_category(),
            EncodedFieldMask::from_bits_retain(opcode.opcode_bits()),
            gated_behind,
        );

        let instr = Instruction::from_raw_parts(word, vram, opcode_decoder, self.flags);

        Some(Ok(instr))
    }
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

    #[cfg(feature = "R4000ALLEGREX")]
    #[test]
    fn test_encoder_r4000allegrex_vcmp() {
        use crate::IsaExtension;

        static DATA: [(u32, &str); 132] = [
            (0x6C401001, "vcmp.s      eq, S400, S002"),
            (0x6C401081, "vcmp.p      eq, C400, C002"),
            (0x6C409001, "vcmp.t      eq, C400, C001"),
            (0x6C409081, "vcmp.q      eq, C400, C002"),
            (0x6C400800, "vcmp.s      fl, S200, S002"),
            (0x6C400880, "vcmp.p      fl, C200, C002"),
            (0x6C408800, "vcmp.t      fl, C200, C001"),
            (0x6C408880, "vcmp.q      fl, C200, C002"),
            (0x6C400801, "vcmp.s      eq, S200, S002"),
            (0x6C400881, "vcmp.p      eq, C200, C002"),
            (0x6C408801, "vcmp.t      eq, C200, C001"),
            (0x6C408881, "vcmp.q      eq, C200, C002"),
            (0x6C400802, "vcmp.s      lt, S200, S002"),
            (0x6C400882, "vcmp.p      lt, C200, C002"),
            (0x6C408802, "vcmp.t      lt, C200, C001"),
            (0x6C408882, "vcmp.q      lt, C200, C002"),
            (0x6C400803, "vcmp.s      le, S200, S002"),
            (0x6C400883, "vcmp.p      le, C200, C002"),
            (0x6C408803, "vcmp.t      le, C200, C001"),
            (0x6C408883, "vcmp.q      le, C200, C002"),
            (0x6C400804, "vcmp.s      tr, S200, S002"),
            (0x6C400884, "vcmp.p      tr, C200, C002"),
            (0x6C408804, "vcmp.t      tr, C200, C001"),
            (0x6C408884, "vcmp.q      tr, C200, C002"),
            (0x6C400805, "vcmp.s      ne, S200, S002"),
            (0x6C400885, "vcmp.p      ne, C200, C002"),
            (0x6C408805, "vcmp.t      ne, C200, C001"),
            (0x6C408885, "vcmp.q      ne, C200, C002"),
            (0x6C400806, "vcmp.s      ge, S200, S002"),
            (0x6C400886, "vcmp.p      ge, C200, C002"),
            (0x6C408806, "vcmp.t      ge, C200, C001"),
            (0x6C408886, "vcmp.q      ge, C200, C002"),
            (0x6C400807, "vcmp.s      gt, S200, S002"),
            (0x6C400887, "vcmp.p      gt, C200, C002"),
            (0x6C408807, "vcmp.t      gt, C200, C001"),
            (0x6C408887, "vcmp.q      gt, C200, C002"),
            (0x6C400808, "vcmp.s      ez, S200, S002"),
            (0x6C400888, "vcmp.p      ez, C200, C002"),
            (0x6C408808, "vcmp.t      ez, C200, C001"),
            (0x6C408888, "vcmp.q      ez, C200, C002"),
            (0x6C400809, "vcmp.s      en, S200, S002"),
            (0x6C400889, "vcmp.p      en, C200, C002"),
            (0x6C408809, "vcmp.t      en, C200, C001"),
            (0x6C408889, "vcmp.q      en, C200, C002"),
            (0x6C40080A, "vcmp.s      ei, S200, S002"),
            (0x6C40088A, "vcmp.p      ei, C200, C002"),
            (0x6C40880A, "vcmp.t      ei, C200, C001"),
            (0x6C40888A, "vcmp.q      ei, C200, C002"),
            (0x6C40080B, "vcmp.s      es, S200, S002"),
            (0x6C40088B, "vcmp.p      es, C200, C002"),
            (0x6C40880B, "vcmp.t      es, C200, C001"),
            (0x6C40888B, "vcmp.q      es, C200, C002"),
            (0x6C40080C, "vcmp.s      nz, S200, S002"),
            (0x6C40088C, "vcmp.p      nz, C200, C002"),
            (0x6C40880C, "vcmp.t      nz, C200, C001"),
            (0x6C40888C, "vcmp.q      nz, C200, C002"),
            (0x6C40080D, "vcmp.s      nn, S200, S002"),
            (0x6C40088D, "vcmp.p      nn, C200, C002"),
            (0x6C40880D, "vcmp.t      nn, C200, C001"),
            (0x6C40888D, "vcmp.q      nn, C200, C002"),
            (0x6C40080E, "vcmp.s      ni, S200, S002"),
            (0x6C40088E, "vcmp.p      ni, C200, C002"),
            (0x6C40880E, "vcmp.t      ni, C200, C001"),
            (0x6C40888E, "vcmp.q      ni, C200, C002"),
            (0x6C40080F, "vcmp.s      ns, S200, S002"),
            (0x6C40088F, "vcmp.p      ns, C200, C002"),
            (0x6C40880F, "vcmp.t      ns, C200, C001"),
            (0x6C40888F, "vcmp.q      ns, C200, C002"),
            (0x6C000000, "vcmp.s      fl"),
            (0x6C000080, "vcmp.p      fl"),
            (0x6C008000, "vcmp.t      fl"),
            (0x6C008080, "vcmp.q      fl"),
            (0x6C000001, "vcmp.s      eq, S000, S000"),
            (0x6C000081, "vcmp.p      eq, C000, C000"),
            (0x6C008001, "vcmp.t      eq, C000, C000"),
            (0x6C008081, "vcmp.q      eq, C000, C000"),
            (0x6C000002, "vcmp.s      lt, S000, S000"),
            (0x6C000082, "vcmp.p      lt, C000, C000"),
            (0x6C008002, "vcmp.t      lt, C000, C000"),
            (0x6C008082, "vcmp.q      lt, C000, C000"),
            (0x6C000003, "vcmp.s      le, S000, S000"),
            (0x6C000083, "vcmp.p      le, C000, C000"),
            (0x6C008003, "vcmp.t      le, C000, C000"),
            (0x6C008083, "vcmp.q      le, C000, C000"),
            (0x6C000004, "vcmp.s      tr"),
            (0x6C000084, "vcmp.p      tr"),
            (0x6C008004, "vcmp.t      tr"),
            (0x6C008084, "vcmp.q      tr"),
            (0x6C000005, "vcmp.s      ne, S000, S000"),
            (0x6C000085, "vcmp.p      ne, C000, C000"),
            (0x6C008005, "vcmp.t      ne, C000, C000"),
            (0x6C008085, "vcmp.q      ne, C000, C000"),
            (0x6C000006, "vcmp.s      ge, S000, S000"),
            (0x6C000086, "vcmp.p      ge, C000, C000"),
            (0x6C008006, "vcmp.t      ge, C000, C000"),
            (0x6C008086, "vcmp.q      ge, C000, C000"),
            (0x6C000007, "vcmp.s      gt, S000, S000"),
            (0x6C000087, "vcmp.p      gt, C000, C000"),
            (0x6C008007, "vcmp.t      gt, C000, C000"),
            (0x6C008087, "vcmp.q      gt, C000, C000"),
            (0x6C000008, "vcmp.s      ez, S000"),
            (0x6C000088, "vcmp.p      ez, C000"),
            (0x6C008008, "vcmp.t      ez, C000"),
            (0x6C008088, "vcmp.q      ez, C000"),
            (0x6C000009, "vcmp.s      en, S000"),
            (0x6C000089, "vcmp.p      en, C000"),
            (0x6C008009, "vcmp.t      en, C000"),
            (0x6C008089, "vcmp.q      en, C000"),
            (0x6C00000A, "vcmp.s      ei, S000"),
            (0x6C00008A, "vcmp.p      ei, C000"),
            (0x6C00800A, "vcmp.t      ei, C000"),
            (0x6C00808A, "vcmp.q      ei, C000"),
            (0x6C00000B, "vcmp.s      es, S000"),
            (0x6C00008B, "vcmp.p      es, C000"),
            (0x6C00800B, "vcmp.t      es, C000"),
            (0x6C00808B, "vcmp.q      es, C000"),
            (0x6C00000C, "vcmp.s      nz, S000"),
            (0x6C00008C, "vcmp.p      nz, C000"),
            (0x6C00800C, "vcmp.t      nz, C000"),
            (0x6C00808C, "vcmp.q      nz, C000"),
            (0x6C00000D, "vcmp.s      nn, S000"),
            (0x6C00008D, "vcmp.p      nn, C000"),
            (0x6C00800D, "vcmp.t      nn, C000"),
            (0x6C00808D, "vcmp.q      nn, C000"),
            (0x6C00000E, "vcmp.s      ni, S000"),
            (0x6C00008E, "vcmp.p      ni, C000"),
            (0x6C00800E, "vcmp.t      ni, C000"),
            (0x6C00808E, "vcmp.q      ni, C000"),
            (0x6C00000F, "vcmp.s      ns, S000"),
            (0x6C00008F, "vcmp.p      ns, C000"),
            (0x6C00800F, "vcmp.t      ns, C000"),
            (0x6C00808F, "vcmp.q      ns, C000"),
        ];
        let vram = Vram::new(0x80000000);
        let flags = InstructionFlags::new_extension(IsaExtension::R4000ALLEGREX);

        for (word, text) in DATA {
            let mut encoder = EncoderIterator::new(text, vram, flags);

            let instr = encoder.next().unwrap().unwrap();
            assert!(instr.is_valid());
            assert_eq!(instr.word(), word);

            assert_eq!(encoder.next(), None);
        }
    }

    #[cfg(feature = "R4000ALLEGREX")]
    #[test]
    fn test_encoder_r4000allegrex_vpfxd() {
        use crate::IsaExtension;

        static DATA: [(u32, &str); 2] = [
            (0xDE000001, "vpfxd       0, , , "),
            (0xDE000400, "vpfxd       , , M, "),
        ];
        let vram = Vram::new(0x80000000);
        let flags = InstructionFlags::new_extension(IsaExtension::R4000ALLEGREX);

        for (word, text) in DATA {
            let mut encoder = EncoderIterator::new(text, vram, flags);

            let instr = encoder.next().unwrap().unwrap();
            assert!(instr.is_valid());
            assert_eq!(instr.word(), word);

            assert_eq!(encoder.next(), None);
        }
    }
}
