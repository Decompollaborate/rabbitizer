/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use rabbitizer::Opcode;

fn main() {
    for x in Opcode::cpu_lw.get_descriptor().operands_iter() {
        println!("{x:?}");
    }

    println!("{:?}", rabbitizer::registers::Gpr::v0);
    println!();

    println!("{}", rabbitizer::registers::Gpr::try_from(32).unwrap_err());
    println!();

    let instr = rabbitizer::Instruction::new_no_extension(
        0x26F7FFF0,
        0x80000000,
        None,
        rabbitizer::IsaVersion::MIPS_III,
    );
    println!("{}", instr.opcode().name());
    println!("{:?}", instr.reg_rs());
    // println!("0x{:X}", instr.field_immediate_unchecked());

    for operand in instr.opcode().operands_iter() {
        println!("{:?}: {}", operand, operand.display(&instr, None));
    }
}
