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
- (Experimental) N64 RSP disassembly support.
  - NOTE: This mode has not been tested to even be assemblable.
