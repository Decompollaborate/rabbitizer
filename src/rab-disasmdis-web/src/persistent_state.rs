/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use log::info;

use crate::settings::*;

pub struct PersistentState {
    pub theme: Theme,
    pub endian: Endian,
    pub isa_version: IsaVersion,
    pub isa_extension: Option<IsaExtension>,
    pub branch_label: DefaultLabelDisplay,
    pub vram: Vram,
}

impl PersistentState {
    pub fn new() -> Self {
        Self {
            theme: Storagable::load_storage(Default::default),
            endian: Storagable::load_storage(Default::default),
            isa_version: Storagable::load_storage(Default::default),
            isa_extension: Storagable::load_storage(Default::default),
            branch_label: Storagable::load_storage(Default::default),
            vram: Storagable::load_storage(|| Vram::new(0x80000000)),
        }
    }

    pub fn save(&self) {
        let Self {
            theme,
            endian,
            isa_version,
            isa_extension,
            branch_label,
            vram,
        } = self;

        info!("Saving theme: {theme:?}");
        info!("Saving endian: {endian:?}");
        info!("Saving isa_version: {isa_version:?}");
        info!("Saving isa_extension: {isa_extension:?}");
        info!("Saving vram: {vram:?}");

        theme.save_storage();
        endian.save_storage();
        isa_version.save_storage();
        isa_extension.save_storage();
        branch_label.save_storage();
        vram.save_storage();
    }
}
