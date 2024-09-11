/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use std::ops::Index;

mod opcode;
mod opcode_descriptor;

pub use opcode::Opcode;
pub use opcode_descriptor::OpcodeDescriptor;
pub use opcode_descriptor::OpcodeDescriptorBuilder;


pub mod opcodes {
    use super::*;

    pub static OPCODE_J: OpcodeDescriptor = OpcodeDescriptorBuilder::new("J").is_jump().is_jump_with_address().build();

    pub static OPCODES: [&OpcodeDescriptor; Opcode::MAX as usize] = {
        let mut a: [&OpcodeDescriptor; Opcode::MAX as usize] = [&OpcodeDescriptor::new_uinit(); Opcode::MAX as usize];
        a[Opcode::J as usize] = &OPCODE_J;
        a
    };

    impl Index<Opcode> for [&'static OpcodeDescriptor<'static>] {
        type Output = &'static OpcodeDescriptor<'static>;

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
        assert_eq!(opcodes::OPCODES[0], &opcodes::OPCODE_J);
        assert_eq!(opcodes::OPCODES[Opcode::J], &opcodes::OPCODE_J);
        assert_eq!(Opcode::J.get_descriptor(), &opcodes::OPCODE_J);

        assert_eq!(*opcodes::OPCODES[0], opcodes::OPCODE_J);
        assert_eq!(*opcodes::OPCODES[Opcode::J], opcodes::OPCODE_J);
        assert_eq!(*Opcode::J.get_descriptor(), opcodes::OPCODE_J);
    }
}
