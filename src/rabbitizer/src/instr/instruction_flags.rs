/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::abi::Abi;
use crate::isa::{IsaExtension, IsaVersion};
use crate::opcodes::DecodingFlags;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstructionFlags {
    isa_version: IsaVersion,
    isa_extension: IsaExtension,

    decoding_flags: DecodingFlags,
    abi: Abi,
    j_as_branch: bool,
}

impl InstructionFlags {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        let isa_extension = IsaExtension::default();

        Self {
            isa_version: isa_extension.isa_version(),
            isa_extension,
            decoding_flags: DecodingFlags::default(),
            abi: Abi::O32,
            j_as_branch: true,
        }
    }

    #[must_use]
    pub const fn new() -> Self {
        Self { ..Self::default() }
    }
}

impl InstructionFlags {
    #[must_use]
    pub const fn isa_version(&self) -> IsaVersion {
        self.isa_version
    }
    pub fn isa_version_mut(&mut self) -> &mut IsaVersion {
        &mut self.isa_version
    }
    #[must_use]
    pub const fn with_isa_version(self, isa_version: IsaVersion) -> Self {
        Self {
            isa_version,
            ..self
        }
    }

    #[must_use]
    pub const fn isa_extension(&self) -> IsaExtension {
        self.isa_extension
    }
    pub fn isa_extension_mut(&mut self) -> &mut IsaExtension {
        &mut self.isa_extension
    }
    #[must_use]
    pub const fn with_isa_extension(self, isa_extension: IsaExtension) -> Self {
        Self {
            isa_version: isa_extension.isa_version(),
            isa_extension,
            ..self
        }
    }

    #[must_use]
    pub(crate) const fn decoding_flags(&self) -> &DecodingFlags {
        &self.decoding_flags
    }
    /*
    pub(crate) fn decoding_flags_mut(&mut self) -> &mut DecodingFlags {
        &mut self.decoding_flags
    }
    */
    #[must_use]
    pub(crate) const fn with_decoding_flags(self, decoding_flags: DecodingFlags) -> Self {
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
            self.decoding_flags.union(DecodingFlags::enable_pseudos)
        } else {
            self.decoding_flags
                .intersection(DecodingFlags::enable_pseudos.complement())
        };
        self.with_decoding_flags(other)
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
            self.decoding_flags.union(DecodingFlags::pseudo_move)
        } else {
            self.decoding_flags
                .intersection(DecodingFlags::pseudo_move.complement())
        };
        self.with_decoding_flags(other)
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
            self.decoding_flags.union(DecodingFlags::pseudo_beqz)
        } else {
            self.decoding_flags
                .intersection(DecodingFlags::pseudo_beqz.complement())
        };
        self.with_decoding_flags(other)
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
            self.decoding_flags.union(DecodingFlags::pseudo_bnez)
        } else {
            self.decoding_flags
                .intersection(DecodingFlags::pseudo_bnez.complement())
        };
        self.with_decoding_flags(other)
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
            self.decoding_flags.union(DecodingFlags::pseudo_b)
        } else {
            self.decoding_flags
                .intersection(DecodingFlags::pseudo_b.complement())
        };
        self.with_decoding_flags(other)
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
            self.decoding_flags.union(DecodingFlags::pseudo_bal)
        } else {
            self.decoding_flags
                .intersection(DecodingFlags::pseudo_bal.complement())
        };
        self.with_decoding_flags(other)
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
            self.decoding_flags.union(DecodingFlags::pseudo_not)
        } else {
            self.decoding_flags
                .intersection(DecodingFlags::pseudo_not.complement())
        };
        self.with_decoding_flags(other)
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
            self.decoding_flags.union(DecodingFlags::pseudo_neg)
        } else {
            self.decoding_flags
                .intersection(DecodingFlags::pseudo_neg.complement())
        };
        self.with_decoding_flags(other)
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
            self.decoding_flags.union(DecodingFlags::pseudo_negu)
        } else {
            self.decoding_flags
                .intersection(DecodingFlags::pseudo_negu.complement())
        };
        self.with_decoding_flags(other)
    }

    pub fn set_all_pseudos(&mut self, turn_on: bool) {
        self.decoding_flags.set_all_pseudos(turn_on)
    }
    #[must_use]
    pub const fn with_all_pseudos(self, turn_on: bool) -> Self {
        self.with_decoding_flags(self.decoding_flags.with_all_pseudos(turn_on))
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