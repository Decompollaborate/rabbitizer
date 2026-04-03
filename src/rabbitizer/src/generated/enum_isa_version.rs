/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "pyo3", pyclass(module = "rabbitizer", eq, from_py_object))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IsaVersion {
    MIPS_I,
    #[cfg(feature = "MIPS_II")]
    MIPS_II,
    #[cfg(feature = "MIPS_III")]
    MIPS_III,
    #[cfg(feature = "MIPS_IV")]
    MIPS_IV,
    #[doc(hidden)]
    EXTENSION,
}
