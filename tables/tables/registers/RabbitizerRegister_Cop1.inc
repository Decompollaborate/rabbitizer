/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_REG(
        COP1, fv0,   0,
        name_n32: Some("$fv0"),
        name_n64: Some("$fv0"),
        name_eabi32: Some("$fv0"),
        name_eabi64: Some("$fv0"),
    )
    RABBITIZER_DEF_REG(
        COP1, fv0f,  1,
        name_n32: Some("$ft14"),
        name_n64: Some("$ft12"),
        name_eabi32: Some("$fv0f"),
        name_eabi64: Some("$fv1"),
    )
    RABBITIZER_DEF_REG(
        COP1, fv1,   2,
        name_n32: Some("$fv1"),
        name_n64: Some("$fv1"),
        name_eabi32: Some("$fv1"),
        name_eabi64: Some("$ft0"),
    )
    RABBITIZER_DEF_REG(
        COP1, fv1f,  3,
        name_n32: Some("$ft15"),
        name_n64: Some("$ft13"),
        name_eabi32: Some("$fv1f"),
        name_eabi64: Some("$ft1"),
    )
    RABBITIZER_DEF_REG(
        COP1, ft0,   4,
        name_n32: Some("$ft0"),
        name_n64: Some("$ft0"),
        name_eabi32: Some("$ft0"),
        name_eabi64: Some("$ft2"),
    )
    RABBITIZER_DEF_REG(
        COP1, ft0f,  5,
        name_n32: Some("$ft1"),
        name_n64: Some("$ft1"),
        name_eabi32: Some("$ft0f"),
        name_eabi64: Some("$ft3"),
    )
    RABBITIZER_DEF_REG(
        COP1, ft1,   6,
        name_n32: Some("$ft2"),
        name_n64: Some("$ft2"),
        name_eabi32: Some("$ft1"),
        name_eabi64: Some("$ft4"),
    )
    RABBITIZER_DEF_REG(
        COP1, ft1f,  7,
        name_n32: Some("$ft3"),
        name_n64: Some("$ft3"),
        name_eabi32: Some("$ft1f"),
        name_eabi64: Some("$ft5"),
    )
    RABBITIZER_DEF_REG(
        COP1, ft2,   8,
        name_n32: Some("$ft4"),
        name_n64: Some("$ft4"),
        name_eabi32: Some("$ft2"),
        name_eabi64: Some("$ft6"),
    )
    RABBITIZER_DEF_REG(
        COP1, ft2f,  9,
        name_n32: Some("$ft5"),
        name_n64: Some("$ft5"),
        name_eabi32: Some("$ft2f"),
        name_eabi64: Some("$ft7"),
    )
    RABBITIZER_DEF_REG(
        COP1, ft3,  10,
        name_n32: Some("$ft6"),
        name_n64: Some("$ft6"),
        name_eabi32: Some("$ft3"),
        name_eabi64: Some("$ft8"),
    )
    RABBITIZER_DEF_REG(
        COP1, ft3f, 11,
        name_n32: Some("$ft7"),
        name_n64: Some("$ft7"),
        name_eabi32: Some("$ft3f"),
        name_eabi64: Some("$ft9"),
    )
    RABBITIZER_DEF_REG(
        COP1, fa0,  12,
        name_n32: Some("$fa0"),
        name_n64: Some("$fa0"),
        name_eabi32: Some("$fa0"),
        name_eabi64: Some("$fa0"),
    )
    RABBITIZER_DEF_REG(
        COP1, fa0f, 13,
        name_n32: Some("$fa1"),
        name_n64: Some("$fa1"),
        name_eabi32: Some("$fa0f"),
        name_eabi64: Some("$fa1"),
    )
    RABBITIZER_DEF_REG(
        COP1, fa1,  14,
        name_n32: Some("$fa2"),
        name_n64: Some("$fa2"),
        name_eabi32: Some("$fa1"),
        name_eabi64: Some("$fa2"),
    )
    RABBITIZER_DEF_REG(
        COP1, fa1f, 15,
        name_n32: Some("$fa3"),
        name_n64: Some("$fa3"),
        name_eabi32: Some("$fa1f"),
        name_eabi64: Some("$fa3"),
    )
    RABBITIZER_DEF_REG(
        COP1, ft4,  16,
        name_n32: Some("$fa4"),
        name_n64: Some("$fa4"),
        name_eabi32: Some("$fa2"),
        name_eabi64: Some("$fa4"),
    )
    RABBITIZER_DEF_REG(
        COP1, ft4f, 17,
        name_n32: Some("$fa5"),
        name_n64: Some("$fa5"),
        name_eabi32: Some("$fa2f"),
        name_eabi64: Some("$fa5"),
    )
    RABBITIZER_DEF_REG(
        COP1, ft5,  18,
        name_n32: Some("$fa6"),
        name_n64: Some("$fa6"),
        name_eabi32: Some("$fa3"),
        name_eabi64: Some("$fa6"),
    )
    RABBITIZER_DEF_REG(
        COP1, ft5f, 19,
        name_n32: Some("$fa7"),
        name_n64: Some("$fa7"),
        name_eabi32: Some("$fa3f"),
        name_eabi64: Some("$fa7"),
    )
    RABBITIZER_DEF_REG(
        COP1, fs0,  20,
        name_n32: Some("$fs0"),
        name_n64: Some("$ft8"),
        name_eabi32: Some("$fs0"),
        name_eabi64: Some("$fs0"),
    )
    RABBITIZER_DEF_REG(
        COP1, fs0f, 21,
        name_n32: Some("$ft8"),
        name_n64: Some("$ft9"),
        name_eabi32: Some("$fs0f"),
        name_eabi64: Some("$fs1"),
    )
    RABBITIZER_DEF_REG(
        COP1, fs1,  22,
        name_n32: Some("$fs1"),
        name_n64: Some("$ft10"),
        name_eabi32: Some("$fs1"),
        name_eabi64: Some("$fs2"),
    )
    RABBITIZER_DEF_REG(
        COP1, fs1f, 23,
        name_n32: Some("$ft9"),
        name_n64: Some("$ft11"),
        name_eabi32: Some("$fs1f"),
        name_eabi64: Some("$fs3"),
    )
    RABBITIZER_DEF_REG(
        COP1, fs2,  24,
        name_n32: Some("$fs2"),
        name_n64: Some("$fs0"),
        name_eabi32: Some("$fs2"),
        name_eabi64: Some("$fs4"),
    )
    RABBITIZER_DEF_REG(
        COP1, fs2f, 25,
        name_n32: Some("$ft10"),
        name_n64: Some("$fs1"),
        name_eabi32: Some("$fs2f"),
        name_eabi64: Some("$fs5"),
    )
    RABBITIZER_DEF_REG(
        COP1, fs3,  26,
        name_n32: Some("$fs3"),
        name_n64: Some("$fs2"),
        name_eabi32: Some("$fs3"),
        name_eabi64: Some("$fs6"),
    )
    RABBITIZER_DEF_REG(
        COP1, fs3f, 27,
        name_n32: Some("$ft11"),
        name_n64: Some("$fs3"),
        name_eabi32: Some("$fs3f"),
        name_eabi64: Some("$fs7"),
    )
    RABBITIZER_DEF_REG(
        COP1, fs4,  28,
        name_n32: Some("$fs4"),
        name_n64: Some("$fs4"),
        name_eabi32: Some("$fs4"),
        name_eabi64: Some("$fs8"),
    )
    RABBITIZER_DEF_REG(
        COP1, fs4f, 29,
        name_n32: Some("$ft12"),
        name_n64: Some("$fs5"),
        name_eabi32: Some("$fs4f"),
        name_eabi64: Some("$fs9"),
    )
    RABBITIZER_DEF_REG(
        COP1, fs5,  30,
        name_n32: Some("$fs5"),
        name_n64: Some("$fs6"),
        name_eabi32: Some("$fs5"),
        name_eabi64: Some("$fs10"),
    )
    RABBITIZER_DEF_REG(
        COP1, fs5f, 31,
        name_n32: Some("$ft13"),
        name_n64: Some("$fs7"),
        name_eabi32: Some("$fs5f"),
        name_eabi64: Some("$fs11"),
    )
