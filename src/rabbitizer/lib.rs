/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![cfg_attr(not(feature = "std"), no_std)]

mod generated;

mod access_type;
mod instr_suffix;
mod instr_type;
mod opcode;
mod opcode_descriptor;
mod operand;

mod utils;

pub use generated::AccessType;
pub use generated::InstrSuffix;
pub use generated::Opcode;
pub use generated::Operand;
#[allow(deprecated)]
pub use instr_type::InstrType;

pub use opcode_descriptor::OpcodeDescriptor;

pub use generated::OPCODES;

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

    #[test]
    fn test_addiu_operands() {
        let mut operands = Opcode::cpu_addiu.get_descriptor().operands_iter();

        assert_eq!(operands.next(), Some(Operand::cpu_rt).as_ref());
        assert_eq!(operands.next(), Some(Operand::cpu_rs).as_ref());
        assert_eq!(operands.next(), Some(Operand::cpu_immediate).as_ref());
        assert_eq!(operands.next(), None);
    }
}
