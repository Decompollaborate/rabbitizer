/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use crate::registers_meta::IntRegisterConversionError;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R5900EEVI {
    vi0 = 0,
    vi1 = 1,
    vi2 = 2,
    vi3 = 3,
    vi4 = 4,
    vi5 = 5,
    vi6 = 6,
    vi7 = 7,
    vi8 = 8,
    vi9 = 9,
    vi10 = 10,
    vi11 = 11,
    vi12 = 12,
    vi13 = 13,
    vi14 = 14,
    vi15 = 15,
    vi16 = 16,
    vi17 = 17,
    vi18 = 18,
    vi19 = 19,
    vi20 = 20,
    vi21 = 21,
    vi22 = 22,
    vi23 = 23,
    vi24 = 24,
    vi25 = 25,
    vi26 = 26,
    vi27 = 27,
    vi28 = 28,
    vi29 = 29,
    vi30 = 30,
    vi31 = 31,
}
pub static R5900EE_VI: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[R5900EEVI::vi0 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi0"), 0, concat!("$vi", "0"), true)
    };
    table[R5900EEVI::vi1 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi1"), 1, concat!("$vi", "1"), true)
    };
    table[R5900EEVI::vi2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi2"), 2, concat!("$vi", "2"), true)
    };
    table[R5900EEVI::vi3 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi3"), 3, concat!("$vi", "3"), true)
    };
    table[R5900EEVI::vi4 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi4"), 4, concat!("$vi", "4"), true)
    };
    table[R5900EEVI::vi5 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi5"), 5, concat!("$vi", "5"), true)
    };
    table[R5900EEVI::vi6 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi6"), 6, concat!("$vi", "6"), true)
    };
    table[R5900EEVI::vi7 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi7"), 7, concat!("$vi", "7"), true)
    };
    table[R5900EEVI::vi8 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi8"), 8, concat!("$vi", "8"), true)
    };
    table[R5900EEVI::vi9 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi9"), 9, concat!("$vi", "9"), true)
    };
    table[R5900EEVI::vi10 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi10"), 10, concat!("$vi", "10"), true)
    };
    table[R5900EEVI::vi11 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi11"), 11, concat!("$vi", "11"), true)
    };
    table[R5900EEVI::vi12 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi12"), 12, concat!("$vi", "12"), true)
    };
    table[R5900EEVI::vi13 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi13"), 13, concat!("$vi", "13"), true)
    };
    table[R5900EEVI::vi14 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi14"), 14, concat!("$vi", "14"), true)
    };
    table[R5900EEVI::vi15 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi15"), 15, concat!("$vi", "15"), true)
    };
    table[R5900EEVI::vi16 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi16"), 16, concat!("$vi", "16"), true)
    };
    table[R5900EEVI::vi17 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi17"), 17, concat!("$vi", "17"), true)
    };
    table[R5900EEVI::vi18 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi18"), 18, concat!("$vi", "18"), true)
    };
    table[R5900EEVI::vi19 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi19"), 19, concat!("$vi", "19"), true)
    };
    table[R5900EEVI::vi20 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi20"), 20, concat!("$vi", "20"), true)
    };
    table[R5900EEVI::vi21 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi21"), 21, concat!("$vi", "21"), true)
    };
    table[R5900EEVI::vi22 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi22"), 22, concat!("$vi", "22"), true)
    };
    table[R5900EEVI::vi23 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi23"), 23, concat!("$vi", "23"), true)
    };
    table[R5900EEVI::vi24 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi24"), 24, concat!("$vi", "24"), true)
    };
    table[R5900EEVI::vi25 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi25"), 25, concat!("$vi", "25"), true)
    };
    table[R5900EEVI::vi26 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi26"), 26, concat!("$vi", "26"), true)
    };
    table[R5900EEVI::vi27 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi27"), 27, concat!("$vi", "27"), true)
    };
    table[R5900EEVI::vi28 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi28"), 28, concat!("$vi", "28"), true)
    };
    table[R5900EEVI::vi29 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi29"), 29, concat!("$vi", "29"), true)
    };
    table[R5900EEVI::vi30 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi30"), 30, concat!("$vi", "30"), true)
    };
    table[R5900EEVI::vi31 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi31"), 31, concat!("$vi", "31"), true)
    };
    table
};
impl R5900EEVI {
    pub const fn try_from_u32(value: u32) -> Result<Self, IntRegisterConversionError> {
        match value {
            0 => Ok(Self::vi0),
            1 => Ok(Self::vi1),
            2 => Ok(Self::vi2),
            3 => Ok(Self::vi3),
            4 => Ok(Self::vi4),
            5 => Ok(Self::vi5),
            6 => Ok(Self::vi6),
            7 => Ok(Self::vi7),
            8 => Ok(Self::vi8),
            9 => Ok(Self::vi9),
            10 => Ok(Self::vi10),
            11 => Ok(Self::vi11),
            12 => Ok(Self::vi12),
            13 => Ok(Self::vi13),
            14 => Ok(Self::vi14),
            15 => Ok(Self::vi15),
            16 => Ok(Self::vi16),
            17 => Ok(Self::vi17),
            18 => Ok(Self::vi18),
            19 => Ok(Self::vi19),
            20 => Ok(Self::vi20),
            21 => Ok(Self::vi21),
            22 => Ok(Self::vi22),
            23 => Ok(Self::vi23),
            24 => Ok(Self::vi24),
            25 => Ok(Self::vi25),
            26 => Ok(Self::vi26),
            27 => Ok(Self::vi27),
            28 => Ok(Self::vi28),
            29 => Ok(Self::vi29),
            30 => Ok(Self::vi30),
            31 => Ok(Self::vi31),
            x => Err(IntRegisterConversionError::new_out_of_range(
                x,
                32,
                "R5900EEVI",
            )),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        32
    }
}
impl TryFrom<u32> for R5900EEVI {
    type Error = IntRegisterConversionError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for R5900EEVI {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<R5900EEVI> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: R5900EEVI) -> &Self::Output {
        &self[index as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_dollar() {
        for x in &R5900EE_VI {
            if x.has_dollar {
                assert!(
                    x.name.starts_with('$'),
                    "Register {} is missing dollar sign",
                    x.name
                );
                assert!(
                    x.name_o32.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_o32
                );
                assert!(
                    x.name_o64.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_o64
                );
                assert!(
                    x.name_n32.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_n32
                );
                assert!(
                    x.name_n64.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_n64
                );
                assert!(
                    x.name_eabi32.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_eabi32
                );
                assert!(
                    x.name_eabi64.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_eabi64
                );
            } else {
                assert!(
                    !x.name.starts_with('$'),
                    "Register {} has dollar sign when it shouldn't",
                    x.name
                );
                assert!(
                    x.name_o32.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_o32
                );
                assert!(
                    x.name_o64.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_o64
                );
                assert!(
                    x.name_n32.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_n32
                );
                assert!(
                    x.name_n64.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_n64
                );
                assert!(
                    x.name_eabi32.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_eabi32
                );
                assert!(
                    x.name_eabi64.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_eabi64
                );
            }
        }
    }
    #[test]
    fn check_descriptor_valid() {
        for (i, x) in R5900EE_VI.iter().enumerate() {
            assert!(x.value() as usize == i, "Broken register index?");
            x.check_valid_entry();
        }
    }
}
