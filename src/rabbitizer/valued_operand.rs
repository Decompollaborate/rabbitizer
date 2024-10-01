/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::num::NonZero;

use crate::{operand::OPERAND_COUNT_MAX, Operand, OperandDisplay, ValuedOperand};
use crate::{traits::Register, DisplayFlags, Instruction};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_enums)]
pub enum IU16 {
    Integer(i16),
    Unsigned(u16),
}

impl ValuedOperand {
    #[must_use]
    pub const fn default() -> Self {
        Self::ALL_EMPTY()
    }

    #[must_use]
    pub const fn display<'ins, 'imm, 'flg>(
        &self,
        instr: &'ins Instruction,
        imm_override: Option<&'imm str>,
        display_flags: &'flg DisplayFlags,
    ) -> OperandDisplay<'ins, 'imm, 'flg> {
        OperandDisplay::new(
            Operand::from_valued_operand(*self),
            instr,
            imm_override,
            display_flags,
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
                NonZero::new(instr.field_code_lower_unchecked()),
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
                Self::core_branch_target_label(instr.get_processed_immediate_unchecked() as i16)
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
                let reg = if rd.holds_return_address(instr.flags().abi()) {
                    None
                } else {
                    Some(rd)
                };
                Self::core_maybe_rd_rs(reg, instr.field_rs_unchecked())
            }
            Operand::core_maybe_zero_rs => Self::core_maybe_zero_rs((), instr.field_rs_unchecked()),
            Operand::rsp_rs => todo!(),
            Operand::rsp_rt => todo!(),
            Operand::rsp_rd => todo!(),
            Operand::rsp_cop0d => todo!(),
            Operand::rsp_cop2t => todo!(),
            Operand::rsp_cop2cd => todo!(),
            Operand::rsp_vs => todo!(),
            Operand::rsp_vt => todo!(),
            Operand::rsp_vd => todo!(),
            Operand::rsp_hint => todo!(),
            Operand::rsp_vt_elementhigh => todo!(),
            Operand::rsp_vt_elementlow => todo!(),
            Operand::rsp_vd_de => todo!(),
            Operand::rsp_vs_index => todo!(),
            Operand::rsp_offset_rs => todo!(),
            Operand::rsp_immediate_base => todo!(),
            Operand::rsp_maybe_rd_rs => todo!(),
            Operand::r3000gte_sf => Self::r3000gte_sf(instr.field_r3000gte_sf_unchecked()),
            Operand::r3000gte_mx => Self::r3000gte_mx(instr.field_r3000gte_mx_unchecked()),
            Operand::r3000gte_v => Self::r3000gte_v(instr.field_r3000gte_v_unchecked()),
            Operand::r3000gte_cv => Self::r3000gte_cv(instr.field_r3000gte_cv_unchecked()),
            Operand::r3000gte_lm => Self::r3000gte_lm(instr.field_r3000gte_lm_unchecked()),
            Operand::r4000allegrex_s_vs => todo!(),
            Operand::r4000allegrex_s_vt => todo!(),
            Operand::r4000allegrex_s_vd => todo!(),
            Operand::r4000allegrex_s_vt_imm => todo!(),
            Operand::r4000allegrex_s_vd_imm => todo!(),
            Operand::r4000allegrex_p_vs => todo!(),
            Operand::r4000allegrex_p_vt => todo!(),
            Operand::r4000allegrex_p_vd => todo!(),
            Operand::r4000allegrex_t_vs => todo!(),
            Operand::r4000allegrex_t_vt => todo!(),
            Operand::r4000allegrex_t_vd => todo!(),
            Operand::r4000allegrex_q_vs => todo!(),
            Operand::r4000allegrex_q_vt => todo!(),
            Operand::r4000allegrex_q_vd => todo!(),
            Operand::r4000allegrex_q_vt_imm => todo!(),
            Operand::r4000allegrex_mp_vs => todo!(),
            Operand::r4000allegrex_mp_vt => todo!(),
            Operand::r4000allegrex_mp_vd => todo!(),
            Operand::r4000allegrex_mp_vs_transpose => todo!(),
            Operand::r4000allegrex_mt_vs => todo!(),
            Operand::r4000allegrex_mt_vt => todo!(),
            Operand::r4000allegrex_mt_vd => todo!(),
            Operand::r4000allegrex_mt_vs_transpose => todo!(),
            Operand::r4000allegrex_mq_vs => todo!(),
            Operand::r4000allegrex_mq_vt => todo!(),
            Operand::r4000allegrex_mq_vd => todo!(),
            Operand::r4000allegrex_mq_vs_transpose => todo!(),
            Operand::r4000allegrex_cop2cs => todo!(),
            Operand::r4000allegrex_cop2cd => todo!(),
            Operand::r4000allegrex_pos => todo!(),
            Operand::r4000allegrex_size => todo!(),
            Operand::r4000allegrex_size_plus_pos => todo!(),
            Operand::r4000allegrex_imm3 => todo!(),
            Operand::r4000allegrex_offset14_base => todo!(),
            Operand::r4000allegrex_offset14_base_maybe_wb => todo!(),
            Operand::r4000allegrex_vcmp_cond => todo!(),
            Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt => todo!(),
            Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt => todo!(),
            Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt => todo!(),
            Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt => todo!(),
            Operand::r4000allegrex_vconstant => todo!(),
            Operand::r4000allegrex_power_of_two => todo!(),
            Operand::r4000allegrex_vfpu_cc_bit => todo!(),
            Operand::r4000allegrex_bn => todo!(),
            Operand::r4000allegrex_int16 => todo!(),
            Operand::r4000allegrex_float16 => todo!(),
            Operand::r4000allegrex_p_vrot_code => todo!(),
            Operand::r4000allegrex_t_vrot_code => todo!(),
            Operand::r4000allegrex_q_vrot_code => todo!(),
            Operand::r4000allegrex_rpx => todo!(),
            Operand::r4000allegrex_rpy => todo!(),
            Operand::r4000allegrex_rpz => todo!(),
            Operand::r4000allegrex_rpw => todo!(),
            Operand::r4000allegrex_wpx => todo!(),
            Operand::r4000allegrex_wpy => todo!(),
            Operand::r4000allegrex_wpz => todo!(),
            Operand::r4000allegrex_wpw => todo!(),
            Operand::r5900_I => Self::r5900_I(),
            Operand::r5900_Q => Self::r5900_Q(),
            Operand::r5900_R => Self::r5900_R(),
            Operand::r5900_ACC => Self::r5900_ACC(),
            Operand::r5900_ACCxyzw => Self::r5900_ACCxyzw(
                instr.field_r5900_xyzw_x_unchecked(),
                instr.field_r5900_xyzw_y_unchecked(),
                instr.field_r5900_xyzw_z_unchecked(),
                instr.field_r5900_xyzw_w_unchecked(),
            ),
            Operand::r5900_vfs => Self::r5900_vfs(instr.field_r5900_vfs_unchecked()),
            Operand::r5900_vft => Self::r5900_vft(instr.field_r5900_vft_unchecked()),
            Operand::r5900_vfd => Self::r5900_vfd(instr.field_r5900_vfd_unchecked()),
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
            Operand::r5900_vis => Self::r5900_vis(instr.field_r5900_vis_unchecked()),
            Operand::r5900_vit => Self::r5900_vit(instr.field_r5900_vit_unchecked()),
            Operand::r5900_vid => Self::r5900_vid(instr.field_r5900_vid_unchecked()),
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
            Operand::r5900_immediate5 => {
                Self::r5900_immediate5(instr.field_r5900_immediate5_unchecked())
            }
            Operand::r5900_immediate15 => {
                Self::r5900_immediate15(instr.field_r5900_immediate15_unchecked())
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

impl<'ins> Iterator for ValuedOperandIterator<'ins> {
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
}
