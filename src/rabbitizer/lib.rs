/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![cfg_attr(not(feature = "std"), no_std)]

mod generated;

mod instr_suffix;
mod instr_type;
mod opcode;
mod opcode_descriptor;
mod operand;
mod access_type;

mod utils;

pub use generated::AccessType;
pub use generated::InstrSuffix;
pub use generated::Opcode;
pub use generated::Operand;
#[allow(deprecated)]
pub use instr_type::InstrType;

pub use opcode_descriptor::OpcodeDescriptor;

pub mod opcodes {
    use super::*;

    pub static OPCODES: [OpcodeDescriptor; opcode::OPCODE_COUNT] = {
        let mut table =
            [OpcodeDescriptor::new("", Operand::arr_0()); opcode::OPCODE_COUNT as usize];

        table[Opcode::cpu_INVALID as usize] = OpcodeDescriptor {
            ..OpcodeDescriptor::new("INVALID", Operand::arr_0())
        }
        .check_panic_chain();
        table[Opcode::cpu_j as usize] = OpcodeDescriptor {
            is_jump: true,
            is_jump_with_address: true,
            ..OpcodeDescriptor::new("j", Operand::arr_1(Operand::cpu_label))
        }
        .check_panic_chain();

        let mut i = 0;
        while i < opcode::OPCODE_COUNT {
            table[i].check_panic();
            i += 1;
        }

        table
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_j() {
        assert!(opcodes::OPCODES[Opcode::cpu_j].is_jump);
        assert!(Opcode::cpu_j.get_descriptor().is_jump);
        assert!(opcodes::OPCODES[Opcode::cpu_j].is_jump_with_address);
        assert!(Opcode::cpu_j.get_descriptor().is_jump_with_address);
        assert!(!opcodes::OPCODES[Opcode::cpu_j].is_branch);
        assert!(!Opcode::cpu_j.get_descriptor().is_branch);
    }
}
