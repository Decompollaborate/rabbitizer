/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
#[allow(non_camel_case_types)]
pub enum ArgIsaVersion {
    #[clap(aliases=&["mips1"])]
    MIPS_I,
    #[clap(aliases=&["mips2"])]
    MIPS_II,
    #[clap(aliases=&["mips3"])]
    MIPS_III,
    #[clap(aliases=&["mips4"])]
    MIPS_IV,
}

impl Default for ArgIsaVersion {
    fn default() -> Self {
        rabbitizer::IsaVersion::default().into()
    }
}

impl From<ArgIsaVersion> for rabbitizer::IsaVersion {
    fn from(value: ArgIsaVersion) -> Self {
        match value {
            ArgIsaVersion::MIPS_I => Self::MIPS_I,
            ArgIsaVersion::MIPS_II => Self::MIPS_II,
            ArgIsaVersion::MIPS_III => Self::MIPS_III,
            ArgIsaVersion::MIPS_IV => Self::MIPS_IV,
        }
    }
}

impl From<rabbitizer::IsaVersion> for ArgIsaVersion {
    fn from(value: rabbitizer::IsaVersion) -> Self {
        match value {
            rabbitizer::IsaVersion::MIPS_I => Self::MIPS_I,
            rabbitizer::IsaVersion::MIPS_II => Self::MIPS_II,
            rabbitizer::IsaVersion::MIPS_III => Self::MIPS_III,
            rabbitizer::IsaVersion::MIPS_IV => Self::MIPS_IV,
            rabbitizer::IsaVersion::EXTENSION => panic!("This shouldn't be reached"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum ArgIsaExtension {
    RSP,
    R3000GTE,
    R4000ALLEGREX,
    R5900EE,
}

impl From<ArgIsaExtension> for rabbitizer::IsaExtension {
    fn from(value: ArgIsaExtension) -> Self {
        match value {
            ArgIsaExtension::RSP => Self::RSP,
            ArgIsaExtension::R3000GTE => Self::R3000GTE,
            ArgIsaExtension::R4000ALLEGREX => Self::R4000ALLEGREX,
            ArgIsaExtension::R5900EE => Self::R5900EE,
        }
    }
}

impl From<rabbitizer::IsaExtension> for ArgIsaExtension {
    fn from(value: rabbitizer::IsaExtension) -> Self {
        match value {
            rabbitizer::IsaExtension::RSP => Self::RSP,
            rabbitizer::IsaExtension::R3000GTE => Self::R3000GTE,
            rabbitizer::IsaExtension::R4000ALLEGREX => Self::R4000ALLEGREX,
            rabbitizer::IsaExtension::R5900EE => Self::R5900EE,
        }
    }
}
