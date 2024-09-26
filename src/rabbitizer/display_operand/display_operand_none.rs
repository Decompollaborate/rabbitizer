/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::{display_operand, traits::Register, DisplayOperand};

impl<'ins, 'imm> DisplayOperand<'ins, 'imm> {
    pub(crate) fn display_cpu_rs(
        myself: &DisplayOperand,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.reg_rs_unchecked().named_reg(instr.flags().abi_gpr());

        write!(f, "{}", s)
    }

    pub(crate) fn display_cpu_rt(
        myself: &DisplayOperand,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.reg_rt_unchecked().named_reg(instr.flags().abi_gpr());

        write!(f, "{}", s)
    }

    pub(crate) fn display_cpu_rd(
        myself: &DisplayOperand,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.reg_rd_unchecked().named_reg(instr.flags().abi_gpr());

        write!(f, "{}", s)
    }
    pub(crate) fn display_cpu_sa(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_zero(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_cop0d(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_fs(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_ft(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_fd(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_cop1cs(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_cop2t(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_cop2cd(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_op(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_hint(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_code(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_code_lower(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_copraw(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_label(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_immediate(
        myself: &DisplayOperand,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if let Some(imm_override) = myself.imm_override {
            write!(f, "{}", imm_override)
        } else {
            let instr = myself.instr;
            let s = instr.get_processed_immediate_unchecked();

            display_operand::display_hex(s, f)
        }
    }
    pub(crate) fn display_cpu_branch_target_label(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_immediate_base(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_cpu_maybe_rd_rs(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
}
