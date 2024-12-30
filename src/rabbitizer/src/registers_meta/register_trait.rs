/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::abi::Abi;
use crate::register_descriptors::RegisterDescriptor;

use super::RegisterIterator;

/// A trait that provides common functionality for every kind of register.
pub trait Register: PartialEq + PartialOrd + Default {
    /// Converts this register into a raw number.
    ///
    /// This number can be used as an index of an array of [`count`] elements.
    ///
    /// [`count`]: Register::count
    #[must_use]
    fn as_index(&self) -> usize;

    /// How many registers this type has.
    ///
    /// For example `Gpr` returns 32.
    #[must_use]
    fn count() -> usize;

    /// Returns an object which allows iterating over every register of this specific register type
    /// in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rabbitizer::{registers::Gpr, registers_meta::Register};
    ///
    /// let mut gpr_iter = Gpr::iter();
    ///
    /// assert_eq!(gpr_iter.next(), Some(Gpr::zero));
    /// assert_eq!(gpr_iter.next(), Some(Gpr::at));
    /// assert_eq!(gpr_iter.next(), Some(Gpr::v0));
    /// ```
    #[must_use]
    fn iter() -> RegisterIterator<Self> {
        RegisterIterator::new()
    }

    /// Returns the descriptor with the actual data of the register.
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor;

    /// Return the numeric "name" for the register.
    #[must_use]
    fn name_numeric(&self) -> &'static str {
        self.get_descriptor().name_numeric()
    }
    /// Return the an actual name for the register.
    ///
    /// This name may be different depending on the `abi` parameter.
    #[must_use]
    fn name_abi(&self, abi: Abi) -> &'static str {
        self.get_descriptor().name_abi(abi)
    }
    /// Returns either the numeric name or the abi name based on the `use_named` parameter.
    #[must_use]
    fn either_name(&self, abi: Abi, use_named: bool) -> &'static str {
        if use_named {
            self.get_descriptor().name_abi(abi)
        } else {
            self.name_numeric()
        }
    }

    /// A function call invalidates the value hold by the register.
    #[must_use]
    fn is_clobbered_by_func_call(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_clobbered_by_func_call()
    }
    /// This register is not implemented and is reserved for future use.
    #[must_use]
    fn is_reserved(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_reserved()
    }
    /// Kernel-only registers ($k0, $k1).
    #[must_use]
    fn is_kernel(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_kernel()
    }
    /// This register is hardcoded to the zero value ($zero).
    #[must_use]
    fn is_zero(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_zero()
    }
    /// The assembler uses this register as a temporary value to expand and reorder instructions
    /// ($at).
    #[must_use]
    fn is_assembler_temp(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_assembler_temp()
    }
    /// The register usually is used to hold the return value of a function ($v0, $v1)-
    #[must_use]
    fn holds_return_value(&self, _abi: Abi) -> bool {
        self.get_descriptor().holds_return_value()
    }
    /// The register holds the address to return to when returning to the caller of the current
    /// function ($ra).
    #[must_use]
    fn holds_return_address(&self, _abi: Abi) -> bool {
        self.get_descriptor().holds_return_address()
    }

    /// The register holds the stack pointer ($sp).
    #[must_use]
    fn is_stack_pointer(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_stack_pointer()
    }
    /// The register holds the global pointer ($gp).
    #[must_use]
    fn is_global_pointer(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_global_pointer()
    }
    /// A saved register. It will hold the same value after calling a function ($sX).
    #[must_use]
    fn is_saved(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_saved()
    }
    /// A temporary register. The value of this register may not be preserved after calling a
    /// function ($tX).
    #[must_use]
    fn is_temp(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_temp()
    }
    /// The register is used as arguments for functions ($aX).
    #[must_use]
    fn is_arg(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_arg()
    }
}

pub(crate) trait R4000AllegrexVectorRegister: Register {}
