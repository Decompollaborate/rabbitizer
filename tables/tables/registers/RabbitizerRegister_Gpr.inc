/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_REG(
        GPR, zero,  0,
        name_numeric: "$zero",
        is_zero: true,
    )
    RABBITIZER_DEF_REG(
        GPR, at,    1,
        is_clobbered_by_func_call: true,
        is_assembler_temp: true,
    )
    RABBITIZER_DEF_REG(
        GPR, v0,    2,
        is_clobbered_by_func_call: true,
        holds_return_value: true,
    )
    RABBITIZER_DEF_REG(
        GPR, v1,    3,
        is_clobbered_by_func_call: true,
        holds_return_value: true,
    )
    RABBITIZER_DEF_REG(
        GPR, a0,    4,
        is_clobbered_by_func_call: true,
        is_arg: true,
    )
    RABBITIZER_DEF_REG(
        GPR, a1,    5,
        is_clobbered_by_func_call: true,
        is_arg: true,
    )
    RABBITIZER_DEF_REG(
        GPR, a2,    6,
        is_clobbered_by_func_call: true,
        is_arg: true,
    )
    RABBITIZER_DEF_REG(
        GPR, a3,    7,
        is_clobbered_by_func_call: true,
        is_arg: true,
    )
    RABBITIZER_DEF_REG(
        GPR, t0,    8,
        name_n32: Some("$a4"),
        name_n64: Some("$a4"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
    )
    RABBITIZER_DEF_REG(
        GPR, t1,    9,
        name_n32: Some("$a5"),
        name_n64: Some("$a5"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
    )
    RABBITIZER_DEF_REG(
        GPR, t2,   10,
        name_n32: Some("$a6"),
        name_n64: Some("$a6"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
    )
    RABBITIZER_DEF_REG(
        GPR, t3,   11,
        name_n32: Some("$a7"),
        name_n64: Some("$a7"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
    )
    RABBITIZER_DEF_REG(
        GPR, t4,   12,
        is_clobbered_by_func_call: true,
        is_temp: true,
    )
    RABBITIZER_DEF_REG(
        GPR, t5,   13,
        is_clobbered_by_func_call: true,
        is_temp: true,
    )
    RABBITIZER_DEF_REG(
        GPR, t6,   14,
        is_clobbered_by_func_call: true,
        is_temp: true,
    )
    RABBITIZER_DEF_REG(
        GPR, t7,   15,
        is_clobbered_by_func_call: true,
        is_temp: true,
    )
    RABBITIZER_DEF_REG(
        GPR, s0,   16,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        GPR, s1,   17,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        GPR, s2,   18,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        GPR, s3,   19,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        GPR, s4,   20,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        GPR, s5,   21,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        GPR, s6,   22,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        GPR, s7,   23,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        GPR, t8,   24,
        is_clobbered_by_func_call: true,
        is_temp: true,
    )
    RABBITIZER_DEF_REG(
        GPR, t9,   25,
        is_clobbered_by_func_call: true,
        is_temp: true,
    )
    RABBITIZER_DEF_REG(
        GPR, k0,   26,
        is_kernel: true,
    )
    RABBITIZER_DEF_REG(
        GPR, k1,   27,
        is_kernel: true,
    )
    RABBITIZER_DEF_REG(
        GPR, gp,   28,
        is_global_pointer: true,
    )
    RABBITIZER_DEF_REG(
        GPR, sp,   29,
        is_stack_pointer: true,
    )
    RABBITIZER_DEF_REG(
        GPR, s8,   30,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        GPR, ra,   31,
        name_numeric: "$ra",
        is_clobbered_by_func_call: true,
        holds_return_address: true,
    )
