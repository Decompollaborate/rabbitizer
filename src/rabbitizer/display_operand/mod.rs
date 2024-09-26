/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt::{self, UpperHex};

use crate::{DisplayFlags, Instruction, Operand, DISPLAY_OPERAND_CALLBACKS};

pub(crate) mod display_operand_none;
pub(crate) mod display_operand_r3000gte;
pub(crate) mod display_operand_r4000allegrex;
pub(crate) mod display_operand_r5900;
pub(crate) mod display_operand_rsp;

pub struct DisplayOperand<'ins, 'imm> {
    operand: Operand,
    instr: &'ins Instruction,
    imm_override: Option<&'imm str>,
    display_flags: DisplayFlags,
}

impl<'ins, 'imm> DisplayOperand<'ins, 'imm> {
    pub(crate) const fn new(
        operand: Operand,
        instr: &'ins Instruction,
        imm_override: Option<&'imm str>,
        display_flags: DisplayFlags,
    ) -> Self {
        Self {
            operand,
            instr,
            imm_override,
            display_flags,
        }
    }
}

impl<'ins, 'imm> DisplayOperand<'ins, 'imm> {
    #[allow(non_snake_case)]
    pub(crate) fn display_ALL_EMPTY(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Err(fmt::Error)
    }
}

pub(crate) type DisplayOperandCallback =
    for<'a, 'b, 'c> fn(&'a DisplayOperand, &'b mut fmt::Formatter<'c>) -> fmt::Result;

impl fmt::Display for DisplayOperand<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DISPLAY_OPERAND_CALLBACKS[self.operand as usize](self, f)
    }
}

pub(crate) fn display_hex<T>(number: T, f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    T: PartialOrd + UpperHex + Default + core::ops::Neg,
    <T as core::ops::Neg>::Output: UpperHex,
{
    if number < T::default() {
        write!(f, "-0x{:X}", -number)
    } else {
        write!(f, "0x{:X}", number)
    }
}

#[cfg(feature = "bindings_c")]
mod stuff {
    use core::fmt;
    use core::fmt::Write;

    struct Counter {
        size: usize,
    }
    impl fmt::Write for Counter {
        #[inline]
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.size += s.len();
            Ok(())
        }
    }

    struct Buffer<'data> {
        data: &'data mut [u8],
        pos: usize,
    }
    impl fmt::Write for Buffer<'_> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            // TODO: test this implementation actually works.

            if self.pos + s.len() > self.data.len() {
                return Err(fmt::Error);
            }

            self.data[self.pos..].copy_from_slice(s.as_bytes());
            self.pos += s.len();

            Ok(())
        }
    }

    fn test(a: &super::DisplayOperand, buf: &mut Buffer) -> fmt::Result {
        write!(buf, "{}", a)
    }

    fn test2(a: &super::DisplayOperand, buf: &mut Counter) -> fmt::Result {
        write!(buf, "{}", a)
    }
}
