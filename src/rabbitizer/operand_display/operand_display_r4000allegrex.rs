/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::{traits::Register, OperandDisplay};

impl<'ins, 'imm, 'flg> OperandDisplay<'ins, 'imm, 'flg> {
    pub(crate) fn display_r4000allegrex_s_vs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_s_vs_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_s_vt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_s_vt_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_s_vd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_s_vd_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_s_vt_imm(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_s_vt_imm_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_s_vd_imm(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_s_vd_imm_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_p_vs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_p_vs_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_p_vt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_p_vt_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_p_vd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_p_vd_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_t_vs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_t_vs_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_t_vt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_t_vt_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_t_vd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_t_vd_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_q_vs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_q_vs_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_q_vt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_q_vt_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_q_vd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_q_vd_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_q_vt_imm(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_q_vt_imm_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_mp_vs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mp_vs_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mp_vt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mp_vt_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mp_vd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mp_vd_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mp_vs_transpose(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mp_vs_transpose_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_mt_vs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mt_vs_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mt_vt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mt_vt_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mt_vd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mt_vd_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mt_vs_transpose(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mt_vs_transpose_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_mq_vs(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mq_vs_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mq_vt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mq_vt_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mq_vd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mq_vd_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mq_vs_transpose(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mq_vs_transpose_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_cop2cs(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_cop2cd(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }

    pub(crate) fn display_r4000allegrex_pos(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_size(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_size_plus_pos(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }

    pub(crate) fn display_r4000allegrex_imm3(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }

    pub(crate) fn display_r4000allegrex_offset14_base(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_offset14_base_maybe_wb(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }

    pub(crate) fn display_r4000allegrex_vcmp_cond(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }

    pub(crate) fn display_r4000allegrex_vconstant(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }

    pub(crate) fn display_r4000allegrex_power_of_two(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_vfpu_cc_bit(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_bn(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }

    pub(crate) fn display_r4000allegrex_int16(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_float16(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }

    pub(crate) fn display_r4000allegrex_p_vrot_code(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_t_vrot_code(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_q_vrot_code(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }

    pub(crate) fn display_r4000allegrex_rpx(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_rpy(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_rpz(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_rpw(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }

    pub(crate) fn display_r4000allegrex_wpx(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_wpy(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_wpz(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_r4000allegrex_wpw(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
}
