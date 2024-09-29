/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{
    DisplayFlags, Instruction, Operand, OperandDescriptor, OperandDisplay, ValuedOperand, OPERANDS,
};

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `Opcode::MAX` value, so instead we manually maintain
// this constant.
pub(crate) const OPERAND_COUNT: usize = 134;

pub(crate) const OPERAND_COUNT_MAX: usize = 5;

impl Operand {
    #[must_use]
    pub fn get_descriptor(&self) -> &'static OperandDescriptor {
        &OPERANDS[*self]
    }
}

impl Operand {
    #[must_use]
    pub const fn display<'ins, 'imm, 'flg>(
        &self,
        instr: &'ins Instruction,
        imm_override: Option<&'imm str>,
        display_flags: &'flg DisplayFlags,
    ) -> OperandDisplay<'ins, 'imm, 'flg> {
        OperandDisplay::new(*self, instr, imm_override, display_flags)
    }
}

impl Operand {
    pub(crate) fn to_valued_operand(self, instr: &Instruction) -> ValuedOperand {
        match self {
            Self::ALL_EMPTY => ValuedOperand::ALL_EMPTY(),
            Self::cpu_rs => ValuedOperand::cpu_rs(instr.reg_rs_unchecked()),
            Self::cpu_rt => ValuedOperand::cpu_rt(instr.reg_rt_unchecked()),
            Self::cpu_rd => ValuedOperand::cpu_rd(instr.reg_rd_unchecked()),
            Self::cpu_sa => todo!(),
            Self::cpu_zero => ValuedOperand::cpu_zero(),
            Self::cpu_cop0d => todo!(),
            Self::cpu_fs => todo!(),
            Self::cpu_ft => todo!(),
            Self::cpu_fd => todo!(),
            Self::cpu_cop1cs => todo!(),
            Self::cpu_cop2t => todo!(),
            Self::cpu_cop2cd => todo!(),
            Self::cpu_op => todo!(),
            Self::cpu_hint => todo!(),
            Self::cpu_code => todo!(),
            Self::cpu_code_lower => todo!(),
            Self::cpu_copraw => todo!(),
            Self::cpu_label => todo!(),
            Self::cpu_immediate => todo!(),
            Self::cpu_branch_target_label => todo!(),
            Self::cpu_immediate_base => todo!(),
            Self::cpu_maybe_rd_rs => todo!(),
            Self::rsp_rs => todo!(),
            Self::rsp_rt => todo!(),
            Self::rsp_rd => todo!(),
            Self::rsp_cop0d => todo!(),
            Self::rsp_cop2t => todo!(),
            Self::rsp_cop2cd => todo!(),
            Self::rsp_vs => todo!(),
            Self::rsp_vt => todo!(),
            Self::rsp_vd => todo!(),
            Self::rsp_hint => todo!(),
            Self::rsp_vt_elementhigh => todo!(),
            Self::rsp_vt_elementlow => todo!(),
            Self::rsp_vd_de => todo!(),
            Self::rsp_vs_index => todo!(),
            Self::rsp_offset_rs => todo!(),
            Self::rsp_immediate_base => todo!(),
            Self::rsp_maybe_rd_rs => todo!(),
            Self::r3000gte_sf => todo!(),
            Self::r3000gte_mx => todo!(),
            Self::r3000gte_v => todo!(),
            Self::r3000gte_cv => todo!(),
            Self::r3000gte_lm => todo!(),
            Self::r4000allegrex_s_vs => todo!(),
            Self::r4000allegrex_s_vt => todo!(),
            Self::r4000allegrex_s_vd => todo!(),
            Self::r4000allegrex_s_vt_imm => todo!(),
            Self::r4000allegrex_s_vd_imm => todo!(),
            Self::r4000allegrex_p_vs => todo!(),
            Self::r4000allegrex_p_vt => todo!(),
            Self::r4000allegrex_p_vd => todo!(),
            Self::r4000allegrex_t_vs => todo!(),
            Self::r4000allegrex_t_vt => todo!(),
            Self::r4000allegrex_t_vd => todo!(),
            Self::r4000allegrex_q_vs => todo!(),
            Self::r4000allegrex_q_vt => todo!(),
            Self::r4000allegrex_q_vd => todo!(),
            Self::r4000allegrex_q_vt_imm => todo!(),
            Self::r4000allegrex_mp_vs => todo!(),
            Self::r4000allegrex_mp_vt => todo!(),
            Self::r4000allegrex_mp_vd => todo!(),
            Self::r4000allegrex_mp_vs_transpose => todo!(),
            Self::r4000allegrex_mt_vs => todo!(),
            Self::r4000allegrex_mt_vt => todo!(),
            Self::r4000allegrex_mt_vd => todo!(),
            Self::r4000allegrex_mt_vs_transpose => todo!(),
            Self::r4000allegrex_mq_vs => todo!(),
            Self::r4000allegrex_mq_vt => todo!(),
            Self::r4000allegrex_mq_vd => todo!(),
            Self::r4000allegrex_mq_vs_transpose => todo!(),
            Self::r4000allegrex_cop2cs => todo!(),
            Self::r4000allegrex_cop2cd => todo!(),
            Self::r4000allegrex_pos => todo!(),
            Self::r4000allegrex_size => todo!(),
            Self::r4000allegrex_size_plus_pos => todo!(),
            Self::r4000allegrex_imm3 => todo!(),
            Self::r4000allegrex_offset14_base => todo!(),
            Self::r4000allegrex_offset14_base_maybe_wb => todo!(),
            Self::r4000allegrex_vcmp_cond => todo!(),
            Self::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt => todo!(),
            Self::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt => todo!(),
            Self::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt => todo!(),
            Self::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt => todo!(),
            Self::r4000allegrex_vconstant => todo!(),
            Self::r4000allegrex_power_of_two => todo!(),
            Self::r4000allegrex_vfpu_cc_bit => todo!(),
            Self::r4000allegrex_bn => todo!(),
            Self::r4000allegrex_int16 => todo!(),
            Self::r4000allegrex_float16 => todo!(),
            Self::r4000allegrex_p_vrot_code => todo!(),
            Self::r4000allegrex_t_vrot_code => todo!(),
            Self::r4000allegrex_q_vrot_code => todo!(),
            Self::r4000allegrex_rpx => todo!(),
            Self::r4000allegrex_rpy => todo!(),
            Self::r4000allegrex_rpz => todo!(),
            Self::r4000allegrex_rpw => todo!(),
            Self::r4000allegrex_wpx => todo!(),
            Self::r4000allegrex_wpy => todo!(),
            Self::r4000allegrex_wpz => todo!(),
            Self::r4000allegrex_wpw => todo!(),
            Self::r5900_I => ValuedOperand::r5900_I(),
            Self::r5900_Q => ValuedOperand::r5900_Q(),
            Self::r5900_R => ValuedOperand::r5900_R(),
            Self::r5900_ACC => ValuedOperand::r5900_ACC(),
            Self::r5900_ACCxyzw => todo!(),
            Self::r5900_vfs => todo!(),
            Self::r5900_vft => todo!(),
            Self::r5900_vfd => todo!(),
            Self::r5900_vfsxyzw => todo!(),
            Self::r5900_vftxyzw => todo!(),
            Self::r5900_vfdxyzw => todo!(),
            Self::r5900_vfsn => todo!(),
            Self::r5900_vftn => todo!(),
            Self::r5900_vfdn => todo!(),
            Self::r5900_vfsl => todo!(),
            Self::r5900_vftl => todo!(),
            Self::r5900_vfdl => todo!(),
            Self::r5900_vfsm => todo!(),
            Self::r5900_vftm => todo!(),
            Self::r5900_vfdm => todo!(),
            Self::r5900_vis => todo!(),
            Self::r5900_vit => todo!(),
            Self::r5900_vid => todo!(),
            Self::r5900_vis_predecr => todo!(),
            Self::r5900_vit_predecr => todo!(),
            Self::r5900_vid_predecr => todo!(),
            Self::r5900_vis_postincr => todo!(),
            Self::r5900_vit_postincr => todo!(),
            Self::r5900_vid_postincr => todo!(),
            Self::r5900_vis_parenthesis => todo!(),
            Self::r5900_immediate5 => todo!(),
            Self::r5900_immediate15 => todo!(),
        }
    }
}

