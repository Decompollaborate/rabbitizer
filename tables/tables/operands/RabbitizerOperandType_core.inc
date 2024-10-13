/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPERAND(core, rs, EncodedFieldMask::rs, (Gpr))
    RAB_DEF_OPERAND(core, rt, EncodedFieldMask::rt, (Gpr))
    RAB_DEF_OPERAND(core, rd, EncodedFieldMask::rd, (Gpr))
    RAB_DEF_OPERAND(core, sa, EncodedFieldMask::sa, (u8))
    RAB_DEF_OPERAND(core, zero, EncodedFieldMask::empty(), ()) // $zero
    // RAB_DEF_OPERAND(core, function, EncodedFieldMask::function, ())
    RAB_DEF_OPERAND(core, cop0d, EncodedFieldMask::cop0d, (Cop0))
    RAB_DEF_OPERAND(core, cop0cd, EncodedFieldMask::cop0cd, (Cop0Control)) // Coprocessor 0 control rd
    RAB_DEF_OPERAND(core, fs, EncodedFieldMask::fs, (Cop1))
    RAB_DEF_OPERAND(core, ft, EncodedFieldMask::ft, (Cop1))
    RAB_DEF_OPERAND(core, fd, EncodedFieldMask::fd, (Cop1))
    RAB_DEF_OPERAND(core, cop1cs, EncodedFieldMask::cop1cs, (Cop1Control)) // Coprocessor 1 control fs
    RAB_DEF_OPERAND(core, cop2t, EncodedFieldMask::cop2t, (Cop2))
    RAB_DEF_OPERAND(core, cop2d, EncodedFieldMask::cop2d, (Cop2))
    RAB_DEF_OPERAND(core, cop2cd, EncodedFieldMask::cop2cd, (Cop2Control)) // Coprocessor 2 control rd
    RAB_DEF_OPERAND(core, op, EncodedFieldMask::op, (u8)) // TODO: maybe merge with hint?
    RAB_DEF_OPERAND(core, hint, EncodedFieldMask::hint, (u8)) // TODO: maybe merge with op?
    RAB_DEF_OPERAND(core, code, EncodedFieldMask::code, (u16, Option<NonZero<u16>>))
    RAB_DEF_OPERAND(core, code_lower, EncodedFieldMask::code_lower, (u16))
    RAB_DEF_OPERAND(core, copraw, EncodedFieldMask::copraw, (u32)) // TODO: what?
    RAB_DEF_OPERAND(core, label, EncodedFieldMask::instr_index, (Vram))
    RAB_DEF_OPERAND(core, immediate, EncodedFieldMask::immediate, (IU16))
    RAB_DEF_OPERAND(core, branch_target_label, EncodedFieldMask::immediate, (VramOffset))

    // composed/aliased operands
    RAB_DEF_OPERAND(core, immediate_base, EncodedFieldMask::immediate.union(EncodedFieldMask::rs), (IU16, Gpr))
    RAB_DEF_OPERAND(core, maybe_rd_rs, EncodedFieldMask::rd.union(EncodedFieldMask::rs), (Option<Gpr>, Gpr)) // $rd is omitted if it is $ra
    RAB_DEF_OPERAND(core, maybe_zero_rs, EncodedFieldMask::empty().union(EncodedFieldMask::rs), ((), Gpr)) // This operand may have an extra prefixed `$zero, ` depending on the display flags.
