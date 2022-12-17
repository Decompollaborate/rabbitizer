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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
