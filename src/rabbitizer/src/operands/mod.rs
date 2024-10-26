/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod operand;
mod operand_descriptor;
mod operand_display;
mod valued_operand;

pub use crate::generated::Operand;
pub use crate::generated::ValuedOperand;
pub(crate) use crate::generated::DISPLAY_OPERAND_CALLBACKS;
pub use crate::generated::OPERANDS;

pub(crate) use operand::OPERAND_COUNT;
pub use operand::{OperandIterator, OPERAND_COUNT_MAX};
pub use operand_descriptor::OperandDescriptor;
pub use operand_display::OperandDisplay;
pub(crate) use operand_display::OperandDisplayCallback;
pub use valued_operand::ValuedOperandIterator;
pub use valued_operand::IU16;
