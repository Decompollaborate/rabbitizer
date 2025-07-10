/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod operand;
mod operand_descriptor;
mod operand_display;
mod valued_operand;

pub use crate::generated::Operand;
pub use crate::generated::ValuedOperand;
pub use crate::generated::OPERANDS;

pub(crate) use operand::OPERAND_COUNT;
pub use operand::{OperandIterator, OPERAND_COUNT_MAX};
pub use operand_descriptor::OperandDescriptor;
pub use operand_display::{DefaultLabelDisplay, OperandDisplay};
pub use valued_operand::ValuedOperandIterator;
