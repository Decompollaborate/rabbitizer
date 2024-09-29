/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPERAND(core, rs, EncodedFieldMask::rs)
    RAB_DEF_OPERAND(core, rt, EncodedFieldMask::rt)
    RAB_DEF_OPERAND(core, rd, EncodedFieldMask::rd)
    RAB_DEF_OPERAND(core, sa, EncodedFieldMask::sa)
    RAB_DEF_OPERAND(core, zero, EncodedFieldMask::empty()) // $zero
    // RAB_DEF_OPERAND(core, function, EncodedFieldMask::function)
    RAB_DEF_OPERAND(core, cop0d, EncodedFieldMask::cop0d)
    RAB_DEF_OPERAND(core, fs, EncodedFieldMask::fs)
    RAB_DEF_OPERAND(core, ft, EncodedFieldMask::ft)
    RAB_DEF_OPERAND(core, fd, EncodedFieldMask::fd)
    RAB_DEF_OPERAND(core, cop1cs, EncodedFieldMask::cop1cs) // Coprocessor 1 control fs
    RAB_DEF_OPERAND(core, cop2t, EncodedFieldMask::cop2t)
    RAB_DEF_OPERAND(core, cop2cd, EncodedFieldMask::cop2cd) // Coprocessor 2 control rd
    RAB_DEF_OPERAND(core, op, EncodedFieldMask::op)
    RAB_DEF_OPERAND(core, hint, EncodedFieldMask::hint)
    RAB_DEF_OPERAND(core, code, EncodedFieldMask::code)
    RAB_DEF_OPERAND(core, code_lower, EncodedFieldMask::code_lower)
    RAB_DEF_OPERAND(core, copraw, EncodedFieldMask::copraw)
    RAB_DEF_OPERAND(core, label, EncodedFieldMask::instr_index)
    RAB_DEF_OPERAND(core, immediate, EncodedFieldMask::immediate)
    RAB_DEF_OPERAND(core, branch_target_label, EncodedFieldMask::immediate)

    // composed/aliased operands
    RAB_DEF_OPERAND(core, immediate_base, EncodedFieldMask::immediate.union(EncodedFieldMask::rs))
    RAB_DEF_OPERAND(core, maybe_rd_rs, EncodedFieldMask::rd.union(EncodedFieldMask::rs)) // $rd is omitted if it is $ra
