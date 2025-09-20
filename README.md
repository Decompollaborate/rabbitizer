# rabbitizer

<!--[![PyPI - Downloads](https://img.shields.io/pypi/dm/rabbitizer)](https://pypi.org/project/rabbitizer/)-->
![GitHub](https://img.shields.io/github/license/Decompollaborate/rabbitizer)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/Decompollaborate/rabbitizer)
<!--![PyPI](https://img.shields.io/pypi/v/rabbitizer)-->
![crate.io](https://img.shields.io/crates/dv/rabbitizer)
![GitHub contributors](https://img.shields.io/github/contributors/Decompollaborate/rabbitizer?logo=purple)

rabbitizer (whole name is lowercase) is a MIPS instruction decoder API.

## Quick example

Check the [Rust-specific README](src/rabbitizer/README.md) for examples of the
Rust API.

`rab-disasmdis` is a an example program that allows decoding hex words on your
terminal. Check out [its README](src/rab-disasmdis/README.md).

`rab-disasmdis-web` contains the code for running a static website for decoding
and encoding MIPS instructions. It gets compiled to WASM, so no webserver is
needed. Check out [its README](src/rab-disasmdis-web/README.md).

## Features

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
  - ISAs are "feature gated", so you can only pay for what you use.
    - This may not be configurable if using a language binding other than the
      native Rust API.
- Includes an experimental instruction encoder, allowing to "assemble" MIPS asm
  instructions back to their hex representation.
  - This is not an assembler. Most of the features supported by an assembler
    are completely missing, without any plans to support them.
  - The supported syntax is pretty restrictive, only supporting the syntax
    emitted by the rabbitizer's decoder.
  - Explicit relocations are not supported.

### Planned features

- Bindings for other programming languages, so you can unleash rabbitizer's
  power on your prefered language.
  - Python bindings
  - C bindings.
  - C++ bindings

## Non-features

In order to keep it simple the following features will never be supported:

- Pseudo-instructions which expands to more than one instruction.

## Installation

rabbitizer is available on [crates.io](https://crates.io/crates/rabbitizer) and
can be included in your Cargo enabled project like this:

```toml
[dependencies]
rabbitizer = "2.0.0-alpha.7"
```

For more details and feature configuration, consult the
[Rust-specific README](src/rabbitizer/README.md#installation).

<!--
### Python bindings

The recommended way to install is using from the PyPi release, via `pip`:

```bash
python3 -m pip install -U rabbitizer
```

If you use a `requirements.txt` file in your repository, then you can add this
library with the following line:

```txt
rabbitizer>=1.12.0,<2.0.0
```

### Development version

The unstable development version is located at the
[develop](https://github.com/Decompollaborate/rabbitizer/tree/develop)
branch. PRs should be made into that branch instead of the main one.

Note that building the Python bindings from source require the Python
development package. Under Ubuntu/Debian based distros it can be installed with:

```bash
apt install python3-dev
```

In case you want to mess with the latest development version without wanting to
clone the repository, then you could use the following command:

```bash
python3 -m pip uninstall rabbitizer
python3 -m pip install git+https://github.com/Decompollaborate/rabbitizer.git@develop
```

NOTE: Installing the development version is not recommended. Proceed at your own
risk.

See this package at <https://pypi.org/project/rabbitizer/>.

### Rust bindings

Add this crate to your project with Cargo:

```bash
cargo add rabbitizer
```

Or you can add it manually to your `Cargo.toml`:

```toml
rabbitizer = "1.12.0"
```

See this crate at <https://crates.io/crates/rabbitizer>.

-->

## References

- MIPS CPU:
  - MIPS IV Instruction Set (Revision 3.2): <https://www.cs.cmu.edu/afs/cs/academic/class/15740-f97/public/doc/mips-isa.pdf>
  - MIPS Calling Convention Summary: <https://courses.cs.washington.edu/courses/cse410/09sp/examples/MIPSCallingConventionsSummary.pdf>
  - mipt-mips pseudo instructions: <https://github.com/MIPT-ILab/mipt-mips/wiki/MIPS-pseudo-instructions>
  - IDT R30xx Family Software Reference Manual, page 325 (for `rfe`): <https://psx.arthus.net/docs/R3000.pdf>
  - Community research on register's names based on each ABI.
    - <https://hackmd.io/mybJeh7IRE-3BYuv1uo4TQ>
    - <https://gist.github.com/EllipticEllipsis/27eef11205c7a59d8ea85632bc49224d#general-purpose-registers>

- N64's RSP (Reality Signal Processor):
  - Nintendo Ultra64 RSP Programmer’s Guide: <https://ultra64.ca/files/documentation/silicon-graphics/SGI_Nintendo_64_RSP_Programmers_Guide.pdf>
  - N64brew Reality Signal Processor/CPU Core: <https://n64brew.dev/wiki/Reality_Signal_Processor/CPU_Core>
  - Tharo's binutils rsp: <https://github.com/Thar0/binutils-rsp/>
    - VICE MSP binutils port: <https://web.archive.org/web/20220704165937/https://www.linux-mips.org/~glaurung/VICE/binutils-2.13-msp.diff>

- R3000 GTE:
  - PSYQ SDK headers: <https://github.com/FoxdieTeam/psyq_sdk/blob/master/psyq_4.4/INCLUDE/INLINE_A.H>
  - no$psx documentation: <https://problemkaputt.de/psxspx-gte-opcode-summary.htm>
  - no$psx documentation: <http://problemkaputt.de/psx-spx.htm#geometrytransformationenginegte>
  - <http://www.raphnet.net/electronique/psx_adaptor/Playstation.txt>

- R4000 ALLEGREX:
  - ALLEGREX-Instruction_Manual-English <https://github.com/Decompollaborate/rabbitizer/files/11356332/ALLEGREX-Instruction_Manual-English.pdf>
  - FPU-Instruction_Manual-English <https://github.com/Decompollaborate/rabbitizer/files/14950191/FPU-Instruction_Manual-English.pdf>
  - VFPU-Instruction_Manual-English <https://github.com/Decompollaborate/rabbitizer/files/11356335/VFPU-Instruction_Manual-English.pdf>
  - VFPU-Users_Manual-English <https://github.com/Decompollaborate/rabbitizer/files/11356333/VFPU-Users_Manual-English.pdf>
  - yet another PlayStationPortable Documentation <http://hitmen.c02.at/files/yapspd/psp_doc/frames.html>
    - Chapter "4.8  Allegrex Instructions" <http://hitmen.c02.at/files/yapspd/psp_doc/chap4.html#sec4.8>
  - GNU binutils: <https://github.com/bminor/binutils-gdb/compare/011365b...a0176d8>

- R5900EE:
  - EmotionEngine instruction decoding: <https://psi-rockin.github.io/ps2tek/#eeinstructiondecoding>
  - Official documentation from Toshiba: <https://wiki.qemu.org/images/2/2a/C790.pdf>
  - VU instruction manual: <http://lukasz.dk/files/vu-instruction-manual.pdf>
  - GNU binutils: <https://github.com/bminor/binutils-gdb/blob/master/opcodes/mips-opc.c>

## Versioning and changelog

This library follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
We try to always keep backwards compatibility, so no breaking changes should
happen until a major release (i.e. jumping from 1.X.X to 2.0.0).

To see what changed on each release check either the [CHANGELOG.md](CHANGELOG.md)
file or check the [releases page on Github](https://github.com/Decompollaborate/rabbitizer/releases).
You can also use [this link](https://github.com/Decompollaborate/rabbitizer/releases/latest)
to check the latest release.
