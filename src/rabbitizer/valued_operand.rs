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
