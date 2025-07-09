/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use log::info;

use crate::settings::*;

pub struct PersistentState {
    pub theme: Theme,
    pub endian: Endian,
    pub isa_version: IsaVersion,
    pub isa_extension: Option<IsaExtension>,
}

impl PersistentState {
    pub fn new() -> Self {
        Self {
            theme: DropdownEnum::load_storage(),
            endian: DropdownEnum::load_storage(),
            isa_version: DropdownEnum::load_storage(),
            isa_extension: DropdownEnum::load_storage(),
        }
    }

    pub fn save(&self) {
        let Self {
            theme,
            endian,
            isa_version,
            isa_extension,
        } = self;

        info!("Saving theme: {theme:?}");
        info!("Saving endian: {endian:?}");
        info!("Saving isa_version: {isa_version:?}");
        info!("Saving isa_extension: {isa_extension:?}");

        theme.save_storage();
        endian.save_storage();
        isa_version.save_storage();
        isa_extension.save_storage();
    }
}
