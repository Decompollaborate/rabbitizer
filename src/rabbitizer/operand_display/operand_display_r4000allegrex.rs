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
        const CONDITION_MNEMONICS: [&'static str; 16] = [
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
            8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
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
            8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
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
            8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
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
            8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
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
