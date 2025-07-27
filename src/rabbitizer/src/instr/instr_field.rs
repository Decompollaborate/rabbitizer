/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::encoded_field_mask::EncodedFieldMask;
use crate::instr::Instruction;
use crate::operands::Operand;
use crate::registers::*;

#[cfg(any(feature = "R4000ALLEGREX", feature = "R5900EE",))]
use crate::utils;

/// This struct is created by [`Instruction::field()`].
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrField<'ins> {
    instr: &'ins Instruction,
}

impl<'ins> InstrField<'ins> {
    pub(crate) const fn new(instr: &'ins Instruction) -> Self {
        Self { instr }
    }
}

/// Opcode fields
impl InstrField<'_> {
    /// Returns either the [`Gpr`] register embedded on the `rs` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// an `rs` operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`None`]: Option::None
    #[must_use]
    pub fn rs(&self) -> Option<Gpr> {
        if !self.instr.opcode().has_operand_alias(Operand::core_rs) {
            return None;
        }
        Some(self.rs_impl())
    }

    /// Returns either the [`Gpr`] register embedded on the `rt` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// an `rt` operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`None`]: Option::None
    #[must_use]
    pub fn rt(&self) -> Option<Gpr> {
        if !self.instr.opcode().has_operand_alias(Operand::core_rt) {
            return None;
        }
        Some(self.rt_impl())
    }

    /// Returns either the [`Gpr`] register embedded on the `rd` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// an `rd` operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`None`]: Option::None
    #[must_use]
    pub fn rd(&self) -> Option<Gpr> {
        if !self.instr.opcode().has_operand_alias(Operand::core_rd) {
            return None;
        }
        Some(self.rd_impl())
    }

    /// Returns either the `sa` value embedded on the `sa`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn sa(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::core_sa) {
            Some(self.sa_impl())
        } else {
            None
        }
    }

    /// Returns either the [`Cop0`] register embedded on the `cop0d` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop0`]: crate::registers::Cop0
    /// [`None`]: Option::None
    #[must_use]
    pub fn cop0d(&self) -> Option<Cop0> {
        if self.instr.opcode().has_operand_alias(Operand::core_cop0d) {
            Some(self.cop0d_impl())
        } else {
            None
        }
    }

    /// Returns either the [`Cop0Control`] register embedded on the `cop0cd` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop0Control`]: crate::registers::Cop0Control
    /// [`None`]: Option::None
    #[must_use]
    pub fn cop0cd(&self) -> Option<Cop0Control> {
        if self.instr.opcode().has_operand_alias(Operand::core_cop0cd) {
            Some(self.cop0cd_impl())
        } else {
            None
        }
    }

    /// Returns either the [`Cop1`] register embedded on the `fs` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`None`]: Option::None
    #[must_use]
    pub fn fs(&self) -> Option<Cop1> {
        if self.instr.opcode().has_operand_alias(Operand::core_fs) {
            Some(self.fs_impl())
        } else {
            None
        }
    }

    /// Returns either the [`Cop1`] register embedded on the `ft` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`None`]: Option::None
    #[must_use]
    pub fn ft(&self) -> Option<Cop1> {
        if self.instr.opcode().has_operand_alias(Operand::core_ft) {
            Some(self.ft_impl())
        } else {
            None
        }
    }

    /// Returns either the [`Cop1`] register embedded on the `fd` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`None`]: Option::None
    #[must_use]
    pub fn fd(&self) -> Option<Cop1> {
        if self.instr.opcode().has_operand_alias(Operand::core_fd) {
            Some(self.fd_impl())
        } else {
            None
        }
    }

    /// Returns either the [`Cop1Control`] register embedded on the `cop1cs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have this field.
    ///
    /// [`Cop1Control`]: crate::registers::Cop1Control
    /// [`None`]: Option::None
    #[must_use]
    pub fn cop1cs(&self) -> Option<Cop1Control> {
        if self.instr.opcode().has_operand_alias(Operand::core_cop1cs) {
            Some(self.cop1cs_impl())
        } else {
            None
        }
    }

    /// Returns either the [`Cop2`] register embedded on the `cop2t` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop2`]: crate::registers::Cop2
    /// [`None`]: Option::None
    #[must_use]
    pub fn cop2t(&self) -> Option<Cop2> {
        if self.instr.opcode().has_operand_alias(Operand::core_cop2t) {
            Some(self.cop2d_impl())
        } else {
            None
        }
    }

    /// Returns either the [`Cop2`] register embedded on the `cop2d` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop2`]: crate::registers::Cop2
    /// [`None`]: Option::None
    #[must_use]
    pub fn cop2d(&self) -> Option<Cop2> {
        if self.instr.opcode().has_operand_alias(Operand::core_cop2d) {
            Some(self.cop2d_impl())
        } else {
            None
        }
    }

    /// Returns either the [`Cop2Control`] register embedded on the `cop2cd` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop2Control`]: crate::registers::Cop2Control
    /// [`None`]: Option::None
    #[must_use]
    pub fn cop2cd(&self) -> Option<Cop2Control> {
        if self.instr.opcode().has_operand_alias(Operand::core_cop2cd) {
            Some(self.cop2cd_impl())
        } else {
            None
        }
    }

    /// Returns either the `instr_index` value embedded on the `instr_index`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have an `instr_index` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn instr_index(&self) -> Option<u32> {
        if self.instr.opcode().has_operand_alias(Operand::core_label) {
            Some(self.instr_index_impl())
        } else {
            None
        }
    }

    /// Returns either the `imm_i16` value embedded on the `immediate` field
    /// of the word of this instruction, or [`None`] if this instruction does
    /// not have an `imm_i16` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn imm_i16(&self) -> Option<i16> {
        if !self.instr.opcode().has_operand_alias(Operand::core_imm_i16) {
            return None;
        }
        Some(self.imm_i16_impl())
    }

    /// Returns either the `imm_u16` value embedded on the `immediate` field
    /// of the word of this instruction, or [`None`] if this instruction does
    /// not have an `imm_u16` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn imm_u16(&self) -> Option<u16> {
        if !self.instr.opcode().has_operand_alias(Operand::core_imm_u16) {
            return None;
        }
        Some(self.imm_u16_impl())
    }

    /// Returns either the `op` value embedded on the `op`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have an `op` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn op(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::core_op) {
            Some(self.op_impl())
        } else {
            None
        }
    }

    /// Returns either the `hint` value embedded on the `hint`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have an `hint` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn hint(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::core_hint) {
            Some(self.hint_impl())
        } else {
            None
        }
    }

    /// Returns either the `code_upper` value embedded on the `code_upper`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have an `code_upper` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn code_upper(&self) -> Option<u16> {
        if self.instr.opcode().has_operand_alias(Operand::core_code) {
            Some(self.code_upper_impl())
        } else {
            None
        }
    }

    /// Returns either the `code_lower` value embedded on the `code_lower`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have an `code_lower` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn code_lower(&self) -> Option<u16> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::core_code_lower)
        {
            Some(self.code_lower_impl())
        } else {
            None
        }
    }
}

/// RSP opcode fields
#[cfg(feature = "RSP")]
impl InstrField<'_> {
    /// Returns either the [`RspCop0`] register embedded on the `rsp_cop0d`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`RspCop0`]: crate::registers::RspCop0
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_cop0d(&self) -> Option<RspCop0> {
        if self.instr.opcode().has_operand_alias(Operand::rsp_cop0d) {
            Some(self.rsp_cop0d_impl())
        } else {
            None
        }
    }

    /// Returns either the [`RspCop2`] register embedded on the `rsp_cop2cd`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`RspCop2`]: crate::registers::RspCop2
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_cop2cd(&self) -> Option<RspCop2> {
        if self.instr.opcode().has_operand_alias(Operand::rsp_cop2cd) {
            Some(self.rsp_cop2cd_impl())
        } else {
            None
        }
    }

    /// Returns either the [`RspVector`] register value embedded on the `rsp_vs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`RspCop2`]: crate::registers::RspVector
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_vs(&self) -> Option<RspVector> {
        if self.instr.opcode().has_operand_alias(Operand::rsp_vs) {
            Some(self.rsp_vs_impl())
        } else {
            None
        }
    }

    /// Returns either the [`RspVector`] register embedded on the `rsp_vt`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`RspCop2`]: crate::registers::RspVector
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_vt(&self) -> Option<RspVector> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::rsp_vt_elementhigh)
            || self
                .instr
                .opcode()
                .has_operand_alias(Operand::rsp_vt_elementlow)
        {
            Some(self.rsp_vt_impl())
        } else {
            None
        }
    }

    /// Returns either the [`RspVector`] register embedded on the `rsp_vd`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`RspCop2`]: crate::registers::RspVector
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_vd(&self) -> Option<RspVector> {
        if self.instr.opcode().has_operand_alias(Operand::rsp_vd) {
            Some(self.rsp_vd_impl())
        } else {
            None
        }
    }

    /// Returns either the `rsp_elementhigh` value embedded on the `rsp_elementhigh`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_elementhigh(&self) -> Option<u8> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::rsp_vt_elementhigh)
        {
            Some(self.rsp_elementhigh_impl())
        } else {
            None
        }
    }

    /// Returns either the `rsp_elementlow` value embedded on the `rsp_elementlow`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_elementlow(&self) -> Option<u8> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::rsp_vt_elementlow)
        {
            Some(self.rsp_elementlow_impl())
        } else {
            None
        }
    }

    /// Returns either the `rsp_index` value embedded on the `rsp_index`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_index(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::rsp_vs_index) {
            Some(self.rsp_index_impl())
        } else {
            None
        }
    }

    /// Returns either the `rsp_offset7` value embedded on the `rsp_offset7`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_offset7(&self) -> Option<u8> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::rsp_offset7_rs)
        {
            Some(self.rsp_offset7_impl())
        } else {
            None
        }
    }

    /// Returns either the `rsp_offset8` value embedded on the `rsp_offset8`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_offset8(&self) -> Option<u8> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::rsp_offset8_rs)
        {
            Some(self.rsp_offset8_impl())
        } else {
            None
        }
    }

    /// Returns either the `rsp_offset9` value embedded on the `rsp_offset9`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_offset9(&self) -> Option<u16> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::rsp_offset9_rs)
        {
            Some(self.rsp_offset9_impl())
        } else {
            None
        }
    }

    /// Returns either the `rsp_offset10` value embedded on the `rsp_offset10`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_offset10(&self) -> Option<u16> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::rsp_offset10_rs)
        {
            Some(self.rsp_offset10_impl())
        } else {
            None
        }
    }

    /// Returns either the `rsp_offset11` value embedded on the `rsp_offset11`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_offset11(&self) -> Option<u16> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::rsp_offset11_rs)
        {
            Some(self.rsp_offset11_impl())
        } else {
            None
        }
    }

    /// Returns either the `rsp_de` value embedded on the `rsp_de`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn rsp_de(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::rsp_vd_de) {
            Some(self.rsp_de_impl())
        } else {
            None
        }
    }
}

/// R3000GTE opcode fields
#[cfg(feature = "R3000GTE")]
impl InstrField<'_> {
    /// Returns either the `r3000gte_sf` value embedded on the `r3000gte_sf`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r3000gte_sf(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::r3000gte_sf) {
            Some(self.r3000gte_sf_impl())
        } else {
            None
        }
    }

    /// Returns either the `r3000gte_mx` value embedded on the `r3000gte_mx`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r3000gte_mx(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::r3000gte_mx) {
            Some(self.r3000gte_mx_impl())
        } else {
            None
        }
    }

    /// Returns either the `r3000gte_v` value embedded on the `r3000gte_v`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r3000gte_v(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::r3000gte_v) {
            Some(self.r3000gte_v_impl())
        } else {
            None
        }
    }

    /// Returns either the `r3000gte_cv` value embedded on the `r3000gte_cv`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r3000gte_cv(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::r3000gte_cv) {
            Some(self.r3000gte_cv_impl())
        } else {
            None
        }
    }

    /// Returns either the `r3000gte_lm` value embedded on the `r3000gte_lm`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r3000gte_lm(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::r3000gte_lm) {
            Some(self.r3000gte_lm_impl())
        } else {
            None
        }
    }
}

