/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;
use core::iter::FusedIterator;
use core::num::NonZeroU16;

use crate::display_flags::InstructionDisplayFlags;
use crate::instr::Instruction;
use crate::operands::{Operand, OperandDisplay, ValuedOperand, OPERAND_COUNT_MAX};
use crate::registers_meta::Register;

/// A 16bits number, either signed or unsigned.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_enums)]
pub enum IU16 {
    /// A signed 16bits value.
    Integer(i16),
    /// An unsigned 16bits value.
    Unsigned(u16),
}

impl ValuedOperand {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::ALL_EMPTY()
    }

    pub const fn display<'ins, 'flg, T>(
        &self,
        instr: &'ins Instruction,
        display_flags: &'flg InstructionDisplayFlags,
        imm_override: Option<T>,
    ) -> OperandDisplay<'ins, 'flg, T>
    where
        T: fmt::Display,
    {
        OperandDisplay::new(
            Operand::from_valued_operand(*self),
            instr,
            display_flags,
            imm_override,
        )
    }
}

impl ValuedOperand {
    pub(crate) fn from_operand(operand: Operand, instr: &Instruction) -> Self {
        match operand {
            Operand::ALL_EMPTY => Self::ALL_EMPTY(),
            Operand::core_rs => Self::core_rs(instr.field_rs_unchecked()),
            Operand::core_rt => Self::core_rt(instr.field_rt_unchecked()),
            Operand::core_rd => Self::core_rd(instr.field_rd_unchecked()),
            Operand::core_sa => Self::core_sa(instr.field_sa_unchecked()),
            Operand::core_zero => Self::core_zero(),
            Operand::core_cop0d => Self::core_cop0d(instr.field_cop0d_unchecked()),
            Operand::core_cop0cd => Self::core_cop0cd(instr.field_cop0cd_unchecked()),
            Operand::core_fs => Self::core_fs(instr.field_fs_unchecked()),
            Operand::core_ft => Self::core_ft(instr.field_ft_unchecked()),
            Operand::core_fd => Self::core_fd(instr.field_fd_unchecked()),
            Operand::core_cop1cs => Self::core_cop1cs(instr.field_cop1cs_unchecked()),
            Operand::core_cop2t => Self::core_cop2t(instr.field_cop2t_unchecked()),
            Operand::core_cop2d => Self::core_cop2d(instr.field_cop2d_unchecked()),
            Operand::core_cop2cd => Self::core_cop2cd(instr.field_cop2cd_unchecked()),
            Operand::core_op => Self::core_op(instr.field_op_unchecked()),
            Operand::core_hint => Self::core_hint(instr.field_hint_unchecked()),
            Operand::core_code => Self::core_code(
                instr.field_code_upper_unchecked(),
                NonZeroU16::new(instr.field_code_lower_unchecked()),
            ),
            Operand::core_code_lower => Self::core_code_lower(instr.field_code_lower_unchecked()),
            Operand::core_copraw => todo!(),
            Operand::core_label => Self::core_label(instr.get_instr_index_as_vram_unchecked()),
            Operand::core_immediate => {
                let imm = if instr.opcode().is_unsigned() {
                    IU16::Unsigned(instr.field_immediate_unchecked())
                } else {
                    IU16::Integer(instr.get_processed_immediate_unchecked() as i16)
                };
                Self::core_immediate(imm)
            }
            Operand::core_branch_target_label => {
                Self::core_branch_target_label(instr.get_branch_offset_unchecked())
            }
            Operand::core_immediate_base => {
                let imm = if instr.opcode().is_unsigned() {
                    IU16::Unsigned(instr.field_immediate_unchecked())
                } else {
                    IU16::Integer(instr.get_processed_immediate_unchecked() as i16)
                };
                Self::core_immediate_base(imm, instr.field_rs_unchecked())
            }
            Operand::core_maybe_rd_rs => {
                let rd = instr.field_rd_unchecked();
                let reg = if rd.holds_return_address(instr.abi()) {
                    None
                } else {
                    Some(rd)
                };
                Self::core_maybe_rd_rs(reg, instr.field_rs_unchecked())
            }
            Operand::core_maybe_zero_rs => Self::core_maybe_zero_rs((), instr.field_rs_unchecked()),
            Operand::rsp_cop0d => Self::rsp_cop0d(instr.field_rsp_cop0d_unchecked()),
            Operand::rsp_cop2t => Self::rsp_cop2t(instr.field_rsp_cop2t_unchecked()),
            Operand::rsp_cop2cd => Self::rsp_cop2cd(instr.field_rsp_cop2cd_unchecked()),
            Operand::rsp_vs => Self::rsp_vs(instr.field_rsp_vs_unchecked()),
            Operand::rsp_vd => Self::rsp_vd(instr.field_rsp_vd_unchecked()),
            Operand::rsp_vt_elementhigh => Self::rsp_vt_elementhigh(
                instr.field_rsp_vt_unchecked(),
                instr.field_rsp_elementhigh_unchecked(),
            ),
            Operand::rsp_vt_elementlow => Self::rsp_vt_elementlow(
                instr.field_rsp_vt_unchecked(),
                instr.field_rsp_elementlow_unchecked(),
            ),
            Operand::rsp_vd_de => Self::rsp_vd_de(
                instr.field_rsp_vd_unchecked(),
                instr.field_rsp_de_unchecked(),
            ),
            Operand::rsp_vs_index => Self::rsp_vs_index(
                instr.field_rsp_vs_unchecked(),
                instr.field_rsp_index_unchecked(),
            ),
            Operand::rsp_offset_rs => Self::rsp_offset_rs(
                instr.get_processed_rsp_offset_unchecked(),
                instr.field_rs_unchecked(),
            ),
            Operand::r3000gte_sf => Self::r3000gte_sf(instr.field_r3000gte_sf_unchecked()),
            Operand::r3000gte_mx => Self::r3000gte_mx(instr.field_r3000gte_mx_unchecked()),
            Operand::r3000gte_v => Self::r3000gte_v(instr.field_r3000gte_v_unchecked()),
            Operand::r3000gte_cv => Self::r3000gte_cv(instr.field_r3000gte_cv_unchecked()),
            Operand::r3000gte_lm => Self::r3000gte_lm(instr.field_r3000gte_lm_unchecked()),
            Operand::r4000allegrex_s_vs => {
                Self::r4000allegrex_s_vs(instr.field_r4000allegrex_s_vs_unchecked())
            }
            Operand::r4000allegrex_s_vt => {
                Self::r4000allegrex_s_vt(instr.field_r4000allegrex_s_vt_unchecked())
            }
            Operand::r4000allegrex_s_vd => {
                Self::r4000allegrex_s_vd(instr.field_r4000allegrex_s_vd_unchecked())
            }
            Operand::r4000allegrex_s_vt_imm => {
                Self::r4000allegrex_s_vt_imm(instr.field_r4000allegrex_s_vt_imm_unchecked())
            }
            Operand::r4000allegrex_s_vd_imm => {
                Self::r4000allegrex_s_vd_imm(instr.field_r4000allegrex_s_vd_imm_unchecked())
            }
            Operand::r4000allegrex_p_vs => {
                Self::r4000allegrex_p_vs(instr.field_r4000allegrex_p_vs_unchecked())
            }
            Operand::r4000allegrex_p_vt => {
                Self::r4000allegrex_p_vt(instr.field_r4000allegrex_p_vt_unchecked())
            }
            Operand::r4000allegrex_p_vd => {
                Self::r4000allegrex_p_vd(instr.field_r4000allegrex_p_vd_unchecked())
            }
            Operand::r4000allegrex_t_vs => {
                Self::r4000allegrex_t_vs(instr.field_r4000allegrex_t_vs_unchecked())
            }
            Operand::r4000allegrex_t_vt => {
                Self::r4000allegrex_t_vt(instr.field_r4000allegrex_t_vt_unchecked())
            }
            Operand::r4000allegrex_t_vd => {
                Self::r4000allegrex_t_vd(instr.field_r4000allegrex_t_vd_unchecked())
            }
            Operand::r4000allegrex_q_vs => {
                Self::r4000allegrex_q_vs(instr.field_r4000allegrex_q_vs_unchecked())
            }
            Operand::r4000allegrex_q_vt => {
                Self::r4000allegrex_q_vt(instr.field_r4000allegrex_q_vt_unchecked())
            }
            Operand::r4000allegrex_q_vd => {
                Self::r4000allegrex_q_vd(instr.field_r4000allegrex_q_vd_unchecked())
            }
            Operand::r4000allegrex_q_vt_imm => {
                Self::r4000allegrex_q_vt_imm(instr.field_r4000allegrex_q_vt_imm_unchecked())
            }
            Operand::r4000allegrex_mp_vs => {
                Self::r4000allegrex_mp_vs(instr.field_r4000allegrex_mp_vs_unchecked())
            }
            Operand::r4000allegrex_mp_vt => {
                Self::r4000allegrex_mp_vt(instr.field_r4000allegrex_mp_vt_unchecked())
            }
            Operand::r4000allegrex_mp_vd => {
                Self::r4000allegrex_mp_vd(instr.field_r4000allegrex_mp_vd_unchecked())
            }
            Operand::r4000allegrex_mp_vs_transpose => Self::r4000allegrex_mp_vs_transpose(
                instr.field_r4000allegrex_mp_vs_transpose_unchecked(),
            ),
            Operand::r4000allegrex_mt_vs => {
                Self::r4000allegrex_mt_vs(instr.field_r4000allegrex_mt_vs_unchecked())
            }
            Operand::r4000allegrex_mt_vt => {
                Self::r4000allegrex_mt_vt(instr.field_r4000allegrex_mt_vt_unchecked())
            }
            Operand::r4000allegrex_mt_vd => {
                Self::r4000allegrex_mt_vd(instr.field_r4000allegrex_mt_vd_unchecked())
            }
            Operand::r4000allegrex_mt_vs_transpose => Self::r4000allegrex_mt_vs_transpose(
                instr.field_r4000allegrex_mt_vs_transpose_unchecked(),
            ),
            Operand::r4000allegrex_mq_vs => {
                Self::r4000allegrex_mq_vs(instr.field_r4000allegrex_mq_vs_unchecked())
            }
            Operand::r4000allegrex_mq_vt => {
                Self::r4000allegrex_mq_vt(instr.field_r4000allegrex_mq_vt_unchecked())
            }
            Operand::r4000allegrex_mq_vd => {
                Self::r4000allegrex_mq_vd(instr.field_r4000allegrex_mq_vd_unchecked())
            }
            Operand::r4000allegrex_mq_vs_transpose => Self::r4000allegrex_mq_vs_transpose(
                instr.field_r4000allegrex_mq_vs_transpose_unchecked(),
            ),
            Operand::r4000allegrex_cop2cs => {
                Self::r4000allegrex_cop2cs(instr.field_r4000allegrex_cop2cs_unchecked())
            }
            Operand::r4000allegrex_cop2cd => {
                Self::r4000allegrex_cop2cd(instr.field_r4000allegrex_cop2cd_unchecked())
            }
            Operand::r4000allegrex_pos => {
                Self::r4000allegrex_pos(instr.field_r4000allegrex_pos_unchecked())
            }
            Operand::r4000allegrex_size => {
                Self::r4000allegrex_size(instr.field_r4000allegrex_size_unchecked())
            }
            Operand::r4000allegrex_size_plus_pos => Self::r4000allegrex_size_plus_pos(
                instr.field_r4000allegrex_size_plus_pos_unchecked(),
            ),
            Operand::r4000allegrex_imm3 => {
                Self::r4000allegrex_imm3(instr.field_r4000allegrex_imm3_unchecked())
            }
            Operand::r4000allegrex_offset14_base => Self::r4000allegrex_offset14_base(
                instr.field_r4000allegrex_offset14_unchecked(),
                instr.field_rs_unchecked(),
            ),
            Operand::r4000allegrex_offset14_base_maybe_wb => {
                Self::r4000allegrex_offset14_base_maybe_wb(
                    instr.field_r4000allegrex_offset14_unchecked(),
                    instr.field_rs_unchecked(),
                    instr.field_r4000allegrex_wb_unchecked(),
                )
            }
            Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt => {
                let (cond, vs, vt) = instr.get_r4000allegrex_vcmp_s_args_unchecked();

                Self::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt(cond, vs, vt)
            }
            Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt => {
                let (cond, vs, vt) = instr.get_r4000allegrex_vcmp_p_args_unchecked();

                Self::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt(cond, vs, vt)
            }
            Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt => {
                let (cond, vs, vt) = instr.get_r4000allegrex_vcmp_t_args_unchecked();

                Self::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt(cond, vs, vt)
            }
            Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt => {
                let (cond, vs, vt) = instr.get_r4000allegrex_vcmp_q_args_unchecked();

                Self::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt(cond, vs, vt)
            }
            Operand::r4000allegrex_vconstant => {
                Self::r4000allegrex_vconstant(instr.field_r4000allegrex_vconstant_unchecked())
            }
            Operand::r4000allegrex_power_of_two => {
                Self::r4000allegrex_power_of_two(instr.field_r4000allegrex_power_of_two_unchecked())
            }
            Operand::r4000allegrex_vfpu_cc_bit => {
                Self::r4000allegrex_vfpu_cc_bit(instr.field_r4000allegrex_vfpu_cc_bit_unchecked())
            }
            Operand::r4000allegrex_bn => {
                Self::r4000allegrex_bn(instr.field_r4000allegrex_bn_unchecked())
            }
            Operand::r4000allegrex_int16 => {
                Self::r4000allegrex_int16(instr.field_r4000allegrex_int16_unchecked())
            }
            Operand::r4000allegrex_float16 => {
                Self::r4000allegrex_float16(instr.field_r4000allegrex_float16_unchecked())
            }
            Operand::r4000allegrex_p_vrot_code => {
                Self::r4000allegrex_p_vrot_code(instr.field_r4000allegrex_vrot_code_unchecked())
            }
            Operand::r4000allegrex_t_vrot_code => {
                Self::r4000allegrex_t_vrot_code(instr.field_r4000allegrex_vrot_code_unchecked())
            }
            Operand::r4000allegrex_q_vrot_code => {
                Self::r4000allegrex_q_vrot_code(instr.field_r4000allegrex_vrot_code_unchecked())
            }
            Operand::r4000allegrex_wpx => {
                Self::r4000allegrex_wpx(instr.field_r4000allegrex_wpx_unchecked())
            }
            Operand::r4000allegrex_wpy => {
                Self::r4000allegrex_wpy(instr.field_r4000allegrex_wpy_unchecked())
            }
            Operand::r4000allegrex_wpz => {
                Self::r4000allegrex_wpz(instr.field_r4000allegrex_wpz_unchecked())
            }
            Operand::r4000allegrex_wpw => {
                Self::r4000allegrex_wpw(instr.field_r4000allegrex_wpw_unchecked())
            }
            Operand::r4000allegrex_rpx => {
                Self::r4000allegrex_rpx(instr.field_r4000allegrex_rpx_unchecked())
            }
            Operand::r4000allegrex_rpy => {
                Self::r4000allegrex_rpy(instr.field_r4000allegrex_rpy_unchecked())
            }
            Operand::r4000allegrex_rpz => {
                Self::r4000allegrex_rpz(instr.field_r4000allegrex_rpz_unchecked())
            }
            Operand::r4000allegrex_rpw => {
                Self::r4000allegrex_rpw(instr.field_r4000allegrex_rpw_unchecked())
            }
            Operand::r5900_I => Self::r5900_I(),
            Operand::r5900_Q => Self::r5900_Q(),
            Operand::r5900_R => Self::r5900_R(),
            Operand::r5900_ACC => Self::r5900_ACC(),
            Operand::r5900_immediate5 => {
                Self::r5900_immediate5(instr.field_r5900_immediate5_unchecked())
            }
            Operand::r5900_immediate15 => {
                Self::r5900_immediate15(instr.field_r5900_immediate15_unchecked())
            }
            Operand::r5900_vfs => Self::r5900_vfs(instr.field_r5900_vfs_unchecked()),
            Operand::r5900_vft => Self::r5900_vft(instr.field_r5900_vft_unchecked()),
            Operand::r5900_vfd => Self::r5900_vfd(instr.field_r5900_vfd_unchecked()),
            Operand::r5900_vis => Self::r5900_vis(instr.field_r5900_vis_unchecked()),
            Operand::r5900_vit => Self::r5900_vit(instr.field_r5900_vit_unchecked()),
            Operand::r5900_vid => Self::r5900_vid(instr.field_r5900_vid_unchecked()),
            Operand::r5900_ACCxyzw => Self::r5900_ACCxyzw(
                instr.field_r5900_xyzw_x_unchecked(),
                instr.field_r5900_xyzw_y_unchecked(),
                instr.field_r5900_xyzw_z_unchecked(),
                instr.field_r5900_xyzw_w_unchecked(),
            ),
            Operand::r5900_vfsxyzw => Self::r5900_vfsxyzw(
                instr.field_r5900_vfs_unchecked(),
                instr.field_r5900_xyzw_x_unchecked(),
                instr.field_r5900_xyzw_y_unchecked(),
                instr.field_r5900_xyzw_z_unchecked(),
                instr.field_r5900_xyzw_w_unchecked(),
            ),
            Operand::r5900_vftxyzw => Self::r5900_vftxyzw(
                instr.field_r5900_vft_unchecked(),
                instr.field_r5900_xyzw_x_unchecked(),
                instr.field_r5900_xyzw_y_unchecked(),
                instr.field_r5900_xyzw_z_unchecked(),
                instr.field_r5900_xyzw_w_unchecked(),
            ),
            Operand::r5900_vfdxyzw => Self::r5900_vfdxyzw(
                instr.field_r5900_vfd_unchecked(),
                instr.field_r5900_xyzw_x_unchecked(),
                instr.field_r5900_xyzw_y_unchecked(),
                instr.field_r5900_xyzw_z_unchecked(),
                instr.field_r5900_xyzw_w_unchecked(),
            ),
            Operand::r5900_vftn => Self::r5900_vftn(
                instr.field_r5900_vft_unchecked(),
                instr.field_r5900_n_unchecked(),
            ),
            Operand::r5900_vfsl => Self::r5900_vfsl(
                instr.field_r5900_vfs_unchecked(),
                instr.field_r5900_l_unchecked(),
            ),
            Operand::r5900_vftm => Self::r5900_vftm(
                instr.field_r5900_vft_unchecked(),
                instr.field_r5900_m_unchecked(),
            ),
            Operand::r5900_vis_predecr => {
                Self::r5900_vis_predecr((), instr.field_r5900_vis_unchecked())
            }
            Operand::r5900_vit_predecr => {
                Self::r5900_vit_predecr((), instr.field_r5900_vit_unchecked())
            }
            Operand::r5900_vis_postincr => {
                Self::r5900_vis_postincr(instr.field_r5900_vis_unchecked(), ())
            }
            Operand::r5900_vit_postincr => {
                Self::r5900_vit_postincr(instr.field_r5900_vit_unchecked(), ())
            }
            Operand::r5900_vis_parenthesis => {
                Self::r5900_vis_parenthesis(instr.field_r5900_vis_unchecked())
            }
        }
    }
}

impl Default for ValuedOperand {
    fn default() -> Self {
        Self::default()
    }
}

impl From<ValuedOperand> for Operand {
    fn from(val: ValuedOperand) -> Self {
        Self::from_valued_operand(val)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValuedOperandIterator<'ins> {
    instr: &'ins Instruction,
    operands: &'ins [Operand; OPERAND_COUNT_MAX],
    index: usize,
}

impl<'ins> ValuedOperandIterator<'ins> {
    pub(crate) fn new(instr: &'ins Instruction) -> Self {
        Self {
            instr,
            operands: instr.opcode().get_descriptor().operands(),
            index: 0,
        }
    }
}

impl Iterator for ValuedOperandIterator<'_> {
    type Item = ValuedOperand;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.operands.len() {
            return None;
        }

        let val = &self.operands[self.index];
        if *val == Operand::default() {
            return None;
        }

        self.index += 1;
        Some(val.to_valued_operand(self.instr))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.operands.len() - self.index;

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for ValuedOperandIterator<'_> {}
impl FusedIterator for ValuedOperandIterator<'_> {}
