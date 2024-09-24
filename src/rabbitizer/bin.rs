/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use rabbitizer::Opcode;

fn main() {
    for x in Opcode::cpu_lw.get_descriptor().operands_iter() {
        println!("{x:?}");
    }

    println!("{:?}", rabbitizer::registers::Gpr::v0);

    let instr = rabbitizer::Instruction::new_no_extension(
        0x26F70018,
        0x80000000,
        None,
        rabbitizer::IsaVersion::MIPS_III,
    );
    println!("{:?}", instr.reg_rs());

    println!("{}", rabbitizer::registers::Gpr::try_from(32).unwrap_err());
}
