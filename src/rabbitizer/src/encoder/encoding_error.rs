/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

#[cfg(not(feature = "std"))]
use core::error;
#[cfg(feature = "std")]
use std::error;

use crate::encoder::BracketType;
use crate::opcodes::Opcode;
use crate::operands::Operand;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncodingError<'s> {
    CommaInsteadOfOpcode,
    UnrecognizedOpcode(&'s str),
    BracketedInsteadOfOpcode(&'s str, &'s str, BracketType),
    RanOutOfTokens(Opcode, Operand),
    EndTokenInsteadOfOperand(Opcode, Operand),
    CommaInsteadOfOperand(Opcode, Operand),
    BracketedInsteadOfSingleOperand(Opcode, Operand, &'s str, &'s str, BracketType),
    TextInsteadOfBracketedOperand(Opcode, Operand, &'s str, BracketType),
    WrongBracketedOperand(Opcode, Operand, &'s str, &'s str, BracketType, BracketType),
    UnrecognizedOperand(Opcode, &'s str, Option<(&'s str, BracketType)>, Operand),
    EndButMissingOperands(Opcode, usize),
    TokenInsteadOfCommaEnd(Opcode, Operand, &'s str),
    BracketedInsteadOfCommaEnd(Opcode, Operand, &'s str, &'s str, BracketType),
    MissingCommaInComposedOperand(Opcode, Operand),
}

impl fmt::Display for EncodingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::CommaInsteadOfOpcode => write!(f, "Found a comma instead of an opcode"),
            Self::UnrecognizedOpcode(text) => write!(f, "The token '{}' did not match any known opcode", text),
            Self::BracketedInsteadOfOpcode(left, right, bracket_type) => {
                write!(f, "The token '")?;
                write_bracketed(f, bracket_type, left, right)?;
                write!(f, "' is not a valid opcode")
            }
            Self::RanOutOfTokens(opcode, operand) => write!(f, "Unable to encode opcode '{:?}': Ran out of tokens before being able to encode operand '{:?}'", opcode, operand),
            Self::EndTokenInsteadOfOperand(opcode, operand) => write!(f, "Unable to encode opcode '{:?}': End token found instead of operand for '{:?}'", opcode, operand),
            Self::CommaInsteadOfOperand(opcode, operand) => write!(f, "Unable to encode opcode '{:?}': Comma found instead of operand for '{:?}'", opcode, operand),
            Self::BracketedInsteadOfSingleOperand(opcode, operand, left, right, bracket_type) => {
                write!(f, "Unable to encode opcode '{:?}': Token '", opcode)?;
                write_bracketed(f, bracket_type, left, right)?;
                write!(f, "' found instead of operand for '{:?}'", operand)
            }
            Self::TextInsteadOfBracketedOperand(opcode, operand, text, bracket_type) => write!(f, "Unable to encode opcode '{:?}': Found token '{}' instead of bracketed expression '{}{}' when trying to encode operand '{:?}'", opcode, text, bracket_type.left(), bracket_type.right(), operand),
            Self::WrongBracketedOperand(opcode, operand, left, right, bracket_type, required_bracket_type) => {
                write!(f, "Unable to encode opcode '{:?}': Found wrong kind of bracketed expression when trying to encode operand '{:?}'. ", opcode, operand)?;
                write!(f, "Expected expression '")?;
                write_bracketed(f, required_bracket_type, left, right)?;
                write!(f, "', but found '")?;
                write_bracketed(f, bracket_type, left, right)?;
                write!(f, "'")
            }
            Self::UnrecognizedOperand(opcode, text, right, operand) => {
                write!(f, "Unable to encode opcode '{:?}': The token '", opcode)?;
                if let Some((right, bracket_type)) = right {
                    write_bracketed(f, bracket_type, text, right)?;
                } else {
                    write!(f, "{}", text)?;
                }
                write!(f, "' could not be encoded as operand '{:?}'", operand)
            }
            Self::EndButMissingOperands(opcode, reamining_operands) => write!(f, "Unable to encode opcode '{:?}': Reached end of stream, but there were still {} operands to process", opcode, reamining_operands),
            Self::TokenInsteadOfCommaEnd(opcode, operand, text) => write!(f, "Unable to encode opcode '{:?}': Found unrecognized token '{}' instead of comma or end while processing operand '{:?}'", opcode, text, operand),
            Self::BracketedInsteadOfCommaEnd(opcode, operand, left, right, bracket_type) => {
                write!(f, "Unable to encode opcode '{:?}': Found unrecognized token '", opcode)?;
                write_bracketed(f, bracket_type, left, right)?;
                write!(f, "' instead of comma or end while processing operand '{:?}'", operand)
            }
            Self::MissingCommaInComposedOperand(opcode, operand) => write!(f, "Unable to encode opcode '{:?}': Missing comma in composed operand '{:?}'", opcode, operand),
        }
    }
}

impl error::Error for EncodingError<'_> {}

fn write_bracketed(
    f: &mut fmt::Formatter<'_>,
    bracket_type: BracketType,
    left: &str,
    right: &str,
) -> fmt::Result {
    write!(
        f,
        "{}{}{}{}",
        left,
        bracket_type.left(),
        right,
        bracket_type.right()
    )
}
