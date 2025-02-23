/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::abi::Abi;

/// Describes properties of a given register.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct RegisterDescriptor {
    pub(crate) name: &'static str,
    pub(crate) value: u8,
    pub(crate) name_numeric: &'static str,

    pub(crate) name_o32: Option<&'static str>,
    pub(crate) name_o64: Option<&'static str>,
    pub(crate) name_n32: Option<&'static str>,
    pub(crate) name_n64: Option<&'static str>,
    pub(crate) name_eabi32: Option<&'static str>,
    pub(crate) name_eabi64: Option<&'static str>,

    /// A function call invalidates the value hold by the register.
    pub(crate) is_clobbered_by_func_call: bool,
    /// This register is not implemented and is reserved for future use.
    pub(crate) is_reserved: bool,
    /// Kernel-only registers ($k0, $k1)
    pub(crate) is_kernel: bool,
    /// $zero
    pub(crate) is_zero: bool,
    /// $at
    pub(crate) is_assembler_temp: bool,
    /// $v0, $v1
    pub(crate) holds_return_value: bool,
    /// $ra
    pub(crate) holds_return_address: bool,
    /// $sp
    pub(crate) is_stack_pointer: bool,
    /// $gp
    pub(crate) is_global_pointer: bool,
    /// $sX
    pub(crate) is_saved: bool,
    /// $tX
    pub(crate) is_temp: bool,
    /// $aX
    pub(crate) is_arg: bool,
    /// $aX or $tX
    pub(crate) either_arg_or_temp: bool,
}

impl RegisterDescriptor {
    pub(crate) const fn default() -> Self {
        Self {
            name: "$0",
            value: 0,
            name_numeric: "$0",

            name_o32: None,
            name_o64: None,
            name_n32: None,
            name_n64: None,
            name_eabi32: None,
            name_eabi64: None,

            is_clobbered_by_func_call: false,
            is_reserved: false,
            is_kernel: false,
            is_zero: false,
            is_assembler_temp: false,
            holds_return_value: false,
            holds_return_address: false,
            is_stack_pointer: false,
            is_global_pointer: false,
            is_saved: false,
            is_temp: false,
            is_arg: false,
            either_arg_or_temp: false,
        }
    }

