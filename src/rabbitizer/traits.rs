/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{Abi, RegisterDescriptor};

pub trait Register {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor;

    #[must_use]
    fn name_numeric(&self) -> &'static str {
        self.get_descriptor().name_numeric()
    }
    #[must_use]
    fn name_abi(&self, abi: Abi) -> &'static str {
        self.get_descriptor().name_abi(abi)
    }
    #[must_use]
    fn either_name(&self, abi: Abi, use_named: bool) -> &'static str {
        if use_named {
            self.get_descriptor().name_abi(abi)
        } else {
            self.name_numeric()
        }
    }

    #[must_use]
    fn is_clobbered_by_func_call(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_clobbered_by_func_call()
    }
    #[must_use]
    fn is_reserved(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_reserved()
    }
    #[must_use]
    fn is_kernel(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_kernel()
    }
    #[must_use]
    fn is_zero(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_zero()
    }
    #[must_use]
    fn is_assembler_temp(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_assembler_temp()
    }
    #[must_use]
    fn holds_return_value(&self, _abi: Abi) -> bool {
        self.get_descriptor().holds_return_value()
    }
    #[must_use]
    fn holds_return_address(&self, _abi: Abi) -> bool {
        self.get_descriptor().holds_return_address()
    }
    #[must_use]
    fn is_stack_pointer(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_stack_pointer()
    }
    #[must_use]
    fn is_global_pointer(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_global_pointer()
    }
    #[must_use]
    fn is_saved(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_saved()
    }
    #[must_use]
    fn is_temp(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_temp()
    }
    #[must_use]
    fn is_arg(&self, _abi: Abi) -> bool {
        self.get_descriptor().is_arg()
    }
}
