/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R5900VI {
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
pub static R5900_VI: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[R5900VI::vi0 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi0"), 0, concat!("$vi", "0"))
    }
    .check_panic_chain();
    table[R5900VI::vi1 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi1"), 1, concat!("$vi", "1"))
    }
    .check_panic_chain();
    table[R5900VI::vi2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi2"), 2, concat!("$vi", "2"))
    }
    .check_panic_chain();
    table[R5900VI::vi3 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi3"), 3, concat!("$vi", "3"))
    }
    .check_panic_chain();
    table[R5900VI::vi4 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi4"), 4, concat!("$vi", "4"))
    }
    .check_panic_chain();
    table[R5900VI::vi5 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi5"), 5, concat!("$vi", "5"))
    }
    .check_panic_chain();
    table[R5900VI::vi6 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi6"), 6, concat!("$vi", "6"))
    }
    .check_panic_chain();
    table[R5900VI::vi7 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi7"), 7, concat!("$vi", "7"))
    }
    .check_panic_chain();
    table[R5900VI::vi8 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi8"), 8, concat!("$vi", "8"))
    }
    .check_panic_chain();
    table[R5900VI::vi9 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi9"), 9, concat!("$vi", "9"))
    }
    .check_panic_chain();
    table[R5900VI::vi10 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi10"), 10, concat!("$vi", "10"))
    }
    .check_panic_chain();
    table[R5900VI::vi11 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi11"), 11, concat!("$vi", "11"))
    }
    .check_panic_chain();
    table[R5900VI::vi12 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi12"), 12, concat!("$vi", "12"))
    }
    .check_panic_chain();
    table[R5900VI::vi13 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi13"), 13, concat!("$vi", "13"))
    }
    .check_panic_chain();
    table[R5900VI::vi14 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi14"), 14, concat!("$vi", "14"))
    }
    .check_panic_chain();
    table[R5900VI::vi15 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi15"), 15, concat!("$vi", "15"))
    }
    .check_panic_chain();
    table[R5900VI::vi16 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi16"), 16, concat!("$vi", "16"))
    }
    .check_panic_chain();
    table[R5900VI::vi17 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi17"), 17, concat!("$vi", "17"))
    }
    .check_panic_chain();
    table[R5900VI::vi18 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi18"), 18, concat!("$vi", "18"))
    }
    .check_panic_chain();
    table[R5900VI::vi19 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi19"), 19, concat!("$vi", "19"))
    }
    .check_panic_chain();
    table[R5900VI::vi20 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi20"), 20, concat!("$vi", "20"))
    }
    .check_panic_chain();
    table[R5900VI::vi21 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi21"), 21, concat!("$vi", "21"))
    }
    .check_panic_chain();
    table[R5900VI::vi22 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi22"), 22, concat!("$vi", "22"))
    }
    .check_panic_chain();
    table[R5900VI::vi23 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi23"), 23, concat!("$vi", "23"))
    }
    .check_panic_chain();
    table[R5900VI::vi24 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi24"), 24, concat!("$vi", "24"))
    }
    .check_panic_chain();
    table[R5900VI::vi25 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi25"), 25, concat!("$vi", "25"))
    }
    .check_panic_chain();
    table[R5900VI::vi26 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi26"), 26, concat!("$vi", "26"))
    }
    .check_panic_chain();
    table[R5900VI::vi27 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi27"), 27, concat!("$vi", "27"))
    }
    .check_panic_chain();
    table[R5900VI::vi28 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi28"), 28, concat!("$vi", "28"))
    }
    .check_panic_chain();
    table[R5900VI::vi29 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi29"), 29, concat!("$vi", "29"))
    }
    .check_panic_chain();
    table[R5900VI::vi30 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi30"), 30, concat!("$vi", "30"))
    }
    .check_panic_chain();
    table[R5900VI::vi31 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vi31"), 31, concat!("$vi", "31"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
impl R5900VI {
    pub const fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
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
            x => Err(crate::Error::OutOfRangeRegisterIndex {
                index: x,
                count: 32,
                register_kind: "R5900VI",
            }),
        }
    }
}
impl TryFrom<u32> for R5900VI {
    type Error = crate::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for R5900VI {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<R5900VI> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: R5900VI) -> &Self::Output {
        &self[index as usize]
    }
}