    pub(crate) const fn new(name: &'static str, value: u8, name_numeric: &'static str) -> Self {
        Self {
            name,
            value,
            name_numeric,
            ..Self::default()
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub(crate) const fn check_panic(&self) {
        if self.is_reserved {
            assert!(
                !self.is_clobbered_by_func_call,
                "Broken register descriptor"
            );
            assert!(!self.is_kernel, "Broken register descriptor");
            assert!(!self.is_zero, "Broken register descriptor");
            assert!(!self.is_assembler_temp, "Broken register descriptor");
            assert!(!self.holds_return_value, "Broken register descriptor");
            assert!(!self.holds_return_address, "Broken register descriptor");
            assert!(!self.is_stack_pointer, "Broken register descriptor");
            assert!(!self.is_global_pointer, "Broken register descriptor");
            assert!(!self.is_saved, "Broken register descriptor");
            assert!(!self.is_temp, "Broken register descriptor");
            assert!(!self.is_arg, "Broken register descriptor");
            assert!(!self.either_arg_or_temp, "Broken register descriptor");
        }

        if self.is_zero {
            assert!(
                !self.is_clobbered_by_func_call,
                "Broken register descriptor"
            );
            assert!(!self.is_reserved, "Broken register descriptor");
            assert!(!self.is_kernel, "Broken register descriptor");
            assert!(!self.is_assembler_temp, "Broken register descriptor");
            assert!(!self.holds_return_value, "Broken register descriptor");
            assert!(!self.holds_return_address, "Broken register descriptor");
            assert!(!self.is_stack_pointer, "Broken register descriptor");
            assert!(!self.is_global_pointer, "Broken register descriptor");
            assert!(!self.is_saved, "Broken register descriptor");
            assert!(!self.is_temp, "Broken register descriptor");
            assert!(!self.is_arg, "Broken register descriptor");
            assert!(!self.either_arg_or_temp, "Broken register descriptor");
        }

        if self.is_assembler_temp {
            assert!(self.is_clobbered_by_func_call, "Broken register descriptor");
            assert!(!self.is_reserved, "Broken register descriptor");
            assert!(!self.is_kernel, "Broken register descriptor");
            assert!(!self.is_zero, "Broken register descriptor");
            assert!(!self.holds_return_value, "Broken register descriptor");
            assert!(!self.holds_return_address, "Broken register descriptor");
            assert!(!self.is_stack_pointer, "Broken register descriptor");
            assert!(!self.is_global_pointer, "Broken register descriptor");
            assert!(!self.is_saved, "Broken register descriptor");
            assert!(!self.is_temp, "Broken register descriptor");
            assert!(!self.is_arg, "Broken register descriptor");
            assert!(!self.either_arg_or_temp, "Broken register descriptor");
        }

        if self.holds_return_value {
            assert!(self.is_clobbered_by_func_call, "Broken register descriptor");
            assert!(!self.is_reserved, "Broken register descriptor");
            assert!(!self.is_kernel, "Broken register descriptor");
            assert!(!self.is_zero, "Broken register descriptor");
            assert!(!self.is_assembler_temp, "Broken register descriptor");
            assert!(!self.holds_return_address, "Broken register descriptor");
            assert!(!self.is_stack_pointer, "Broken register descriptor");
            assert!(!self.is_global_pointer, "Broken register descriptor");
            assert!(!self.is_saved, "Broken register descriptor");
            assert!(!self.is_temp, "Broken register descriptor");
            assert!(!self.is_arg, "Broken register descriptor");
            assert!(!self.either_arg_or_temp, "Broken register descriptor");
        }

        if self.holds_return_address {
            assert!(self.is_clobbered_by_func_call, "Broken register descriptor");
            assert!(!self.is_reserved, "Broken register descriptor");
            assert!(!self.is_kernel, "Broken register descriptor");
            assert!(!self.is_zero, "Broken register descriptor");
            assert!(!self.is_assembler_temp, "Broken register descriptor");
            assert!(!self.holds_return_value, "Broken register descriptor");
            assert!(!self.is_stack_pointer, "Broken register descriptor");
            assert!(!self.is_global_pointer, "Broken register descriptor");
            assert!(!self.is_saved, "Broken register descriptor");
            assert!(!self.is_temp, "Broken register descriptor");
            assert!(!self.is_arg, "Broken register descriptor");
            assert!(!self.either_arg_or_temp, "Broken register descriptor");
        }

        if self.is_stack_pointer {
            assert!(
                !self.is_clobbered_by_func_call,
                "Broken register descriptor"
            );
            assert!(!self.is_reserved, "Broken register descriptor");
            assert!(!self.is_kernel, "Broken register descriptor");
            assert!(!self.is_zero, "Broken register descriptor");
            assert!(!self.is_assembler_temp, "Broken register descriptor");
            assert!(!self.holds_return_value, "Broken register descriptor");
            assert!(!self.holds_return_address, "Broken register descriptor");
            assert!(!self.is_global_pointer, "Broken register descriptor");
            assert!(!self.is_saved, "Broken register descriptor");
            assert!(!self.is_temp, "Broken register descriptor");
            assert!(!self.is_arg, "Broken register descriptor");
            assert!(!self.either_arg_or_temp, "Broken register descriptor");
        }

        if self.is_global_pointer {
            assert!(
                !self.is_clobbered_by_func_call,
                "Broken register descriptor"
            );
            assert!(!self.is_reserved, "Broken register descriptor");
            assert!(!self.is_kernel, "Broken register descriptor");
            assert!(!self.is_zero, "Broken register descriptor");
            assert!(!self.is_assembler_temp, "Broken register descriptor");
            assert!(!self.holds_return_value, "Broken register descriptor");
            assert!(!self.holds_return_address, "Broken register descriptor");
            assert!(!self.is_stack_pointer, "Broken register descriptor");
            assert!(!self.is_saved, "Broken register descriptor");
            assert!(!self.is_temp, "Broken register descriptor");
            assert!(!self.is_arg, "Broken register descriptor");
            assert!(!self.either_arg_or_temp, "Broken register descriptor");
        }

        if self.is_saved {
            assert!(
                !self.is_clobbered_by_func_call,
                "Broken register descriptor"
            );
            assert!(!self.is_reserved, "Broken register descriptor");
            assert!(!self.is_kernel, "Broken register descriptor");
            assert!(!self.is_zero, "Broken register descriptor");
            assert!(!self.is_assembler_temp, "Broken register descriptor");
            assert!(!self.holds_return_value, "Broken register descriptor");
            assert!(!self.holds_return_address, "Broken register descriptor");
            assert!(!self.is_stack_pointer, "Broken register descriptor");
            assert!(!self.is_global_pointer, "Broken register descriptor");
            assert!(!self.is_temp, "Broken register descriptor");
            assert!(!self.is_arg, "Broken register descriptor");
            assert!(!self.either_arg_or_temp, "Broken register descriptor");
        }

        if self.is_temp {
            assert!(self.is_clobbered_by_func_call, "Broken register descriptor");
            assert!(!self.is_reserved, "Broken register descriptor");
            assert!(!self.is_kernel, "Broken register descriptor");
            assert!(!self.is_zero, "Broken register descriptor");
            assert!(!self.is_assembler_temp, "Broken register descriptor");
            assert!(!self.holds_return_value, "Broken register descriptor");
            assert!(!self.holds_return_address, "Broken register descriptor");
            assert!(!self.is_stack_pointer, "Broken register descriptor");
            assert!(!self.is_global_pointer, "Broken register descriptor");
            assert!(!self.is_saved, "Broken register descriptor");
            if self.either_arg_or_temp {
                assert!(self.is_arg, "Broken register descriptor");
            } else {
                assert!(!self.is_arg, "Broken register descriptor");
            }
        }

        if self.is_arg {
            assert!(self.is_clobbered_by_func_call, "Broken register descriptor");
            assert!(!self.is_reserved, "Broken register descriptor");
            assert!(!self.is_kernel, "Broken register descriptor");
            assert!(!self.is_zero, "Broken register descriptor");
            assert!(!self.is_assembler_temp, "Broken register descriptor");
            assert!(!self.holds_return_value, "Broken register descriptor");
            assert!(!self.holds_return_address, "Broken register descriptor");
            assert!(!self.is_stack_pointer, "Broken register descriptor");
            assert!(!self.is_global_pointer, "Broken register descriptor");
            assert!(!self.is_saved, "Broken register descriptor");
            if self.either_arg_or_temp {
                assert!(self.is_temp, "Broken register descriptor");
            } else {
                assert!(!self.is_temp, "Broken register descriptor");
            }
        }

        if self.either_arg_or_temp {
            assert!(self.is_clobbered_by_func_call, "Broken register descriptor");
            assert!(!self.is_reserved, "Broken register descriptor");
            assert!(!self.is_kernel, "Broken register descriptor");
            assert!(!self.is_zero, "Broken register descriptor");
            assert!(!self.is_assembler_temp, "Broken register descriptor");
            assert!(!self.holds_return_value, "Broken register descriptor");
            assert!(!self.holds_return_address, "Broken register descriptor");
            assert!(!self.is_stack_pointer, "Broken register descriptor");
            assert!(!self.is_global_pointer, "Broken register descriptor");
            assert!(!self.is_saved, "Broken register descriptor");
            assert!(self.is_temp, "Broken register descriptor");
            assert!(self.is_arg, "Broken register descriptor");
        }
    }

    pub(crate) const fn check_panic_chain(self) -> Self {
        self.check_panic();
        self
    }
}

impl RegisterDescriptor {
    /// Return the numeric "name" for the register.
    #[must_use]
    pub const fn name_numeric(&self) -> &'static str {
        self.name_numeric
    }

    /// Return the an actual name for the register.
    ///
    /// This name may be different depending on the `abi` parameter.
    #[must_use]
    pub const fn name_abi(&self, abi: Abi) -> &'static str {
        match abi {
            Abi::O32 => {
                if let Some(x) = self.name_o32 {
                    x
                } else {
                    self.name
                }
            }
            Abi::O64 => {
                if let Some(x) = self.name_o64 {
                    x
                } else {
                    self.name
                }
            }
            Abi::N32 => {
                if let Some(x) = self.name_n32 {
                    x
                } else {
                    self.name
                }
            }
            Abi::N64 => {
                if let Some(x) = self.name_n64 {
                    x
                } else {
                    self.name
                }
            }
            Abi::EABI32 => {
                if let Some(x) = self.name_eabi32 {
                    x
                } else {
                    self.name
                }
            }
            Abi::EABI64 => {
                if let Some(x) = self.name_eabi64 {
                    x
                } else {
                    self.name
                }
            }
        }
    }

