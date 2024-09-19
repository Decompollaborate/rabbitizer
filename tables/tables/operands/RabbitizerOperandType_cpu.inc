/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPERAND(cpu, rs, EncodedFieldMask::rs)
    RAB_DEF_OPERAND(cpu, rt, EncodedFieldMask::rt)
    RAB_DEF_OPERAND(cpu, rd, EncodedFieldMask::rd)
    RAB_DEF_OPERAND(cpu, sa, EncodedFieldMask::sa)
    RAB_DEF_OPERAND(cpu, zero, EncodedFieldMask::empty()) // $zero
    // RAB_DEF_OPERAND(cpu, function, EncodedFieldMask::function)
    RAB_DEF_OPERAND(cpu, cop0d, EncodedFieldMask::cop0d)
    RAB_DEF_OPERAND(cpu, fs, EncodedFieldMask::fs)
    RAB_DEF_OPERAND(cpu, ft, EncodedFieldMask::ft)
    RAB_DEF_OPERAND(cpu, fd, EncodedFieldMask::fd)
    RAB_DEF_OPERAND(cpu, cop1cs, EncodedFieldMask::cop1cs) // Coprocessor 1 control fs
    RAB_DEF_OPERAND(cpu, cop2t, EncodedFieldMask::cop2t)
    RAB_DEF_OPERAND(cpu, cop2cd, EncodedFieldMask::cop2cd) // Coprocessor 2 control rd
    RAB_DEF_OPERAND(cpu, op, EncodedFieldMask::op)
    RAB_DEF_OPERAND(cpu, hint, EncodedFieldMask::hint)
    RAB_DEF_OPERAND(cpu, code, EncodedFieldMask::code)
    RAB_DEF_OPERAND(cpu, code_lower, EncodedFieldMask::code_lower)
    RAB_DEF_OPERAND(cpu, copraw, EncodedFieldMask::copraw)
    RAB_DEF_OPERAND(cpu, label, EncodedFieldMask::instr_index)
    RAB_DEF_OPERAND(cpu, immediate, EncodedFieldMask::immediate)
    RAB_DEF_OPERAND(cpu, branch_target_label, EncodedFieldMask::immediate)

    // composed/aliased operands
    RAB_DEF_OPERAND(cpu, immediate_base, EncodedFieldMask::immediate.union(EncodedFieldMask::rs))
    RAB_DEF_OPERAND(cpu, maybe_rd_rs, EncodedFieldMask::rd.union(EncodedFieldMask::rs)) // $rd is omitted if it is $ra
