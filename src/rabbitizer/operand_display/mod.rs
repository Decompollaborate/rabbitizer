/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::{DisplayFlags, Instruction, Operand, DISPLAY_OPERAND_CALLBACKS};

pub(crate) mod operand_display_none;
pub(crate) mod operand_display_r3000gte;
pub(crate) mod operand_display_r4000allegrex;
pub(crate) mod operand_display_r5900;
pub(crate) mod operand_display_rsp;

pub struct OperandDisplay<'ins, 'imm, 'flg> {
    operand: Operand,
    instr: &'ins Instruction,
    imm_override: Option<&'imm str>,
    display_flags: &'flg DisplayFlags,
}

impl<'ins, 'imm, 'flg> OperandDisplay<'ins, 'imm, 'flg> {
    pub(crate) const fn new(
        operand: Operand,
        instr: &'ins Instruction,
        imm_override: Option<&'imm str>,
        display_flags: &'flg DisplayFlags,
    ) -> Self {
        Self {
            operand,
            instr,
            imm_override,
            display_flags,
        }
    }
}

impl<'ins, 'imm, 'flg> OperandDisplay<'ins, 'imm, 'flg> {
    #[allow(non_snake_case)]
    pub(crate) fn display_ALL_EMPTY(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Err(fmt::Error)
    }

    pub(crate) fn display_imm_override_or(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
        callback: fn(&OperandDisplay, &mut fmt::Formatter<'_>) -> fmt::Result,
    ) -> fmt::Result {
        if let Some(imm_override) = myself.imm_override {
            write!(f, "{}", imm_override)
        } else {
            callback(myself, f)
        }
    }
}

pub(crate) type OperandDisplayCallback =
    for<'a, 'b, 'c> fn(&'a OperandDisplay, &'b mut fmt::Formatter<'c>) -> fmt::Result;

impl fmt::Display for OperandDisplay<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DISPLAY_OPERAND_CALLBACKS[self.operand as usize](self, f)
    }
}

pub(crate) fn display_signed_imm(
    number: i32,
    f: &mut fmt::Formatter<'_>,
    display_flags: &DisplayFlags,
) -> fmt::Result {
    if display_flags.omit_0x_on_small_imm() && number > -10 && number < 10 {
        return write!(f, "{}", number);
    }

    if number < 0 {
        write!(f, "-0x{:X}", -number)
    } else {
        write!(f, "0x{:X}", number)
    }
}

#[cfg(feature = "bindings_c")]
#[allow(dead_code)]
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

    fn test(a: &super::OperandDisplay, buf: &mut Buffer) -> fmt::Result {
        write!(buf, "{}", a)
    }

    fn test2(a: &super::OperandDisplay, buf: &mut Counter) -> fmt::Result {
        write!(buf, "{}", a)
    }
}
