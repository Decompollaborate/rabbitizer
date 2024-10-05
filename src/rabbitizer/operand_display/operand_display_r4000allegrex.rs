/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::registers::{R4000AllegrexS, R4000AllegrexV2D, R4000AllegrexV3D, R4000AllegrexV4D};
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
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_cop2cs_unchecked();
        let s = reg.either_name(
            instr.flags().abi(),
            myself.display_flags.named_r4000allegrex_vfpucontrol(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_cop2cd(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_cop2cd_unchecked();
        let s = reg.either_name(
            instr.flags().abi(),
            myself.display_flags.named_r4000allegrex_vfpucontrol(),
        );

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_pos(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_pos_unchecked();

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_size(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_size_unchecked() + 1;

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_size_plus_pos(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_size_plus_pos_unchecked() + 1
            - (instr.field_r4000allegrex_pos_unchecked() as i8);

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_imm3(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_imm3_unchecked();

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_offset14(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            let instr = myself.instr;
            let s = instr.field_r4000allegrex_offset14_unchecked();

            write!(f, "{}", s)
        })
    }
    pub(crate) fn display_r4000allegrex_offset14_base(
        myself: &OperandDisplay,
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
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_r4000allegrex_offset14_base(myself, f)?;

        if myself.instr.field_r4000allegrex_wb_unchecked() {
            write!(f, ", wb")?;
        }

        Ok(())
    }

    pub(crate) fn display_r4000allegrex_vcmp_cond(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        const CONDITION_MNEMONICS: [&str; 16] = [
            "fl", // [0] Always false
            "eq", // [1] Equal
            "lt", // [2] Less than
            "le", // [3] Less than or equal
            "tr", // [4] Always true
            "ne", // [5] Not equal
            "ge", // [6] Greater than or equal
            "gt", // [7] Greater than
            "ez", // [8] Equal to zero
            "en", // [9] Equal to NaN
            "ei", // [10] Absolute value equal to infinity
            "es", // [11] Equal to infinity or NaN
            "nz", // [12] Not equal to zero
            "nn", // [13] Not equal to NaN
            "ni", // [14] Absolute value not equal to infinity
            "ns", // [15] Not equal to infinity and not equal to NaN
        ];
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_vcmp_cond_unchecked();

        write!(f, "{}", CONDITION_MNEMONICS[s as usize])
    }
    pub(crate) fn display_r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let cond = instr.field_r4000allegrex_vcmp_cond_unchecked();
        let vs = instr.field_r4000allegrex_s_vs_unchecked();
        let vt = instr.field_r4000allegrex_s_vs_unchecked();

        Self::display_r4000allegrex_vcmp_cond(myself, f)?;

        match cond {
            // fl | tr
            0 | 4 => {
                // If the other operands are 0 then we can omit them
                if vs == R4000AllegrexS::default() && vt == R4000AllegrexS::default() {
                    return Ok(());
                }
            }
            _ => {}
        }

        write!(f, ", ")?;
        Self::display_r4000allegrex_s_vs(myself, f)?;

        match cond {
            0 => {} // fl
            4 => {} // tr

            1 => {} // eq
            2 => {} // lt
            3 => {} // le
            5 => {} // ne
            6 => {} // ge
            7 => {} // gt

            // ez | en | ei | es | nz | nn | ni | ns
            8..=15 => {
                // If the vt operands is 0 then we can omit it
                if vt == R4000AllegrexS::default() {
                    return Ok(());
                }
            }
            _ => {}
        }

        write!(f, ", ")?;
        Self::display_r4000allegrex_s_vt(myself, f)
    }
    pub(crate) fn display_r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let cond = instr.field_r4000allegrex_vcmp_cond_unchecked();
        let vs = instr.field_r4000allegrex_p_vs_unchecked();
        let vt = instr.field_r4000allegrex_p_vs_unchecked();

        Self::display_r4000allegrex_vcmp_cond(myself, f)?;

        match cond {
            // fl | tr
            0 | 4 => {
                // If the other operands are 0 then we can omit them
                if vs == R4000AllegrexV2D::default() && vt == R4000AllegrexV2D::default() {
                    return Ok(());
                }
            }
            _ => {}
        }

        write!(f, ", ")?;
        Self::display_r4000allegrex_p_vs(myself, f)?;

        match cond {
            0 => {} // fl
            4 => {} // tr

            1 => {} // eq
            2 => {} // lt
            3 => {} // le
            5 => {} // ne
            6 => {} // ge
            7 => {} // gt

            // ez | en | ei | es | nz | nn | ni | ns
            8..=15 => {
                // If the vt operands is 0 then we can omit it
                if vt == R4000AllegrexV2D::default() {
                    return Ok(());
                }
            }
            _ => {}
        }

        write!(f, ", ")?;
        Self::display_r4000allegrex_p_vt(myself, f)
    }
    pub(crate) fn display_r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let cond = instr.field_r4000allegrex_vcmp_cond_unchecked();
        let vs = instr.field_r4000allegrex_t_vs_unchecked();
        let vt = instr.field_r4000allegrex_t_vs_unchecked();

        Self::display_r4000allegrex_vcmp_cond(myself, f)?;

        match cond {
            // fl | tr
            0 | 4 => {
                // If the other operands are 0 then we can omit them
                if vs == R4000AllegrexV3D::default() && vt == R4000AllegrexV3D::default() {
                    return Ok(());
                }
            }
            _ => {}
        }

        write!(f, ", ")?;
        Self::display_r4000allegrex_t_vs(myself, f)?;

        match cond {
            0 => {} // fl
            4 => {} // tr

            1 => {} // eq
            2 => {} // lt
            3 => {} // le
            5 => {} // ne
            6 => {} // ge
            7 => {} // gt

            // ez | en | ei | es | nz | nn | ni | ns
            8..=15 => {
                // If the vt operands is 0 then we can omit it
                if vt == R4000AllegrexV3D::default() {
                    return Ok(());
                }
            }
            _ => {}
        }

        write!(f, ", ")?;
        Self::display_r4000allegrex_t_vt(myself, f)
    }
    pub(crate) fn display_r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let cond = instr.field_r4000allegrex_vcmp_cond_unchecked();
        let vs = instr.field_r4000allegrex_q_vs_unchecked();
        let vt = instr.field_r4000allegrex_q_vs_unchecked();

        Self::display_r4000allegrex_vcmp_cond(myself, f)?;

        match cond {
            // fl | tr
            0 | 4 => {
                // If the other operands are 0 then we can omit them
                if vs == R4000AllegrexV4D::default() && vt == R4000AllegrexV4D::default() {
                    return Ok(());
                }
            }
            _ => {}
        }

        write!(f, ", ")?;
        Self::display_r4000allegrex_q_vs(myself, f)?;

        match cond {
            0 => {} // fl
            4 => {} // tr

            1 => {} // eq
            2 => {} // lt
            3 => {} // le
            5 => {} // ne
            6 => {} // ge
            7 => {} // gt

            // ez | en | ei | es | nz | nn | ni | ns
            8..=15 => {
                // If the vt operands is 0 then we can omit it
                if vt == R4000AllegrexV4D::default() {
                    return Ok(());
                }
            }
            _ => {}
        }

        write!(f, ", ")?;
        Self::display_r4000allegrex_q_vt(myself, f)
    }

    pub(crate) fn display_r4000allegrex_vconstant(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r4000allegrex_vconstant_unchecked();
        let s = reg.either_name(instr.flags().abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_power_of_two(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_power_of_two_unchecked();

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_vfpu_cc_bit(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_vfpu_cc_bit_unchecked();

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_bn(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_bn_unchecked();

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_int16(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_int16_unchecked();

        write!(f, "{}", s)
    }
    pub(crate) fn display_r4000allegrex_float16(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_float16_unchecked();

        write!(f, "{}", s)
    }

    pub(crate) fn display_r4000allegrex_p_vrot_code(
        myself: &OperandDisplay,
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
        myself: &OperandDisplay,
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
        myself: &OperandDisplay,
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

    const DESTINATION_PREFIX_INSTRUCTION_FORMATS: [&'static str; 8] = [
        "",          // [0]
        "0",         // [1]
        "INVALID_2", // [2]
        "1",         // [3]
        "M",         // [4]
        "INVALID_5", // [5]
        "INVALID_6", // [6]
        "INVALID_7", // [7]
    ];

    pub(crate) fn display_r4000allegrex_wpx(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_wpx_unchecked();

        write!(
            f,
            "{}",
            Self::DESTINATION_PREFIX_INSTRUCTION_FORMATS[s as usize]
        )
    }
    pub(crate) fn display_r4000allegrex_wpy(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_wpy_unchecked();

        write!(
            f,
            "{}",
            Self::DESTINATION_PREFIX_INSTRUCTION_FORMATS[s as usize]
        )
    }
    pub(crate) fn display_r4000allegrex_wpz(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_wpz_unchecked();

        write!(
            f,
            "{}",
            Self::DESTINATION_PREFIX_INSTRUCTION_FORMATS[s as usize]
        )
    }
    pub(crate) fn display_r4000allegrex_wpw(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_wpw_unchecked();

        write!(
            f,
            "{}",
            Self::DESTINATION_PREFIX_INSTRUCTION_FORMATS[s as usize]
        )
    }

    const SOURCE_TARGET_PREFIX_INSTRUCTION_FORMATS: [&'static str; 32] = [
        "X",    // [0]
        "Y",    // [1]
        "Z",    // [2]
        "W",    // [3]
        "|X|",  // [4]
        "|Y|",  // [5]
        "|Z|",  // [6]
        "|W|",  // [7]
        "0",    // [8]
        "1",    // [9]
        "2",    // [10]
        "1/2",  // [11]
        "3",    // [12]
        "1/3",  // [13]
        "1/4",  // [14]
        "1/6",  // [15]
        "-X",   // [16]
        "-Y",   // [17]
        "-Z",   // [18]
        "-W",   // [19]
        "-|X|", // [20]
        "-|Y|", // [21]
        "-|Z|", // [22]
        "-|W|", // [23]
        "-0",   // [24]
        "-1",   // [25]
        "-2",   // [26]
        "-1/2", // [27]
        "-3",   // [28]
        "-1/3", // [29]
        "-1/4", // [30]
        "-1/6", // [31]
    ];

    pub(crate) fn display_r4000allegrex_rpx(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_rpx_unchecked();

        write!(
            f,
            "{}",
            Self::SOURCE_TARGET_PREFIX_INSTRUCTION_FORMATS[s as usize]
        )
    }
    pub(crate) fn display_r4000allegrex_rpy(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_rpy_unchecked();

        write!(
            f,
            "{}",
            Self::SOURCE_TARGET_PREFIX_INSTRUCTION_FORMATS[s as usize]
        )
    }
    pub(crate) fn display_r4000allegrex_rpz(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_rpz_unchecked();

        write!(
            f,
            "{}",
            Self::SOURCE_TARGET_PREFIX_INSTRUCTION_FORMATS[s as usize]
        )
    }
    pub(crate) fn display_r4000allegrex_rpw(
        myself: &OperandDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field_r4000allegrex_rpw_unchecked();

        write!(
            f,
            "{}",
            Self::SOURCE_TARGET_PREFIX_INSTRUCTION_FORMATS[s as usize]
        )
    }
}
