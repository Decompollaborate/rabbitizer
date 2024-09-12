/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![no_std]

use core::ops::Index;

mod generated;

mod opcode;
mod opcode_descriptor;

mod utils;

pub use generated::Opcode;

pub use opcode_descriptor::OpcodeDescriptor;

pub mod opcodes {
    use super::*;

    pub static OPCODES: [OpcodeDescriptor; opcode::OPCODE_COUNT] = {
        let mut table = [OpcodeDescriptor::new(""); opcode::OPCODE_COUNT as usize];

        table[Opcode::cpu_INVALID as usize] = OpcodeDescriptor {
            ..OpcodeDescriptor::new("INVALID")
        }
        .check_panic_chain();
        table[Opcode::cpu_j as usize] = OpcodeDescriptor {
            is_jump: true,
            is_jump_with_address: true,
            ..OpcodeDescriptor::new("j")
        }
        .check_panic_chain();

        let mut i = 0;
        while i < opcode::OPCODE_COUNT {
            table[i].check_panic();
            i += 1;
        }

        table
    };

    impl Index<Opcode> for [OpcodeDescriptor<'static>] {
        type Output = OpcodeDescriptor<'static>;

        fn index(&self, index: Opcode) -> &Self::Output {
            &self[index as usize]
        }
    }
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
