/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::operands::OperandDisplay;
use crate::traits::Register;

impl<T> OperandDisplay<'_, '_, T>
where
    T: fmt::Display,
{
    pub(crate) fn display_r4000allegrex_s_vs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_s_vs_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_s_vt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_s_vt_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_s_vd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_s_vd_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_s_vt_imm(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_s_vt_imm_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_s_vd_imm(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_s_vd_imm_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_p_vs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_p_vs_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_p_vt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_p_vt_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_p_vd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_p_vd_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_t_vs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_t_vs_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_t_vt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_t_vt_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_t_vd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_t_vd_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_q_vs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_q_vs_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_q_vt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_q_vt_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_q_vd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_q_vd_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_q_vt_imm(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_q_vt_imm_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_mp_vs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mp_vs_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mp_vt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mp_vt_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mp_vd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mp_vd_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mp_vs_transpose(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mp_vs_transpose_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_mt_vs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mt_vs_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mt_vt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mt_vt_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mt_vd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mt_vd_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mt_vs_transpose(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mt_vs_transpose_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_mq_vs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mq_vs_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mq_vt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mq_vt_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mq_vd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mq_vd_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_mq_vs_transpose(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_mq_vs_transpose_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_cop2cs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_cop2cs_unchecked();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_r4000allegrex_vfpucontrol(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_cop2cd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_cop2cd_unchecked();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_r4000allegrex_vfpucontrol(),
        );

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_pos(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_pos_unchecked();

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_size(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_size_unchecked() + 1;

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_size_plus_pos(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_size_plus_pos_unchecked() + 1
            - (instr.field_r4000allegrex_pos_unchecked() as i8);

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_imm3(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_imm3_unchecked();

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_offset14(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            let instr = myself.instr;
            let s = instr.field_r4000allegrex_offset14_unchecked();

            write!(f, "0x{:X}", s)
        })
    }
    pub(crate) fn display_r4000allegrex_offset14_base(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if false {
            if myself.imm_override.is_some()
                || myself.instr.field_r4000allegrex_offset14_unchecked() != 0
            {
                Self::display_r4000allegrex_offset14(myself, f)?;
            }
        } else {
            Self::display_r4000allegrex_offset14(myself, f)?;
        }

        write!(f, "(")?;
        Self::display_core_rs(myself, f)?;
        write!(f, ")")
    }
    pub(crate) fn display_r4000allegrex_offset14_base_maybe_wb(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_r4000allegrex_offset14_base(myself, f)?;

        if myself.instr.field_r4000allegrex_wb_unchecked() {
            write!(f, ", wb")?;
        }

        Ok(())
    }

    pub(crate) fn display_r4000allegrex_vcmp_cond(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_vcmp_cond_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let (_, vs, vt) = instr.get_r4000allegrex_vcmp_s_args_unchecked();

        Self::display_r4000allegrex_vcmp_cond(myself, f)?;

        if vs.is_some() {
            write!(f, ", ")?;
            Self::display_r4000allegrex_s_vs(myself, f)?;

            if vt.is_some() {
                write!(f, ", ")?;
                Self::display_r4000allegrex_s_vt(myself, f)?;
            }
        }
        Ok(())
    }
    pub(crate) fn display_r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let (_, vs, vt) = instr.get_r4000allegrex_vcmp_p_args_unchecked();

        Self::display_r4000allegrex_vcmp_cond(myself, f)?;

        if vs.is_some() {
            write!(f, ", ")?;
            Self::display_r4000allegrex_p_vs(myself, f)?;

            if vt.is_some() {
                write!(f, ", ")?;
                Self::display_r4000allegrex_p_vt(myself, f)?;
            }
        }
        Ok(())
    }
    pub(crate) fn display_r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let (_, vs, vt) = instr.get_r4000allegrex_vcmp_t_args_unchecked();

        Self::display_r4000allegrex_vcmp_cond(myself, f)?;

        if vs.is_some() {
            write!(f, ", ")?;
            Self::display_r4000allegrex_t_vs(myself, f)?;

            if vt.is_some() {
                write!(f, ", ")?;
                Self::display_r4000allegrex_t_vt(myself, f)?;
            }
        }
        Ok(())
    }
    pub(crate) fn display_r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let (_, vs, vt) = instr.get_r4000allegrex_vcmp_q_args_unchecked();

        Self::display_r4000allegrex_vcmp_cond(myself, f)?;

        if vs.is_some() {
            write!(f, ", ")?;
            Self::display_r4000allegrex_q_vs(myself, f)?;

            if vt.is_some() {
                write!(f, ", ")?;
                Self::display_r4000allegrex_q_vt(myself, f)?;
            }
        }
        Ok(())
    }

    pub(crate) fn display_r4000allegrex_vconstant(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_vconstant_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_power_of_two(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_power_of_two_unchecked();

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_vfpu_cc_bit(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_vfpu_cc_bit_unchecked();

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_bn(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_bn_unchecked();

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_int16(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_int16_unchecked();

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_float16(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_float16_unchecked();

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_p_vrot_code(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        const MNEMONICS: [&str; 32] = [
            "[C,S]",   // [0]
            "[S,C]",   // [1]
            "[S,0]",   // [2]
            "[S,0]",   // [3]
            "[C,S]",   // [4]
            "[S,C]",   // [5]
            "[0,S]",   // [6]
            "[0,S]",   // [7]
            "[C,0]",   // [8]
            "[0,C]",   // [9]
            "[S,S]",   // [10]
            "[0,0]",   // [11]
            "[C,0]",   // [12]
            "[0,C]",   // [13]
            "[0,0]",   // [14]
            "[S,S]",   // [15]
            "[C,-S]",  // [16]
            "[-S,C]",  // [17]
            "[-S,0]",  // [18]
            "[-S,0]",  // [19]
            "[C,-S]",  // [20]
            "[-S,C]",  // [21]
            "[0,-S]",  // [22]
            "[0,-S]",  // [23]
            "[C,0]",   // [24]
            "[0,C]",   // [25]
            "[-S,-S]", // [26]
            "[0,0]",   // [27]
            "[C,0]",   // [28]
            "[0,C]",   // [29]
            "[0,0]",   // [30]
            "[-S,-S]", // [31]
        ];
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_vrot_code_unchecked();

        write!(f, "{}", MNEMONICS[s as usize])
    }
    pub(crate) fn display_r4000allegrex_t_vrot_code(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        const MNEMONICS: [&str; 32] = [
            "[C,S,S]",    // [0]
            "[S,C,0]",    // [1]
            "[S,0,C]",    // [2]
            "[S,0,0]",    // [3]
            "[C,S,0]",    // [4]
            "[S,C,S]",    // [5]
            "[0,S,C]",    // [6]
            "[0,S,0]",    // [7]
            "[C,0,S]",    // [8]
            "[0,C,S]",    // [9]
            "[S,S,C]",    // [10]
            "[0,0,S]",    // [11]
            "[C,0,0]",    // [12]
            "[0,C,0]",    // [13]
            "[0,0,C]",    // [14]
            "[S,S,S]",    // [15]
            "[C,-S,-S]",  // [16]
            "[-S,C,0]",   // [17]
            "[-S,0,C]",   // [18]
            "[-S,0,0]",   // [19]
            "[C,-S,0]",   // [20]
            "[-S,C,-S]",  // [21]
            "[0,-S,C]",   // [22]
            "[0,-S,0]",   // [23]
            "[C,0,-S]",   // [24]
            "[0,C,-S]",   // [25]
            "[-S,-S,C]",  // [26]
            "[0,0,-S]",   // [27]
            "[C,0,0]",    // [28]
            "[0,C,0]",    // [29]
            "[0,0,C]",    // [30]
            "[-S,-S,-S]", // [31]
        ];
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_vrot_code_unchecked();

        write!(f, "{}", MNEMONICS[s as usize])
    }
    pub(crate) fn display_r4000allegrex_q_vrot_code(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        const MNEMONICS: [&str; 32] = [
            "[C,S,S,S]",    // [0]
            "[S,C,0,0]",    // [1]
            "[S,0,C,0]",    // [2]
            "[S,0,0,C]",    // [3]
            "[C,S,0,0]",    // [4]
            "[S,C,S,S]",    // [5]
            "[0,S,C,0]",    // [6]
            "[0,S,0,C]",    // [7]
            "[C,0,S,0]",    // [8]
            "[0,C,S,0]",    // [9]
            "[S,S,C,S]",    // [10]
            "[0,0,S,C]",    // [11]
            "[C,0,0,S]",    // [12]
            "[0,C,0,S]",    // [13]
            "[0,0,C,S]",    // [14]
            "[S,S,S,C]",    // [15]
            "[C,-S,-S,-S]", // [16]
            "[-S,C,0,0]",   // [17]
            "[-S,0,C,0]",   // [18]
            "[-S,0,0,C]",   // [19]
            "[C,-S,0,0]",   // [20]
            "[-S,C,-S,-S]", // [21]
            "[0,-S,C,0]",   // [22]
            "[0,-S,0,C]",   // [23]
            "[C,0,-S,0]",   // [24]
            "[0,C,-S,0]",   // [25]
            "[-S,-S,C,-S]", // [26]
            "[0,0,-S,C]",   // [27]
            "[C,0,0,-S]",   // [28]
            "[0,C,0,-S]",   // [29]
            "[0,0,C,-S]",   // [30]
            "[-S,-S,-S,C]", // [31]
        ];
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_vrot_code_unchecked();

        write!(f, "{}", MNEMONICS[s as usize])
    }

    pub(crate) fn display_r4000allegrex_wpx(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_wpx_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_wpy(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_wpy_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_wpz(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_wpz_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_wpw(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_wpw_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_rpx(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_rpx_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_rpy(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_rpy_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_rpz(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_rpz_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_rpw(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_rpw_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
}
