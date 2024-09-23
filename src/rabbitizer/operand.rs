/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{Instruction, Operand, OperandDescriptor, ValuedOperand, OPERANDS};

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `Opcode::MAX` value, so instead we manually maintain
// this constant.
pub(crate) const OPERAND_COUNT: usize = 134;

pub(crate) const OPERAND_COUNT_MAX: usize = 5;

impl<'a> Operand {
    #[must_use]
    pub fn get_descriptor(&self) -> &'a OperandDescriptor {
        &OPERANDS[*self]
    }
}

impl Operand {
    pub(crate) const fn to_valued_operand(&self, _instr: &Instruction) -> ValuedOperand {
        match *self {
            Operand::ALL_EMPTY => ValuedOperand::ALL_EMPTY(),
            Operand::cpu_rs => todo!(),
            Operand::cpu_rt => todo!(),
            Operand::cpu_rd => todo!(),
            Operand::cpu_sa => todo!(),
            Operand::cpu_zero => ValuedOperand::cpu_zero(),
            Operand::cpu_cop0d => todo!(),
            Operand::cpu_fs => todo!(),
            Operand::cpu_ft => todo!(),
            Operand::cpu_fd => todo!(),
            Operand::cpu_cop1cs => todo!(),
            Operand::cpu_cop2t => todo!(),
            Operand::cpu_cop2cd => todo!(),
            Operand::cpu_op => todo!(),
            Operand::cpu_hint => todo!(),
            Operand::cpu_code => todo!(),
            Operand::cpu_code_lower => todo!(),
            Operand::cpu_copraw => todo!(),
            Operand::cpu_label => todo!(),
            Operand::cpu_immediate => todo!(),
            Operand::cpu_branch_target_label => todo!(),
            Operand::cpu_immediate_base => todo!(),
            Operand::cpu_maybe_rd_rs => todo!(),
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
            Operand::r3000gte_sf => todo!(),
            Operand::r3000gte_mx => todo!(),
            Operand::r3000gte_v => todo!(),
            Operand::r3000gte_cv => todo!(),
            Operand::r3000gte_lm => todo!(),
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
            Operand::r5900_I => ValuedOperand::r5900_I(),
            Operand::r5900_Q => ValuedOperand::r5900_Q(),
            Operand::r5900_R => ValuedOperand::r5900_R(),
            Operand::r5900_ACC => ValuedOperand::r5900_ACC(),
            Operand::r5900_ACCxyzw => todo!(),
            Operand::r5900_vfs => todo!(),
            Operand::r5900_vft => todo!(),
            Operand::r5900_vfd => todo!(),
            Operand::r5900_vfsxyzw => todo!(),
            Operand::r5900_vftxyzw => todo!(),
            Operand::r5900_vfdxyzw => todo!(),
            Operand::r5900_vfsn => todo!(),
            Operand::r5900_vftn => todo!(),
            Operand::r5900_vfdn => todo!(),
            Operand::r5900_vfsl => todo!(),
            Operand::r5900_vftl => todo!(),
            Operand::r5900_vfdl => todo!(),
            Operand::r5900_vfsm => todo!(),
            Operand::r5900_vftm => todo!(),
            Operand::r5900_vfdm => todo!(),
            Operand::r5900_vis => todo!(),
            Operand::r5900_vit => todo!(),
            Operand::r5900_vid => todo!(),
            Operand::r5900_vis_predecr => todo!(),
            Operand::r5900_vit_predecr => todo!(),
            Operand::r5900_vid_predecr => todo!(),
            Operand::r5900_vis_postincr => todo!(),
            Operand::r5900_vit_postincr => todo!(),
            Operand::r5900_vid_postincr => todo!(),
            Operand::r5900_vis_parenthesis => todo!(),
            Operand::r5900_immediate5 => todo!(),
            Operand::r5900_immediate15 => todo!(),
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

pub struct OperandIterator<'a> {
    operands: &'a [Operand; OPERAND_COUNT_MAX],
    index: usize,
}

impl<'a> OperandIterator<'a> {
    pub(crate) const fn new(operands: &'a [Operand; OPERAND_COUNT_MAX]) -> Self {
        Self { operands, index: 0 }
    }
}

impl<'a> Iterator for OperandIterator<'a> {
    type Item = &'a Operand;

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