/// R4000ALLEGREX opcode fields
#[cfg(feature = "R4000ALLEGREX")]
impl InstrField<'_> {
    /// Returns either the `r4000allegrex_s_vs` value embedded on the `r4000allegrex_s_vs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_s_vs(&self) -> Option<R4000AllegrexS> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_s_vs)
        {
            Some(self.r4000allegrex_s_vs_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_s_vt` value embedded on the `r4000allegrex_s_vt`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_s_vt(&self) -> Option<R4000AllegrexS> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_s_vt)
        {
            Some(self.r4000allegrex_s_vt_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_s_vd` value embedded on the `r4000allegrex_s_vd`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_s_vd(&self) -> Option<R4000AllegrexS> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_s_vd)
        {
            Some(self.r4000allegrex_s_vd_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_s_vt_imm` value embedded on the `r4000allegrex_s_vt_imm`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_s_vt_imm(&self) -> Option<R4000AllegrexS> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_s_vt_imm)
        {
            Some(self.r4000allegrex_s_vt_imm_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_s_vd_imm` value embedded on the `r4000allegrex_s_vd_imm`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_s_vd_imm(&self) -> Option<R4000AllegrexS> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_s_vd_imm)
        {
            Some(self.r4000allegrex_s_vd_imm_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_p_vs` value embedded on the `r4000allegrex_p_vs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_p_vs(&self) -> Option<R4000AllegrexV2D> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_p_vs)
        {
            Some(self.r4000allegrex_p_vs_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_p_vt` value embedded on the `r4000allegrex_p_vt`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_p_vt(&self) -> Option<R4000AllegrexV2D> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_p_vt)
        {
            Some(self.r4000allegrex_p_vt_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_p_vd` value embedded on the `r4000allegrex_p_vd`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_p_vd(&self) -> Option<R4000AllegrexV2D> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_p_vd)
        {
            Some(self.r4000allegrex_p_vd_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_t_vs` value embedded on the `r4000allegrex_t_vs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_t_vs(&self) -> Option<R4000AllegrexV3D> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_t_vs)
        {
            Some(self.r4000allegrex_t_vs_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_t_vt` value embedded on the `r4000allegrex_t_vt`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_t_vt(&self) -> Option<R4000AllegrexV3D> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_t_vt)
        {
            Some(self.r4000allegrex_t_vt_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_t_vd` value embedded on the `r4000allegrex_t_vd`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_t_vd(&self) -> Option<R4000AllegrexV3D> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_t_vd)
        {
            Some(self.r4000allegrex_t_vd_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_q_vs` value embedded on the `r4000allegrex_q_vs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_q_vs(&self) -> Option<R4000AllegrexV4D> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_q_vs)
        {
            Some(self.r4000allegrex_q_vs_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_q_vt` value embedded on the `r4000allegrex_q_vt`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_q_vt(&self) -> Option<R4000AllegrexV4D> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_q_vt)
        {
            Some(self.r4000allegrex_q_vt_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_q_vd` value embedded on the `r4000allegrex_q_vd`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_q_vd(&self) -> Option<R4000AllegrexV4D> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_q_vd)
        {
            Some(self.r4000allegrex_q_vd_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_q_vt_imm` value embedded on the `r4000allegrex_q_vt_imm`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_q_vt_imm(&self) -> Option<R4000AllegrexV4D> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_q_vt_imm)
        {
            Some(self.r4000allegrex_q_vt_imm_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_mp_vs` value embedded on the `r4000allegrex_mp_vs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_mp_vs(&self) -> Option<R4000AllegrexM2x2> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_mp_vs)
        {
            Some(self.r4000allegrex_mp_vs_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_mp_vt` value embedded on the `r4000allegrex_mp_vt`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_mp_vt(&self) -> Option<R4000AllegrexM2x2> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_mp_vt)
        {
            Some(self.r4000allegrex_mp_vt_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_mp_vd` value embedded on the `r4000allegrex_mp_vd`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_mp_vd(&self) -> Option<R4000AllegrexM2x2> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_mp_vd)
        {
            Some(self.r4000allegrex_mp_vd_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_mp_vs_transpose` value embedded on the `r4000allegrex_mp_vs_transpose`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_mp_vs_transpose(&self) -> Option<R4000AllegrexM2x2> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_mp_vs_transpose)
        {
            Some(self.r4000allegrex_mp_vs_transpose_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_mt_vs` value embedded on the `r4000allegrex_mt_vs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_mt_vs(&self) -> Option<R4000AllegrexM3x3> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_mt_vs)
        {
            Some(self.r4000allegrex_mt_vs_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_mt_vt` value embedded on the `r4000allegrex_mt_vt`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_mt_vt(&self) -> Option<R4000AllegrexM3x3> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_mt_vt)
        {
            Some(self.r4000allegrex_mt_vt_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_mt_vd` value embedded on the `r4000allegrex_mt_vd`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_mt_vd(&self) -> Option<R4000AllegrexM3x3> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_mt_vd)
        {
            Some(self.r4000allegrex_mt_vd_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_mt_vs_transpose` value embedded on the `r4000allegrex_mt_vs_transpose`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_mt_vs_transpose(&self) -> Option<R4000AllegrexM3x3> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_mt_vs_transpose)
        {
            Some(self.r4000allegrex_mt_vs_transpose_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_mq_vs` value embedded on the `r4000allegrex_mq_vs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_mq_vs(&self) -> Option<R4000AllegrexM4x4> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_mq_vs)
        {
            Some(self.r4000allegrex_mq_vs_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_mq_vt` value embedded on the `r4000allegrex_mq_vt`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_mq_vt(&self) -> Option<R4000AllegrexM4x4> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_mq_vt)
        {
            Some(self.r4000allegrex_mq_vt_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_mq_vd` value embedded on the `r4000allegrex_mq_vd`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_mq_vd(&self) -> Option<R4000AllegrexM4x4> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_mq_vd)
        {
            Some(self.r4000allegrex_mq_vd_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_mq_vs_transpose` value embedded on the `r4000allegrex_mq_vs_transpose`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_mq_vs_transpose(&self) -> Option<R4000AllegrexM4x4> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_mq_vs_transpose)
        {
            Some(self.r4000allegrex_mq_vs_transpose_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_cop2cs` value embedded on the `r4000allegrex_cop2cs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_cop2cs(&self) -> Option<R4000AllegrexVfpuControl> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_cop2cs)
        {
            Some(self.r4000allegrex_cop2cs_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_cop2cd` value embedded on the `r4000allegrex_cop2cd`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_cop2cd(&self) -> Option<R4000AllegrexVfpuControl> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_cop2cd)
        {
            Some(self.r4000allegrex_cop2cd_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_pos` value embedded on the `r4000allegrex_pos`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_pos(&self) -> Option<u8> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_pos)
        {
            Some(self.r4000allegrex_pos_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_size` value embedded on the `r4000allegrex_size`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_size(&self) -> Option<u8> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_size)
        {
            Some(self.r4000allegrex_size_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_size_plus_pos` value embedded on the `r4000allegrex_size_plus_pos`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_size_plus_pos(&self) -> Option<i8> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_size_plus_pos)
        {
            Some(self.r4000allegrex_size_plus_pos_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_imm3` value embedded on the `r4000allegrex_imm3`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_imm3(&self) -> Option<u8> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_imm3)
        {
            Some(self.r4000allegrex_imm3_impl())
        } else {
            None
        }
    }

    /*
    /// Returns either the `r4000allegrex_offset14` value embedded on the `r4000allegrex_offset14`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_offset14(&self) -> Option<u16> {
        if self.instr.opcode().has_operand_alias(Operand::r4000allegrex_offset14) {
            Some(self.r4000allegrex_offset14_impl())
        } else {
            None
        }
    }
    */

    /*
    /// Returns either the `r4000allegrex_wb` value embedded on the `r4000allegrex_wb`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_wb(&self) -> Option<bool> {
        if self.instr.opcode().has_operand_alias(Operand::r4000allegrex_wb) {
            Some(self.r4000allegrex_wb_impl())
        } else {
            None
        }
    }
    */

    /*
    /// Returns either the `r4000allegrex_vcmp_cond` value embedded on the `r4000allegrex_vcmp_cond`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_vcmp_cond(&self) -> Option<R4000AllegrexVCond> {
        if self.instr.opcode().has_operand_alias(Operand::r4000allegrex_vcmp_cond) {
            Some(self.r4000allegrex_vcmp_cond_impl())
        } else {
            None
        }
    }
    */

    /// Returns either the `r4000allegrex_vconstant` value embedded on the `r4000allegrex_vconstant`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_vconstant(&self) -> Option<R4000AllegrexVConstant> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_vconstant)
        {
            Some(self.r4000allegrex_vconstant_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_power_of_two` value embedded on the `r4000allegrex_power_of_two`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_power_of_two(&self) -> Option<u8> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_power_of_two)
        {
            Some(self.r4000allegrex_power_of_two_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_vfpu_cc_bit` value embedded on the `r4000allegrex_vfpu_cc_bit`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_vfpu_cc_bit(&self) -> Option<u8> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_vfpu_cc_bit)
        {
            Some(self.r4000allegrex_vfpu_cc_bit_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_bn` value embedded on the `r4000allegrex_bn`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_bn(&self) -> Option<u8> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_bn)
        {
            Some(self.r4000allegrex_bn_impl())
        } else {
            None
        }
    }

    /*
    /// Returns either the `r4000allegrex_intfloat16` value embedded on the `r4000allegrex_intfloat16`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_intfloat16(&self) -> Option<u16> {
        if self.instr.opcode().has_operand_alias(Operand::r4000allegrex_intfloat16) {
            Some(self.r4000allegrex_intfloat16_impl())
        } else {
            None
        }
    }
    */

    /// Returns either the `r4000allegrex_int16` value embedded on the `r4000allegrex_int16`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_int16(&self) -> Option<i16> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_int16)
        {
            Some(self.r4000allegrex_int16_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_float16` value embedded on the `r4000allegrex_float16`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_float16(&self) -> Option<f32> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_float16)
        {
            Some(self.r4000allegrex_float16_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_vrot_code` value embedded on the `r4000allegrex_vrot_code`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_vrot_code(&self) -> Option<u8> {
        let opcode = self.instr.opcode();
        if opcode.has_operand_alias(Operand::r4000allegrex_p_vrot_code)
            || opcode.has_operand_alias(Operand::r4000allegrex_t_vrot_code)
            || opcode.has_operand_alias(Operand::r4000allegrex_q_vrot_code)
        {
            Some(self.r4000allegrex_vrot_code_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_wpx` value embedded on the `r4000allegrex_wpx`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_wpx(&self) -> Option<R4000AllegrexPrefixDst> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_wpx)
        {
            Some(self.r4000allegrex_wpx_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_wpy` value embedded on the `r4000allegrex_wpy`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_wpy(&self) -> Option<R4000AllegrexPrefixDst> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_wpy)
        {
            Some(self.r4000allegrex_wpy_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_wpz` value embedded on the `r4000allegrex_wpz`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_wpz(&self) -> Option<R4000AllegrexPrefixDst> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_wpz)
        {
            Some(self.r4000allegrex_wpz_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_wpw` value embedded on the `r4000allegrex_wpw`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_wpw(&self) -> Option<R4000AllegrexPrefixDst> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_wpw)
        {
            Some(self.r4000allegrex_wpw_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_rpx` value embedded on the `r4000allegrex_rpx`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_rpx(&self) -> Option<R4000AllegrexPrefixSrc> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_rpx)
        {
            Some(self.r4000allegrex_rpx_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_rpy` value embedded on the `r4000allegrex_rpy`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_rpy(&self) -> Option<R4000AllegrexPrefixSrc> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_rpy)
        {
            Some(self.r4000allegrex_rpy_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_rpz` value embedded on the `r4000allegrex_rpz`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_rpz(&self) -> Option<R4000AllegrexPrefixSrc> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_rpz)
        {
            Some(self.r4000allegrex_rpz_impl())
        } else {
            None
        }
    }

    /// Returns either the `r4000allegrex_rpw` value embedded on the `r4000allegrex_rpw`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r4000allegrex_rpw(&self) -> Option<R4000AllegrexPrefixSrc> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_rpw)
        {
            Some(self.r4000allegrex_rpw_impl())
        } else {
            None
        }
    }
}

/// R5900EE opcode fields
#[cfg(feature = "R5900EE")]
impl InstrField<'_> {
    /// Returns either the `r5900ee_imm5` value embedded on the `r5900ee_imm5`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_imm5(&self) -> Option<i8> {
        if self.instr.opcode().has_operand_alias(Operand::r5900ee_imm5) {
            Some(self.r5900ee_imm5_impl())
        } else {
            None
        }
    }

    /// Returns either the `r5900ee_imm15` value embedded on the `r5900ee_imm15`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_imm15(&self) -> Option<u16> {
        if self
            .instr
            .opcode()
            .has_operand_alias(Operand::r5900ee_imm15)
        {
            Some(self.r5900ee_imm15_impl())
        } else {
            None
        }
    }

    /// Returns either the [`R5900EEVF`] register embedded on the `r5900ee_vfs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`R5900EEVF`]: crate::registers::R5900EEVF
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_vfs(&self) -> Option<R5900EEVF> {
        if self.instr.opcode().has_operand_alias(Operand::r5900ee_vfs) {
            Some(self.r5900ee_vfs_impl())
        } else {
            None
        }
    }

    /// Returns either the [`R5900EEVF`] register embedded on the `r5900ee_vft`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`R5900EEVF`]: crate::registers::R5900EEVF
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_vft(&self) -> Option<R5900EEVF> {
        if self.instr.opcode().has_operand_alias(Operand::r5900ee_vft) {
            Some(self.r5900ee_vft_impl())
        } else {
            None
        }
    }

    /// Returns either the [`R5900EEVF`] register embedded on the `r5900ee_vfd`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`R5900EEVF`]: crate::registers::R5900EEVF
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_vfd(&self) -> Option<R5900EEVF> {
        if self.instr.opcode().has_operand_alias(Operand::r5900ee_vfd) {
            Some(self.r5900ee_vfd_impl())
        } else {
            None
        }
    }

    /// Returns either the [`R5900EEVI`] register value embedded on the `r5900ee_vis`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`R5900EEVI`]: crate::registers::R5900EEVI
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_vis(&self) -> Option<R5900EEVI> {
        if self.instr.opcode().has_operand_alias(Operand::r5900ee_vis) {
            Some(self.r5900ee_vis_impl())
        } else {
            None
        }
    }

    /// Returns either the [`R5900EEVI`] register embedded on the `r5900ee_vit`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`R5900EEVI`]: crate::registers::R5900EEVI
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_vit(&self) -> Option<R5900EEVI> {
        if self.instr.opcode().has_operand_alias(Operand::r5900ee_vit) {
            Some(self.r5900ee_vit_impl())
        } else {
            None
        }
    }

    /// Returns either the [`R5900EEVI`] register embedded on the `r5900ee_vid`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`R5900EEVI`]: crate::registers::R5900EEVI
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_vid(&self) -> Option<R5900EEVI> {
        if self.instr.opcode().has_operand_alias(Operand::r5900ee_vid) {
            Some(self.r5900ee_vid_impl())
        } else {
            None
        }
    }

    /// Returns either the `r5900ee_xyzw_x` value embedded on the `r5900ee_xyzw_x`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_xyzw_x(&self) -> Option<bool> {
        let opcode = self.instr.opcode();

        if opcode.has_operand_alias(Operand::r5900ee_ACCxyzw)
            || opcode.has_operand_alias(Operand::r5900ee_vfsxyzw)
            || opcode.has_operand_alias(Operand::r5900ee_vftxyzw)
            || opcode.has_operand_alias(Operand::r5900ee_vfdxyzw)
        {
            Some(self.r5900ee_xyzw_x_impl())
        } else {
            None
        }
    }

    /// Returns either the `r5900ee_xyzw_y` value embedded on the `r5900ee_xyzw_y`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_xyzw_y(&self) -> Option<bool> {
        let opcode = self.instr.opcode();

        if opcode.has_operand_alias(Operand::r5900ee_ACCxyzw)
            || opcode.has_operand_alias(Operand::r5900ee_vfsxyzw)
            || opcode.has_operand_alias(Operand::r5900ee_vftxyzw)
            || opcode.has_operand_alias(Operand::r5900ee_vfdxyzw)
        {
            Some(self.r5900ee_xyzw_y_impl())
        } else {
            None
        }
    }

    /// Returns either the `r5900ee_xyzw_z` value embedded on the `r5900ee_xyzw_z`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_xyzw_z(&self) -> Option<bool> {
        let opcode = self.instr.opcode();

        if opcode.has_operand_alias(Operand::r5900ee_ACCxyzw)
            || opcode.has_operand_alias(Operand::r5900ee_vfsxyzw)
            || opcode.has_operand_alias(Operand::r5900ee_vftxyzw)
            || opcode.has_operand_alias(Operand::r5900ee_vfdxyzw)
        {
            Some(self.r5900ee_xyzw_z_impl())
        } else {
            None
        }
    }

    /// Returns either the `r5900ee_xyzw_w` value embedded on the `r5900ee_xyzw_w`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_xyzw_w(&self) -> Option<bool> {
        let opcode = self.instr.opcode();

        if opcode.has_operand_alias(Operand::r5900ee_ACCxyzw)
            || opcode.has_operand_alias(Operand::r5900ee_vfsxyzw)
            || opcode.has_operand_alias(Operand::r5900ee_vftxyzw)
            || opcode.has_operand_alias(Operand::r5900ee_vfdxyzw)
        {
            Some(self.r5900ee_xyzw_w_impl())
        } else {
            None
        }
    }

    /// Returns either the `r5900ee_n` value embedded on the `r5900ee_n`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_n(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::r5900ee_vftn) {
            Some(self.r5900ee_n_impl())
        } else {
            None
        }
    }

    /// Returns either the `r5900ee_l` value embedded on the `r5900ee_l`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_l(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::r5900ee_vfsl) {
            Some(self.r5900ee_l_impl())
        } else {
            None
        }
    }

    /// Returns either the `r5900ee_m` value embedded on the `r5900ee_m`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn r5900ee_m(&self) -> Option<u8> {
        if self.instr.opcode().has_operand_alias(Operand::r5900ee_vftm) {
            Some(self.r5900ee_m_impl())
        } else {
            None
        }
    }
}

/// Unchecked opcode fields
#[cfg(feature = "unchecked_instr_fields")]
#[cfg_attr(docsrs, doc(cfg(feature = "unchecked_instr_fields")))]
impl InstrField<'_> {
    /// Returns the [`Gpr`] register embedded on the `rs` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use the [`rs`] function
    /// instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_rs`] operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`rs`]: InstrField::rs
    #[must_use]
    pub unsafe fn rs_unchecked(&self) -> Gpr {
        self.rs_impl()
    }

    /// Returns the [`Gpr`] register embedded on the `rt` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use the [`rt`] function
    /// instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_rt`] operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`rt`]: InstrField::rt
    #[must_use]
    pub unsafe fn rt_unchecked(&self) -> Gpr {
        self.rt_impl()
    }

    /// Returns the [`Gpr`] register embedded on the `rd` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use the [`rd`] function
    /// instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_rd`] operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`rd`]: InstrField::rd
    #[must_use]
    pub unsafe fn rd_unchecked(&self) -> Gpr {
        self.rd_impl()
    }

    /// Returns the `sa` value embedded on the `sa` field of the word of this
    /// instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`sa`] function
    /// instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_sa`] operand.
    ///
    /// [`sa`]: InstrField::sa
    #[must_use]
    pub unsafe fn sa_unchecked(&self) -> u8 {
        self.sa_impl()
    }

    /// Returns the [`Cop0`] register embedded on the `cop0d` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop0`] register. It is recommended to use the [`cop0d`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_cop0d`] operand.
    ///
    /// [`Cop0`]: crate::registers::Cop0
    /// [`cop0d`]: InstrField::cop0d
    #[must_use]
    pub unsafe fn cop0d_unchecked(&self) -> Cop0 {
        self.cop0d_impl()
    }

    /// Returns the [`Cop0Control`] register embedded on the `cop0cd` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop0Control`] register. It is recommended to use the
    /// [`cop0cd`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_cop0cd`] operand.
    ///
    /// [`Cop0Control`]: crate::registers::Cop0Control
    /// [`cop0cd`]: InstrField::cop0cd
    #[must_use]
    pub unsafe fn cop0cd_unchecked(&self) -> Cop0Control {
        self.cop0cd_impl()
    }

    /// Returns the [`Cop1`] register embedded on the `fs` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop1`] register. It is recommended to use the [`fs`] function
    /// instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_fs`] operand.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`fs`]: InstrField::fs
    #[must_use]
    pub unsafe fn fs_unchecked(&self) -> Cop1 {
        self.fs_impl()
    }

    /// Returns the [`Cop1`] register embedded on the `ft` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop1`] register. It is recommended to use the [`ft`] function
    /// instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_ft`] operand.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`ft`]: InstrField::ft
    #[must_use]
    pub unsafe fn ft_unchecked(&self) -> Cop1 {
        self.ft_impl()
    }

    /// Returns the [`Cop1`] register embedded on the `fd` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop1`] register. It is recommended to use the [`fd`] function
    /// instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_fd`] operand.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`fd`]: InstrField::fd
    #[must_use]
    pub unsafe fn fd_unchecked(&self) -> Cop1 {
        self.fd_impl()
    }

    /// Returns the [`Cop1Control`] register embedded on the `cop1cs` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop1Control`] register. It is recommended to use the
    /// [`cop1cs`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_cop1cs`] operand.
    ///
    /// [`Cop1Control`]: crate::registers::Cop1Control
    /// [`cop1cs`]: InstrField::cop1cs
    #[must_use]
    pub unsafe fn cop1cs_unchecked(&self) -> Cop1Control {
        self.cop1cs_impl()
    }

    /// Returns the [`Cop2`] register embedded on the `cop2t` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop2`] register. It is recommended to use the [`cop2t`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_cop2t`] operand.
    ///
    /// [`Cop2`]: crate::registers::Cop2
    /// [`cop2t`]: InstrField::cop2t
    #[must_use]
    pub unsafe fn cop2t_unchecked(&self) -> Cop2 {
        self.cop2t_impl()
    }

    /// Returns the [`Cop2`] register embedded on the `cop2d` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop2`] register. It is recommended to use the [`cop2d`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_cop2d`] operand.
    ///
    /// [`Cop2`]: crate::registers::Cop2
    /// [`cop2d`]: InstrField::cop2d
    #[must_use]
    pub unsafe fn cop2d_unchecked(&self) -> Cop2 {
        self.cop2d_impl()
    }

    /// Returns the [`Cop2Control`] register embedded on the `cop2cd` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop2Control`] register. It is recommended to use the
    /// [`cop2cd`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_cop2cd`] operand.
    ///
    /// [`Cop2Control`]: crate::registers::Cop2Control
    /// [`cop2cd`]: InstrField::cop2cd
    #[must_use]
    pub unsafe fn cop2cd_unchecked(&self) -> Cop2Control {
        self.cop2cd_impl()
    }

    /// Returns the `instr_index` value embedded on the `instr_index` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the
    /// [`instr_index`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_label`] operand.
    ///
    /// [`instr_index`]: InstrField::instr_index
    #[must_use]
    pub const unsafe fn instr_index_unchecked(&self) -> u32 {
        self.instr_index_impl()
    }

    /// Returns the `imm_i16` value embedded on the `immediate` field of the
    /// word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `imm_i16` value. It is recommended to use the
    /// [`imm_i16`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_imm_i16`] operand.
    ///
    /// [`imm_i16`]: InstrField::imm_i16
    #[must_use]
    pub unsafe fn imm_i16_unchecked(&self) -> i16 {
        self.imm_i16_impl()
    }

    /// Returns the `imm_u16` value embedded on the `immediate` field of the
    /// word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `imm_u16` value. It is recommended to use the
    /// [`imm_u16`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_imm_u16`] operand.
    ///
    /// [`imm_u16`]: InstrField::imm_u16
    #[must_use]
    pub unsafe fn imm_u16_unchecked(&self) -> u16 {
        self.imm_u16_impl()
    }

    /// Returns the `op` value embedded on the `op` field of the word of this
    /// instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `op` value. It is recommended to use the
    /// [`op`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_op`] operand.
    ///
    /// [`op`]: InstrField::op
    #[must_use]
    pub unsafe fn op_unchecked(&self) -> u8 {
        self.op_impl()
    }

    /// Returns the `hint` value embedded on the `hint` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `hint` value. It is recommended to use the
    /// [`hint`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_hint`] operand.
    ///
    /// [`hint`]: InstrField::hint
    #[must_use]
    pub unsafe fn hint_unchecked(&self) -> u8 {
        self.hint_impl()
    }

    /// Returns the `code_upper` value embedded on the `code_upper` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `code_upper` value. It is recommended to use the
    /// [`code_upper`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_code`] operand.
    ///
    /// [`code_upper`]: InstrField::code_upper
    #[must_use]
    pub unsafe fn code_upper_unchecked(&self) -> u16 {
        self.code_upper_impl()
    }

    /// Returns the `code_lower` value embedded on the `code_lower` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `code_lower` value. It is recommended to use the
    /// [`code_lower`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::core_code_lower`] operand.
    ///
    /// [`code_lower`]: InstrField::code_lower
    #[must_use]
    pub unsafe fn code_lower_unchecked(&self) -> u16 {
        self.code_lower_impl()
    }
}

/// Unchecked RSP opcode fields
#[cfg(all(feature = "RSP", feature = "unchecked_instr_fields"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "RSP", feature = "unchecked_instr_fields"))))]
impl InstrField<'_> {
    /// Returns the [`RspCop0`] register embedded on the `rsp_cop0d` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`rsp_cop0d`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_cop0d`] operand.
    ///
    /// [`RspCop0`]: crate::registers::RspCop0
    /// [`rsp_cop0d`]: InstrField::rsp_cop0d
    #[must_use]
    pub unsafe fn rsp_cop0d_unchecked(&self) -> RspCop0 {
        self.rsp_cop0d_impl()
    }

    /// Returns the [`RspCop2`] register embedded on the `rsp_cop2cd` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`rsp_cop2cd`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_cop2cd`] operand.
    ///
    /// [`RspCop2`]: crate::registers::RspCop2
    /// [`rsp_cop2cd`]: InstrField::rsp_cop2cd
    #[must_use]
    pub unsafe fn rsp_cop2cd_unchecked(&self) -> RspCop2 {
        self.rsp_cop2cd_impl()
    }

    /// Returns the [`RspVector`] register embedded on the `rsp_vs` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`rsp_vs`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_cop2cd`] operand.
    ///
    /// [`RspVector`]: crate::registers::RspVector
    /// [`rsp_vs`]: InstrField::rsp_vs
    #[must_use]
    pub unsafe fn rsp_vs_unchecked(&self) -> RspVector {
        self.rsp_vs_impl()
    }

    /// Returns the [`RspVector`] register embedded on the `rsp_vt` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`rsp_vt`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_vt_elementhigh`]
    ///   operand or the [`Operand::rsp_vt_elementlow`] operand.
    ///
    /// [`RspVector`]: crate::registers::RspVector
    /// [`rsp_vt`]: InstrField::rsp_vt
    #[must_use]
    pub unsafe fn rsp_vt_unchecked(&self) -> RspVector {
        self.rsp_vt_impl()
    }

    /// Returns the [`RspVector`] register embedded on the `rsp_vd` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`rsp_vd`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_vd`] operand.
    ///
    /// [`RspVector`]: crate::registers::RspVector
    /// [`rsp_vd`]: InstrField::rsp_vd
    #[must_use]
    pub unsafe fn rsp_vd_unchecked(&self) -> RspVector {
        self.rsp_vd_impl()
    }

    /// Returns the `rsp_elementhigh` value embedded on the `rsp_elementhigh`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_elementhigh`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_vt_elementhigh`] operand.
    ///
    /// [`rsp_elementhigh`]: InstrField::rsp_elementhigh
    #[must_use]
    pub unsafe fn rsp_elementhigh_unchecked(&self) -> u8 {
        self.rsp_elementhigh_impl()
    }

    /// Returns the `rsp_elementlow` value embedded on the `rsp_elementlow` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_elementlow`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_vt_elementlow`] operand.
    ///
    /// [`rsp_elementlow`]: InstrField::rsp_elementlow
    #[must_use]
    pub unsafe fn rsp_elementlow_unchecked(&self) -> u8 {
        self.rsp_elementlow_impl()
    }

    /// Returns the `rsp_index` value embedded on the `rsp_index` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_index`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_vs_index`] operand.
    ///
    /// [`rsp_index`]: InstrField::rsp_index
    #[must_use]
    pub unsafe fn rsp_index_unchecked(&self) -> u8 {
        self.rsp_index_impl()
    }

    /// Returns the `rsp_offset7` value embedded on the `rsp_offset7` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_offset7`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_vs_index`] operand.
    ///
    /// [`rsp_offset7`]: InstrField::rsp_offset7
    #[must_use]
    pub unsafe fn rsp_offset7_unchecked(&self) -> u8 {
        self.rsp_offset7_impl()
    }

    /// Returns the `rsp_offset8` value embedded on the `rsp_offset8` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_offset8`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_vs_index`] operand.
    ///
    /// [`rsp_offset8`]: InstrField::rsp_offset8
    #[must_use]
    pub unsafe fn rsp_offset8_unchecked(&self) -> u8 {
        self.rsp_offset8_impl()
    }

    /// Returns the `rsp_offset9` value embedded on the `rsp_offset9` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_offset9`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_vs_index`] operand.
    ///
    /// [`rsp_offset9`]: InstrField::rsp_offset9
    #[must_use]
    pub unsafe fn rsp_offset9_unchecked(&self) -> u16 {
        self.rsp_offset9_impl()
    }

    /// Returns the `rsp_offset10` value embedded on the `rsp_offset10` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_offset10`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_vs_index`] operand.
    ///
    /// [`rsp_offset10`]: InstrField::rsp_offset10
    #[must_use]
    pub unsafe fn rsp_offset10_unchecked(&self) -> u16 {
        self.rsp_offset10_impl()
    }

    /// Returns the `rsp_offset11` value embedded on the `rsp_offset11` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_offset11`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_vs_index`] operand.
    ///
    /// [`rsp_offset11`]: InstrField::rsp_offset11
    #[must_use]
    pub unsafe fn rsp_offset11_unchecked(&self) -> u16 {
        self.rsp_offset11_impl()
    }

    /// Returns the `rsp_de` value embedded on the `rsp_de` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_de`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::rsp_vd_de`] operand.
    ///
    /// [`rsp_de`]: InstrField::rsp_de
    #[must_use]
    pub unsafe fn rsp_de_unchecked(&self) -> u8 {
        self.rsp_de_impl()
    }
}

/// Unchecked R3000GTE opcode fields
#[cfg(all(feature = "R3000GTE", feature = "unchecked_instr_fields"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "R3000GTE", feature = "unchecked_instr_fields"))))]
impl InstrField<'_> {
    /// Returns the `r3000gte_sf` value embedded on the `r3000gte_sf` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r3000gte_sf`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r3000gte_sf`] operand.
    ///
    /// [`r3000gte_sf`]: InstrField::r3000gte_sf
    #[must_use]
    pub unsafe fn r3000gte_sf_unchecked(&self) -> u8 {
        self.r3000gte_sf_impl()
    }

    /// Returns the `r3000gte_mx` value embedded on the `r3000gte_mx` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r3000gte_mx`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r3000gte_mx`] operand.
    ///
    /// [`r3000gte_mx`]: InstrField::r3000gte_mx
    #[must_use]
    pub unsafe fn r3000gte_mx_unchecked(&self) -> u8 {
        self.r3000gte_mx_impl()
    }

    /// Returns the `r3000gte_v` value embedded on the `r3000gte_v` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r3000gte_v`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r3000gte_v`] operand.
    ///
    /// [`r3000gte_v`]: InstrField::r3000gte_v
    #[must_use]
    pub unsafe fn r3000gte_v_unchecked(&self) -> u8 {
        self.r3000gte_v_impl()
    }

    /// Returns the `r3000gte_cv` value embedded on the `r3000gte_cv` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r3000gte_cv`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r3000gte_cv`] operand.
    ///
    /// [`r3000gte_cv`]: InstrField::r3000gte_cv
    #[must_use]
    pub unsafe fn r3000gte_cv_unchecked(&self) -> u8 {
        self.r3000gte_cv_impl()
    }

    /// Returns the `r3000gte_lm` value embedded on the `r3000gte_lm` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r3000gte_lm`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r3000gte_lm`] operand.
    ///
    /// [`r3000gte_lm`]: InstrField::r3000gte_lm
    #[must_use]
    pub unsafe fn r3000gte_lm_unchecked(&self) -> u8 {
        self.r3000gte_lm_impl()
    }
}

/// Unchecked R4000ALLEGREX opcode fields
#[cfg(all(feature = "R4000ALLEGREX", feature = "unchecked_instr_fields"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "R4000ALLEGREX", feature = "unchecked_instr_fields"))))]
impl InstrField<'_> {
    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_s_vs`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_s_vs`] operand.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`r4000allegrex_s_vs`]: InstrField::r4000allegrex_s_vs
    #[must_use]
    pub unsafe fn r4000allegrex_s_vs_unchecked(&self) -> R4000AllegrexS {
        self.r4000allegrex_s_vs_impl()
    }

    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_s_vt`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_s_vt`] operand.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`r4000allegrex_s_vt`]: InstrField::r4000allegrex_s_vt
    #[must_use]
    pub unsafe fn r4000allegrex_s_vt_unchecked(&self) -> R4000AllegrexS {
        self.r4000allegrex_s_vt_impl()
    }

    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_s_vd`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_s_vd`] operand.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`r4000allegrex_s_vd`]: InstrField::r4000allegrex_s_vd
    #[must_use]
    pub unsafe fn r4000allegrex_s_vd_unchecked(&self) -> R4000AllegrexS {
        self.r4000allegrex_s_vd_impl()
    }

    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vt_imm`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_s_vt_imm`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_s_vt_imm`] operand.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`r4000allegrex_s_vt_imm`]: InstrField::r4000allegrex_s_vt_imm
    #[must_use]
    pub unsafe fn r4000allegrex_s_vt_imm_unchecked(&self) -> R4000AllegrexS {
        self.r4000allegrex_s_vt_imm_impl()
    }

    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vd_imm`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_s_vd_imm`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_s_vd_imm`] operand.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`r4000allegrex_s_vd_imm`]: InstrField::r4000allegrex_s_vd_imm
    #[must_use]
    pub unsafe fn r4000allegrex_s_vd_imm_unchecked(&self) -> R4000AllegrexS {
        self.r4000allegrex_s_vd_imm_impl()
    }

    /// Returns the [`R4000AllegrexV2D`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_p_vs`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_p_vs`] operand.
    ///
    /// [`R4000AllegrexV2D`]: crate::registers::R4000AllegrexV2D
    /// [`r4000allegrex_p_vs`]: InstrField::r4000allegrex_p_vs
    #[must_use]
    pub unsafe fn r4000allegrex_p_vs_unchecked(&self) -> R4000AllegrexV2D {
        self.r4000allegrex_p_vs_impl()
    }

    /// Returns the [`R4000AllegrexV2D`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_p_vt`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_p_vt`] operand.
    ///
    /// [`R4000AllegrexV2D`]: crate::registers::R4000AllegrexV2D
    /// [`r4000allegrex_p_vt`]: InstrField::r4000allegrex_p_vt
    #[must_use]
    pub unsafe fn r4000allegrex_p_vt_unchecked(&self) -> R4000AllegrexV2D {
        self.r4000allegrex_p_vt_impl()
    }

    /// Returns the [`R4000AllegrexV2D`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_p_vd`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_p_vd`] operand.
    ///
    /// [`R4000AllegrexV2D`]: crate::registers::R4000AllegrexV2D
    /// [`r4000allegrex_p_vd`]: InstrField::r4000allegrex_p_vd
    #[must_use]
    pub unsafe fn r4000allegrex_p_vd_unchecked(&self) -> R4000AllegrexV2D {
        self.r4000allegrex_p_vd_impl()
    }

    /// Returns the [`R4000AllegrexV3D`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_t_vs`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_t_vs`] operand.
    ///
    /// [`R4000AllegrexV3D`]: crate::registers::R4000AllegrexV3D
    /// [`r4000allegrex_t_vs`]: InstrField::r4000allegrex_t_vs
    #[must_use]
    pub unsafe fn r4000allegrex_t_vs_unchecked(&self) -> R4000AllegrexV3D {
        self.r4000allegrex_t_vs_impl()
    }

    /// Returns the [`R4000AllegrexV3D`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_t_vt`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_t_vt`] operand.
    ///
    /// [`R4000AllegrexV3D`]: crate::registers::R4000AllegrexV3D
    /// [`r4000allegrex_t_vt`]: InstrField::r4000allegrex_t_vt
    #[must_use]
    pub unsafe fn r4000allegrex_t_vt_unchecked(&self) -> R4000AllegrexV3D {
        self.r4000allegrex_t_vt_impl()
    }

    /// Returns the [`R4000AllegrexV3D`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_t_vd`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_t_vd`] operand.
    ///
    /// [`R4000AllegrexV3D`]: crate::registers::R4000AllegrexV3D
    /// [`r4000allegrex_t_vd`]: InstrField::r4000allegrex_t_vd
    #[must_use]
    pub unsafe fn r4000allegrex_t_vd_unchecked(&self) -> R4000AllegrexV3D {
        self.r4000allegrex_t_vd_impl()
    }

    /// Returns the [`R4000AllegrexV4D`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_q_vs`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_q_vs`] operand.
    ///
    /// [`R4000AllegrexV4D`]: crate::registers::R4000AllegrexV4D
    /// [`r4000allegrex_q_vs`]: InstrField::r4000allegrex_q_vs
    #[must_use]
    pub unsafe fn r4000allegrex_q_vs_unchecked(&self) -> R4000AllegrexV4D {
        self.r4000allegrex_q_vs_impl()
    }

    /// Returns the [`R4000AllegrexV4D`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_q_vt`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_q_vt`] operand.
    ///
    /// [`R4000AllegrexV4D`]: crate::registers::R4000AllegrexV4D
    /// [`r4000allegrex_q_vt`]: InstrField::r4000allegrex_q_vt
    #[must_use]
    pub unsafe fn r4000allegrex_q_vt_unchecked(&self) -> R4000AllegrexV4D {
        self.r4000allegrex_q_vt_impl()
    }

    /// Returns the [`R4000AllegrexV4D`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_q_vd`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_q_vd`] operand.
    ///
    /// [`R4000AllegrexV4D`]: crate::registers::R4000AllegrexV4D
    /// [`r4000allegrex_q_vd`]: InstrField::r4000allegrex_q_vd
    #[must_use]
    pub unsafe fn r4000allegrex_q_vd_unchecked(&self) -> R4000AllegrexV4D {
        self.r4000allegrex_q_vd_impl()
    }

    /// Returns the [`R4000AllegrexV4D`] register embedded on the `r4000allegrex_vt_imm`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_q_vt_imm`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_q_vt_imm`] operand.
    ///
    /// [`R4000AllegrexV4D`]: crate::registers::R4000AllegrexV4D
    /// [`r4000allegrex_q_vt_imm`]: InstrField::r4000allegrex_q_vt_imm
    #[must_use]
    pub unsafe fn r4000allegrex_q_vt_imm_unchecked(&self) -> R4000AllegrexV4D {
        self.r4000allegrex_q_vt_imm_impl()
    }

    /// Returns the [`R4000AllegrexM2x2`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mp_vs`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_mp_vs`] operand.
    ///
    /// [`R4000AllegrexM2x2`]: crate::registers::R4000AllegrexM2x2
    /// [`r4000allegrex_mp_vs`]: InstrField::r4000allegrex_mp_vs
    #[must_use]
    pub unsafe fn r4000allegrex_mp_vs_unchecked(&self) -> R4000AllegrexM2x2 {
        self.r4000allegrex_mp_vs_impl()
    }

    /// Returns the [`R4000AllegrexM2x2`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mp_vt`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_mp_vt`] operand.
    ///
    /// [`R4000AllegrexM2x2`]: crate::registers::R4000AllegrexM2x2
    /// [`r4000allegrex_mp_vt`]: InstrField::r4000allegrex_mp_vt
    #[must_use]
    pub unsafe fn r4000allegrex_mp_vt_unchecked(&self) -> R4000AllegrexM2x2 {
        self.r4000allegrex_mp_vt_impl()
    }

    /// Returns the [`R4000AllegrexM2x2`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mp_vd`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_mp_vd`] operand.
    ///
    /// [`R4000AllegrexM2x2`]: crate::registers::R4000AllegrexM2x2
    /// [`r4000allegrex_mp_vd`]: InstrField::r4000allegrex_mp_vd
    #[must_use]
    pub unsafe fn r4000allegrex_mp_vd_unchecked(&self) -> R4000AllegrexM2x2 {
        self.r4000allegrex_mp_vd_impl()
    }

    /// Returns the [`R4000AllegrexM2x2`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mp_vs_transpose`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_mp_vs_transpose`] operand.
    ///
    /// [`R4000AllegrexM2x2`]: crate::registers::R4000AllegrexM2x2
    /// [`r4000allegrex_mp_vs_transpose`]: InstrField::r4000allegrex_mp_vs_transpose
    #[must_use]
    pub unsafe fn r4000allegrex_mp_vs_transpose_unchecked(&self) -> R4000AllegrexM2x2 {
        self.r4000allegrex_mp_vs_transpose_impl()
    }

    /// Returns the [`R4000AllegrexM3x3`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mt_vs`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_mt_vs`] operand.
    ///
    /// [`R4000AllegrexM3x3`]: crate::registers::R4000AllegrexM3x3
    /// [`r4000allegrex_mt_vs`]: InstrField::r4000allegrex_mt_vs
    #[must_use]
    pub unsafe fn r4000allegrex_mt_vs_unchecked(&self) -> R4000AllegrexM3x3 {
        self.r4000allegrex_mt_vs_impl()
    }

    /// Returns the [`R4000AllegrexM3x3`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mt_vt`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_mt_vt`] operand.
    ///
    /// [`R4000AllegrexM3x3`]: crate::registers::R4000AllegrexM3x3
    /// [`r4000allegrex_mt_vt`]: InstrField::r4000allegrex_mt_vt
    #[must_use]
    pub unsafe fn r4000allegrex_mt_vt_unchecked(&self) -> R4000AllegrexM3x3 {
        self.r4000allegrex_mt_vt_impl()
    }

    /// Returns the [`R4000AllegrexM3x3`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mt_vd`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_mt_vd`] operand.
    ///
    /// [`R4000AllegrexM3x3`]: crate::registers::R4000AllegrexM3x3
    /// [`r4000allegrex_mt_vd`]: InstrField::r4000allegrex_mt_vd
    #[must_use]
    pub unsafe fn r4000allegrex_mt_vd_unchecked(&self) -> R4000AllegrexM3x3 {
        self.r4000allegrex_mt_vd_impl()
    }

    /// Returns the [`R4000AllegrexM3x3`] register embedded on the `r4000allegrex_vs_transpose`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mt_vs_transpose`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_mt_vs_transpose`] operand.
    ///
    /// [`R4000AllegrexM3x3`]: crate::registers::R4000AllegrexM3x3
    /// [`r4000allegrex_mt_vs_transpose`]: InstrField::r4000allegrex_mt_vs_transpose
    #[must_use]
    pub unsafe fn r4000allegrex_mt_vs_transpose_unchecked(&self) -> R4000AllegrexM3x3 {
        self.r4000allegrex_mt_vs_transpose_impl()
    }

    /// Returns the [`R4000AllegrexM4x4`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mq_vs`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_mq_vs`] operand.
    ///
    /// [`R4000AllegrexM4x4`]: crate::registers::R4000AllegrexM4x4
    /// [`r4000allegrex_mq_vs`]: InstrField::r4000allegrex_mq_vs
    #[must_use]
    pub unsafe fn r4000allegrex_mq_vs_unchecked(&self) -> R4000AllegrexM4x4 {
        self.r4000allegrex_mq_vs_impl()
    }

    /// Returns the [`R4000AllegrexM4x4`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mq_vt`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_mq_vt`] operand.
    ///
    /// [`R4000AllegrexM4x4`]: crate::registers::R4000AllegrexM4x4
    /// [`r4000allegrex_mq_vt`]: InstrField::r4000allegrex_mq_vt
    #[must_use]
    pub unsafe fn r4000allegrex_mq_vt_unchecked(&self) -> R4000AllegrexM4x4 {
        self.r4000allegrex_mq_vt_impl()
    }

    /// Returns the [`R4000AllegrexM4x4`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mq_vd`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_mq_vd`] operand.
    ///
    /// [`R4000AllegrexM4x4`]: crate::registers::R4000AllegrexM4x4
    /// [`r4000allegrex_mq_vd`]: InstrField::r4000allegrex_mq_vd
    #[must_use]
    pub unsafe fn r4000allegrex_mq_vd_unchecked(&self) -> R4000AllegrexM4x4 {
        self.r4000allegrex_mq_vd_impl()
    }

    /// Returns the [`R4000AllegrexM4x4`] register embedded on the `r4000allegrex_vs_transpose`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mq_vs_transpose`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_mq_vs_transpose`] operand.
    ///
    /// [`R4000AllegrexM4x4`]: crate::registers::R4000AllegrexM4x4
    /// [`r4000allegrex_mq_vs_transpose`]: InstrField::r4000allegrex_mq_vs_transpose
    #[must_use]
    pub unsafe fn r4000allegrex_mq_vs_transpose_unchecked(&self) -> R4000AllegrexM4x4 {
        self.r4000allegrex_mq_vs_transpose_impl()
    }

    /// Returns the [`R4000AllegrexVfpuControl`] register embedded on the `r4000allegrex_cop2cs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_cop2cs`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_cop2cs`] operand.
    ///
    /// [`R4000AllegrexVfpuControl`]: crate::registers::R4000AllegrexVfpuControl
    /// [`r4000allegrex_cop2cs`]: InstrField::r4000allegrex_cop2cs
    #[must_use]
    pub unsafe fn r4000allegrex_cop2cs_unchecked(&self) -> R4000AllegrexVfpuControl {
        self.r4000allegrex_cop2cs_impl()
    }

    /// Returns the [`R4000AllegrexVfpuControl`] register embedded on the `r4000allegrex_cop2cd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_cop2cd`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_cop2cd`] operand.
    ///
    /// [`R4000AllegrexVfpuControl`]: crate::registers::R4000AllegrexVfpuControl
    /// [`r4000allegrex_cop2cd`]: InstrField::r4000allegrex_cop2cd
    #[must_use]
    pub unsafe fn r4000allegrex_cop2cd_unchecked(&self) -> R4000AllegrexVfpuControl {
        self.r4000allegrex_cop2cd_impl()
    }

    /// Returns the `r4000allegrex_pos` value embedded on the `r4000allegrex_pos` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_pos`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_pos`] operand.
    ///
    /// [`r4000allegrex_pos`]: InstrField::r4000allegrex_pos
    #[must_use]
    pub unsafe fn r4000allegrex_pos_unchecked(&self) -> u8 {
        self.r4000allegrex_pos_impl()
    }

    /// Returns the `r4000allegrex_size` value embedded on the `r4000allegrex_size` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_size`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_size`] operand.
    ///
    /// [`r4000allegrex_size`]: InstrField::r4000allegrex_size
    #[must_use]
    pub unsafe fn r4000allegrex_size_unchecked(&self) -> u8 {
        self.r4000allegrex_size_impl()
    }

    /// Returns the `r4000allegrex_size_plus_pos` value embedded on the `r4000allegrex_size_plus_pos` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_size_plus_pos`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_size_plus_pos`] operand.
    ///
    /// [`r4000allegrex_size_plus_pos`]: InstrField::r4000allegrex_size_plus_pos
    #[must_use]
    pub unsafe fn r4000allegrex_size_plus_pos_unchecked(&self) -> i8 {
        self.r4000allegrex_size_plus_pos_impl()
    }

    /// Returns the `r4000allegrex_imm3` value embedded on the `r4000allegrex_imm3` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_imm3`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_imm3`] operand.
    ///
    /// [`r4000allegrex_imm3`]: InstrField::r4000allegrex_imm3
    #[must_use]
    pub unsafe fn r4000allegrex_imm3_unchecked(&self) -> u8 {
        self.r4000allegrex_imm3_impl()
    }

    /*
    /// Returns the `r4000allegrex_offset14` value embedded on the `r4000allegrex_offset14` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_offset14`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::`] operand.
    ///
    /// [`r4000allegrex_offset14`]: InstrField::r4000allegrex_offset14
    #[must_use]
    pub unsafe fn r4000allegrex_offset14_unchecked(&self) -> u16 {
        self.r4000allegrex_offset14_impl();

        value << 2
    }
    */

    /*
    /// Returns the `r4000allegrex_wb` value embedded on the `r4000allegrex_wb` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_wb`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::`] operand.
    ///
    /// [`r4000allegrex_wb`]: InstrField::r4000allegrex_wb
    #[must_use]
    pub const unsafe fn r4000allegrex_wb_unchecked(&self) -> bool {
        self.r4000allegrex_wb_impl()
    }
    */

    /*
    /// Returns the `r4000allegrex_vcmp_cond` value embedded on the `r4000allegrex_vcmp_cond` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_vcmp_cond`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::`] operand.
    ///
    /// [`r4000allegrex_vcmp_cond`]: InstrField::r4000allegrex_vcmp_cond
    #[must_use]
    pub unsafe fn r4000allegrex_vcmp_cond_unchecked(&self) -> R4000AllegrexVCond {
        self.r4000allegrex_vcmp_cond_impl()
    }
    */

    /// Returns the [`R4000AllegrexVConstant`] register embedded on the `r4000allegrex_vconstant`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_vconstant`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_vconstant`] operand.
    ///
    /// [`R4000AllegrexVConstant`]: crate::registers::R4000AllegrexVConstant
    /// [`r4000allegrex_vconstant`]: InstrField::r4000allegrex_vconstant
    #[must_use]
    pub unsafe fn r4000allegrex_vconstant_unchecked(&self) -> R4000AllegrexVConstant {
        self.r4000allegrex_vconstant_impl()
    }

    /// Returns the `r4000allegrex_power_of_two` value embedded on the `r4000allegrex_power_of_two` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_power_of_two`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_power_of_two`] operand.
    ///
    /// [`r4000allegrex_power_of_two`]: InstrField::r4000allegrex_power_of_two
    #[must_use]
    pub unsafe fn r4000allegrex_power_of_two_unchecked(&self) -> u8 {
        self.r4000allegrex_power_of_two_impl()
    }

    /// Returns the `r4000allegrex_vfpu_cc_bit` value embedded on the `r4000allegrex_vfpu_cc_bit` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_vfpu_cc_bit`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_vfpu_cc_bit`] operand.
    ///
    /// [`r4000allegrex_vfpu_cc_bit`]: InstrField::r4000allegrex_vfpu_cc_bit
    #[must_use]
    pub unsafe fn r4000allegrex_vfpu_cc_bit_unchecked(&self) -> u8 {
        self.r4000allegrex_vfpu_cc_bit_impl()
    }

    /// Returns the `r4000allegrex_bn` value embedded on the `r4000allegrex_bn` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_bn`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_bn`] operand.
    ///
    /// [`r4000allegrex_bn`]: InstrField::r4000allegrex_bn
    #[must_use]
    pub unsafe fn r4000allegrex_bn_unchecked(&self) -> u8 {
        self.r4000allegrex_bn_impl()
    }

    /*
    /// Returns the `r4000allegrex_intfloat16` value embedded on the `r4000allegrex_intfloat16` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_intfloat16`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::`] operand.
    ///
    /// [`r4000allegrex_intfloat16`]: InstrField::r4000allegrex_intfloat16
    #[must_use]
    pub unsafe fn r4000allegrex_intfloat16_unchecked(&self) -> u16 {
        self.r4000allegrex_intfloat16_impl()
    }
    */

    /// Returns the value embedded on the `r4000allegrex_intfloat16` field of
    /// the word of this instruction, but interpreted as a 16 bits signed value.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the
    /// [`r4000allegrex_int16`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_int16`] operand.
    ///
    /// [`r4000allegrex_int16`]: InstrField::r4000allegrex_int16
    #[must_use]
    pub unsafe fn r4000allegrex_int16_unchecked(&self) -> i16 {
        self.r4000allegrex_int16_impl()
    }

    /// Returns the value embedded on the `r4000allegrex_intfloat16` field of
    /// the word of this instruction, but interpreted as a 16 bits float.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the
    /// [`r4000allegrex_float16`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_float16`] operand.
    ///
    /// [`r4000allegrex_float16`]: InstrField::r4000allegrex_float16
    #[must_use]
    pub unsafe fn r4000allegrex_float16_unchecked(&self) -> f32 {
        self.r4000allegrex_float16_impl()
    }

    /// Returns the `r4000allegrex_vrot_code` value embedded on the `r4000allegrex_vrot_code` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_vrot_code`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain either the [`Operand::r4000allegrex_p_vrot_code`],
    ///   [`Operand::r4000allegrex_t_vrot_code`] or the [`Operand::r4000allegrex_q_vrot_code`]
    ///   operand.
    ///
    /// [`r4000allegrex_vrot_code`]: InstrField::r4000allegrex_vrot_code
    #[must_use]
    pub unsafe fn r4000allegrex_vrot_code_unchecked(&self) -> u8 {
        self.r4000allegrex_vrot_code_impl()
    }

    /// Returns the `r4000allegrex_wpx` value embedded on the `r4000allegrex_wpx` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_wpx`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_wpx`] operand.
    ///
    /// [`r4000allegrex_wpx`]: InstrField::r4000allegrex_wpx
    #[must_use]
    pub unsafe fn r4000allegrex_wpx_unchecked(&self) -> R4000AllegrexPrefixDst {
        self.r4000allegrex_wpx_impl()
    }

    /// Returns the `r4000allegrex_wpy` value embedded on the `r4000allegrex_wpy` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_wpy`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_wpy`] operand.
    ///
    /// [`r4000allegrex_wpy`]: InstrField::r4000allegrex_wpy
    #[must_use]
    pub unsafe fn r4000allegrex_wpy_unchecked(&self) -> R4000AllegrexPrefixDst {
        self.r4000allegrex_wpy_impl()
    }

    /// Returns the `r4000allegrex_wpz` value embedded on the `r4000allegrex_wpz` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_wpz`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_wpz`] operand.
    ///
    /// [`r4000allegrex_wpz`]: InstrField::r4000allegrex_wpz
    #[must_use]
    pub unsafe fn r4000allegrex_wpz_unchecked(&self) -> R4000AllegrexPrefixDst {
        self.r4000allegrex_wpz_impl()
    }

    /// Returns the `r4000allegrex_wpw` value embedded on the `r4000allegrex_wpw` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_wpw`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_wpw`] operand.
    ///
    /// [`r4000allegrex_wpw`]: InstrField::r4000allegrex_wpw
    #[must_use]
    pub unsafe fn r4000allegrex_wpw_unchecked(&self) -> R4000AllegrexPrefixDst {
        self.r4000allegrex_wpw_impl()
    }

    /// Returns the `r4000allegrex_rpx` value embedded on the `r4000allegrex_rpx` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_rpx`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_rpx`] operand.
    ///
    /// [`r4000allegrex_rpx`]: InstrField::r4000allegrex_rpx
    #[must_use]
    pub unsafe fn r4000allegrex_rpx_unchecked(&self) -> R4000AllegrexPrefixSrc {
        self.r4000allegrex_rpx_impl()
    }

    /// Returns the `r4000allegrex_rpy` value embedded on the `r4000allegrex_rpy` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_rpy`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_rpy`] operand.
    ///
    /// [`r4000allegrex_rpy`]: InstrField::r4000allegrex_rpy
    #[must_use]
    pub unsafe fn r4000allegrex_rpy_unchecked(&self) -> R4000AllegrexPrefixSrc {
        self.r4000allegrex_rpy_impl()
    }

    /// Returns the `r4000allegrex_rpz` value embedded on the `r4000allegrex_rpz` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_rpz`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_rpz`] operand.
    ///
    /// [`r4000allegrex_rpz`]: InstrField::r4000allegrex_rpz
    #[must_use]
    pub unsafe fn r4000allegrex_rpz_unchecked(&self) -> R4000AllegrexPrefixSrc {
        self.r4000allegrex_rpz_impl()
    }

    /// Returns the `r4000allegrex_rpw` value embedded on the `r4000allegrex_rpw` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_rpw`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r4000allegrex_rpw`] operand.
    ///
    /// [`r4000allegrex_rpw`]: InstrField::r4000allegrex_rpw
    #[must_use]
    pub unsafe fn r4000allegrex_rpw_unchecked(&self) -> R4000AllegrexPrefixSrc {
        self.r4000allegrex_rpw_impl()
    }
}

/// Unchecked R5900EE opcode fields
#[cfg(all(feature = "R5900EE", feature = "unchecked_instr_fields"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "R5900EE", feature = "unchecked_instr_fields"))))]
impl InstrField<'_> {
    /// Returns the `r5900ee_imm5` value embedded on the `r5900ee_imm5` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_imm5` value. It is recommended to use the
    /// [`r5900ee_imm5`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r5900ee_imm5`] operand.
    ///
    /// [`r5900ee_imm5`]: InstrField::r5900ee_imm5
    #[must_use]
    pub unsafe fn r5900ee_imm5_unchecked(&self) -> i8 {
        self.r5900ee_imm5_impl()
    }

    /// Returns the `r5900ee_imm15` value embedded on the `r5900ee_imm15` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_imm15` value. It is recommended to use the
    /// [`r5900ee_imm15`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r5900ee_imm15`] operand.
    ///
    /// [`r5900ee_imm15`]: InstrField::r5900ee_imm15
    #[must_use]
    pub unsafe fn r5900ee_imm15_unchecked(&self) -> u16 {
        self.r5900ee_imm15_impl()
    }

    /// Returns the [`R5900EEVF`] register embedded on the `r5900ee_vfs` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r5900ee_vfs`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r5900ee_vfs`] operand.
    ///
    /// [`R5900EEVF`]: crate::registers::R5900EEVF
    /// [`r5900ee_vfs`]: InstrField::r5900ee_vfs
    #[must_use]
    pub unsafe fn r5900ee_vfs_unchecked(&self) -> R5900EEVF {
        self.r5900ee_vfs_impl()
    }

    /// Returns the [`R5900EEVF`] register embedded on the `r5900ee_vft` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r5900ee_vft`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r5900ee_vft`] operand.
    ///
    /// [`R5900EEVF`]: crate::registers::R5900EEVF
    /// [`r5900ee_vft`]: InstrField::r5900ee_vft
    #[must_use]
    pub unsafe fn r5900ee_vft_unchecked(&self) -> R5900EEVF {
        self.r5900ee_vft_impl()
    }

    /// Returns the [`R5900EEVF`] register embedded on the `r5900ee_vfd` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r5900ee_vfd`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r5900ee_vfd`] operand.
    ///
    /// [`R5900EEVF`]: crate::registers::R5900EEVF
    /// [`r5900ee_vfd`]: InstrField::r5900ee_vfd
    #[must_use]
    pub unsafe fn r5900ee_vfd_unchecked(&self) -> R5900EEVF {
        self.r5900ee_vfd_impl()
    }

    /// Returns the [`R5900EEVI`] register embedded on the `r5900ee_vis` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r5900ee_vis`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r5900ee_vis`] operand.
    ///
    /// [`R5900EEVI`]: crate::registers::R5900EEVI
    /// [`r5900ee_vis`]: InstrField::r5900ee_vis
    #[must_use]
    pub unsafe fn r5900ee_vis_unchecked(&self) -> R5900EEVI {
        self.r5900ee_vis_impl()
    }

    /// Returns the [`R5900EEVI`] register embedded on the `r5900ee_vit` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r5900ee_vit`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r5900ee_vit`] operand.
    ///
    /// [`R5900EEVI`]: crate::registers::R5900EEVI
    /// [`r5900ee_vit`]: InstrField::r5900ee_vit
    #[must_use]
    pub unsafe fn r5900ee_vit_unchecked(&self) -> R5900EEVI {
        self.r5900ee_vit_impl()
    }

    /// Returns the [`R5900EEVI`] register embedded on the `r5900ee_vid` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r5900ee_vid`]
    /// function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r5900ee_vid`] operand.
    ///
    /// [`R5900EEVI`]: crate::registers::R5900EEVI
    /// [`r5900ee_vid`]: InstrField::r5900ee_vid
    #[must_use]
    pub unsafe fn r5900ee_vid_unchecked(&self) -> R5900EEVI {
        self.r5900ee_vid_impl()
    }

    /// Returns the `r5900ee_xyzw_x` value embedded on the `r5900ee_xyzw_x` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_xyzw_x` value. It is recommended to use the
    /// [`r5900ee_xyzw_x`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain either the [`Operand::r5900ee_ACCxyzw`],
    ///   [`Operand::r5900ee_vfsxyzw`], [`Operand::r5900ee_vftxyzw`] or the
    ///   [`Operand::r5900ee_vfdxyzw`] operand.
    ///
    /// [`r5900ee_xyzw_x`]: InstrField::r5900ee_xyzw_x
    #[must_use]
    pub const unsafe fn r5900ee_xyzw_x_unchecked(&self) -> bool {
        self.r5900ee_xyzw_x_impl()
    }

    /// Returns the `r5900ee_xyzw_y` value embedded on the `r5900ee_xyzw_y` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_xyzw_y` value. It is recommended to use the
    /// [`r5900ee_xyzw_y`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain either the [`Operand::r5900ee_ACCxyzw`],
    ///   [`Operand::r5900ee_vfsxyzw`], [`Operand::r5900ee_vftxyzw`] or the
    ///   [`Operand::r5900ee_vfdxyzw`] operand.
    ///
    /// [`r5900ee_xyzw_y`]: InstrField::r5900ee_xyzw_y
    #[must_use]
    pub const unsafe fn r5900ee_xyzw_y_unchecked(&self) -> bool {
        self.r5900ee_xyzw_y_impl()
    }

    /// Returns the `r5900ee_xyzw_z` value embedded on the `r5900ee_xyzw_z` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_xyzw_z` value. It is recommended to use the
    /// [`r5900ee_xyzw_z`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain either the [`Operand::r5900ee_ACCxyzw`],
    ///   [`Operand::r5900ee_vfsxyzw`], [`Operand::r5900ee_vftxyzw`] or the
    ///   [`Operand::r5900ee_vfdxyzw`] operand.
    ///
    /// [`r5900ee_xyzw_z`]: InstrField::r5900ee_xyzw_z
    #[must_use]
    pub const unsafe fn r5900ee_xyzw_z_unchecked(&self) -> bool {
        self.r5900ee_xyzw_z_impl()
    }

    /// Returns the `r5900ee_xyzw_w` value embedded on the `r5900ee_xyzw_w` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_xyzw_w` value. It is recommended to use the
    /// [`r5900ee_xyzw_w`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain either the [`Operand::r5900ee_ACCxyzw`],
    ///   [`Operand::r5900ee_vfsxyzw`], [`Operand::r5900ee_vftxyzw`] or the
    ///   [`Operand::r5900ee_vfdxyzw`] operand.
    ///
    /// [`r5900ee_xyzw_w`]: InstrField::r5900ee_xyzw_w
    #[must_use]
    pub const unsafe fn r5900ee_xyzw_w_unchecked(&self) -> bool {
        self.r5900ee_xyzw_w_impl()
    }

    /// Returns the `r5900ee_n` value embedded on the `r5900ee_n` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_n` value. It is recommended to use the
    /// [`r5900ee_n`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r5900ee_vftn`] operand.
    ///
    /// [`r5900ee_n`]: InstrField::r5900ee_n
    #[must_use]
    pub unsafe fn r5900ee_n_unchecked(&self) -> u8 {
        self.r5900ee_n_impl()
    }

    /// Returns the `r5900ee_l` value embedded on the `r5900ee_l` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_l` value. It is recommended to use the
    /// [`r5900ee_l`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r5900ee_vfsl`] operand.
    ///
    /// [`r5900ee_l`]: InstrField::r5900ee_l
    #[must_use]
    pub unsafe fn r5900ee_l_unchecked(&self) -> u8 {
        self.r5900ee_l_impl()
    }

    /// Returns the `r5900ee_m` value embedded on the `r5900ee_m` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_m` value. It is recommended to use the
    /// [`r5900ee_m`] function instead.
    ///
    /// # Safety
    ///
    /// - The instruction must contain the [`Operand::r5900ee_vftm`] operand.
    ///
    /// [`r5900ee_m`]: InstrField::r5900ee_m
    #[must_use]
    pub unsafe fn r5900ee_m_unchecked(&self) -> u8 {
        self.r5900ee_m_impl()
    }
}

/// Impls opcode fields
impl InstrField<'_> {
    // Should be safe to use `.unwrap()` on all the `_impl` functions that
    // return an enum because using the correct `EncodedFieldMask` should yield
    // a bit of range that should not escape the range of the given enum.
    // If the unwrap ends up panicking then we have an internal library error
    // somewhere.

    /// Returns the [`Gpr`] register embedded on the `rs` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use the [`rs`] function
    /// instead.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`rs`]: InstrField::rs
    #[must_use]
    pub(crate) fn rs_impl(&self) -> Gpr {
        Gpr::try_from(EncodedFieldMask::rs.get_shifted(self.instr.word())).unwrap()
    }

    /// Returns the [`Gpr`] register embedded on the `rt` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use the [`rt`] function
    /// instead.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`rt`]: InstrField::rt
    #[must_use]
    pub(crate) fn rt_impl(&self) -> Gpr {
        Gpr::try_from(EncodedFieldMask::rt.get_shifted(self.instr.word())).unwrap()
    }

    /// Returns the [`Gpr`] register embedded on the `rd` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use the [`rd`] function
    /// instead.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`rd`]: InstrField::rd
    #[must_use]
    pub(crate) fn rd_impl(&self) -> Gpr {
        Gpr::try_from(EncodedFieldMask::rd.get_shifted(self.instr.word())).unwrap()
    }

    /// Returns the `sa` value embedded on the `sa` field of the word of this
    /// instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`sa`] function
    /// instead.
    ///
    /// [`sa`]: InstrField::sa
    #[must_use]
    pub(crate) fn sa_impl(&self) -> u8 {
        EncodedFieldMask::sa
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`Cop0`] register embedded on the `cop0d` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop0`] register. It is recommended to use the [`cop0d`]
    /// function instead.
    ///
    /// [`Cop0`]: crate::registers::Cop0
    /// [`cop0d`]: InstrField::cop0d
    #[must_use]
    pub(crate) fn cop0d_impl(&self) -> Cop0 {
        Cop0::try_from(EncodedFieldMask::cop0d.get_shifted(self.instr.word())).unwrap()
    }

    /// Returns the [`Cop0Control`] register embedded on the `cop0cd` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop0Control`] register. It is recommended to use the
    /// [`cop0cd`] function instead.
    ///
    /// [`Cop0Control`]: crate::registers::Cop0Control
    /// [`cop0cd`]: InstrField::cop0cd
    #[must_use]
    pub(crate) fn cop0cd_impl(&self) -> Cop0Control {
        Cop0Control::try_from(EncodedFieldMask::cop0cd.get_shifted(self.instr.word())).unwrap()
    }

    /// Returns the [`Cop1`] register embedded on the `fs` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop1`] register. It is recommended to use the [`fs`] function
    /// instead.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`fs`]: InstrField::fs
    #[must_use]
    pub(crate) fn fs_impl(&self) -> Cop1 {
        Cop1::try_from(EncodedFieldMask::fs.get_shifted(self.instr.word())).unwrap()
    }

    /// Returns the [`Cop1`] register embedded on the `ft` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop1`] register. It is recommended to use the [`ft`] function
    /// instead.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`ft`]: InstrField::ft
    #[must_use]
    pub(crate) fn ft_impl(&self) -> Cop1 {
        Cop1::try_from(EncodedFieldMask::ft.get_shifted(self.instr.word())).unwrap()
    }

    /// Returns the [`Cop1`] register embedded on the `fd` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop1`] register. It is recommended to use the [`fd`] function
    /// instead.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`fd`]: InstrField::fd
    #[must_use]
    pub(crate) fn fd_impl(&self) -> Cop1 {
        Cop1::try_from(EncodedFieldMask::fd.get_shifted(self.instr.word())).unwrap()
    }

    /// Returns the [`Cop1Control`] register embedded on the `cop1cs` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop1Control`] register. It is recommended to use the
    /// [`cop1cs`] function instead.
    ///
    /// [`Cop1Control`]: crate::registers::Cop1Control
    /// [`cop1cs`]: InstrField::cop1cs
    #[must_use]
    pub(crate) fn cop1cs_impl(&self) -> Cop1Control {
        Cop1Control::try_from(EncodedFieldMask::cop1cs.get_shifted(self.instr.word())).unwrap()
    }

    /// Returns the [`Cop2`] register embedded on the `cop2t` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop2`] register. It is recommended to use the [`cop2t`]
    /// function instead.
    ///
    /// [`Cop2`]: crate::registers::Cop2
    /// [`cop2t`]: InstrField::cop2t
    #[must_use]
    pub(crate) fn cop2t_impl(&self) -> Cop2 {
        Cop2::try_from(EncodedFieldMask::cop2t.get_shifted(self.instr.word())).unwrap()
    }

    /// Returns the [`Cop2`] register embedded on the `cop2d` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop2`] register. It is recommended to use the [`cop2d`]
    /// function instead.
    ///
    /// [`Cop2`]: crate::registers::Cop2
    /// [`cop2d`]: InstrField::cop2d
    #[must_use]
    pub(crate) fn cop2d_impl(&self) -> Cop2 {
        Cop2::try_from(EncodedFieldMask::cop2d.get_shifted(self.instr.word())).unwrap()
    }

    /// Returns the [`Cop2Control`] register embedded on the `cop2cd` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop2Control`] register. It is recommended to use the
    /// [`cop2cd`] function instead.
    ///
    /// [`Cop2Control`]: crate::registers::Cop2Control
    /// [`cop2cd`]: InstrField::cop2cd
    #[must_use]
    pub(crate) fn cop2cd_impl(&self) -> Cop2Control {
        Cop2Control::try_from(EncodedFieldMask::cop2cd.get_shifted(self.instr.word())).unwrap()
    }

    /// Returns the `instr_index` value embedded on the `instr_index` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the
    /// [`instr_index`] function instead.
    ///
    /// [`instr_index`]: InstrField::instr_index
    #[must_use]
    pub(crate) const fn instr_index_impl(&self) -> u32 {
        EncodedFieldMask::instr_index.get_shifted(self.instr.word())
    }

    /// Returns the `imm_i16` value embedded on the `immediate` field of the
    /// word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `immediate` value. It is recommended to use the
    /// [`imm_i16`] function instead.
    ///
    /// [`imm_i16`]: InstrField::imm_i16
    #[must_use]
    pub(crate) fn imm_i16_impl(&self) -> i16 {
        self.imm_u16_impl() as i16
    }

    /// Returns the `imm_u16` value embedded on the `immediate` field of the
    /// word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `immediate` value. It is recommended to use the
    /// [`imm_u16`] function instead.
    ///
    /// [`imm_u16`]: InstrField::imm_u16
    #[must_use]
    pub(crate) fn imm_u16_impl(&self) -> u16 {
        EncodedFieldMask::immediate
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `op` value embedded on the `op` field of the word of this
    /// instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `op` value. It is recommended to use the
    /// [`op`] function instead.
    ///
    /// [`op`]: InstrField::op
    #[must_use]
    pub(crate) fn op_impl(&self) -> u8 {
        EncodedFieldMask::op
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `hint` value embedded on the `hint` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `hint` value. It is recommended to use the
    /// [`hint`] function instead.
    ///
    /// [`hint`]: InstrField::hint
    #[must_use]
    pub(crate) fn hint_impl(&self) -> u8 {
        EncodedFieldMask::hint
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `code_upper` value embedded on the `code_upper` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `code_upper` value. It is recommended to use the
    /// [`code_upper`] function instead.
    ///
    /// [`code_upper`]: InstrField::code_upper
    #[must_use]
    pub(crate) fn code_upper_impl(&self) -> u16 {
        EncodedFieldMask::code_upper
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `code_lower` value embedded on the `code_lower` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `code_lower` value. It is recommended to use the
    /// [`code_lower`] function instead.
    ///
    /// [`code_lower`]: InstrField::code_lower
    #[must_use]
    pub(crate) fn code_lower_impl(&self) -> u16 {
        EncodedFieldMask::code_lower
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }
}

/// Impls RSP opcode fields
#[cfg(feature = "RSP")]
impl InstrField<'_> {
    /// Returns the [`RspCop0`] register embedded on the `rsp_cop0d` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`rsp_cop0d`]
    /// function instead.
    ///
    /// [`RspCop0`]: crate::registers::RspCop0
    /// [`rsp_cop0d`]: InstrField::rsp_cop0d
    #[must_use]
    pub(crate) fn rsp_cop0d_impl(&self) -> RspCop0 {
        EncodedFieldMask::cop0d
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`RspCop2`] register embedded on the `rsp_cop2cd` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`rsp_cop2cd`]
    /// function instead.
    ///
    /// [`RspCop2`]: crate::registers::RspCop2
    /// [`rsp_cop2cd`]: InstrField::rsp_cop2cd
    #[must_use]
    pub(crate) fn rsp_cop2cd_impl(&self) -> RspCop2 {
        EncodedFieldMask::cop2cd
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`RspVector`] register embedded on the `rsp_vs` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`rsp_vs`]
    /// function instead.
    ///
    /// [`RspVector`]: crate::registers::RspVector
    /// [`rsp_vs`]: InstrField::rsp_vs
    #[must_use]
    pub(crate) fn rsp_vs_impl(&self) -> RspVector {
        EncodedFieldMask::rsp_vs
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`RspVector`] register embedded on the `rsp_vt` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`rsp_vt`]
    /// function instead.
    ///
    /// [`RspVector`]: crate::registers::RspVector
    /// [`rsp_vt`]: InstrField::rsp_vt
    #[must_use]
    pub(crate) fn rsp_vt_impl(&self) -> RspVector {
        EncodedFieldMask::rsp_vt
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`RspVector`] register embedded on the `rsp_vd` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`rsp_vd`]
    /// function instead.
    ///
    /// [`RspVector`]: crate::registers::RspVector
    /// [`rsp_vd`]: InstrField::rsp_vd
    #[must_use]
    pub(crate) fn rsp_vd_impl(&self) -> RspVector {
        EncodedFieldMask::rsp_vd
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `rsp_elementhigh` value embedded on the `rsp_elementhigh`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_elementhigh`] function instead.
    ///
    /// [`rsp_elementhigh`]: InstrField::rsp_elementhigh
    #[must_use]
    pub(crate) fn rsp_elementhigh_impl(&self) -> u8 {
        EncodedFieldMask::rsp_elementhigh
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `rsp_elementlow` value embedded on the `rsp_elementlow` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_elementlow`] function instead.
    ///
    /// [`rsp_elementlow`]: InstrField::rsp_elementlow
    #[must_use]
    pub(crate) fn rsp_elementlow_impl(&self) -> u8 {
        EncodedFieldMask::rsp_elementlow
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `rsp_index` value embedded on the `rsp_index` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_index`] function instead.
    ///
    /// [`rsp_index`]: InstrField::rsp_index
    #[must_use]
    pub(crate) fn rsp_index_impl(&self) -> u8 {
        EncodedFieldMask::rsp_index
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `rsp_offset7` value embedded on the `rsp_offset7` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_offset7`] function instead.
    ///
    /// [`rsp_offset7`]: InstrField::rsp_offset7
    #[must_use]
    pub(crate) fn rsp_offset7_impl(&self) -> u8 {
        EncodedFieldMask::rsp_offset
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `rsp_offset8` value embedded on the `rsp_offset8` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_offset8`] function instead.
    ///
    /// [`rsp_offset8`]: InstrField::rsp_offset8
    #[must_use]
    pub(crate) fn rsp_offset8_impl(&self) -> u8 {
        let val: u8 = EncodedFieldMask::rsp_offset
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap();

        val << 1
    }

    /// Returns the `rsp_offset9` value embedded on the `rsp_offset9` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_offset9`] function instead.
    ///
    /// [`rsp_offset9`]: InstrField::rsp_offset9
    #[must_use]
    pub(crate) fn rsp_offset9_impl(&self) -> u16 {
        let val: u16 = EncodedFieldMask::rsp_offset
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap();

        val << 2
    }

    /// Returns the `rsp_offset10` value embedded on the `rsp_offset10` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_offset10`] function instead.
    ///
    /// [`rsp_offset10`]: InstrField::rsp_offset10
    #[must_use]
    pub(crate) fn rsp_offset10_impl(&self) -> u16 {
        let val: u16 = EncodedFieldMask::rsp_offset
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap();

        val << 3
    }

    /// Returns the `rsp_offset11` value embedded on the `rsp_offset11` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_offset11`] function instead.
    ///
    /// [`rsp_offset11`]: InstrField::rsp_offset11
    #[must_use]
    pub(crate) fn rsp_offset11_impl(&self) -> u16 {
        let val: u16 = EncodedFieldMask::rsp_offset
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap();

        val << 4
    }

    /// Returns the `rsp_de` value embedded on the `rsp_de` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`rsp_de`] function instead.
    ///
    /// [`rsp_de`]: InstrField::rsp_de
    #[must_use]
    pub(crate) fn rsp_de_impl(&self) -> u8 {
        EncodedFieldMask::rsp_de
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }
}

