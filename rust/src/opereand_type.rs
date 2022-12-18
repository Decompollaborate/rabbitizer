/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{operand_type_enum, instruction, utils};

extern "C" {
    fn RabbitizerOperandType_getBufferSize(
        operand: operand_type_enum::OperandType,
        instr: *const instruction::InstructionBase,
        immOverrideLength: utils::SizeT,
    ) -> utils::SizeT;

    fn RabbitizerOperandType_disassemble(
        operand: operand_type_enum::OperandType,
        instr: *const instruction::InstructionBase,
        dst: *mut cty::c_char,
        immOverride: *const cty::c_char,
        immOverrideLength: utils::SizeT,
    ) -> utils::SizeT;
}

impl operand_type_enum::OperandType {
    pub fn disassemble(&self, instr: instruction::Instruction, imm_override: Option<&str>) -> String {
        let (imm_override_ptr, imm_override_len) = utils::c_string_from_str(imm_override);

        unsafe {
            let buffer_size = RabbitizerOperandType_getBufferSize(*self, &instr.instr, imm_override_len);

            let mut buffer: Vec<u8> = vec![0; buffer_size.try_into().unwrap()];
            let disassembled_size = RabbitizerOperandType_disassemble(
                *self,
                &instr.instr,
                buffer.as_mut_ptr() as *mut cty::c_char,
                imm_override_ptr,
                imm_override_len,
            );
            buffer.truncate(disassembled_size.try_into().unwrap());

            String::from_utf8(buffer).unwrap()
        }
    }
}
