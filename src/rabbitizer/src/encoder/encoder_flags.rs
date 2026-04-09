/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::instr::InstructionFlags;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncoderFlags {
    instruction_flags: InstructionFlags,
    allow_dollarless: bool,
    #[cfg(feature = "R5900EE")]
    r5900ee_prodg_sn_as_inverted_regs: bool,
}

impl EncoderFlags {
    #[must_use]
    pub const fn new(instruction_flags: InstructionFlags) -> Self {
        Self {
            instruction_flags,
            allow_dollarless: false,
            #[cfg(feature = "R5900EE")]
            r5900ee_prodg_sn_as_inverted_regs: false,
        }
    }
}

impl EncoderFlags {
    #[must_use]
    pub const fn instruction_flags(&self) -> &InstructionFlags {
        &self.instruction_flags
    }
    pub fn instruction_flags_mut(&mut self) -> &mut InstructionFlags {
        &mut self.instruction_flags
    }
    #[must_use]
    pub const fn with_instruction_flags(self, instruction_flags: InstructionFlags) -> Self {
        Self {
            instruction_flags,
            ..self
        }
    }

    #[must_use]
    pub const fn allow_dollarless(&self) -> bool {
        self.allow_dollarless
    }
    pub fn allow_dollarless_mut(&mut self) -> &mut bool {
        &mut self.allow_dollarless
    }
    #[must_use]
    pub const fn with_allow_dollarless(self, allow_dollarless: bool) -> Self {
        Self {
            allow_dollarless,
            ..self
        }
    }

    #[must_use]
    #[cfg(feature = "R5900EE")]
    pub const fn r5900ee_prodg_sn_as_inverted_regs(&self) -> bool {
        self.r5900ee_prodg_sn_as_inverted_regs
    }
    #[cfg(feature = "R5900EE")]
    pub fn r5900ee_prodg_sn_as_inverted_regs_mut(&mut self) -> &mut bool {
        &mut self.r5900ee_prodg_sn_as_inverted_regs
    }
    #[must_use]
    #[cfg(feature = "R5900EE")]
    pub const fn with_r5900ee_prodg_sn_as_inverted_regs(
        self,
        r5900ee_prodg_sn_as_inverted_regs: bool,
    ) -> Self {
        Self {
            r5900ee_prodg_sn_as_inverted_regs,
            ..self
        }
    }
}
