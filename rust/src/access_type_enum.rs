/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum AccessType {
    INVALID,
    BYTE,
    SHORT,
    WORD,
    DOUBLEWORD,
    QUADWORD,
    FLOAT,
    DOUBLEFLOAT,
    WORD_LEFT,
    WORD_RIGHT,
    DOUBLEWORD_LEFT,
    DOUBLEWORD_RIGHT,
    MAX,
}