    /// A function call invalidates the value hold by the register.
    #[must_use]
    pub const fn is_clobbered_by_func_call(&self) -> bool {
        self.is_clobbered_by_func_call
    }
    /// This register is not implemented and is reserved for future use.
    #[must_use]
    pub const fn is_reserved(&self) -> bool {
        self.is_reserved
    }
    /// Kernel-only registers ($k0, $k1).
    #[must_use]
    pub const fn is_kernel(&self) -> bool {
        self.is_kernel
    }
    /// This register is hardcoded to the zero value ($zero).
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.is_zero
    }
    /// The assembler uses this register as a temporary value to expand and reorder instructions
    /// ($at).
    #[must_use]
    pub const fn is_assembler_temp(&self) -> bool {
        self.is_assembler_temp
    }
    /// The register usually is used to hold the return value of a function ($v0, $v1)-
    #[must_use]
    pub const fn holds_return_value(&self) -> bool {
        self.holds_return_value
    }
    /// The register holds the address to return to when returning to the caller of the current
    /// function ($ra).
    #[must_use]
    pub const fn holds_return_address(&self) -> bool {
        self.holds_return_address
    }

    /// The register holds the stack pointer ($sp).
    #[must_use]
    pub const fn is_stack_pointer(&self) -> bool {
        self.is_stack_pointer
    }
    /// The register holds the global pointer ($gp).
    #[must_use]
    pub const fn is_global_pointer(&self) -> bool {
        self.is_global_pointer
    }
    /// A saved register. It will hold the same value after calling a function ($sX).
    #[must_use]
    pub const fn is_saved(&self) -> bool {
        self.is_saved
    }
    /// A temporary register. The value of this register may not be preserved after calling a
    /// function ($tX).
    #[must_use]
    pub const fn is_temp(&self) -> bool {
        self.is_temp
    }
    /// The register is used as arguments for functions ($aX).
    #[must_use]
    pub const fn is_arg(&self) -> bool {
        self.is_arg
    }
    /// The register may be either an argument or a temp register depending on the abi.
    #[must_use]
    pub const fn either_arg_or_temp(&self) -> bool {
        self.either_arg_or_temp
    }
}
