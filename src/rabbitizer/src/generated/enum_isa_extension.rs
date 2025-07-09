/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "pyo3", pyclass(module = "rabbitizer", eq))]
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
