/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
use rabbitizer::Opcode;

fn main() {
    for x in Opcode::core_lw.get_descriptor().operands_iter() {
        println!("{x:?}");
    }

    println!("{:?}", rabbitizer::registers::Gpr::v0);
    println!();

    println!("{}", rabbitizer::registers::Gpr::try_from(32).unwrap_err());
    println!();

    let instr = rabbitizer::Instruction::new_no_extension(
        0x26F7FFF0,
        0x80000000,
        rabbitizer::InstructionFlags::default(),
        rabbitizer::IsaVersion::MIPS_III,
    );
    println!("{}", instr.opcode().name());
    println!("{:?}", instr.field_rs());
    // println!("0x{:X}", instr.field_immediate_unchecked());

    let display_flags = rabbitizer::DisplayFlags::default();

    for operand in instr.opcode().operands_iter() {
        println!(
            "{:?}: {}",
            operand,
            operand.display(&instr, None, &display_flags)
        );
    }

    println!("{}", instr.display(None, &display_flags));
}
*/

fn main() {
    let instr = rabbitizer::instr::Instruction::new_r4000allegrex(
        0xD0119001,
        rabbitizer::Vram::new(0x80000000),
        rabbitizer::instr::InstructionFlags::default(),
    );

    println!(
        "{}",
        instr.display(None, &rabbitizer::display_flags::DisplayFlags::default())
    );

    println!("{:?}", instr);
}
