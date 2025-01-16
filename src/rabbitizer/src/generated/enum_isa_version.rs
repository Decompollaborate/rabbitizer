/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[cfg_attr(feature = "pyo3", pyclass(module = "rabbitizer", eq))]
pub enum IsaVersion {
    MIPS_I,
    MIPS_II,
    MIPS_III,
    MIPS_IV,
    EXTENSION,
}
