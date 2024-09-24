/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::Gpr;
use crate::traits::Register;
use crate::Abi;
use crate::RegisterDescriptor;

impl Gpr {
    #[must_use]
    pub const fn default() -> Self {
        Self::zero
    }
}

impl Register for Gpr {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::GPR[*self]
    }

    #[must_use]
    fn is_temp(&self, abi: Abi) -> bool {
        let descriptor = self.get_descriptor();

        if descriptor.either_arg_or_temp() {
            match abi {
                Abi::NUMERIC | Abi::O32 => descriptor.is_temp(),
                Abi::N32 | Abi::N64 => false,
            }
        } else {
            descriptor.is_temp()
        }
    }
    #[must_use]
    fn is_arg(&self, abi: Abi) -> bool {
        let descriptor = self.get_descriptor();

        if descriptor.either_arg_or_temp() {
            match abi {
                Abi::NUMERIC | Abi::O32 => false,
                Abi::N32 | Abi::N64 => descriptor.is_arg(),
            }
        } else {
            descriptor.is_arg()
        }
    }
}

impl Default for Gpr {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<Gpr> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: Gpr) -> &Self::Output {
        &self[index as usize]
    }
}

impl Gpr {
    pub const fn try_from_u32(value: u32) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::zero),
            1 => Ok(Self::at),
            2 => Ok(Self::v0),
            3 => Ok(Self::v1),
            4 => Ok(Self::a0),
            5 => Ok(Self::a1),
            6 => Ok(Self::a2),
            7 => Ok(Self::a3),
            8 => Ok(Self::t0),
            9 => Ok(Self::t1),
            10 => Ok(Self::t2),
            11 => Ok(Self::t3),
            12 => Ok(Self::t4),
            13 => Ok(Self::t5),
            14 => Ok(Self::t6),
            15 => Ok(Self::t7),
            16 => Ok(Self::s0),
            17 => Ok(Self::s1),
            18 => Ok(Self::s2),
            19 => Ok(Self::s3),
            20 => Ok(Self::s4),
            21 => Ok(Self::s5),
            22 => Ok(Self::s6),
            23 => Ok(Self::s7),
            24 => Ok(Self::t8),
            25 => Ok(Self::t9),
            26 => Ok(Self::k0),
            27 => Ok(Self::k1),
            28 => Ok(Self::gp),
            29 => Ok(Self::sp),
            30 => Ok(Self::s8),
            31 => Ok(Self::ra),
            _ => Err(()),
        }
    }
}

impl TryFrom<u32> for Gpr {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
