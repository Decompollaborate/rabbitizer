/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

#[cfg(not(feature = "std"))]
use core::error;
#[cfg(feature = "std")]
use std::error;

use crate::opcodes::Opcode;
use crate::operands::Operand;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncodingError<'s> {
    CommaInsteadOfOpcode,
    UnrecognizedOpcode(&'s str),
    RanOutOfTokens(Opcode, Operand),
    EndTokenInsteadOfOperand(Opcode, Operand),
    CommaInsteadOfOperand(Opcode, Operand),
    UnrecognizedOperand(Opcode, &'s str, Operand),
    EndButMissingOperands(Opcode, usize),
    TokenInsteadOfComma(Opcode, &'s str),
    MissingCommaInComposedOperand(Opcode, Operand),
}

impl fmt::Display for EncodingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::CommaInsteadOfOpcode => write!(f, "Found a comma instead of an opcode"),
            Self::UnrecognizedOpcode(text) => write!(f, "The token '{}' did not match any known opcode", text),
            Self::RanOutOfTokens(opcode, operand) => write!(f, "Unable to decode opcode '{:?}': Ran out of tokens before being able to decode operand '{:?}'", opcode, operand),
            Self::EndTokenInsteadOfOperand(opcode, operand) => write!(f, "Unable to decode opcode '{:?}': End token found instead of operand for '{:?}'", opcode, operand),
            Self::CommaInsteadOfOperand(opcode, operand) => write!(f, "Unable to decode opcode '{:?}': Comma found instead of operand for '{:?}'", opcode, operand),
            Self::UnrecognizedOperand(opcode, text, operand) => write!(f, "Unable to decode opcode '{:?}': The token '{}' could not be encoded as operand '{:?}'", opcode, text, operand),
            Self::EndButMissingOperands(opcode, reamining_operands) => write!(f, "Unable to decode opcode '{:?}': Reached end of stream, but there were still {} operands to process", opcode, reamining_operands),
            Self::TokenInsteadOfComma(opcode, text) => write!(f, "Unable to decode opcode '{:?}': Found token '{}' instead of comma while parsing operands", opcode, text),
            Self::MissingCommaInComposedOperand(opcode, operand) => write!(f, "Unable to decode opcode '{:?}': Missing comma in composed operand '{:?}'", opcode, operand),
        }
    }
}

impl error::Error for EncodingError<'_> {}
