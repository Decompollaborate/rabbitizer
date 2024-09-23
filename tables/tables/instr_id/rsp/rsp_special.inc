/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP rd, rt, sa
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x00, sll, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Left Logical
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x02, srl, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Right Logical
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x03, sra, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Right Arithmetic

    // OP rd, rt, rs
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x04, sllv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Left Logical Variable
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x06, srlv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Right Logical Variable
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x07, srav, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Right Arithmetic Variable

    // OP rs
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x08, jr, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr1(Operand::rsp_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        is_jump: true
    ) // Jump Register
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x09, jalr, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr1(Operand::rsp_maybe_rd_rs),
        instr_type: InstrType::R,
        is_jump: true,
        modifies_rd: true,
        reads_rs: true,
        does_link: true
    ) // Jump And Link Register

    // OP rd, rs, rt
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0A, movz, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MOVe conditional on Zero
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0B, movn, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MOVe conditional on Not zero

    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x20, add, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // ADD word
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x21, addu, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // ADD Unsigned word
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x22, sub, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        not_emitted_by_compilers: true,
        reads_rs: true,
        reads_rt: true
    ) // Subtract word
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x23, subu, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // SUBtract Unsigned word
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x24, and, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // AND
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x25, or, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // OR
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x26, xor, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // eXclusive OR
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x27, nor, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Not OR
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x2A, slt, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Set on Less Than
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x2B, sltu, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Set on Less Than Unsigned

    // OP code
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0D, break, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr1(Operand::cpu_code),
        instr_type: InstrType::R
    ) // Break


    // Pseudo-Instruction Unique IDs
    // OP
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0xFFFFFF01, nop, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        is_pseudo: true
    ) // No OPeration

    // OP rd, rs
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0xFFFFFF25, move, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        maybe_is_move: true,
        is_pseudo: true
    ) // Move
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0xFFFFFF27, not, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        is_pseudo: true
    ) // Not

    // OP rd, rt
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0xFFFFFF22, neg, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rt),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true
    )

    // OP rd, rt
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0xFFFFFF23, negu, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true
    )
