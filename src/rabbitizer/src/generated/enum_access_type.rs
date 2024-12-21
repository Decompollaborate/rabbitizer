/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::access_type::ACCESS_TYPE_COUNT;
use crate::access_type_descriptor::AccessTypeDescriptor;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum AccessType {
    NONE,
    BYTE,
    SHORT,
    WORD,
    DOUBLEWORD,
    QUADWORD,
    FLOAT,
    DOUBLEFLOAT,
    WORD_LEFT,
    WORD_RIGHT,
    DOUBLEWORD_LEFT,
    DOUBLEWORD_RIGHT,
}
pub static ACCESS_TYPES: [AccessTypeDescriptor; ACCESS_TYPE_COUNT] = {
    let mut table = [AccessTypeDescriptor::default(); ACCESS_TYPE_COUNT];
    table[AccessType::NONE as usize] = AccessTypeDescriptor {
        ..AccessTypeDescriptor::new("NONE")
    };
    table[AccessType::BYTE as usize] = AccessTypeDescriptor {
        min_size: 1,
        min_alignment: 1,
        ..AccessTypeDescriptor::new("BYTE")
    };
    table[AccessType::SHORT as usize] = AccessTypeDescriptor {
        min_size: 2,
        min_alignment: 2,
        ..AccessTypeDescriptor::new("SHORT")
    };
    table[AccessType::WORD as usize] = AccessTypeDescriptor {
        min_size: 4,
        min_alignment: 4,
        ..AccessTypeDescriptor::new("WORD")
    };
    table[AccessType::DOUBLEWORD as usize] = AccessTypeDescriptor {
        min_size: 8,
        min_alignment: 8,
        ..AccessTypeDescriptor::new("DOUBLEWORD")
    };
    table[AccessType::QUADWORD as usize] = AccessTypeDescriptor {
        min_size: 16,
        min_alignment: 16,
        ..AccessTypeDescriptor::new("QUADWORD")
    };
    table[AccessType::FLOAT as usize] = AccessTypeDescriptor {
        min_size: 4,
        min_alignment: 4,
        ..AccessTypeDescriptor::new("FLOAT")
    };
    table[AccessType::DOUBLEFLOAT as usize] = AccessTypeDescriptor {
        min_size: 8,
        min_alignment: 8,
        ..AccessTypeDescriptor::new("DOUBLEFLOAT")
    };
    table[AccessType::WORD_LEFT as usize] = AccessTypeDescriptor {
        min_size: 4,
        ..AccessTypeDescriptor::new("WORD_LEFT")
    };
    table[AccessType::WORD_RIGHT as usize] = AccessTypeDescriptor {
        min_size: 4,
        ..AccessTypeDescriptor::new("WORD_RIGHT")
    };
    table[AccessType::DOUBLEWORD_LEFT as usize] = AccessTypeDescriptor {
        min_size: 8,
        ..AccessTypeDescriptor::new("DOUBLEWORD_LEFT")
    };
    table[AccessType::DOUBLEWORD_RIGHT as usize] = AccessTypeDescriptor {
        min_size: 8,
        ..AccessTypeDescriptor::new("DOUBLEWORD_RIGHT")
    };
    table
};