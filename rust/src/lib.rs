/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub mod access_type_enum;
pub mod instr_category_enum;
pub mod instr_id_enum;
pub mod operand_type_enum;
pub mod registers_enum;
pub mod instr_suffix_enum;
pub mod instruction;
pub mod instr_descriptor;
pub mod abi_enum;
pub mod config;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            instruction::Instruction::new(0x8D4A7E18, 0x80000000, instr_category_enum::InstrCategory::CPU).disassemble(None, 0),
            "lw          $t2, 0x7E18($t2)".to_string()
        );
    }
}
