# rabbitizer

[![PyPI - Downloads](https://img.shields.io/pypi/dm/rabbitizer)](https://pypi.org/project/rabbitizer/)
![GitHub](https://img.shields.io/github/license/Decompollaborate/rabbitizer)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/Decompollaborate/rabbitizer)
![PyPI](https://img.shields.io/pypi/v/rabbitizer)
![GitHub contributors](https://img.shields.io/github/contributors/Decompollaborate/rabbitizer?logo=purple)

A MIPS instruction decoder API.

Currently supports all the CPU instructions for MIPS I, II, III and IV.

## Installing

The recommended way to install is using from the PyPi release, via `pip`:

```bash
pip install rabbitizer
```

In case you want to mess with the latest development version without wanting to clone the repository, then you could use the following command:

```bash
pip install git+https://github.com/Decompollaborate/rabbitizer.git@develop
```

NOTE: Installing the development version is not recommended. Proceed at your own risk.

## Features

- Should produces matching assembly.
- Fully written in C for fast decoding.
- Python bindings.
  - The minimal Python version is 3.7, older versions are not guaranteed to work.
- C++ bindings
- Rust bindings
- Includes some minor tools to build your own pointer/symbol detection.
- Configurable, many features can be turned on and off.
- MIPS instructions features:
  - Named registers for MIPS VR4300's coprocessors.
  - Support for many pseudoinstructions.
  - Properly handle move to/from coprocessor instructions.
  - Support for numeric, o32, n32 and n64 ABI register names.
- Some workarounds for some specific compilers/assemblers:
  - `SN64`:
    - `div`/`divu` fix: tweaks a bit the produced `div`, `divu` and `break` instructions.
- N64 RSP instruction decoding support.
  - RSP decoding has been tested to build back to matching assemblies with [armips](https://github.com/Kingcom/armips/).
- R5900 (PS2's Emotion Engine processor) decoding support.

## References

- MIPS CPU:
  - MIPS IV Instruction Set (Revision 3.2): <https://www.cs.cmu.edu/afs/cs/academic/class/15740-f97/public/doc/mips-isa.pdf>
  - MIPS Calling Convention Summary: <https://courses.cs.washington.edu/courses/cse410/09sp/examples/MIPSCallingConventionsSummary.pdf>
  - mipt-mips pseudo instructions: <https://github.com/MIPT-ILab/mipt-mips/wiki/MIPS-pseudo-instructions>

- N64's RSP (Reality Signal Processor):
  - Nintendo Ultra64 RSP Programmer’s Guide: <https://ultra64.ca/files/documentation/silicon-graphics/SGI_Nintendo_64_RSP_Programmers_Guide.pdf>
  - N64brew Reality Signal Processor/CPU Core: <https://n64brew.dev/wiki/Reality_Signal_Processor/CPU_Core>

- R5900:
  - EmotionEngine instruction decoding: <https://psi-rockin.github.io/ps2tek/#eeinstructiondecoding>
  - Official documentation from Toshiba: <https://wiki.qemu.org/images/2/2a/C790.pdf>
  - VU instruction manual: <http://lukasz.dk/files/vu-instruction-manual.pdf>
  - GNU binutils: <https://github.com/bminor/binutils-gdb/blob/master/opcodes/mips-opc.c>
