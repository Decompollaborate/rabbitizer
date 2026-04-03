/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "pyo3", pyclass(module = "rabbitizer", eq, from_py_object))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IsaExtension {
    #[cfg(feature = "RSP")]
    RSP,
    #[cfg(feature = "R3000GTE")]
    R3000GTE,
    #[cfg(feature = "R4000ALLEGREX")]
    R4000ALLEGREX,
    #[cfg(feature = "R5900EE")]
    R5900EE,
}
