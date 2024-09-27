/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{
    operand::{OperandIterator, OPERAND_COUNT_MAX},
    AccessType, EncodedFieldMask, InstrType, IsaExtension, IsaVersion, Opcode, OpcodeDescriptor,
    Operand, OPCODES,
};

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `Opcode::MAX` value, so instead we manually maintain
// this constant.
pub(crate) const OPCODE_COUNT: usize = 943;

impl Opcode {
    #[must_use]
    pub fn get_descriptor(&self) -> &'static OpcodeDescriptor {
        &OPCODES[*self]
    }
}

/// Getters
impl Opcode {
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.get_descriptor().name()
    }

    #[must_use]
    pub fn isa_version(&self) -> IsaVersion {
        self.get_descriptor().isa_version()
    }
    #[must_use]
    pub fn isa_extension(&self) -> IsaExtension {
        self.get_descriptor().isa_extension()
    }

    #[must_use]
    pub fn operands(&self) -> &[Operand; OPERAND_COUNT_MAX] {
        self.get_descriptor().operands()
    }
    #[must_use]
    pub fn operands_iter(&self) -> OperandIterator {
        self.get_descriptor().operands_iter()
    }

    #[must_use]
    pub fn instr_type(&self) -> InstrType {
        self.get_descriptor().instr_type()
    }
    // #[must_use]
    // pub fn instr_suffix(&self) -> InstrSuffix {
    //     self.get_descriptor().instr_suffix()
    // }
    #[must_use]
    pub fn is_branch(&self) -> bool {
        self.get_descriptor().is_branch()
    }
    #[must_use]
    pub fn is_branch_likely(&self) -> bool {
        self.get_descriptor().is_branch_likely()
    }
    #[must_use]
    pub fn is_jump(&self) -> bool {
        self.get_descriptor().is_jump()
    }
    #[must_use]
    pub fn is_jump_with_address(&self) -> bool {
        self.get_descriptor().is_jump_with_address()
    }
    #[must_use]
    pub fn is_trap(&self) -> bool {
        self.get_descriptor().is_trap()
    }
    #[must_use]
    pub fn is_float(&self) -> bool {
        self.get_descriptor().is_float()
    }
    #[must_use]
    pub fn is_double(&self) -> bool {
        self.get_descriptor().is_double()
    }
    #[must_use]
    pub fn is_unsigned(&self) -> bool {
        self.get_descriptor().is_unsigned()
    }
    #[must_use]
    pub fn modifies_rs(&self) -> bool {
        self.get_descriptor().modifies_rs()
    }
    #[must_use]
    pub fn modifies_rt(&self) -> bool {
        self.get_descriptor().modifies_rt()
    }
    #[must_use]
    pub fn modifies_rd(&self) -> bool {
        self.get_descriptor().modifies_rd()
    }
    #[must_use]
    pub fn reads_rs(&self) -> bool {
        self.get_descriptor().reads_rs()
    }
    #[must_use]
    pub fn reads_rt(&self) -> bool {
        self.get_descriptor().reads_rt()
    }
    #[must_use]
    pub fn reads_rd(&self) -> bool {
        self.get_descriptor().reads_rd()
    }
    #[must_use]
    pub fn reads_hi(&self) -> bool {
        self.get_descriptor().reads_hi()
    }
    #[must_use]
    pub fn reads_lo(&self) -> bool {
        self.get_descriptor().reads_lo()
    }
    #[must_use]
    pub fn modifies_hi(&self) -> bool {
        self.get_descriptor().modifies_hi()
    }
    #[must_use]
    pub fn modifies_lo(&self) -> bool {
        self.get_descriptor().modifies_lo()
    }
    #[must_use]
    pub fn modifies_fs(&self) -> bool {
        self.get_descriptor().modifies_fs()
    }
    #[must_use]
    pub fn modifies_ft(&self) -> bool {
        self.get_descriptor().modifies_ft()
    }
    #[must_use]
    pub fn modifies_fd(&self) -> bool {
        self.get_descriptor().modifies_fd()
    }
    #[must_use]
    pub fn reads_fs(&self) -> bool {
        self.get_descriptor().reads_fs()
    }
    #[must_use]
    pub fn reads_ft(&self) -> bool {
        self.get_descriptor().reads_ft()
    }
    #[must_use]
    pub fn reads_fd(&self) -> bool {
        self.get_descriptor().reads_fd()
    }
    #[must_use]
    pub fn not_emitted_by_compilers(&self) -> bool {
        self.get_descriptor().not_emitted_by_compilers()
    }
    #[must_use]
    pub fn can_be_hi(&self) -> bool {
        self.get_descriptor().can_be_hi()
    }
    #[must_use]
    pub fn can_be_lo(&self) -> bool {
        self.get_descriptor().can_be_lo()
    }
    #[must_use]
    pub fn does_link(&self) -> bool {
        self.get_descriptor().does_link()
    }
    #[must_use]
    pub fn does_dereference(&self) -> bool {
        self.get_descriptor().does_dereference()
    }
    #[must_use]
    pub fn does_load(&self) -> bool {
        self.get_descriptor().does_load()
    }
    #[must_use]
    pub fn does_store(&self) -> bool {
        self.get_descriptor().does_store()
    }
    #[must_use]
    pub fn maybe_is_move(&self) -> bool {
        self.get_descriptor().maybe_is_move()
    }
    #[must_use]
    pub fn is_pseudo(&self) -> bool {
        self.get_descriptor().is_pseudo()
    }
    #[must_use]
    pub fn access_type(&self) -> AccessType {
        self.get_descriptor().access_type()
    }
    #[must_use]
    pub fn does_unsigned_memory_access(&self) -> bool {
        self.get_descriptor().does_unsigned_memory_access()
    }
}

impl Opcode {
    #[must_use]
    pub fn valid_bits(&self) -> EncodedFieldMask {
        self.get_descriptor().valid_bits()
    }

    #[must_use]
    pub fn has_any_operands(&self) -> bool {
        self.get_descriptor().has_any_operands()
    }

    #[must_use]
    pub fn has_specific_operand(&self, operand: Operand) -> bool {
        self.get_descriptor().has_specific_operand(operand)
    }

    #[must_use]
    pub fn has_operand_alias(&self, operand: Operand) -> bool {
        self.get_descriptor().has_operand_alias(operand)
    }
}

impl Opcode {
    #[must_use]
    pub const fn default() -> Self {
        Self::ALL_INVALID
    }

    #[must_use]
    pub const fn is_valid(&self) -> bool {
        !matches!(*self, Self::ALL_INVALID)
    }
}

impl Default for Opcode {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_j() {
        assert!(OPCODES[Opcode::cpu_j].is_jump);
        assert!(Opcode::cpu_j.get_descriptor().is_jump);
        assert!(OPCODES[Opcode::cpu_j].is_jump_with_address);
        assert!(Opcode::cpu_j.get_descriptor().is_jump_with_address);
        assert!(!OPCODES[Opcode::cpu_j].is_branch);
        assert!(!Opcode::cpu_j.get_descriptor().is_branch);
    }
}
