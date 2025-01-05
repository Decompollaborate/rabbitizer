/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_ACCESSTYPE(BYTE, min_size: Some(1), min_alignment: Some(1), )
    RAB_DEF_ACCESSTYPE(SHORT, min_size: Some(2), min_alignment: Some(2), )
    RAB_DEF_ACCESSTYPE(WORD, min_size: Some(4), min_alignment: Some(4), )
    RAB_DEF_ACCESSTYPE(DOUBLEWORD, min_size: Some(8), min_alignment: Some(8), )
    RAB_DEF_ACCESSTYPE(QUADWORD, min_size: Some(16), min_alignment: Some(16), )

    RAB_DEF_ACCESSTYPE(FLOAT, min_size: Some(4), min_alignment: Some(4), )
    RAB_DEF_ACCESSTYPE(DOUBLEFLOAT, min_size: Some(8), min_alignment: Some(8), )

    RAB_DEF_ACCESSTYPE(WORD_LEFT, min_size: Some(4), )
    RAB_DEF_ACCESSTYPE(WORD_RIGHT, min_size: Some(4), )
    RAB_DEF_ACCESSTYPE(DOUBLEWORD_LEFT, min_size: Some(8), )
    RAB_DEF_ACCESSTYPE(DOUBLEWORD_RIGHT, min_size: Some(8), )