impl Operand {
    #[must_use]
    pub const fn default() -> Self {
        Self::ALL_EMPTY
    }

    #[must_use]
    pub const fn arr0() -> [Self; OPERAND_COUNT_MAX] {
        [Self::default(); OPERAND_COUNT_MAX]
    }
    #[must_use]
    pub const fn arr1(op0: Self) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr
    }
    #[must_use]
    pub const fn arr2(op0: Self, op1: Self) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr[1] = op1;
        arr
    }
    #[must_use]
    pub const fn arr3(op0: Self, op1: Self, op2: Self) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr[1] = op1;
        arr[2] = op2;
        arr
    }
    #[must_use]
    pub const fn arr4(op0: Self, op1: Self, op2: Self, op3: Self) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr[1] = op1;
        arr[2] = op2;
        arr[3] = op3;
        arr
    }
    #[must_use]
    pub const fn arr5(
        op0: Self,
        op1: Self,
        op2: Self,
        op3: Self,
        op4: Self,
    ) -> [Self; OPERAND_COUNT_MAX] {
        let mut arr = [Self::default(); OPERAND_COUNT_MAX];
        arr[0] = op0;
        arr[1] = op1;
        arr[2] = op2;
        arr[3] = op3;
        arr[4] = op4;
        arr
    }
}

impl Default for Operand {
    fn default() -> Self {
        Self::default()
    }
}

pub struct OperandIterator<'ins> {
    operands: &'ins [Operand; OPERAND_COUNT_MAX],
    index: usize,
}

impl<'ins> OperandIterator<'ins> {
    pub(crate) const fn new(operands: &'ins [Operand; OPERAND_COUNT_MAX]) -> Self {
        Self { operands, index: 0 }
    }
}

impl<'ins> Iterator for OperandIterator<'ins> {
    type Item = &'ins Operand;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.operands.len() {
            return None;
        }

        let val = &self.operands[self.index];
        if *val == Operand::default() {
            return None;
        }

        self.index += 1;
        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use crate::Opcode;

    use super::*;

    #[test]
    fn test_addiu_operands() {
        let mut operands = Opcode::cpu_addiu.get_descriptor().operands_iter();

        assert_eq!(operands.next(), Some(Operand::cpu_rt).as_ref());
        assert_eq!(operands.next(), Some(Operand::cpu_rs).as_ref());
        assert_eq!(operands.next(), Some(Operand::cpu_immediate).as_ref());
        assert_eq!(operands.next(), None);
    }
}
