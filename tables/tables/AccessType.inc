/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_ACCESSTYPE(NONE, )

    RAB_DEF_ACCESSTYPE(BYTE, min_size: 1, min_alignment: 1, )
    RAB_DEF_ACCESSTYPE(SHORT, min_size: 2, min_alignment: 2, )
    RAB_DEF_ACCESSTYPE(WORD, min_size: 4, min_alignment: 4, )
    RAB_DEF_ACCESSTYPE(DOUBLEWORD, min_size: 8, min_alignment: 8, )
    RAB_DEF_ACCESSTYPE(QUADWORD, min_size: 16, min_alignment: 16, )

    RAB_DEF_ACCESSTYPE(FLOAT, min_size: 4, min_alignment: 4, )
    RAB_DEF_ACCESSTYPE(DOUBLEFLOAT, min_size: 8, min_alignment: 8, )

    RAB_DEF_ACCESSTYPE(WORD_LEFT, min_size: 4, )
    RAB_DEF_ACCESSTYPE(WORD_RIGHT, min_size: 4, )
    RAB_DEF_ACCESSTYPE(DOUBLEWORD_LEFT, min_size: 8, )
    RAB_DEF_ACCESSTYPE(DOUBLEWORD_RIGHT, min_size: 8, )
