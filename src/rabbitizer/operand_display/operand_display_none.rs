/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::{generated::Gpr, operand_display, traits::Register, OperandDisplay};

impl<'ins, 'imm, 'flg> OperandDisplay<'ins, 'imm, 'flg> {
    pub(crate) fn display_core_rs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_rs_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_gpr());

        write!(f, "{}", s)
    }

    pub(crate) fn display_core_rt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_rt_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_gpr());

        write!(f, "{}", s)
    }

    pub(crate) fn display_core_rd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_rd_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_gpr());

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_sa(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_sa_unchecked();

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_zero(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = Gpr::zero;
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_gpr());

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_cop0d(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_cop0d_unchecked();
        let s = reg.either_name(
            instr.flags().abi(),
            myself.display_flags.named_vr4300_cop0(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_cop0cd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_cop0cd_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_fs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_fs_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_fpr());

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_ft(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_ft_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_fpr());

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_fd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_fd_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_fpr());

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_cop1cs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_cop1cs_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_cop2t(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_cop2t_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_cop2d(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_cop2d_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_cop2cd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_cop2cd_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_core_op(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_op_unchecked();

        write!(f, "0x{:02X}", s)
    }
    pub(crate) fn display_core_hint(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_hint_unchecked();

        write!(f, "0x{:02X}", s)
    }
    pub(crate) fn display_core_code(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let code_upper = instr.field_code_upper_unchecked();
        let code_lower = instr.field_code_lower_unchecked();

        write!(f, "{}", code_upper)?;
        if code_lower != 0 {
            write!(f, ", {}", code_lower)?;
        }
        Ok(())
    }
    pub(crate) fn display_core_code_lower(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let code_lower = instr.field_code_lower_unchecked();

        write!(f, "{}", code_lower)
    }
    pub(crate) fn display_core_copraw(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_core_label(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            let instr = myself.instr;
            let s = instr.get_instr_index_as_vram_unchecked();

            write!(f, "func_{:08X}", s)
        })
    }
    pub(crate) fn display_core_immediate(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            let instr = myself.instr;
            let s = instr.get_processed_immediate_unchecked();

            operand_display::display_signed_imm(s, f, myself.display_flags)
        })
    }
    pub(crate) fn display_core_branch_target_label(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            write!(f, ". + 4 + (")?;
            Self::display_core_immediate(myself, f)?;
            write!(f, " << 2)")
        })
    }
    pub(crate) fn display_core_immediate_base(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if false {
            if myself.imm_override.is_some() || myself.instr.field_immediate_unchecked() != 0 {
                Self::display_core_immediate(myself, f)?;
            }
        } else {
            Self::display_core_immediate(myself, f)?;
        }

        write!(f, "(")?;
        Self::display_core_rs(myself, f)?;
        write!(f, ")")
    }
    pub(crate) fn display_core_maybe_rd_rs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_rd_unchecked();

        if !reg.holds_return_address(instr.flags().abi()) || myself.display_flags.expand_jalr() {
            Self::display_core_rd(myself, f)?;
            write!(f, ", ")?;
        }

        Self::display_core_rs(myself, f)
    }
    pub(crate) fn display_core_maybe_zero_rs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if myself.display_flags.gnu_div() {
            Self::display_core_zero(myself, f)?;
            write!(f, ", ")?;
        }

        Self::display_core_rs(myself, f)
    }
}
