/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");
}
