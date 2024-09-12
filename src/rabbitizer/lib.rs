/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
#[macro_use]
extern crate std;

mod generated;

mod access_type;
mod encoded_field_mask;
mod flags;
mod instr_suffix;
mod instr_type;
mod instruction;
mod isa_extension;
mod isa_version;
mod opcode;
mod opcode_decoder;
mod opcode_descriptor;
mod operand;

mod utils;

pub use generated::AccessType;
pub use generated::InstrSuffix;
pub use generated::IsaExtension;
pub use generated::IsaVersion;
pub use generated::Opcode;
pub use generated::OpcodeCategory;
pub use generated::Operand;
#[allow(deprecated)]
pub use instr_type::InstrType;

pub use generated::OPCODES;

pub use encoded_field_mask::EncodedFieldMask;
pub use instruction::Instruction;
pub(crate) use opcode_decoder::OpcodeDecoder;
pub use opcode_descriptor::OpcodeDescriptor;

pub use flags::DecodingFlags;
pub use flags::InstructionFlags;

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
