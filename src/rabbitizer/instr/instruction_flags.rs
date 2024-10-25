/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::abi::Abi;
use crate::opcodes::DecodingFlags;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstructionFlags {
    pub(crate) decoding_flags: DecodingFlags,
    pub(crate) abi: Abi,
    pub(crate) j_as_branch: bool,
}

impl InstructionFlags {
    #[must_use]
    pub const fn default() -> Self {
        Self {
            decoding_flags: DecodingFlags::default(),
            abi: Abi::O32,
            j_as_branch: true,
        }
    }

    #[must_use]
    pub const fn new() -> Self {
        Self::default()
    }
}

impl InstructionFlags {
    #[must_use]
    pub const fn decoding_flags(&self) -> &DecodingFlags {
        &self.decoding_flags
    }
    pub fn decoding_flags_mut(&mut self) -> &mut DecodingFlags {
        &mut self.decoding_flags
    }
    #[must_use]
    pub const fn with_decoding_flags(self, decoding_flags: DecodingFlags) -> Self {
        Self {
            decoding_flags,
            ..self
        }
    }

    #[must_use]
    pub const fn enable_pseudos(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::enable_pseudos)
    }
    pub fn set_enable_pseudos(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::enable_pseudos);
        } else {
            self.decoding_flags.remove(DecodingFlags::enable_pseudos);
        }
    }
    #[must_use]
    pub const fn with_enable_pseudos(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::enable_pseudos
        } else {
            DecodingFlags::enable_pseudos.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_move(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_move)
    }
    pub fn set_pseudo_move(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_move);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_move);
        }
    }
    #[must_use]
    pub const fn with_pseudo_move(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_move
        } else {
            DecodingFlags::pseudo_move.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_beqz(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_beqz)
    }
    pub fn set_pseudo_beqz(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_beqz);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_beqz);
        }
    }
    #[must_use]
    pub const fn with_pseudo_beqz(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_beqz
        } else {
            DecodingFlags::pseudo_beqz.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_bnez(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_bnez)
    }
    pub fn set_pseudo_bnez(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_bnez);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_bnez);
        }
    }
    #[must_use]
    pub const fn with_pseudo_bnez(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_bnez
        } else {
            DecodingFlags::pseudo_bnez.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_b(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_b)
    }
    pub fn set_pseudo_b(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_b);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_b);
        }
    }
    #[must_use]
    pub const fn with_pseudo_b(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_b
        } else {
            DecodingFlags::pseudo_b.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_bal(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_bal)
    }
    pub fn set_pseudo_bal(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_bal);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_bal);
        }
    }
    #[must_use]
    pub const fn with_pseudo_bal(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_bal
        } else {
            DecodingFlags::pseudo_bal.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_not(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_not)
    }
    pub fn set_pseudo_not(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_not);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_not);
        }
    }
    #[must_use]
    pub const fn with_pseudo_not(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_not
        } else {
            DecodingFlags::pseudo_not.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_neg(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_neg)
    }
    pub fn set_pseudo_neg(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_neg);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_neg);
        }
    }
    #[must_use]
    pub const fn with_pseudo_neg(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_neg
        } else {
            DecodingFlags::pseudo_neg.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_negu(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_negu)
    }
    pub fn set_pseudo_negu(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_negu);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_negu);
        }
    }
    #[must_use]
    pub const fn with_pseudo_negu(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_negu
        } else {
            DecodingFlags::pseudo_negu.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn abi(&self) -> Abi {
        self.abi
    }
    pub fn abi_mut(&mut self) -> &mut Abi {
        &mut self.abi
    }
    #[must_use]
    pub const fn with_abi(self, abi: Abi) -> Self {
        Self { abi, ..self }
    }

    #[must_use]
    pub const fn j_as_branch(&self) -> bool {
        self.j_as_branch
    }
    pub fn j_as_branch_mut(&mut self) -> &mut bool {
        &mut self.j_as_branch
    }
    #[must_use]
    pub const fn with_j_as_branch(self, j_as_branch: bool) -> Self {
        Self {
            j_as_branch,
            ..self
        }
    }
}

impl Default for InstructionFlags {
    fn default() -> Self {
        Self::default()
    }
}
