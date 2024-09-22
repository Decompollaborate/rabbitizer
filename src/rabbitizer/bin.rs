/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use rabbitizer::Opcode;

fn main() {
    for x in Opcode::cpu_lw.get_descriptor().operands_iter() {
        println!("{x:?}");
    }

    println!("{:?}", rabbitizer::registers::Gpr::v0);
}
