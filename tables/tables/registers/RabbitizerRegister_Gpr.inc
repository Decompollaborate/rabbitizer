/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_REG(
        zero,  0,
        is_zero: true,
    )
    RABBITIZER_DEF_REG(
        at,    1,
        is_clobbered_by_func_call: true,
        is_assembler_temp: true,
    )
    RABBITIZER_DEF_REG(
        v0,    2,
        is_clobbered_by_func_call: true,
        holds_return_value: true,
    )
    RABBITIZER_DEF_REG(
        v1,    3,
        is_clobbered_by_func_call: true,
        holds_return_value: true,
    )
    RABBITIZER_DEF_REG(
        a0,    4,
        is_clobbered_by_func_call: true,
        is_arg: true,
    )
    RABBITIZER_DEF_REG(
        a1,    5,
        is_clobbered_by_func_call: true,
        is_arg: true,
    )
    RABBITIZER_DEF_REG(
        a2,    6,
        is_clobbered_by_func_call: true,
        is_arg: true,
    )
    RABBITIZER_DEF_REG(
        a3,    7,
        is_clobbered_by_func_call: true,
        is_arg: true,
    )
    RABBITIZER_DEF_REG(
        t0,    8,
        name_n32: Some("$a4"),
        name_n64: Some("$a4"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
    )
    RABBITIZER_DEF_REG(
        t1,    9,
        name_n32: Some("$a5"),
        name_n64: Some("$a5"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
    )
    RABBITIZER_DEF_REG(
        t2,   10,
        name_n32: Some("$a6"),
        name_n64: Some("$a6"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
    )
    RABBITIZER_DEF_REG(
        t3,   11,
        name_n32: Some("$a7"),
        name_n64: Some("$a7"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
    )
    RABBITIZER_DEF_REG(
        t4,   12,
        is_clobbered_by_func_call: true,
        is_temp: true,
    )
    RABBITIZER_DEF_REG(
        t5,   13,
        is_clobbered_by_func_call: true,
        is_temp: true,
    )
    RABBITIZER_DEF_REG(
        t6,   14,
        is_clobbered_by_func_call: true,
        is_temp: true,
    )
    RABBITIZER_DEF_REG(
        t7,   15,
        is_clobbered_by_func_call: true,
        is_temp: true,
    )
    RABBITIZER_DEF_REG(
        s0,   16,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        s1,   17,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        s2,   18,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        s3,   19,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        s4,   20,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        s5,   21,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        s6,   22,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        s7,   23,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        t8,   24,
        is_clobbered_by_func_call: true,
        is_temp: true,
    )
    RABBITIZER_DEF_REG(
        t9,   25,
        is_clobbered_by_func_call: true,
        is_temp: true,
    )
    RABBITIZER_DEF_REG(
        k0,   26,
        is_kernel: true,
    )
    RABBITIZER_DEF_REG(
        k1,   27,
        is_kernel: true,
    )
    RABBITIZER_DEF_REG(
        gp,   28,
        is_global_pointer: true,
    )
    RABBITIZER_DEF_REG(
        sp,   29,
        is_stack_pointer: true,
    )
    RABBITIZER_DEF_REG(
        s8,   30,
        is_saved: true,
    )
    RABBITIZER_DEF_REG(
        ra,   31,
        is_clobbered_by_func_call: true,
        holds_return_address: true,
    )