/// Impls R3000GTE opcode fields
#[cfg(feature = "R3000GTE")]
impl InstrField<'_> {
    /// Returns the `r3000gte_sf` value embedded on the `r3000gte_sf` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r3000gte_sf`]
    /// function instead.
    ///
    /// [`r3000gte_sf`]: InstrField::r3000gte_sf
    #[must_use]
    pub(crate) fn r3000gte_sf_impl(&self) -> u8 {
        EncodedFieldMask::r3000gte_sf
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r3000gte_mx` value embedded on the `r3000gte_mx` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r3000gte_mx`]
    /// function instead.
    ///
    /// [`r3000gte_mx`]: InstrField::r3000gte_mx
    #[must_use]
    pub(crate) fn r3000gte_mx_impl(&self) -> u8 {
        EncodedFieldMask::r3000gte_mx
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r3000gte_v` value embedded on the `r3000gte_v` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r3000gte_v`]
    /// function instead.
    ///
    /// [`r3000gte_v`]: InstrField::r3000gte_v
    #[must_use]
    pub(crate) fn r3000gte_v_impl(&self) -> u8 {
        EncodedFieldMask::r3000gte_v
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r3000gte_cv` value embedded on the `r3000gte_cv` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r3000gte_cv`]
    /// function instead.
    ///
    /// [`r3000gte_cv`]: InstrField::r3000gte_cv
    #[must_use]
    pub(crate) fn r3000gte_cv_impl(&self) -> u8 {
        EncodedFieldMask::r3000gte_cv
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r3000gte_lm` value embedded on the `r3000gte_lm` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r3000gte_lm`]
    /// function instead.
    ///
    /// [`r3000gte_lm`]: InstrField::r3000gte_lm
    #[must_use]
    pub(crate) fn r3000gte_lm_impl(&self) -> u8 {
        EncodedFieldMask::r3000gte_lm
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }
}

