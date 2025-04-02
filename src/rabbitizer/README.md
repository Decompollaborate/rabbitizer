# rabbitizer

![GitHub](https://img.shields.io/github/license/Decompollaborate/rabbitizer)
![crate.io](https://img.shields.io/crates/dv/rabbitizer)
![Crates.io MSRV](https://img.shields.io/crates/msrv/rabbitizer)
![GitHub License](https://img.shields.io/github/license/Decompollaborate/rabbitizer)
![GitHub contributors](https://img.shields.io/github/contributors/Decompollaborate/rabbitizer?logo=purple)

rabbitizer (whole name is lowercase) is a MIPS instruction decoder API.

## Quick example

```rust
use rabbitizer::{Instruction, Vram, InstructionFlags, InstructionDisplayFlags};
use rabbitizer::isa::IsaVersion;
use rabbitizer::opcodes::Opcode;

let vram = Vram::new(0x80000000);
let flags = InstructionFlags::new(IsaVersion::MIPS_I);
// The instruction word to be decoded.
let word = 0x3C088001;
// Decode the instruction.
let instr = Instruction::new(word, vram, flags);

// Check the decoded instruction.
assert_eq!(instr.opcode(), Opcode::core_lui);

// Produce an assemblable representation.
// `InstructionDisplayFlags` allows to configure the generated representation.
let display_flags = InstructionDisplayFlags::new();
let display = instr.display::<&str>(&display_flags, None, 0);
assert_eq!(
    &display.to_string(),
    "lui         $t0, 0x8001"
);
```

Check the documentation at [crates.io](https://crates.io/crates/rabbitizer).

## Features

- Able to produce an assembly representation of the decoded words that
  assembles back to the original data. This is known as matching disassembly.
- Fully written in Rust.
  - Written with `no_std` and "no alloc".
  - `const` decoding. It is possible to decode MIPS instructions at compile time.
- Simple per-word instruction decoding.
  - The library doesn't try to be too smart by processing multiple instructions
    at a time, neither trying to handle endianness.
- Can perform validation checks for decoded instructions.
  - For example, it can check if a given instruction has garbage data on bits
    where the it should contain zeroes.
- Provides many examination/grouping functions for instructions, allowing to
  simplify checking characteristics of an instruction and minimizing the need to
  check for specific instructions in a hardcoded way.
  - i.e. Check for `.does_link()` instead of checking for `jal`, `jalr`, `bal`
    `bltzal`, etc.
- Configurable, many features can be turned on and off.
- MIPS instructions features:
  - Support for ABI register names: o32, n32, o64, n64, eabi32 and eabi64.
    - You can also turn off ABI names and use numeric names for registers if
      preferred.
  - Configurable behavior for the `jalr` instruction, allowing to disassemble
    the instruction using an implicit or explicit `rd` register depending if
    that register is `$ra` or not.
  - Named registers for MIPS VR4300's coprocessors.
  - Support for many pseudo-instructions.
    - Pseudo-instruction detection can be turned off per pseudo.
- Some workarounds for some specific compilers/assemblers:
  - `SN64`: `div`/`divu` fix: tweaks a bit the produced `div`, `divu` and
    `break` instructions.
  - `gccee`: Many workarounds for R5900EE specific instructions to support the
    differences between the original toolchain and modern gas.
- Multiple MIPS ISAs versions and ISA extensions are supported:
  - MIPS I, II and III ISAs.
    - Partial support for MIPS IV too.
  - N64 RSP instruction support.
    - RSP decoding has been tested to build back to matching assemblies with [armips](https://github.com/Kingcom/armips/).
  - R3000 GTE (PSX's CPU) support.
  - R4000 ALLEGREX (PSP's CPU) support.
  - R5900 EE (PS2's Emotion Engine processor) support.
  - Which ISA version/extension pair is used to decode an instruction can be
    selected at runtime.
  - ISAs are "feature gated", so you can only pay for what you use. See
    [Crate features](#crate-features) for more info.

### Planned features

- Bindings for other programming languages, so you can unleash rabbitizer's
  power on your prefered language.
  - Python bindings
  - C bindings.
  - C++ bindings

## Non-features

In order to keep it simple the following features will never be supported:

- Pseudo-instructions which expands to more than one instruction.

## Minimum Supported Rust Version (MSRV)

The current version of rabbitizer requires **Rust 1.82.0 or greater**.

The current policy is that this may be bumped in minor rabbitizer updates.

## Installation

rabbitizer is available on [crates.io](https://crates.io/crates/rabbitizer) and
can be included in your Cargo enabled project like this:

```toml
[dependencies]
rabbitizer = "2.0.0-alpha.1"
```

### Crate features

There are a few compilation features. Currently none of them are enabled by
default.

- ISA versions:
  - `MIPS_II`: Enables support for MIPS II instructions.
  - `MIPS_III`: Enables support for MIPS III instructions.
    - Enables the `MIPS_II` feature.
  - `MIPS_IV`: Enables support for MIPS IV instructions.
    - Enables the `MIPS_III` feature.
- ISA extensions:
  - `RSP`: Enables support for the RSP extension set.
    - Enables the `MIPS_III` feature.
  - `R3000GTE`: Enables support for the R3000GTE extension set.
  - `R4000ALLEGREX`: Enables support for the R4000ALLEGREX extension set.
    - Enables the `MIPS_III` feature.
  - `R5900EE`: Enables support for the R5900EE extension set.
    - Enables the `MIPS_IV` feature.
  - `RspViceMsp`: Enables support for the Vice Msp extension set, which is an
    extension for the RSP instruction set.
    - Enables the `RSP` feature.
- Blanket featues:
  - `all_extensions`: Enables all common extensions. Currently enables `RSP`,
    `R3000GTE`, `R4000ALLEGREX` and `R5900EE` features.
  - `all_gated_extensions`: Enables gated extensions. Currently enables
    `all_extensions` and `RspViceMsp` features.
- Misc:
  - `std`: Turns on `std` (or turn off `no_std`, depending on how you prefer it).
    This currently doesn't do much besides internally using `std::error` instead
    of `core::error`, which may lower the MSRV.
  - `bindings_c`: Expose C bindings. NOT WORKING YET.
  - `pyo3`: Expose Python3 bindings. NOT WORKING YET.
    - Enables the `std`, `all_extensions` and `all_gated_extensions` features.
    - It also enables the `pyo3` dependency.
