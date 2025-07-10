/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::display_flags::InstructionDisplayFlags;
use crate::instr::Instruction;
use crate::operands::Operand;

pub(crate) mod operand_display_core;
#[cfg(feature = "R3000GTE")]
pub(crate) mod operand_display_r3000gte;
#[cfg(feature = "R4000ALLEGREX")]
pub(crate) mod operand_display_r4000allegrex;
#[cfg(feature = "R5900EE")]
pub(crate) mod operand_display_r5900ee;
#[cfg(feature = "RSP")]
pub(crate) mod operand_display_rsp;

pub(crate) mod default_label_display;

pub use default_label_display::DefaultLabelDisplay;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[must_use]
pub struct OperandDisplay<'ins, 'flg, T> {
    operand: Operand,
    instr: &'ins Instruction,
    display_flags: &'flg InstructionDisplayFlags,
    imm_override: Option<T>,
}

impl<'ins, 'flg, T> OperandDisplay<'ins, 'flg, T>
where
    T: fmt::Display,
{
    pub(crate) const fn new(
        operand: Operand,
        instr: &'ins Instruction,
        display_flags: &'flg InstructionDisplayFlags,
        imm_override: Option<T>,
    ) -> Self {
        Self {
            operand,
            instr,
            display_flags,
            imm_override,
        }
    }

    pub(crate) const fn operand(&self) -> Operand {
        self.operand
    }
}

impl<T> OperandDisplay<'_, '_, T> {
    #[allow(non_snake_case)]
    pub(crate) fn display_ALL_EMPTY(
        _myself: &OperandDisplay<T>,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Err(fmt::Error)
    }
}

impl<T> OperandDisplay<'_, '_, T>
where
    T: fmt::Display,
{
    #[inline]
    pub(crate) fn display_imm_override_or(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
        callback: fn(&OperandDisplay<T>, &mut fmt::Formatter<'_>) -> fmt::Result,
    ) -> fmt::Result {
        if let Some(imm_override) = myself.imm_override.as_ref() {
            write!(f, "{}", imm_override)
        } else {
            callback(myself, f)
        }
    }
}

impl<T> fmt::Display for OperandDisplay<'_, '_, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_this_operand(f)
    }
}

pub(crate) fn display_signed_imm(
    number: i32,
    f: &mut fmt::Formatter<'_>,
    display_flags: &InstructionDisplayFlags,
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

    fn test<T: fmt::Display>(a: &super::OperandDisplay<T>, buf: &mut Buffer) -> fmt::Result {
        write!(buf, "{}", a)
    }

    fn test2<T: fmt::Display>(a: &super::OperandDisplay<T>, buf: &mut Counter) -> fmt::Result {
        write!(buf, "{}", a)
    }
}
