/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::access_type::ACCESS_TYPE_COUNT;
use crate::access_type_descriptor::AccessTypeDescriptor;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum AccessType {
    BYTE,
    SHORT,
    WORD,
    #[cfg(feature = "MIPS_III")]
    DOUBLEWORD,
    #[cfg(feature = "R5900EE")]
    QUADWORD,
    FLOAT,
    #[cfg(feature = "MIPS_II")]
    DOUBLEFLOAT,
    WORD_COP2,
    #[cfg(feature = "MIPS_II")]
    DOUBLEWORD_COP2,
    UNALIGNED_WORD_LEFT,
    UNALIGNED_WORD_RIGHT,
    #[cfg(feature = "MIPS_III")]
    UNALIGNED_DOUBLEWORD_LEFT,
    #[cfg(feature = "MIPS_III")]
    UNALIGNED_DOUBLEWORD_RIGHT,
    #[cfg(feature = "MIPS_II")]
    LINKED_WORD_WORD,
    #[cfg(feature = "MIPS_III")]
    LINKED_WORD_DOUBLEWORD,
}
pub static ACCESS_TYPES: [AccessTypeDescriptor; ACCESS_TYPE_COUNT] = {
    let mut table = [AccessTypeDescriptor::default(); ACCESS_TYPE_COUNT];
    {
        table[AccessType::BYTE as usize] = AccessTypeDescriptor {
            min_size: Some(1),
            min_alignment: Some(1),
            ..AccessTypeDescriptor::new("BYTE")
        };
    }
    {
        table[AccessType::SHORT as usize] = AccessTypeDescriptor {
            min_size: Some(2),
            min_alignment: Some(2),
            ..AccessTypeDescriptor::new("SHORT")
        };
    }
    {
        table[AccessType::WORD as usize] = AccessTypeDescriptor {
            min_size: Some(4),
            min_alignment: Some(4),
            ..AccessTypeDescriptor::new("WORD")
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[AccessType::DOUBLEWORD as usize] = AccessTypeDescriptor {
            min_size: Some(8),
            min_alignment: Some(8),
            ..AccessTypeDescriptor::new("DOUBLEWORD")
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[AccessType::QUADWORD as usize] = AccessTypeDescriptor {
            min_size: Some(16),
            min_alignment: Some(16),
            ..AccessTypeDescriptor::new("QUADWORD")
        };
    }
    {
        table[AccessType::FLOAT as usize] = AccessTypeDescriptor {
            min_size: Some(4),
            min_alignment: Some(4),
            ..AccessTypeDescriptor::new("FLOAT")
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[AccessType::DOUBLEFLOAT as usize] = AccessTypeDescriptor {
            min_size: Some(8),
            min_alignment: Some(8),
            ..AccessTypeDescriptor::new("DOUBLEFLOAT")
        };
    }
    {
        table[AccessType::WORD_COP2 as usize] = AccessTypeDescriptor {
            min_size: Some(4),
            min_alignment: Some(4),
            ..AccessTypeDescriptor::new("WORD_COP2")
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[AccessType::DOUBLEWORD_COP2 as usize] = AccessTypeDescriptor {
            min_size: Some(8),
            min_alignment: Some(8),
            ..AccessTypeDescriptor::new("DOUBLEWORD_COP2")
        };
    }
    {
        table[AccessType::UNALIGNED_WORD_LEFT as usize] = AccessTypeDescriptor {
            min_size: Some(4),
            is_unaligned: true,
            ..AccessTypeDescriptor::new("UNALIGNED_WORD_LEFT")
        };
    }
    {
        table[AccessType::UNALIGNED_WORD_RIGHT as usize] = AccessTypeDescriptor {
            min_size: Some(4),
            is_unaligned: true,
            ..AccessTypeDescriptor::new("UNALIGNED_WORD_RIGHT")
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[AccessType::UNALIGNED_DOUBLEWORD_LEFT as usize] = AccessTypeDescriptor {
            min_size: Some(8),
            is_unaligned: true,
            ..AccessTypeDescriptor::new("UNALIGNED_DOUBLEWORD_LEFT")
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[AccessType::UNALIGNED_DOUBLEWORD_RIGHT as usize] = AccessTypeDescriptor {
            min_size: Some(8),
            is_unaligned: true,
            ..AccessTypeDescriptor::new("UNALIGNED_DOUBLEWORD_RIGHT")
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[AccessType::LINKED_WORD_WORD as usize] = AccessTypeDescriptor {
            min_size: Some(4),
            min_alignment: Some(4),
            ..AccessTypeDescriptor::new("LINKED_WORD_WORD")
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[AccessType::LINKED_WORD_DOUBLEWORD as usize] = AccessTypeDescriptor {
            min_size: Some(8),
            min_alignment: Some(8),
            ..AccessTypeDescriptor::new("LINKED_WORD_DOUBLEWORD")
        };
    }
    table
};
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_descriptor_valid() {
        for x in ACCESS_TYPES {
            x.check_valid_entry();
        }
    }
}
