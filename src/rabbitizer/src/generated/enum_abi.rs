/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "pyo3", pyclass(module = "rabbitizer", eq))]
pub enum Abi {
    O32,
    O64,
    N32,
    N64,
    EABI32,
    EABI64,
}