/// Impls R4000ALLEGREX opcode fields
#[cfg(feature = "R4000ALLEGREX")]
impl InstrField<'_> {
    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_s_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`r4000allegrex_s_vs`]: InstrField::r4000allegrex_s_vs
    #[must_use]
    pub(crate) fn r4000allegrex_s_vs_impl(&self) -> R4000AllegrexS {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_s_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`r4000allegrex_s_vt`]: InstrField::r4000allegrex_s_vt
    #[must_use]
    pub(crate) fn r4000allegrex_s_vt_impl(&self) -> R4000AllegrexS {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_s_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`r4000allegrex_s_vd`]: InstrField::r4000allegrex_s_vd
    #[must_use]
    pub(crate) fn r4000allegrex_s_vd_impl(&self) -> R4000AllegrexS {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vt_imm`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_s_vt_imm`]
    /// function instead.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`r4000allegrex_s_vt_imm`]: InstrField::r4000allegrex_s_vt_imm
    #[must_use]
    pub(crate) fn r4000allegrex_s_vt_imm_impl(&self) -> R4000AllegrexS {
        let upper = EncodedFieldMask::r4000allegrex_vt_imm_upper.get_shifted(self.instr.word());
        let lower = EncodedFieldMask::r4000allegrex_vt_imm_lower.get_shifted(self.instr.word());

        ((upper << 5) | lower).try_into().unwrap()
    }

    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vd_imm`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_s_vd_imm`]
    /// function instead.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`r4000allegrex_s_vd_imm`]: InstrField::r4000allegrex_s_vd_imm
    #[must_use]
    pub(crate) fn r4000allegrex_s_vd_imm_impl(&self) -> R4000AllegrexS {
        EncodedFieldMask::r4000allegrex_vd_imm
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV2D`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_p_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexV2D`]: crate::registers::R4000AllegrexV2D
    /// [`r4000allegrex_p_vs`]: InstrField::r4000allegrex_p_vs
    #[must_use]
    pub(crate) fn r4000allegrex_p_vs_impl(&self) -> R4000AllegrexV2D {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV2D`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_p_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexV2D`]: crate::registers::R4000AllegrexV2D
    /// [`r4000allegrex_p_vt`]: InstrField::r4000allegrex_p_vt
    #[must_use]
    pub(crate) fn r4000allegrex_p_vt_impl(&self) -> R4000AllegrexV2D {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV2D`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_p_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexV2D`]: crate::registers::R4000AllegrexV2D
    /// [`r4000allegrex_p_vd`]: InstrField::r4000allegrex_p_vd
    #[must_use]
    pub(crate) fn r4000allegrex_p_vd_impl(&self) -> R4000AllegrexV2D {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV3D`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_t_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexV3D`]: crate::registers::R4000AllegrexV3D
    /// [`r4000allegrex_t_vs`]: InstrField::r4000allegrex_t_vs
    #[must_use]
    pub(crate) fn r4000allegrex_t_vs_impl(&self) -> R4000AllegrexV3D {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV3D`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_t_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexV3D`]: crate::registers::R4000AllegrexV3D
    /// [`r4000allegrex_t_vt`]: InstrField::r4000allegrex_t_vt
    #[must_use]
    pub(crate) fn r4000allegrex_t_vt_impl(&self) -> R4000AllegrexV3D {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV3D`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_t_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexV3D`]: crate::registers::R4000AllegrexV3D
    /// [`r4000allegrex_t_vd`]: InstrField::r4000allegrex_t_vd
    #[must_use]
    pub(crate) fn r4000allegrex_t_vd_impl(&self) -> R4000AllegrexV3D {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV4D`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_q_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexV4D`]: crate::registers::R4000AllegrexV4D
    /// [`r4000allegrex_q_vs`]: InstrField::r4000allegrex_q_vs
    #[must_use]
    pub(crate) fn r4000allegrex_q_vs_impl(&self) -> R4000AllegrexV4D {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV4D`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_q_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexV4D`]: crate::registers::R4000AllegrexV4D
    /// [`r4000allegrex_q_vt`]: InstrField::r4000allegrex_q_vt
    #[must_use]
    pub(crate) fn r4000allegrex_q_vt_impl(&self) -> R4000AllegrexV4D {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV4D`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_q_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexV4D`]: crate::registers::R4000AllegrexV4D
    /// [`r4000allegrex_q_vd`]: InstrField::r4000allegrex_q_vd
    #[must_use]
    pub(crate) fn r4000allegrex_q_vd_impl(&self) -> R4000AllegrexV4D {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV4D`] register embedded on the `r4000allegrex_vt_imm`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_q_vt_imm`]
    /// function instead.
    ///
    /// [`R4000AllegrexV4D`]: crate::registers::R4000AllegrexV4D
    /// [`r4000allegrex_q_vt_imm`]: InstrField::r4000allegrex_q_vt_imm
    #[must_use]
    pub(crate) fn r4000allegrex_q_vt_imm_impl(&self) -> R4000AllegrexV4D {
        let upper = EncodedFieldMask::r4000allegrex_vt_6_imm_upper.get_shifted(self.instr.word());
        let lower = EncodedFieldMask::r4000allegrex_vt_imm_lower.get_shifted(self.instr.word());

        ((upper << 5) | lower).try_into().unwrap()
    }

    /// Returns the [`R4000AllegrexM2x2`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mp_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexM2x2`]: crate::registers::R4000AllegrexM2x2
    /// [`r4000allegrex_mp_vs`]: InstrField::r4000allegrex_mp_vs
    #[must_use]
    pub(crate) fn r4000allegrex_mp_vs_impl(&self) -> R4000AllegrexM2x2 {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM2x2`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mp_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexM2x2`]: crate::registers::R4000AllegrexM2x2
    /// [`r4000allegrex_mp_vt`]: InstrField::r4000allegrex_mp_vt
    #[must_use]
    pub(crate) fn r4000allegrex_mp_vt_impl(&self) -> R4000AllegrexM2x2 {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM2x2`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_q_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexM2x2`]: crate::registers::R4000AllegrexM2x2
    /// [`r4000allegrex_mp_vd`]: InstrField::r4000allegrex_mp_vd
    #[must_use]
    pub(crate) fn r4000allegrex_mp_vd_impl(&self) -> R4000AllegrexM2x2 {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM2x2`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mp_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexM2x2`]: crate::registers::R4000AllegrexM2x2
    /// [`r4000allegrex_mp_vs_transpose`]: InstrField::r4000allegrex_mp_vs_transpose
    #[must_use]
    pub(crate) fn r4000allegrex_mp_vs_transpose_impl(&self) -> R4000AllegrexM2x2 {
        // For whatever reason the transpose just toggles bit 5, no clue why.

        let value = EncodedFieldMask::r4000allegrex_vs.get_shifted(self.instr.word()) ^ 0x20;

        value.try_into().unwrap()
    }

    /// Returns the [`R4000AllegrexM3x3`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mt_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexM3x3`]: crate::registers::R4000AllegrexM3x3
    /// [`r4000allegrex_mt_vs`]: InstrField::r4000allegrex_mt_vs
    #[must_use]
    pub(crate) fn r4000allegrex_mt_vs_impl(&self) -> R4000AllegrexM3x3 {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM3x3`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mt_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexM3x3`]: crate::registers::R4000AllegrexM3x3
    /// [`r4000allegrex_mt_vt`]: InstrField::r4000allegrex_mt_vt
    #[must_use]
    pub(crate) fn r4000allegrex_mt_vt_impl(&self) -> R4000AllegrexM3x3 {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM3x3`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_q_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexM3x3`]: crate::registers::R4000AllegrexM3x3
    /// [`r4000allegrex_mt_vd`]: InstrField::r4000allegrex_mt_vd
    #[must_use]
    pub(crate) fn r4000allegrex_mt_vd_impl(&self) -> R4000AllegrexM3x3 {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM3x3`] register embedded on the `r4000allegrex_vs_transpose`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mt_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexM3x3`]: crate::registers::R4000AllegrexM3x3
    /// [`r4000allegrex_mt_vs_transpose`]: InstrField::r4000allegrex_mt_vs_transpose
    #[must_use]
    pub(crate) fn r4000allegrex_mt_vs_transpose_impl(&self) -> R4000AllegrexM3x3 {
        // For whatever reason the transpose just toggles bit 5, no clue why.

        let value = EncodedFieldMask::r4000allegrex_vs.get_shifted(self.instr.word()) ^ 0x20;

        value.try_into().unwrap()
    }

    /// Returns the [`R4000AllegrexM4x4`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mq_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexM4x4`]: crate::registers::R4000AllegrexM4x4
    /// [`r4000allegrex_mq_vs`]: InstrField::r4000allegrex_mq_vs
    #[must_use]
    pub(crate) fn r4000allegrex_mq_vs_impl(&self) -> R4000AllegrexM4x4 {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM4x4`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mq_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexM4x4`]: crate::registers::R4000AllegrexM4x4
    /// [`r4000allegrex_mq_vt`]: InstrField::r4000allegrex_mq_vt
    #[must_use]
    pub(crate) fn r4000allegrex_mq_vt_impl(&self) -> R4000AllegrexM4x4 {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM4x4`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_q_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexM4x4`]: crate::registers::R4000AllegrexM4x4
    /// [`r4000allegrex_mq_vd`]: InstrField::r4000allegrex_mq_vd
    #[must_use]
    pub(crate) fn r4000allegrex_mq_vd_impl(&self) -> R4000AllegrexM4x4 {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM4x4`] register embedded on the `r4000allegrex_vs_transpose`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_mq_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexM4x4`]: crate::registers::R4000AllegrexM4x4
    /// [`r4000allegrex_mq_vs_transpose`]: InstrField::r4000allegrex_mq_vs_transpose
    #[must_use]
    pub(crate) fn r4000allegrex_mq_vs_transpose_impl(&self) -> R4000AllegrexM4x4 {
        // For whatever reason the transpose just toggles bit 5, no clue why.

        let value = EncodedFieldMask::r4000allegrex_vs.get_shifted(self.instr.word()) ^ 0x20;

        value.try_into().unwrap()
    }

    /// Returns the [`R4000AllegrexVfpuControl`] register embedded on the `r4000allegrex_cop2cs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_cop2cs`]
    /// function instead.
    ///
    /// [`R4000AllegrexVfpuControl`]: crate::registers::R4000AllegrexVfpuControl
    /// [`r4000allegrex_cop2cs`]: InstrField::r4000allegrex_cop2cs
    #[must_use]
    pub(crate) fn r4000allegrex_cop2cs_impl(&self) -> R4000AllegrexVfpuControl {
        EncodedFieldMask::r4000allegrex_cop2cs
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexVfpuControl`] register embedded on the `r4000allegrex_cop2cd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_cop2cd`]
    /// function instead.
    ///
    /// [`R4000AllegrexVfpuControl`]: crate::registers::R4000AllegrexVfpuControl
    /// [`r4000allegrex_cop2cd`]: InstrField::r4000allegrex_cop2cd
    #[must_use]
    pub(crate) fn r4000allegrex_cop2cd_impl(&self) -> R4000AllegrexVfpuControl {
        EncodedFieldMask::r4000allegrex_cop2cd
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_pos` value embedded on the `r4000allegrex_pos` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_pos`]
    /// function instead.
    ///
    /// [`r4000allegrex_pos`]: InstrField::r4000allegrex_pos
    #[must_use]
    pub(crate) fn r4000allegrex_pos_impl(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_pos
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_size` value embedded on the `r4000allegrex_size` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_size`]
    /// function instead.
    ///
    /// [`r4000allegrex_size`]: InstrField::r4000allegrex_size
    #[must_use]
    pub(crate) fn r4000allegrex_size_impl(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_size
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_size_plus_pos` value embedded on the `r4000allegrex_size_plus_pos` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_size_plus_pos`]
    /// function instead.
    ///
    /// [`r4000allegrex_size_plus_pos`]: InstrField::r4000allegrex_size_plus_pos
    #[must_use]
    pub(crate) fn r4000allegrex_size_plus_pos_impl(&self) -> i8 {
        EncodedFieldMask::r4000allegrex_size_plus_pos
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_imm3` value embedded on the `r4000allegrex_imm3` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_imm3`]
    /// function instead.
    ///
    /// [`r4000allegrex_imm3`]: InstrField::r4000allegrex_imm3
    #[must_use]
    pub(crate) fn r4000allegrex_imm3_impl(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_imm3
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_offset14` value embedded on the `r4000allegrex_offset14` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_offset14`]
    /// function instead.
    ///
    /// [`r4000allegrex_offset14`]: InstrField::r4000allegrex_offset14
    #[must_use]
    pub(crate) fn r4000allegrex_offset14_impl(&self) -> u16 {
        let value: u16 = EncodedFieldMask::r4000allegrex_offset14
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap();

        value << 2
    }

    /// Returns the `r4000allegrex_wb` value embedded on the `r4000allegrex_wb` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_wb`]
    /// function instead.
    ///
    /// [`r4000allegrex_wb`]: InstrField::r4000allegrex_wb
    #[must_use]
    pub(crate) const fn r4000allegrex_wb_impl(&self) -> bool {
        EncodedFieldMask::r4000allegrex_wb.get_shifted(self.instr.word()) != 0
    }

    /// Returns the `r4000allegrex_vcmp_cond` value embedded on the `r4000allegrex_vcmp_cond` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_vcmp_cond`]
    /// function instead.
    ///
    /// [`r4000allegrex_vcmp_cond`]: InstrField::r4000allegrex_vcmp_cond
    #[must_use]
    pub(crate) fn r4000allegrex_vcmp_cond_impl(&self) -> R4000AllegrexVCond {
        EncodedFieldMask::r4000allegrex_vcmp_cond
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexVConstant`] register embedded on the `r4000allegrex_vconstant`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r4000allegrex_vconstant`]
    /// function instead.
    ///
    /// [`R4000AllegrexVConstant`]: crate::registers::R4000AllegrexVConstant
    /// [`r4000allegrex_vconstant`]: InstrField::r4000allegrex_vconstant
    #[must_use]
    pub(crate) fn r4000allegrex_vconstant_impl(&self) -> R4000AllegrexVConstant {
        EncodedFieldMask::r4000allegrex_vconstant
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_power_of_two` value embedded on the `r4000allegrex_power_of_two` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_power_of_two`]
    /// function instead.
    ///
    /// [`r4000allegrex_power_of_two`]: InstrField::r4000allegrex_power_of_two
    #[must_use]
    pub(crate) fn r4000allegrex_power_of_two_impl(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_power_of_two
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_vfpu_cc_bit` value embedded on the `r4000allegrex_vfpu_cc_bit` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_vfpu_cc_bit`]
    /// function instead.
    ///
    /// [`r4000allegrex_vfpu_cc_bit`]: InstrField::r4000allegrex_vfpu_cc_bit
    #[must_use]
    pub(crate) fn r4000allegrex_vfpu_cc_bit_impl(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_vfpu_cc_bit
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_bn` value embedded on the `r4000allegrex_bn` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_bn`]
    /// function instead.
    ///
    /// [`r4000allegrex_bn`]: InstrField::r4000allegrex_bn
    #[must_use]
    pub(crate) fn r4000allegrex_bn_impl(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_bn
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_intfloat16` value embedded on the `r4000allegrex_intfloat16` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_intfloat16`]
    /// function instead.
    ///
    /// [`r4000allegrex_intfloat16`]: InstrField::r4000allegrex_intfloat16
    #[must_use]
    pub(crate) fn r4000allegrex_intfloat16_impl(&self) -> u16 {
        EncodedFieldMask::r4000allegrex_intfloat16
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the value embedded on the `r4000allegrex_intfloat16` field of
    /// the word of this instruction, but interpreted as a 16 bits signed value.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the
    /// [`r4000allegrex_int16`] function instead.
    ///
    /// [`r4000allegrex_int16`]: InstrField::r4000allegrex_int16
    #[must_use]
    pub(crate) fn r4000allegrex_int16_impl(&self) -> i16 {
        utils::from_2s_complement::<16>(self.r4000allegrex_intfloat16_impl() as u32) as i16
    }

    /// Returns the value embedded on the `r4000allegrex_intfloat16` field of
    /// the word of this instruction, but interpreted as a 16 bits float.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the
    /// [`r4000allegrex_float16`] function instead.
    ///
    /// [`r4000allegrex_float16`]: InstrField::r4000allegrex_float16
    #[must_use]
    pub(crate) fn r4000allegrex_float16_impl(&self) -> f32 {
        // Ideally this function would return a f16, but that type is not stable yet.

        let hex = utils::floatrepr_32_from_16(self.r4000allegrex_intfloat16_impl());

        f32::from_bits(hex)
    }

    /// Returns the `r4000allegrex_vrot_code` value embedded on the `r4000allegrex_vrot_code` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_vrot_code`]
    /// function instead.
    ///
    /// [`r4000allegrex_vrot_code`]: InstrField::r4000allegrex_vrot_code
    #[must_use]
    pub(crate) fn r4000allegrex_vrot_code_impl(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_vrot_code
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_wpx` value embedded on the `r4000allegrex_wpx` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_wpx`]
    /// function instead.
    ///
    /// [`r4000allegrex_wpx`]: InstrField::r4000allegrex_wpx
    #[must_use]
    pub(crate) fn r4000allegrex_wpx_impl(&self) -> R4000AllegrexPrefixDst {
        let c = (self.instr.word() & utils::bitmask(8, 1)) >> 8;
        let d = self.instr.word() & utils::bitmask(0, 2);

        ((c << 2) | d).try_into().unwrap()
    }

    /// Returns the `r4000allegrex_wpy` value embedded on the `r4000allegrex_wpy` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_wpy`]
    /// function instead.
    ///
    /// [`r4000allegrex_wpy`]: InstrField::r4000allegrex_wpy
    #[must_use]
    pub(crate) fn r4000allegrex_wpy_impl(&self) -> R4000AllegrexPrefixDst {
        let c = (self.instr.word() & utils::bitmask(9, 1)) >> 9;
        let d = (self.instr.word() & utils::bitmask(2, 2)) >> 2;

        ((c << 2) | d).try_into().unwrap()
    }

    /// Returns the `r4000allegrex_wpz` value embedded on the `r4000allegrex_wpz` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_wpz`]
    /// function instead.
    ///
    /// [`r4000allegrex_wpz`]: InstrField::r4000allegrex_wpz
    #[must_use]
    pub(crate) fn r4000allegrex_wpz_impl(&self) -> R4000AllegrexPrefixDst {
        let c = (self.instr.word() & utils::bitmask(10, 1)) >> 10;
        let d = (self.instr.word() & utils::bitmask(4, 2)) >> 4;

        ((c << 2) | d).try_into().unwrap()
    }

    /// Returns the `r4000allegrex_wpw` value embedded on the `r4000allegrex_wpw` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_wpw`]
    /// function instead.
    ///
    /// [`r4000allegrex_wpw`]: InstrField::r4000allegrex_wpw
    #[must_use]
    pub(crate) fn r4000allegrex_wpw_impl(&self) -> R4000AllegrexPrefixDst {
        let c = (self.instr.word() & utils::bitmask(11, 1)) >> 11;
        let d = (self.instr.word() & utils::bitmask(6, 2)) >> 6;

        ((c << 2) | d).try_into().unwrap()
    }

    /// Returns the `r4000allegrex_rpx` value embedded on the `r4000allegrex_rpx` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_rpx`]
    /// function instead.
    ///
    /// [`r4000allegrex_rpx`]: InstrField::r4000allegrex_rpx
    #[must_use]
    pub(crate) fn r4000allegrex_rpx_impl(&self) -> R4000AllegrexPrefixSrc {
        let a = (self.instr.word() & utils::bitmask(16, 1)) >> 16;
        let b = (self.instr.word() & utils::bitmask(12, 1)) >> 12;
        let c = (self.instr.word() & utils::bitmask(8, 1)) >> 8;
        let d = self.instr.word() & utils::bitmask(0, 2);

        ((a << 4) | (b << 3) | (c << 2) | d).try_into().unwrap()
    }

    /// Returns the `r4000allegrex_rpy` value embedded on the `r4000allegrex_rpy` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_rpy`]
    /// function instead.
    ///
    /// [`r4000allegrex_rpy`]: InstrField::r4000allegrex_rpy
    #[must_use]
    pub(crate) fn r4000allegrex_rpy_impl(&self) -> R4000AllegrexPrefixSrc {
        let a = (self.instr.word() & utils::bitmask(17, 1)) >> 17;
        let b = (self.instr.word() & utils::bitmask(13, 1)) >> 13;
        let c = (self.instr.word() & utils::bitmask(9, 1)) >> 9;
        let d = (self.instr.word() & utils::bitmask(2, 2)) >> 2;

        ((a << 4) | (b << 3) | (c << 2) | d).try_into().unwrap()
    }

    /// Returns the `r4000allegrex_rpz` value embedded on the `r4000allegrex_rpz` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_rpz`]
    /// function instead.
    ///
    /// [`r4000allegrex_rpz`]: InstrField::r4000allegrex_rpz
    #[must_use]
    pub(crate) fn r4000allegrex_rpz_impl(&self) -> R4000AllegrexPrefixSrc {
        let a = (self.instr.word() & utils::bitmask(18, 1)) >> 18;
        let b = (self.instr.word() & utils::bitmask(14, 1)) >> 14;
        let c = (self.instr.word() & utils::bitmask(10, 1)) >> 10;
        let d = (self.instr.word() & utils::bitmask(4, 2)) >> 4;

        ((a << 4) | (b << 3) | (c << 2) | d).try_into().unwrap()
    }

    /// Returns the `r4000allegrex_rpw` value embedded on the `r4000allegrex_rpw` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`r4000allegrex_rpw`]
    /// function instead.
    ///
    /// [`r4000allegrex_rpw`]: InstrField::r4000allegrex_rpw
    #[must_use]
    pub(crate) fn r4000allegrex_rpw_impl(&self) -> R4000AllegrexPrefixSrc {
        let a = (self.instr.word() & utils::bitmask(19, 1)) >> 19;
        let b = (self.instr.word() & utils::bitmask(15, 1)) >> 15;
        let c = (self.instr.word() & utils::bitmask(11, 1)) >> 11;
        let d = (self.instr.word() & utils::bitmask(6, 2)) >> 6;

        ((a << 4) | (b << 3) | (c << 2) | d).try_into().unwrap()
    }
}

