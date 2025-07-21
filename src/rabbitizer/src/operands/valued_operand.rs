/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;
use core::iter::FusedIterator;
use core::num::NonZeroU16;

use crate::display_flags::InstructionDisplayFlags;
use crate::instr::Instruction;
use crate::operands::{Operand, OperandDisplay, ValuedOperand, OPERAND_COUNT_MAX};
use crate::registers_meta::Register;
use crate::utils;

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
        let field = instr.field();

        match operand {
            Operand::ALL_EMPTY => Self::ALL_EMPTY(),
            Operand::core_rs => Self::core_rs(field.rs_impl()),
            Operand::core_rt => Self::core_rt(field.rt_impl()),
            Operand::core_rd => Self::core_rd(field.rd_impl()),
            Operand::core_sa => Self::core_sa(field.sa_impl()),
            Operand::core_zero => Self::core_zero(),
            Operand::core_cop0d => Self::core_cop0d(field.cop0d_impl()),
            Operand::core_cop0cd => Self::core_cop0cd(field.cop0cd_impl()),
            Operand::core_fs => Self::core_fs(field.fs_impl()),
            Operand::core_ft => Self::core_ft(field.ft_impl()),
            Operand::core_fd => Self::core_fd(field.fd_impl()),
            Operand::core_cop1cs => Self::core_cop1cs(field.cop1cs_impl()),
            Operand::core_cop2t => Self::core_cop2t(field.cop2t_impl()),
            Operand::core_cop2d => Self::core_cop2d(field.cop2d_impl()),
            Operand::core_cop2cd => Self::core_cop2cd(field.cop2cd_impl()),
            Operand::core_op => Self::core_op(field.op_impl()),
            Operand::core_hint => Self::core_hint(field.hint_impl()),
            Operand::core_code => Self::core_code(
                field.code_upper_impl(),
                NonZeroU16::new(field.code_lower_impl()),
            ),
            Operand::core_code_lower => Self::core_code_lower(field.code_lower_impl()),
            // TODO: either get rid of core_copraw or move to EncodedFieldMask/add as a Instruction function.
            Operand::core_copraw => Self::core_copraw(instr.word() & utils::bitmask(0, 25)),
            Operand::core_label => Self::core_label(instr.get_instr_index_as_vram_impl()),
            Operand::core_imm_i16 => Self::core_imm_i16(field.imm_i16_impl()),
            Operand::core_imm_u16 => Self::core_imm_u16(field.imm_u16_impl()),
            Operand::core_branch_target_label => {
                Self::core_branch_target_label(instr.get_branch_offset_impl())
            }
            Operand::core_imm_rs => Self::core_imm_rs(field.imm_i16_impl(), field.rs_impl()),
            Operand::core_maybe_rd_rs => {
                let rd = field.rd_impl();
                let reg = if rd.holds_return_address(instr.abi()) {
                    None
                } else {
                    Some(rd)
                };
                Self::core_maybe_rd_rs(reg, field.rs_impl())
            }
            Operand::core_maybe_zero_rs => Self::core_maybe_zero_rs((), field.rs_impl()),
            #[cfg(feature = "RSP")]
            Operand::rsp_cop0d => Self::rsp_cop0d(field.rsp_cop0d_impl()),
            #[cfg(feature = "RSP")]
            Operand::rsp_cop2cd => Self::rsp_cop2cd(field.rsp_cop2cd_impl()),
            #[cfg(feature = "RSP")]
            Operand::rsp_vs => Self::rsp_vs(field.rsp_vs_impl()),
            #[cfg(feature = "RSP")]
            Operand::rsp_vd => Self::rsp_vd(field.rsp_vd_impl()),
            #[cfg(feature = "RSP")]
            Operand::rsp_vt_elementhigh => {
                Self::rsp_vt_elementhigh(field.rsp_vt_impl(), field.rsp_elementhigh_impl())
            }
            #[cfg(feature = "RSP")]
            Operand::rsp_vt_elementlow => {
                Self::rsp_vt_elementlow(field.rsp_vt_impl(), field.rsp_elementlow_impl())
            }
            #[cfg(feature = "RSP")]
            Operand::rsp_vd_de => Self::rsp_vd_de(field.rsp_vd_impl(), field.rsp_de_impl()),
            #[cfg(feature = "RSP")]
            Operand::rsp_vs_index => {
                Self::rsp_vs_index(field.rsp_vs_impl(), field.rsp_index_impl())
            }
            #[cfg(feature = "RSP")]
            Operand::rsp_offset7_rs => {
                Self::rsp_offset7_rs(field.rsp_offset7_impl(), field.rs_impl())
            }
            #[cfg(feature = "RSP")]
            Operand::rsp_offset8_rs => {
                Self::rsp_offset8_rs(field.rsp_offset8_impl(), field.rs_impl())
            }
            #[cfg(feature = "RSP")]
            Operand::rsp_offset9_rs => {
                Self::rsp_offset9_rs(field.rsp_offset9_impl(), field.rs_impl())
            }
            #[cfg(feature = "RSP")]
            Operand::rsp_offset10_rs => {
                Self::rsp_offset10_rs(field.rsp_offset10_impl(), field.rs_impl())
            }
            #[cfg(feature = "RSP")]
            Operand::rsp_offset11_rs => {
                Self::rsp_offset11_rs(field.rsp_offset11_impl(), field.rs_impl())
            }
            #[cfg(feature = "R3000GTE")]
            Operand::r3000gte_sf => Self::r3000gte_sf(field.r3000gte_sf_impl()),
            #[cfg(feature = "R3000GTE")]
            Operand::r3000gte_mx => Self::r3000gte_mx(field.r3000gte_mx_impl()),
            #[cfg(feature = "R3000GTE")]
            Operand::r3000gte_v => Self::r3000gte_v(field.r3000gte_v_impl()),
            #[cfg(feature = "R3000GTE")]
            Operand::r3000gte_cv => Self::r3000gte_cv(field.r3000gte_cv_impl()),
            #[cfg(feature = "R3000GTE")]
            Operand::r3000gte_lm => Self::r3000gte_lm(field.r3000gte_lm_impl()),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_s_vs => {
                Self::r4000allegrex_s_vs(field.r4000allegrex_s_vs_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_s_vt => {
                Self::r4000allegrex_s_vt(field.r4000allegrex_s_vt_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_s_vd => {
                Self::r4000allegrex_s_vd(field.r4000allegrex_s_vd_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_s_vt_imm => {
                Self::r4000allegrex_s_vt_imm(field.r4000allegrex_s_vt_imm_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_s_vd_imm => {
                Self::r4000allegrex_s_vd_imm(field.r4000allegrex_s_vd_imm_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_p_vs => {
                Self::r4000allegrex_p_vs(field.r4000allegrex_p_vs_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_p_vt => {
                Self::r4000allegrex_p_vt(field.r4000allegrex_p_vt_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_p_vd => {
                Self::r4000allegrex_p_vd(field.r4000allegrex_p_vd_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_t_vs => {
                Self::r4000allegrex_t_vs(field.r4000allegrex_t_vs_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_t_vt => {
                Self::r4000allegrex_t_vt(field.r4000allegrex_t_vt_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_t_vd => {
                Self::r4000allegrex_t_vd(field.r4000allegrex_t_vd_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_q_vs => {
                Self::r4000allegrex_q_vs(field.r4000allegrex_q_vs_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_q_vt => {
                Self::r4000allegrex_q_vt(field.r4000allegrex_q_vt_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_q_vd => {
                Self::r4000allegrex_q_vd(field.r4000allegrex_q_vd_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_q_vt_imm => {
                Self::r4000allegrex_q_vt_imm(field.r4000allegrex_q_vt_imm_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mp_vs => {
                Self::r4000allegrex_mp_vs(field.r4000allegrex_mp_vs_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mp_vt => {
                Self::r4000allegrex_mp_vt(field.r4000allegrex_mp_vt_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mp_vd => {
                Self::r4000allegrex_mp_vd(field.r4000allegrex_mp_vd_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mp_vs_transpose => {
                Self::r4000allegrex_mp_vs_transpose(field.r4000allegrex_mp_vs_transpose_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mt_vs => {
                Self::r4000allegrex_mt_vs(field.r4000allegrex_mt_vs_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mt_vt => {
                Self::r4000allegrex_mt_vt(field.r4000allegrex_mt_vt_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mt_vd => {
                Self::r4000allegrex_mt_vd(field.r4000allegrex_mt_vd_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mt_vs_transpose => {
                Self::r4000allegrex_mt_vs_transpose(field.r4000allegrex_mt_vs_transpose_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mq_vs => {
                Self::r4000allegrex_mq_vs(field.r4000allegrex_mq_vs_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mq_vt => {
                Self::r4000allegrex_mq_vt(field.r4000allegrex_mq_vt_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mq_vd => {
                Self::r4000allegrex_mq_vd(field.r4000allegrex_mq_vd_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mq_vs_transpose => {
                Self::r4000allegrex_mq_vs_transpose(field.r4000allegrex_mq_vs_transpose_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_cop2cs => {
                Self::r4000allegrex_cop2cs(field.r4000allegrex_cop2cs_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_cop2cd => {
                Self::r4000allegrex_cop2cd(field.r4000allegrex_cop2cd_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_pos => Self::r4000allegrex_pos(field.r4000allegrex_pos_impl()),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_size => {
                Self::r4000allegrex_size(field.r4000allegrex_size_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_size_plus_pos => {
                Self::r4000allegrex_size_plus_pos(field.r4000allegrex_size_plus_pos_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_imm3 => {
                Self::r4000allegrex_imm3(field.r4000allegrex_imm3_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_offset14_rs => Self::r4000allegrex_offset14_rs(
                field.r4000allegrex_offset14_impl(),
                field.rs_impl(),
            ),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_offset14_rs_maybe_wb => {
                Self::r4000allegrex_offset14_rs_maybe_wb(
                    field.r4000allegrex_offset14_impl(),
                    field.rs_impl(),
                    field.r4000allegrex_wb_impl(),
                )
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt => {
                let (cond, vs, vt) = instr.get_r4000allegrex_vcmp_s_args_impl();

                Self::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt(cond, vs, vt)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt => {
                let (cond, vs, vt) = instr.get_r4000allegrex_vcmp_p_args_impl();

                Self::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt(cond, vs, vt)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt => {
                let (cond, vs, vt) = instr.get_r4000allegrex_vcmp_t_args_impl();

                Self::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt(cond, vs, vt)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt => {
                let (cond, vs, vt) = instr.get_r4000allegrex_vcmp_q_args_impl();

                Self::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt(cond, vs, vt)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_vconstant => {
                Self::r4000allegrex_vconstant(field.r4000allegrex_vconstant_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_power_of_two => {
                Self::r4000allegrex_power_of_two(field.r4000allegrex_power_of_two_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_vfpu_cc_bit => {
                Self::r4000allegrex_vfpu_cc_bit(field.r4000allegrex_vfpu_cc_bit_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_bn => Self::r4000allegrex_bn(field.r4000allegrex_bn_impl()),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_int16 => {
                Self::r4000allegrex_int16(field.r4000allegrex_int16_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_float16 => Self::r4000allegrex_float16(
                ordered_float::OrderedFloat(field.r4000allegrex_float16_impl()),
            ),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_p_vrot_code => {
                Self::r4000allegrex_p_vrot_code(field.r4000allegrex_vrot_code_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_t_vrot_code => {
                Self::r4000allegrex_t_vrot_code(field.r4000allegrex_vrot_code_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_q_vrot_code => {
                Self::r4000allegrex_q_vrot_code(field.r4000allegrex_vrot_code_impl())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_wpx => Self::r4000allegrex_wpx(field.r4000allegrex_wpx_impl()),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_wpy => Self::r4000allegrex_wpy(field.r4000allegrex_wpy_impl()),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_wpz => Self::r4000allegrex_wpz(field.r4000allegrex_wpz_impl()),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_wpw => Self::r4000allegrex_wpw(field.r4000allegrex_wpw_impl()),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_rpx => Self::r4000allegrex_rpx(field.r4000allegrex_rpx_impl()),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_rpy => Self::r4000allegrex_rpy(field.r4000allegrex_rpy_impl()),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_rpz => Self::r4000allegrex_rpz(field.r4000allegrex_rpz_impl()),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_rpw => Self::r4000allegrex_rpw(field.r4000allegrex_rpw_impl()),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_I => Self::r5900ee_I(),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_Q => Self::r5900ee_Q(),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_R => Self::r5900ee_R(),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_ACC => Self::r5900ee_ACC(),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_imm5 => Self::r5900ee_imm5(field.r5900ee_imm5_impl()),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_imm15 => Self::r5900ee_imm15(field.r5900ee_imm15_impl()),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vfs => Self::r5900ee_vfs(field.r5900ee_vfs_impl()),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vft => Self::r5900ee_vft(field.r5900ee_vft_impl()),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vfd => Self::r5900ee_vfd(field.r5900ee_vfd_impl()),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vis => Self::r5900ee_vis(field.r5900ee_vis_impl()),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vit => Self::r5900ee_vit(field.r5900ee_vit_impl()),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vid => Self::r5900ee_vid(field.r5900ee_vid_impl()),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_ACCxyzw => Self::r5900ee_ACCxyzw(
                field.r5900ee_xyzw_x_impl(),
                field.r5900ee_xyzw_y_impl(),
                field.r5900ee_xyzw_z_impl(),
                field.r5900ee_xyzw_w_impl(),
            ),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vfsxyzw => Self::r5900ee_vfsxyzw(
                field.r5900ee_vfs_impl(),
                field.r5900ee_xyzw_x_impl(),
                field.r5900ee_xyzw_y_impl(),
                field.r5900ee_xyzw_z_impl(),
                field.r5900ee_xyzw_w_impl(),
            ),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vftxyzw => Self::r5900ee_vftxyzw(
                field.r5900ee_vft_impl(),
                field.r5900ee_xyzw_x_impl(),
                field.r5900ee_xyzw_y_impl(),
                field.r5900ee_xyzw_z_impl(),
                field.r5900ee_xyzw_w_impl(),
            ),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vfdxyzw => Self::r5900ee_vfdxyzw(
                field.r5900ee_vfd_impl(),
                field.r5900ee_xyzw_x_impl(),
                field.r5900ee_xyzw_y_impl(),
                field.r5900ee_xyzw_z_impl(),
                field.r5900ee_xyzw_w_impl(),
            ),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vftn => {
                Self::r5900ee_vftn(field.r5900ee_vft_impl(), field.r5900ee_n_impl())
            }
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vfsl => {
                Self::r5900ee_vfsl(field.r5900ee_vfs_impl(), field.r5900ee_l_impl())
            }
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vftm => {
                Self::r5900ee_vftm(field.r5900ee_vft_impl(), field.r5900ee_m_impl())
            }
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vis_predecr => Self::r5900ee_vis_predecr((), field.r5900ee_vis_impl()),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vit_predecr => Self::r5900ee_vit_predecr((), field.r5900ee_vit_impl()),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vis_postincr => {
                Self::r5900ee_vis_postincr(field.r5900ee_vis_impl(), ())
            }
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vit_postincr => {
                Self::r5900ee_vit_postincr(field.r5900ee_vit_impl(), ())
            }
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vis_parenthesis => {
                Self::r5900ee_vis_parenthesis(field.r5900ee_vis_impl())
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
    end: usize,
}

impl<'ins> ValuedOperandIterator<'ins> {
    pub(crate) fn new(instr: &'ins Instruction) -> Self {
        let operands = instr.opcode().get_descriptor().operands();
        let end = utils::array_len_non_default(operands);

        Self {
            instr,
            operands,
            index: 0,
            end,
        }
    }
}

impl Iterator for ValuedOperandIterator<'_> {
    type Item = ValuedOperand;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            return None;
        }

        let val = self.operands[self.index];
        if val == Operand::default() {
            return None;
        }

        self.index = self.index.saturating_add(1);
        Some(val.to_valued_operand(self.instr))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index = self.index.saturating_add(n);
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end - self.index;

        (remaining, Some(remaining))
    }

    fn count(mut self) -> usize {
        // The size_hint is always accurate.
        let count = self.size_hint().0;
        self.index = self.end;
        count
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl DoubleEndedIterator for ValuedOperandIterator<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            return None;
        }

        self.end = self.end.saturating_sub(1);
        let val = self.operands[self.end];
        if val == Operand::default() {
            return None;
        }
        Some(val.to_valued_operand(self.instr))
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.end = self.end.saturating_sub(n);
        self.next_back()
    }
}

impl ExactSizeIterator for ValuedOperandIterator<'_> {}
impl FusedIterator for ValuedOperandIterator<'_> {}

#[cfg(test)]
mod tests {
    use crate::instr::InstructionFlags;
    use crate::registers::Gpr;
    use crate::vram::Vram;

    use super::*;

    #[test]
    fn test_valued_operand_iter_addiu() {
        let instr = Instruction::new(
            0x27A40010,
            Vram::new(0x00000000),
            InstructionFlags::default(),
        );
        let mut iter = instr.valued_operands_iter();

        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(ValuedOperand::core_rt(Gpr::a0)));
        assert_eq!(iter.next(), Some(ValuedOperand::core_rs(Gpr::sp)));
        assert_eq!(iter.next(), Some(ValuedOperand::core_imm_i16(0x10)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_valued_operand_iter_addiu_rev() {
        let instr = Instruction::new(
            0x27A40010,
            Vram::new(0x00000000),
            InstructionFlags::default(),
        );
        let mut iter = instr.valued_operands_iter().rev();

        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(ValuedOperand::core_imm_i16(0x10)));
        assert_eq!(iter.next(), Some(ValuedOperand::core_rs(Gpr::sp)));
        assert_eq!(iter.next(), Some(ValuedOperand::core_rt(Gpr::a0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_valued_operand_iter_addiu_forward_back() {
        let instr = Instruction::new(
            0x27A40010,
            Vram::new(0x00000000),
            InstructionFlags::default(),
        );
        let mut iter = instr.valued_operands_iter();

        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(ValuedOperand::core_rt(Gpr::a0)));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next_back(), Some(ValuedOperand::core_imm_i16(0x10)));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some(ValuedOperand::core_rs(Gpr::sp)));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }
}
