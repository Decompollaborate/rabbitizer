/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP rd, rt, sa
    RAB_DEF_OPCODE(
        rsp, 0x00, sll, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Left Logical
    RAB_DEF_OPCODE(
        rsp, 0x02, srl, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Right Logical
    RAB_DEF_OPCODE(
        rsp, 0x03, sra, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Right Arithmetic

    // OP rd, rt, rs
    RAB_DEF_OPCODE(
        rsp, 0x04, sllv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Left Logical Variable
    RAB_DEF_OPCODE(
        rsp, 0x06, srlv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Right Logical Variable
    RAB_DEF_OPCODE(
        rsp, 0x07, srav, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Right Arithmetic Variable

    // OP rs
    RAB_DEF_OPCODE(
        rsp, 0x08, jr, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr1(Operand::rsp_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        is_jump: true
    ) // Jump Register
    RAB_DEF_OPCODE(
        rsp, 0x09, jalr, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr1(Operand::rsp_maybe_rd_rs),
        instr_type: InstrType::R,
        is_jump: true,
        modifies_rd: true,
        reads_rs: true,
        does_link: true
    ) // Jump And Link Register

    // OP rd, rs, rt
    RAB_DEF_OPCODE(
        rsp, 0x0A, movz, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MOVe conditional on Zero
    RAB_DEF_OPCODE(
        rsp, 0x0B, movn, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MOVe conditional on Not zero

    RAB_DEF_OPCODE(
        rsp, 0x20, add, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // ADD word
    RAB_DEF_OPCODE(
        rsp, 0x21, addu, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // ADD Unsigned word
    RAB_DEF_OPCODE(
        rsp, 0x22, sub, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        not_emitted_by_compilers: true,
        reads_rs: true,
        reads_rt: true
    ) // Subtract word
    RAB_DEF_OPCODE(
        rsp, 0x23, subu, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // SUBtract Unsigned word
    RAB_DEF_OPCODE(
        rsp, 0x24, and, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // AND
    RAB_DEF_OPCODE(
        rsp, 0x25, or, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // OR
    RAB_DEF_OPCODE(
        rsp, 0x26, xor, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // eXclusive OR
    RAB_DEF_OPCODE(
        rsp, 0x27, nor, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Not OR
    RAB_DEF_OPCODE(
        rsp, 0x2A, slt, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Set on Less Than
    RAB_DEF_OPCODE(
        rsp, 0x2B, sltu, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Set on Less Than Unsigned

    // OP code
    RAB_DEF_OPCODE(
        rsp, 0x0D, break, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr1(Operand::core_code),
        instr_type: InstrType::R
    ) // Break


    // Pseudo-Instruction Unique IDs
    // OP
    RAB_DEF_OPCODE(
        rsp, 0xFFFFFF01, nop, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        is_pseudo: true
    ) // No OPeration

    // OP rd, rs
    RAB_DEF_OPCODE(
        rsp, 0xFFFFFF25, move, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        maybe_is_move: true,
        is_pseudo: true
    ) // Move
    RAB_DEF_OPCODE(
        rsp, 0xFFFFFF27, not, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        is_pseudo: true
    ) // Not

    // OP rd, rt
    RAB_DEF_OPCODE(
        rsp, 0xFFFFFF22, neg, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rt),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true
    )

    // OP rd, rt
    RAB_DEF_OPCODE(
        rsp, 0xFFFFFF23, negu, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true
    )