// #[doc(hidden)]
/// Impls R5900EE opcode fields
#[cfg(feature = "R5900EE")]
impl InstrField<'_> {
    /// Returns the `r5900ee_imm5` value embedded on the `r5900ee_imm5` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_imm5` value. It is recommended to use the
    /// [`r5900ee_imm5`] function instead.
    ///
    /// [`r5900ee_imm5`]: InstrField::r5900ee_imm5
    #[must_use]
    pub(crate) fn r5900ee_imm5_impl(&self) -> i8 {
        let raw = EncodedFieldMask::r5900ee_imm5.get_shifted(self.instr.word());

        utils::from_2s_complement::<5>(raw).try_into().unwrap()
    }

    /// Returns the `r5900ee_imm15` value embedded on the `r5900ee_imm15` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_imm15` value. It is recommended to use the
    /// [`r5900ee_imm15`] function instead.
    ///
    /// [`r5900ee_imm15`]: InstrField::r5900ee_imm15
    #[must_use]
    pub(crate) fn r5900ee_imm15_impl(&self) -> u16 {
        EncodedFieldMask::r5900ee_imm15
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R5900EEVF`] register embedded on the `r5900ee_vfs` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r5900ee_vfs`]
    /// function instead.
    ///
    /// [`R5900EEVF`]: crate::registers::R5900EEVF
    /// [`r5900ee_vfs`]: InstrField::r5900ee_vfs
    #[must_use]
    pub(crate) fn r5900ee_vfs_impl(&self) -> R5900EEVF {
        EncodedFieldMask::r5900ee_vfs
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R5900EEVF`] register embedded on the `r5900ee_vft` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r5900ee_vft`]
    /// function instead.
    ///
    /// [`R5900EEVF`]: crate::registers::R5900EEVF
    /// [`r5900ee_vft`]: InstrField::r5900ee_vft
    #[must_use]
    pub(crate) fn r5900ee_vft_impl(&self) -> R5900EEVF {
        EncodedFieldMask::r5900ee_vft
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R5900EEVF`] register embedded on the `r5900ee_vfd` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r5900ee_vfd`]
    /// function instead.
    ///
    /// [`R5900EEVF`]: crate::registers::R5900EEVF
    /// [`r5900ee_vfd`]: InstrField::r5900ee_vfd
    #[must_use]
    pub(crate) fn r5900ee_vfd_impl(&self) -> R5900EEVF {
        EncodedFieldMask::r5900ee_vfd
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R5900EEVI`] register embedded on the `r5900ee_vis` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r5900ee_vis`]
    /// function instead.
    ///
    /// [`R5900EEVI`]: crate::registers::R5900EEVI
    /// [`r5900ee_vis`]: InstrField::r5900ee_vis
    #[must_use]
    pub(crate) fn r5900ee_vis_impl(&self) -> R5900EEVI {
        EncodedFieldMask::r5900ee_vis
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R5900EEVI`] register embedded on the `r5900ee_vit` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r5900ee_vit`]
    /// function instead.
    ///
    /// [`R5900EEVI`]: crate::registers::R5900EEVI
    /// [`r5900ee_vit`]: InstrField::r5900ee_vit
    #[must_use]
    pub(crate) fn r5900ee_vit_impl(&self) -> R5900EEVI {
        EncodedFieldMask::r5900ee_vit
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R5900EEVI`] register embedded on the `r5900ee_vid` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`r5900ee_vid`]
    /// function instead.
    ///
    /// [`R5900EEVI`]: crate::registers::R5900EEVI
    /// [`r5900ee_vid`]: InstrField::r5900ee_vid
    #[must_use]
    pub(crate) fn r5900ee_vid_impl(&self) -> R5900EEVI {
        EncodedFieldMask::r5900ee_vid
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r5900ee_xyzw_x` value embedded on the `r5900ee_xyzw_x` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_xyzw_x` value. It is recommended to use the
    /// [`r5900ee_xyzw_x`] function instead.
    ///
    /// [`r5900ee_xyzw_x`]: InstrField::r5900ee_xyzw_x
    #[must_use]
    pub(crate) const fn r5900ee_xyzw_x_impl(&self) -> bool {
        EncodedFieldMask::r5900ee_xyzw_x.get_shifted(self.instr.word()) != 0
    }

    /// Returns the `r5900ee_xyzw_y` value embedded on the `r5900ee_xyzw_y` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_xyzw_y` value. It is recommended to use the
    /// [`r5900ee_xyzw_y`] function instead.
    ///
    /// [`r5900ee_xyzw_y`]: InstrField::r5900ee_xyzw_y
    #[must_use]
    pub(crate) const fn r5900ee_xyzw_y_impl(&self) -> bool {
        EncodedFieldMask::r5900ee_xyzw_y.get_shifted(self.instr.word()) != 0
    }

    /// Returns the `r5900ee_xyzw_z` value embedded on the `r5900ee_xyzw_z` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_xyzw_z` value. It is recommended to use the
    /// [`r5900ee_xyzw_z`] function instead.
    ///
    /// [`r5900ee_xyzw_z`]: InstrField::r5900ee_xyzw_z
    #[must_use]
    pub(crate) const fn r5900ee_xyzw_z_impl(&self) -> bool {
        EncodedFieldMask::r5900ee_xyzw_z.get_shifted(self.instr.word()) != 0
    }

    /// Returns the `r5900ee_xyzw_w` value embedded on the `r5900ee_xyzw_w` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_xyzw_w` value. It is recommended to use the
    /// [`r5900ee_xyzw_w`] function instead.
    ///
    /// [`r5900ee_xyzw_w`]: InstrField::r5900ee_xyzw_w
    #[must_use]
    pub(crate) const fn r5900ee_xyzw_w_impl(&self) -> bool {
        EncodedFieldMask::r5900ee_xyzw_w.get_shifted(self.instr.word()) != 0
    }

    /// Returns the `r5900ee_n` value embedded on the `r5900ee_n` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_n` value. It is recommended to use the
    /// [`r5900ee_n`] function instead.
    ///
    /// [`r5900ee_n`]: InstrField::r5900ee_n
    #[must_use]
    pub(crate) fn r5900ee_n_impl(&self) -> u8 {
        EncodedFieldMask::r5900ee_n
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r5900ee_l` value embedded on the `r5900ee_l` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_l` value. It is recommended to use the
    /// [`r5900ee_l`] function instead.
    ///
    /// [`r5900ee_l`]: InstrField::r5900ee_l
    #[must_use]
    pub(crate) fn r5900ee_l_impl(&self) -> u8 {
        EncodedFieldMask::r5900ee_l
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r5900ee_m` value embedded on the `r5900ee_m` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900ee_m` value. It is recommended to use the
    /// [`r5900ee_m`] function instead.
    ///
    /// [`r5900ee_m`]: InstrField::r5900ee_m
    #[must_use]
    pub(crate) fn r5900ee_m_impl(&self) -> u8 {
        EncodedFieldMask::r5900ee_m
            .get_shifted(self.instr.word())
            .try_into()
            .unwrap()
    }
}
