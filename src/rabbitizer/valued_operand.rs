/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::num::NonZero;

use crate::{operand, registers::*, Instruction, Operand};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum IU16 {
    Integer(i16),
    Unsigned(u16),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum ValuedOperand {
    ALL_EMPTY(),
    cpu_rs(Gpr),
    cpu_rt(Gpr),
    cpu_rd(Gpr),
    cpu_sa(u8),
    cpu_zero(),
    cpu_cop0d(Cop0),
    cpu_fs(Cop1),
    cpu_ft(Cop1),
    cpu_fd(Cop1),
    cpu_cop1cs(Cop1Control),
    cpu_cop2t(Cop2),
    cpu_cop2cd(Cop2),
    cpu_op(u8),
    cpu_hint(u8),
    cpu_code(u16, Option<NonZero<u16>>),
    cpu_code_lower(u16),
    cpu_copraw(u32),
    cpu_label(u32),
    cpu_immediate(IU16),
    cpu_branch_target_label(i16),
    cpu_immediate_base(IU16, Gpr),
    cpu_maybe_rd_rs(Option<Gpr>, Gpr),
    rsp_rs(RspGpr),
    rsp_rt(RspGpr),
    rsp_rd(RspGpr),
    rsp_cop0d(RspCop0),
    rsp_cop2t(RspCop2),
    rsp_cop2cd(RspCop2),
    rsp_vs(RspVector),
    rsp_vt(RspVector),
    rsp_vd(RspVector),
    rsp_hint(u8),
    rsp_vt_elementhigh(RspVector, u8),
    rsp_vt_elementlow(RspVector, u8),
    rsp_vd_de(RspVector, u8),
    rsp_vs_index(RspVector, u8),
    rsp_offset_rs(u16, RspGpr),
    rsp_immediate_base(IU16, Gpr),
    rsp_maybe_rd_rs(Option<Gpr>, Gpr),
    r3000gte_sf(u8),
    r3000gte_mx(u8),
    r3000gte_v(u8),
    r3000gte_cv(u8),
    r3000gte_lm(u8),
    r4000allegrex_s_vs(R4000AllegrexS),
    r4000allegrex_s_vt(R4000AllegrexS),
    r4000allegrex_s_vd(R4000AllegrexS),
    r4000allegrex_s_vt_imm(R4000AllegrexS),
    r4000allegrex_s_vd_imm(R4000AllegrexS),
    r4000allegrex_p_vs(R4000AllegrexV2D),
    r4000allegrex_p_vt(R4000AllegrexV2D),
    r4000allegrex_p_vd(R4000AllegrexV2D),
    r4000allegrex_t_vs(R4000AllegrexV3D),
    r4000allegrex_t_vt(R4000AllegrexV3D),
    r4000allegrex_t_vd(R4000AllegrexV3D),
    r4000allegrex_q_vs(R4000AllegrexV4D),
    r4000allegrex_q_vt(R4000AllegrexV4D),
    r4000allegrex_q_vd(R4000AllegrexV4D),
    r4000allegrex_q_vt_imm(R4000AllegrexV4D),
    r4000allegrex_mp_vs(R4000AllegrexM2x2),
    r4000allegrex_mp_vt(R4000AllegrexM2x2),
    r4000allegrex_mp_vd(R4000AllegrexM2x2),
    r4000allegrex_mp_vs_transpose(R4000AllegrexM2x2),
    r4000allegrex_mt_vs(R4000AllegrexM3x3),
    r4000allegrex_mt_vt(R4000AllegrexM3x3),
    r4000allegrex_mt_vd(R4000AllegrexM3x3),
    r4000allegrex_mt_vs_transpose(R4000AllegrexM3x3),
    r4000allegrex_mq_vs(R4000AllegrexM4x4),
    r4000allegrex_mq_vt(R4000AllegrexM4x4),
    r4000allegrex_mq_vd(R4000AllegrexM4x4),
    r4000allegrex_mq_vs_transpose(R4000AllegrexM4x4),
    r4000allegrex_cop2cs(R4000AllegrexVfpuControl),
    r4000allegrex_cop2cd(R4000AllegrexVfpuControl),
    r4000allegrex_pos(u8),
    r4000allegrex_size(u8),
    r4000allegrex_size_plus_pos(i8),
    r4000allegrex_imm3(u8),
    r4000allegrex_offset14_base(u16, Gpr),
    r4000allegrex_offset14_base_maybe_wb(u16, Gpr, bool),
    r4000allegrex_vcmp_cond(&'static str),
    r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt(
        &'static str,
        Option<R4000AllegrexS>,
        Option<R4000AllegrexS>,
    ),
    r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt(
        &'static str,
        Option<R4000AllegrexS>,
        Option<R4000AllegrexS>,
    ),
    r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt(
        &'static str,
        Option<R4000AllegrexS>,
        Option<R4000AllegrexS>,
    ),
    r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt(
        &'static str,
        Option<R4000AllegrexS>,
        Option<R4000AllegrexS>,
    ),
    r4000allegrex_vconstant(R4000AllegrexVConstant),
    r4000allegrex_power_of_two(u8),
    r4000allegrex_vfpu_cc_bit(u8),
    r4000allegrex_bn(u8),
    r4000allegrex_int16(i16),
    r4000allegrex_float16(f32),
    r4000allegrex_p_vrot_code(&'static str),
    r4000allegrex_t_vrot_code(&'static str),
    r4000allegrex_q_vrot_code(&'static str),
    r4000allegrex_rpx(&'static str),
    r4000allegrex_rpy(&'static str),
    r4000allegrex_rpz(&'static str),
    r4000allegrex_rpw(&'static str),
    r4000allegrex_wpx(&'static str),
    r4000allegrex_wpy(&'static str),
    r4000allegrex_wpz(&'static str),
    r4000allegrex_wpw(&'static str),
    r5900_I(),
    r5900_Q(),
    r5900_R(),
    r5900_ACC(),
    r5900_ACCxyzw(bool, bool, bool, bool),
    r5900_vfs(R5900VF),
    r5900_vft(R5900VF),
    r5900_vfd(R5900VF),
    r5900_vfsxyzw(R5900VF, bool, bool, bool, bool), // TODO: change to enum or something
    r5900_vftxyzw(R5900VF, bool, bool, bool, bool), // TODO: change to enum or something
    r5900_vfdxyzw(R5900VF, bool, bool, bool, bool), // TODO: change to enum or something
    r5900_vfsn(R5900VF, u8),
    r5900_vftn(R5900VF, u8),
    r5900_vfdn(R5900VF, u8),
    r5900_vfsl(R5900VF, bool, bool, bool, bool), // TODO: change to enum or something
    r5900_vftl(R5900VF, bool, bool, bool, bool), // TODO: change to enum or something
    r5900_vfdl(R5900VF, bool, bool, bool, bool), // TODO: change to enum or something
    r5900_vfsm(R5900VF, bool, bool, bool, bool), // TODO: change to enum or something
    r5900_vftm(R5900VF, bool, bool, bool, bool), // TODO: change to enum or something
    r5900_vfdm(R5900VF, bool, bool, bool, bool), // TODO: change to enum or something
    r5900_vis(R5900VI),
    r5900_vit(R5900VI),
    r5900_vid(R5900VI),
    r5900_vis_predecr(R5900VI),
    r5900_vit_predecr(R5900VI),
    r5900_vid_predecr(R5900VI),
    r5900_vis_postincr(R5900VI),
    r5900_vit_postincr(R5900VI),
    r5900_vid_postincr(R5900VI),
    r5900_vis_parenthesis(R5900VI),
    r5900_immediate5(u8),
    r5900_immediate15(u32),
}

impl ValuedOperand {
    #[must_use]
    pub const fn default() -> Self {
        Self::ALL_EMPTY()
    }
}

impl Operand {
    #[must_use]
    pub const fn from_valued_operand(op: ValuedOperand) -> Self {
        match op {
            ValuedOperand::ALL_EMPTY(..) => Self::ALL_EMPTY,
            ValuedOperand::cpu_rs(..) => Self::cpu_rs,
            ValuedOperand::cpu_rt(..) => Self::cpu_rt,
            ValuedOperand::cpu_rd(..) => Self::cpu_rd,
            ValuedOperand::cpu_sa(..) => Self::cpu_sa,
            ValuedOperand::cpu_zero(..) => Self::cpu_zero,
            ValuedOperand::cpu_cop0d(..) => Self::cpu_cop0d,
            ValuedOperand::cpu_fs(..) => Self::cpu_fs,
            ValuedOperand::cpu_ft(..) => Self::cpu_ft,
            ValuedOperand::cpu_fd(..) => Self::cpu_fd,
            ValuedOperand::cpu_cop1cs(..) => Self::cpu_cop1cs,
            ValuedOperand::cpu_cop2t(..) => Self::cpu_cop2t,
            ValuedOperand::cpu_cop2cd(..) => Self::cpu_cop2cd,
            ValuedOperand::cpu_op(..) => Self::cpu_op,
            ValuedOperand::cpu_hint(..) => Self::cpu_hint,
            ValuedOperand::cpu_code(..) => Self::cpu_code,
            ValuedOperand::cpu_code_lower(..) => Self::cpu_code_lower,
            ValuedOperand::cpu_copraw(..) => Self::cpu_copraw,
            ValuedOperand::cpu_label(..) => Self::cpu_label,
            ValuedOperand::cpu_immediate(..) => Self::cpu_immediate,
            ValuedOperand::cpu_branch_target_label(..) => Self::cpu_branch_target_label,
            ValuedOperand::cpu_immediate_base(..) => Self::cpu_immediate_base,
            ValuedOperand::cpu_maybe_rd_rs(..) => Self::cpu_maybe_rd_rs,
            ValuedOperand::rsp_rs(..) => Self::rsp_rs,
            ValuedOperand::rsp_rt(..) => Self::rsp_rt,
            ValuedOperand::rsp_rd(..) => Self::rsp_rd,
            ValuedOperand::rsp_cop0d(..) => Self::rsp_cop0d,
            ValuedOperand::rsp_cop2t(..) => Self::rsp_cop2t,
            ValuedOperand::rsp_cop2cd(..) => Self::rsp_cop2cd,
            ValuedOperand::rsp_vs(..) => Self::rsp_vs,
            ValuedOperand::rsp_vt(..) => Self::rsp_vt,
            ValuedOperand::rsp_vd(..) => Self::rsp_vd,
            ValuedOperand::rsp_hint(..) => Self::rsp_hint,
            ValuedOperand::rsp_vt_elementhigh(..) => Self::rsp_vt_elementhigh,
            ValuedOperand::rsp_vt_elementlow(..) => Self::rsp_vt_elementlow,
            ValuedOperand::rsp_vd_de(..) => Self::rsp_vd_de,
            ValuedOperand::rsp_vs_index(..) => Self::rsp_vs_index,
            ValuedOperand::rsp_offset_rs(..) => Self::rsp_offset_rs,
            ValuedOperand::rsp_immediate_base(..) => Self::rsp_immediate_base,
            ValuedOperand::rsp_maybe_rd_rs(..) => Self::rsp_maybe_rd_rs,
            ValuedOperand::r3000gte_sf(..) => Self::r3000gte_sf,
            ValuedOperand::r3000gte_mx(..) => Self::r3000gte_mx,
            ValuedOperand::r3000gte_v(..) => Self::r3000gte_v,
            ValuedOperand::r3000gte_cv(..) => Self::r3000gte_cv,
            ValuedOperand::r3000gte_lm(..) => Self::r3000gte_lm,
            ValuedOperand::r4000allegrex_s_vs(..) => Self::r4000allegrex_s_vs,
            ValuedOperand::r4000allegrex_s_vt(..) => Self::r4000allegrex_s_vt,
            ValuedOperand::r4000allegrex_s_vd(..) => Self::r4000allegrex_s_vd,
            ValuedOperand::r4000allegrex_s_vt_imm(..) => Self::r4000allegrex_s_vt_imm,
            ValuedOperand::r4000allegrex_s_vd_imm(..) => Self::r4000allegrex_s_vd_imm,
            ValuedOperand::r4000allegrex_p_vs(..) => Self::r4000allegrex_p_vs,
            ValuedOperand::r4000allegrex_p_vt(..) => Self::r4000allegrex_p_vt,
            ValuedOperand::r4000allegrex_p_vd(..) => Self::r4000allegrex_p_vd,
            ValuedOperand::r4000allegrex_t_vs(..) => Self::r4000allegrex_t_vs,
            ValuedOperand::r4000allegrex_t_vt(..) => Self::r4000allegrex_t_vt,
            ValuedOperand::r4000allegrex_t_vd(..) => Self::r4000allegrex_t_vd,
            ValuedOperand::r4000allegrex_q_vs(..) => Self::r4000allegrex_q_vs,
            ValuedOperand::r4000allegrex_q_vt(..) => Self::r4000allegrex_q_vt,
            ValuedOperand::r4000allegrex_q_vd(..) => Self::r4000allegrex_q_vd,
            ValuedOperand::r4000allegrex_q_vt_imm(..) => Self::r4000allegrex_q_vt_imm,
            ValuedOperand::r4000allegrex_mp_vs(..) => Self::r4000allegrex_mp_vs,
            ValuedOperand::r4000allegrex_mp_vt(..) => Self::r4000allegrex_mp_vt,
            ValuedOperand::r4000allegrex_mp_vd(..) => Self::r4000allegrex_mp_vd,
            ValuedOperand::r4000allegrex_mp_vs_transpose(..) => Self::r4000allegrex_mp_vs_transpose,
            ValuedOperand::r4000allegrex_mt_vs(..) => Self::r4000allegrex_mt_vs,
            ValuedOperand::r4000allegrex_mt_vt(..) => Self::r4000allegrex_mt_vt,
            ValuedOperand::r4000allegrex_mt_vd(..) => Self::r4000allegrex_mt_vd,
            ValuedOperand::r4000allegrex_mt_vs_transpose(..) => Self::r4000allegrex_mt_vs_transpose,
            ValuedOperand::r4000allegrex_mq_vs(..) => Self::r4000allegrex_mq_vs,
            ValuedOperand::r4000allegrex_mq_vt(..) => Self::r4000allegrex_mq_vt,
            ValuedOperand::r4000allegrex_mq_vd(..) => Self::r4000allegrex_mq_vd,
            ValuedOperand::r4000allegrex_mq_vs_transpose(..) => Self::r4000allegrex_mq_vs_transpose,
            ValuedOperand::r4000allegrex_cop2cs(..) => Self::r4000allegrex_cop2cs,
            ValuedOperand::r4000allegrex_cop2cd(..) => Self::r4000allegrex_cop2cd,
            ValuedOperand::r4000allegrex_pos(..) => Self::r4000allegrex_pos,
            ValuedOperand::r4000allegrex_size(..) => Self::r4000allegrex_size,
            ValuedOperand::r4000allegrex_size_plus_pos(..) => Self::r4000allegrex_size_plus_pos,
            ValuedOperand::r4000allegrex_imm3(..) => Self::r4000allegrex_imm3,
            ValuedOperand::r4000allegrex_offset14_base(..) => Self::r4000allegrex_offset14_base,
            ValuedOperand::r4000allegrex_offset14_base_maybe_wb(..) => {
                Self::r4000allegrex_offset14_base_maybe_wb
            }
            ValuedOperand::r4000allegrex_vcmp_cond(..) => Self::r4000allegrex_vcmp_cond,
            ValuedOperand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt(..) => {
                Self::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt
            }
            ValuedOperand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt(..) => {
                Self::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt
            }
            ValuedOperand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt(..) => {
                Self::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt
            }
            ValuedOperand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt(..) => {
                Self::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt
            }
            ValuedOperand::r4000allegrex_vconstant(..) => Self::r4000allegrex_vconstant,
            ValuedOperand::r4000allegrex_power_of_two(..) => Self::r4000allegrex_power_of_two,
            ValuedOperand::r4000allegrex_vfpu_cc_bit(..) => Self::r4000allegrex_vfpu_cc_bit,
            ValuedOperand::r4000allegrex_bn(..) => Self::r4000allegrex_bn,
            ValuedOperand::r4000allegrex_int16(..) => Self::r4000allegrex_int16,
            ValuedOperand::r4000allegrex_float16(..) => Self::r4000allegrex_float16,
            ValuedOperand::r4000allegrex_p_vrot_code(..) => Self::r4000allegrex_p_vrot_code,
            ValuedOperand::r4000allegrex_t_vrot_code(..) => Self::r4000allegrex_t_vrot_code,
            ValuedOperand::r4000allegrex_q_vrot_code(..) => Self::r4000allegrex_q_vrot_code,
            ValuedOperand::r4000allegrex_rpx(..) => Self::r4000allegrex_rpx,
            ValuedOperand::r4000allegrex_rpy(..) => Self::r4000allegrex_rpy,
            ValuedOperand::r4000allegrex_rpz(..) => Self::r4000allegrex_rpz,
            ValuedOperand::r4000allegrex_rpw(..) => Self::r4000allegrex_rpw,
            ValuedOperand::r4000allegrex_wpx(..) => Self::r4000allegrex_wpx,
            ValuedOperand::r4000allegrex_wpy(..) => Self::r4000allegrex_wpy,
            ValuedOperand::r4000allegrex_wpz(..) => Self::r4000allegrex_wpz,
            ValuedOperand::r4000allegrex_wpw(..) => Self::r4000allegrex_wpw,
            ValuedOperand::r5900_I(..) => Self::r5900_I,
            ValuedOperand::r5900_Q(..) => Self::r5900_Q,
            ValuedOperand::r5900_R(..) => Self::r5900_R,
            ValuedOperand::r5900_ACC(..) => Self::r5900_ACC,
            ValuedOperand::r5900_ACCxyzw(..) => Self::r5900_ACCxyzw,
            ValuedOperand::r5900_vfs(..) => Self::r5900_vfs,
            ValuedOperand::r5900_vft(..) => Self::r5900_vft,
            ValuedOperand::r5900_vfd(..) => Self::r5900_vfd,
            ValuedOperand::r5900_vfsxyzw(..) => Self::r5900_vfsxyzw,
            ValuedOperand::r5900_vftxyzw(..) => Self::r5900_vftxyzw,
            ValuedOperand::r5900_vfdxyzw(..) => Self::r5900_vfdxyzw,
            ValuedOperand::r5900_vfsn(..) => Self::r5900_vfsn,
            ValuedOperand::r5900_vftn(..) => Self::r5900_vftn,
            ValuedOperand::r5900_vfdn(..) => Self::r5900_vfdn,
            ValuedOperand::r5900_vfsl(..) => Self::r5900_vfsl,
            ValuedOperand::r5900_vftl(..) => Self::r5900_vftl,
            ValuedOperand::r5900_vfdl(..) => Self::r5900_vfdl,
            ValuedOperand::r5900_vfsm(..) => Self::r5900_vfsm,
            ValuedOperand::r5900_vftm(..) => Self::r5900_vftm,
            ValuedOperand::r5900_vfdm(..) => Self::r5900_vfdm,
            ValuedOperand::r5900_vis(..) => Self::r5900_vis,
            ValuedOperand::r5900_vit(..) => Self::r5900_vit,
            ValuedOperand::r5900_vid(..) => Self::r5900_vid,
            ValuedOperand::r5900_vis_predecr(..) => Self::r5900_vis_predecr,
            ValuedOperand::r5900_vit_predecr(..) => Self::r5900_vit_predecr,
            ValuedOperand::r5900_vid_predecr(..) => Self::r5900_vid_predecr,
            ValuedOperand::r5900_vis_postincr(..) => Self::r5900_vis_postincr,
            ValuedOperand::r5900_vit_postincr(..) => Self::r5900_vit_postincr,
            ValuedOperand::r5900_vid_postincr(..) => Self::r5900_vid_postincr,
            ValuedOperand::r5900_vis_parenthesis(..) => Self::r5900_vis_parenthesis,
            ValuedOperand::r5900_immediate5(..) => Self::r5900_immediate5,
            ValuedOperand::r5900_immediate15(..) => Self::r5900_immediate15,
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

pub struct ValuedOperandIterator<'a> {
    instr: &'a Instruction,
    operands: &'a [Operand; operand::OPERAND_COUNT_MAX],
    index: usize,
}

impl<'a> ValuedOperandIterator<'a> {
    pub(crate) fn new(instr: &'a Instruction) -> Self {
        Self {
            instr,
            operands: instr.opcode().get_descriptor().operands(),
            index: 0,
        }
    }
}

impl<'a> Iterator for ValuedOperandIterator<'a> {
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
