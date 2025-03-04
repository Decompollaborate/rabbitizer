/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_ACCESSTYPE(BYTE, min_size: Some(1), min_alignment: Some(1), )
    RAB_DEF_ACCESSTYPE(SHORT, min_size: Some(2), min_alignment: Some(2), )
    RAB_DEF_ACCESSTYPE(WORD, min_size: Some(4), min_alignment: Some(4), )
    RAB_DEF_ACCESSTYPE_ISA(MIPS_III, DOUBLEWORD, min_size: Some(8), min_alignment: Some(8), )
    RAB_DEF_ACCESSTYPE_EXT(R5900EE, QUADWORD, min_size: Some(16), min_alignment: Some(16), )

    RAB_DEF_ACCESSTYPE(FLOAT, min_size: Some(4), min_alignment: Some(4), )
    RAB_DEF_ACCESSTYPE_ISA(MIPS_II, DOUBLEFLOAT, min_size: Some(8), min_alignment: Some(8), )

    RAB_DEF_ACCESSTYPE(WORD_COP2, min_size: Some(4), min_alignment: Some(4), )
    RAB_DEF_ACCESSTYPE_ISA(MIPS_II, DOUBLEWORD_COP2, min_size: Some(8), min_alignment: Some(8), )

    RAB_DEF_ACCESSTYPE(UNALIGNED_WORD, min_size: Some(4), )
    RAB_DEF_ACCESSTYPE_ISA(MIPS_III, UNALIGNED_DOUBLEWORD, min_size: Some(8), )

    RAB_DEF_ACCESSTYPE_ISA(MIPS_II, LINKED_WORD_WORD, min_size: Some(4), min_alignment: Some(4), )
    RAB_DEF_ACCESSTYPE_ISA(MIPS_III, LINKED_WORD_DOUBLEWORD, min_size: Some(8), min_alignment: Some(8), )
