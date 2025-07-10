/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::operands::{operand_display, DefaultLabelDisplay, OperandDisplay};
use crate::utils;
use crate::{registers::Gpr, registers_meta::Register};

impl<T> OperandDisplay<'_, '_, T>
where
    T: fmt::Display,
{
    pub(crate) fn display_core_rs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().rs_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_gpr(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }

    pub(crate) fn display_core_rt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().rt_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_gpr(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }

    pub(crate) fn display_core_rd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().rd_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_gpr(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_sa(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field().sa_impl();

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_zero(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = Gpr::zero;
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_gpr(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_cop0d(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().cop0d_impl();

        let use_named;

        #[cfg(feature = "MIPS_III")]
        {
            use_named = myself.display_flags.named_vr4300_cop0();
        }
        #[cfg(not(feature = "MIPS_III"))]
        {
            use_named = true;
        }

        let s = reg.either_name(instr.abi(), use_named, !myself.display_flags.use_dollar());

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_cop0cd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().cop0cd_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_registers(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_fs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().fs_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_fpr(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_ft(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().ft_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_fpr(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_fd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().fd_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_fpr(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_cop1cs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().cop1cs_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_registers(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_cop2t(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().cop2t_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_registers(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_cop2d(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().cop2d_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_registers(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_cop2cd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().cop2cd_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_registers(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_op(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field().op_impl();

        write!(f, "0x{:02X}", s)
    }
    pub(crate) fn display_core_hint(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field().hint_impl();

        write!(f, "0x{:02X}", s)
    }
    pub(crate) fn display_core_code(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let code_upper = instr.field().code_upper_impl();
        let code_lower = instr.field().code_lower_impl();

        write!(f, "{}", code_upper)?;
        if code_lower != 0 {
            write!(f, ", {}", code_lower)?;
        }
        Ok(())
    }
    pub(crate) fn display_core_code_lower(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let code_lower = instr.field().code_lower_impl();

        write!(f, "{}", code_lower)
    }
    pub(crate) fn display_core_copraw(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        // TODO: either get rid of this or move to EncodedFieldMask/add as a Instruction function.
        let val = myself.instr.word() & utils::bitmask(0, 25);
        write!(f, "0x{:X}", val)
    }
    pub(crate) fn display_core_label(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            match myself.display_flags.jump_default_label_display() {
                DefaultLabelDisplay::FullExpression => {
                    write!(f, ". + 4 + (")?;
                    Self::display_core_imm_i16(myself, f)?;
                    write!(f, " << 2)")
                }
                DefaultLabelDisplay::Computed => {
                    let s = myself.instr.field().imm_i16_impl();
                    let computed = 4 + (i32::from(s) << 2);

                    write!(f, ". + ")?;
                    operand_display::display_signed_imm(computed, f, myself.display_flags)
                }
                DefaultLabelDisplay::Absolute => {
                    let s = myself.instr.get_instr_index_as_vram_impl();

                    write!(f, "func_{}", s)
                }
            }
        })
    }
    pub(crate) fn display_core_imm_i16(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            let instr = myself.instr;
            let s = instr.field().imm_i16_impl();

            operand_display::display_signed_imm(s.into(), f, myself.display_flags)
        })
    }
    pub(crate) fn display_core_imm_u16(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            let instr = myself.instr;
            let s = instr.field().imm_u16_impl();

            operand_display::display_signed_imm(s.into(), f, myself.display_flags)
        })
    }
    pub(crate) fn display_core_branch_target_label(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            match myself.display_flags.branch_default_label_display() {
                DefaultLabelDisplay::FullExpression => {
                    write!(f, ". + 4 + (")?;
                    Self::display_core_imm_i16(myself, f)?;
                    write!(f, " << 2)")
                }
                DefaultLabelDisplay::Computed => {
                    let s = myself.instr.field().imm_i16_impl();
                    let computed = 4 + (i32::from(s) << 2);

                    write!(f, ". + ")?;
                    operand_display::display_signed_imm(computed, f, myself.display_flags)
                }
                DefaultLabelDisplay::Absolute => {
                    let s = myself.instr.field().imm_i16_impl();
                    let absolute = myself
                        .instr
                        .vram()
                        .inner()
                        .wrapping_add_signed(4 + (i32::from(s) << 2));

                    write!(f, "0x{:08X}", absolute)
                }
            }
        })
    }
    pub(crate) fn display_core_imm_rs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if false {
            if myself.imm_override.is_some() || myself.instr.field().imm_i16_impl() != 0 {
                Self::display_core_imm_i16(myself, f)?;
            }
        } else {
            Self::display_core_imm_i16(myself, f)?;
        }

        write!(f, "(")?;
        Self::display_core_rs(myself, f)?;
        write!(f, ")")
    }
    pub(crate) fn display_core_maybe_rd_rs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().rd_impl();

        if !reg.holds_return_address(instr.abi()) || myself.display_flags.expand_jalr() {
            Self::display_core_rd(myself, f)?;
            write!(f, ", ")?;
        }

        Self::display_core_rs(myself, f)
    }
    pub(crate) fn display_core_maybe_zero_rs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if myself.display_flags.gnu_div() {
            Self::display_core_zero(myself, f)?;
            write!(f, ", ")?;
        }

        Self::display_core_rs(myself, f)
    }
}
