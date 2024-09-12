/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// #[deprecated(since="1.3.0")]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum InstrType {
    UNKNOWN,
    J,
    I,
    R,
    REGIMM,
}

#[allow(deprecated)]
impl InstrType {
    pub const fn default() -> Self {
        Self::UNKNOWN
    }
}

#[allow(deprecated)]
impl Default for InstrType {
    fn default() -> Self {
        Self::UNKNOWN
    }
}