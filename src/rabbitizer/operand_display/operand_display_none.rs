/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::{operand_display, traits::Register, OperandDisplay};

impl<'ins, 'imm, 'flg> OperandDisplay<'ins, 'imm, 'flg> {
    pub(crate) fn display_cpu_rs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.reg_rs_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_gpr());

        write!(f, "{}", s)
    }

    pub(crate) fn display_cpu_rt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.reg_rt_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_gpr());

        write!(f, "{}", s)
    }

    pub(crate) fn display_cpu_rd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.reg_rd_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_gpr());

        write!(f, "{}", s)
    }
    pub(crate) fn display_cpu_sa(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_zero(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_cop0d(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_fs(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_ft(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_fd(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_cop1cs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_cop1cs_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_cpu_cop2t(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_cop2cd(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_op(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_hint(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_code(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_code_lower(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_copraw(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_label(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            let instr = myself.instr;
            let s = instr.get_instr_index_as_vram_unchecked();

            write!(f, "func_{:08X}", s)
        })
    }
    pub(crate) fn display_cpu_immediate(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            let instr = myself.instr;
            let s = instr.get_processed_immediate_unchecked();

            operand_display::display_hex(s, f)
        })
    }
    pub(crate) fn display_cpu_branch_target_label(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            write!(f, ". + 4 + (")?;
            Self::display_cpu_immediate(myself, f)?;
            write!(f, " << 2)")
        })
    }
    pub(crate) fn display_cpu_immediate_base(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if false {
            if myself.imm_override.is_some() || myself.instr.field_immediate_unchecked() != 0 {
                Self::display_cpu_immediate(myself, f)?;
            }
        } else {
            Self::display_cpu_immediate(myself, f)?;
        }

        write!(f, "(")?;
        Self::display_cpu_rs(myself, f)?;
        write!(f, ")")
    }
    pub(crate) fn display_cpu_maybe_rd_rs(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
}
