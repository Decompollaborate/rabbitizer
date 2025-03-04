/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub use crate::generated::AccessType;
use crate::{access_type_descriptor::AccessTypeDescriptor, generated::ACCESS_TYPES};

pub const ACCESS_TYPE_COUNT: usize = {
    let mut count = 6;

    if cfg!(feature = "MIPS_II") {
        count += 3;
    }
    if cfg!(feature = "MIPS_III") {
        count += 3;
    }
    if cfg!(feature = "MIPS_IV") {
        count += 0;
    }

    if cfg!(feature = "RSP") {
        count += 0;
    }
    if cfg!(feature = "R3000GTE") {
        count += 0;
    }
    if cfg!(feature = "R4000ALLEGREX") {
        count += 0;
    }
    if cfg!(feature = "R5900EE") {
        count += 1;
    }

    if cfg!(feature = "RspViceMsp") {
        count += 0;
    }

    count
};

impl AccessType {
    #[must_use]
    pub fn get_descriptor(&self) -> &'static AccessTypeDescriptor {
        &ACCESS_TYPES[*self as usize]
    }
}

impl AccessType {
    /// The name of this access type.
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.get_descriptor().name()
    }

    /// The minimal size a symbol should have for this access type.
    ///
    /// For example, a single precision float access type does reference a symbol that is at least
    /// 0x4 bytes big.
    /// The actual symbol may be larger since it could be an struct or an array too.
    #[must_use]
    pub fn min_size(&self) -> Option<u8> {
        self.get_descriptor().min_size()
    }

    /// The minimal alignment a symbol should have for this access type.
    ///
    /// For example, a single precision float access does reference a symbol that is aligned at
    /// least the 0x4 byte boundary.
    /// The actual symbol may be have an stricter alignment since it could be part of an struct.
    #[must_use]
    pub fn min_alignment(&self) -> Option<u8> {
        self.get_descriptor().min_alignment()
    }
}
