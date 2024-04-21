# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- The global `regNames.r4000AllegrexVfpuControlNamedRegisters` option controls
  if named registers should be used for the VFPU control registers of the R4000
  ALLEGREX.
- `Utils.floatRepr_32From16` function.
  - Converts a half float to a single precision float.
  - Both the argument and the return value correspond to their hex
    representation instead of an actual float.

### Changed

- Cleanups in tests code.

## [1.9.5] - 2024-04-03

### Changed

- Consider r5900's `paddub` as a possible move instruction.
- Internal rework to avoid allocating memory when calculating required buffer
  size for disassembly.
  - This is part of the `RabbitizerInstruction_getSizeForBuffer` function.
  - This change may help recent Windows specific issues.

## [1.9.4] - 2024-03-18

### Changed

- Always use named registers for R5900's VU instructions.

## [1.9.3] - 2024-03-17

### Fixed

- Fix the disassembly of `pref`.
- Fix typo on `c.seq.d`.
  - Was typed as `c.deq.d`.

## [1.9.2] - 2024-03-10

### Fixed

- Fix possible stack overflow if `immOverride` is larger than 255 bytes.

## [1.9.1] - 2024-02-18

### Fixed

- Fix PyPI releases being broken due to a GHA update.

## [1.9.0] - 2024-02-18

### Added

- Implement `neg` pseudo.

### Changed

- `Instruction.getProcessedImmediate` now raises an exception if the
  instruction does not contain an immediate field.
- `Instruction.getInstrIndexAsVram` now raises an exception if the
  instruction does not contain an jump label field.
- `Instruction.getBranchOffset` now raises an exception if the
  instruction does not contain an branch label field.
- Extend Python's `Instruction` documentation.
  - Thanks to @Dragorn421 (PR #53)

### Deprecated

- `Instruction.isImplemented`: Use `Instruction.isValid` instead.

## [1.8.3] - 2024-01-28

### Fixed

- Fix Windows issue where jumptables where not properly identified.
  - Caused because of an issue on `RegistersTracker.getJrInfo` not using the
    correct type width for the return value on the Python binding.

## [1.8.2] - 2024-01-28

### Fixed

- Fix Windows issue on which symbols were not properly being paired.
  - This was caused because of a misuse of the type widths on the Python
    binding.

## [1.8.1] - 2023-12-25

### Changed

- Minor cleanups on Rust code.
- Enabled `-Werror=type-limits` in the Makefile.

### Fixed

- `getBranchVramGeneric` returns an unsigned type instead of a signed one.

## [1.8.0] - 2023-11-12

### Added

- Add `flag_r5900DisasmAsData` member to the `Instruction` class.
  - This flag allows to fine-tune R5900 instruction set that are affected by
    the global `gnuMode` option.
    - Currently these instructions are: `trunc.w.s`, `cvt.w.s`, `vclipw` and
      `vsqrt`.
  - `TrinaryValue.TRUE` forces the instruction to be disassembled as data.
  - `TrinaryValue.FALSE` bypasses the global checks for disassembling a word
    as data. A word will still be disassembled as data if it can't be decoded.
  - `TrinaryValue.NONE` leaves this decision to the global settings.
- Add `flag_r5900UseDollar` member to the `Instruction` class.
  - `TrinaryValue.TRUE` forces the use of dollar signs ($) on R5900's VU
    instructions.
  - `TrinaryValue.FALSE` forces disassembling to not use of dollar signs ($) on
    R5900's VU instructions.
  - `TrinaryValue.NONE` leaves this decision to the global settings.

## [1.7.10] - 2023-09-24

### Added

- Add Python binding for `rabbitizer.__version_info__`
- Add Python binding for `rabbitizer.__version__`
- Add Python binding for `rabbitizer.__author__`
- Add `CHANGELOG.md`
- Add markdown linter to CI

### Fixed

- Fix passing `None` to third argument of `RegistersTracker.processLui` on the
  Python bindings
- Fix passing `None` to first argument of `RegistersTracker` on the Python bindings

## [1.7.9] - 2023-09-18

### Changed

- Don't attempt disassembling R5900's `vclipw` / `vsqrt` in gnu mode (#44)
  - Thanks @ethteck

## [1.7.8] - 2023-09-13

### Changed

- Consider `syscall` as an instruction that isn't emited by compilers
- Add `$` to special PS2 regs (#41)
  - Thanks @ethteck
- The `treatJAsUnconditionalBranch` option is no longer deprecated

### Fixed

- Fix size calculation required for the buffer of the disassembled instruction
  for a few edge cases
- Properly implement r5900's `vcallms` operands

## [1.7.7] - 2023-08-27

### Changed

- Reimplement hash function for Enum type.

### Fixed

- Fix type hints for Enum type missing the hash function.

## [1.7.6] - 2023-08-22

### Fixed

- Fix syscall having two arguments instead of just one

## [1.7.5] - 2023-08-08

### Changed

- Catch more errors and raise Python exceptions on them in the instruction
  disassembly binding

### Fixed

- Fix bug where the buffer size for a decoded instruction would be computed
  wrongly if the passed `extraLJust` parameter plus the global `opcodeLJust`
  option resulted in a negative value.

## [1.7.4] - 2023-06-13

### Added

- Provide macros to allow using gte instructions in both assembly and C. Those
  are provided under the [`docs/r3000gte`](./docs/r3000gte/) folder of the repository

### Fixed

- Fix R5900 vf registers not using the `$vf` prefix in numeric register mode.

## [1.7.3] - 2023-06-10

### Fixed

- Fix Rust release

## [1.7.2] - 2023-05-04

### Added

- Add `InstrIdType`
- Add `clippy` rust linter and fix the new warnings

### Changed

- Move all the tables and templates into the `tables/` folder
- Format and cleanup rust code

## [1.7.1] - 2023-05-02

### Changed

- General cleanups in the codebase. Not actual features or fixes were made

## [1.7.0] - 2023-04-30

### Added

- Support for R3000 GTE instructions (a.k.a. the PS1 specific instructions)
  - PR #31
- Add CI for checking the repo is always formatted
- Add CI for checking the tables have been regenerated

### Changed

- Only run the CI that builds the python bindings for every machine on new releases.
  - This action was by far the slowest, so this should speed up PRs
- Other Github Actions cleanups

## [1.6.2] - 2023-04-27

### Fixed

- Fix `getInstrIndexAsVram` taking the upper 8 bits instead of 4 bits for the
  vram calculation

## [1.6.1] - 2023-04-18

### Added

- Expose `fs`, `ft` and `fd` registers to Python bindings.
- Add enums for `RegCop1O32`, `RegCop1N32` and `RegCop1N64` registers for
  Python bindings

### Fixed

- Fix `Instruction`'s `vram` parameter not initialized if it was not passed to
  the constructor on the Python binding.

## [1.6.0] - 2023-04-17

### Added

- Adds `gnuMode` to the configuration.
  - Toggles various tweaks to allow building and matching with GNU `as`, which
    original compiler will not like.
  - Enabled by default.
  - Turning this mode off makes the `div $0,` pseudo instruction to not be used

### Changed

- Removed special treatment for R5900's `trunc.w.s`.
  - `cvt.w.s` and `trunc.w.s` will be decoded as-is when `gnuMode` is turned off.
  - If `gnuMode` is turned on this instruction those two instructions are
    decoded as `.word`s
- R5900's special operands `I`, `Q`, `R` and `ACC` will not longer be decoded
  with a `$` prefix
- All autogenerated files are added the `linguist-generated` attribute.

## [1.5.11] - 2023-04-02

- Fake version bump to convince CI to build binaries for Python 3.11
- Updates some Github Actions to newer versions

## [1.5.10] - 2023-02-23

### Added

- Add a unit test to ensure every bindings use the same version number
  - This is enforced in CI

## [1.5.9] - 2023-02-23

### Added

- Implement `mfc2`, `mtc2`, `cfc2` and `ctc2`
- Document C api usage

### Changed

- Expand list of instructions not emitted by C compilers

### Fixed

- Fix "emitted" typo

## [1.5.8] - 2022-12-20

### Added

- New `modifiesF*` and `readsF*`properties in the InstructionDescriptor

### Changed

- Column limit for C files has been changed to 120

### Fixed

- Fix source distribution of Python package

## [1.5.7] - 2022-12-20

### Fixed

- Fix a Rust binding not taking `&`

## [1.5.6] - 2022-12-19

### Added

- Introduce `RegisterDescriptor`
- `outputsToGprZero` method

### Removed

- `jalr_rd` has been removed. Its old conditional behavior is now handled with
  the new `cpu_maybe_rd_rs` operand

### Fixed

- Fix some typos in Rust bindings

## [1.5.5] - 2022-12-19

### Changed

- Change the global option `treatJAsUnconditionalBranch`'s default to `true`
- Cleanup Rust bindings to reduce useless indirections

### Fixed

- Actually package the C files in the Rust crate

## [1.5.4] - 2022-12-18

### Fixed

- Why Are We Still Here? Just To Suffer[?](https://knowyourmeme.com/memes/why-are-we-still-here-just-to-suffer)
  - Try to fix Rust bindings packaging

## [1.5.3] - 2022-12-18

### Fixed

- Trying to fix crate publishing, again again

## [1.5.2] - 2022-12-18

### Fixed

- Try to fix automatic Rust crate publishing again

## [1.5.1] - 2022-12-18

### Fixed

- Attempt to fix Rust crate publishing

## [1.5.0] - 2022-12-18

1.5.0: Rust bindings

### Added

- Adds Rust bindings
- The table-macro-hell has been changed a little to include pre-processed
  versions of all the tables in the repository. This change should be more
  friendly to IDEs

## [1.4.0] - 2022-12-16

### Added

- New methods to know if an instruction reads the value of a GPR: `.readsRs`,
  `.readsRd` and `.readsRt`
- Descriptor logic errors are now checked in CI

### Changed

- A lot of descriptor information where corrected

### Deprecated

- Deprecate `isJrRa()`, prefer new `isReturn()` method
- Deprecate `isJrNotRa()`, prefer new `isJumptableJump()` method

### Removed

- 3 non-existent RSP instructions where removed (`rsp_cache`, `rsp_lwc1` and
  `rsp_swc1`)
- Remove `setup.cfg` and move all the info to `pyproject.toml`

## [1.3.3] - 2022-11-30

### Fixed

- Fix RSP's jalr being marked as invalid
  - Thanks to @Mr-Wiseguy for noticing

## [1.3.2] - 2022-11-30

### Changed

- Check for `_INVALID` unique ids in `isValid`

### Fixed

- Fixed RSP using CPU cop0 .inc file instead of RSP cop0 and cop2 files (#13)
  - Thanks to @Mr-Wiseguy

## [1.3.1] - 2022-10-16

### Changed

- Removes signedness from `AccessType`

## [1.3.0] - 2022-10-15

### Added

- Adds C++ bindings
- New functions/methods:
  - `Instruction#getBranchOffsetGeneric()`: Like
    `Instruction#getGenericBranchOffset()`, but does not require the
    `currentVram` parameter
  - `Instruction#getBranchVramGeneric()`
  - `Instruction#getDestinationGpr()`
  - `Instruction#hasOperandAlias()`
  - `Instruction#isJumpWithAddress()`
  - `Instruction#readsHI()`
  - `Instruction#readsLO()`
  - `Instruction#modifiesHI()`
  - `Instruction#modifiesLO()`
  - `Instruction#getAccessType()`
  - `InstrCategory#fromStr()`
- New enums:
  - `OperandType`
  - `AccessType`

### Deprecated

- Deprecated functions/methods:
  - `Instruction#getImmediate()`: Use `Instruction#getProcessedImmediate()` instead
  - `Instruction#getGenericBranchOffset()`: Use
    `Instruction#getBranchOffsetGeneric()` instead
  - `Instruction#mapInstrToType()`: Use `Instruction#getAccessType()` instead
  - `Instruction#isUnknownType()`
  - `Instruction#isJType()`: Use `Instruction#isJumpWithAddress()` instead
  - `Instruction#isIType()`: Use
    `Instruction#hasOperandAlias(OperandType.cpu_immediate)` instead
  - `Instruction#isRType()`
  - `Instruction#isRegimmType()`

### Fixed

- Fix missing operands on some trap instructions
- Fix buffer size calculation for disassembly

## [1.2.2] - 2022-10-09

1.2.2: `bal` and proper `\\` escape

### Added

- Add `bal` support

### Changed

- Use special notation for branches which should produce matching instructions
  even when no `immOverride` was passed
- Escape `\` -> `\\` on `RabbitizerUtils_escapeString`
- Add `extern "C"` in every header

## [1.2.1] - 2022-09-26

1.2.1: Static library building in the Makefile

### Added

- Makefile now creates a `librabbitizer.a` file by default.
- Makefile can also build a `librabbitizer.so` with `make dynamic`
- New `include/rabbitizer.h` header which includes every other header
- Added a version header

## [1.2.0] - 2022-09-17

1.2.0: `%got` compatibility

### Added

- New methods in `RegistersTracker` to support tracking `%got` accesses
  - `processGpLoad`, which works similar to `processLui`
  - `preprocessLoAndGetInfo`, which replaces the now deprecated `getLuiOffsetForLo`

### Changed

- Move operand types to the table format. Operands were also renamed to a more
  concise name.

### Fixed

- Fix the wrong returned value on `Utils.From2Complement` when `bits` equals 32.

### Deprecated

- `RegistersTracker.getLuiOffsetForLo`. Use
  `RegistersTracker.preprocessLoAndGetInfo` instead.

## [1.1.0] - 2022-08-27

1.1.0: RSP and R5900 support

### Added

- Add proper instruction decoding for N64's RSP
- Add support for decoding the R5900 processor (PS2's Emotion Engine processor)

### Changed

- Cleanup internal instruction tables format

## [1.0.1] - 2022-07-12

### Added

- New function `Utils.escapeString`: Escapes escape characters.

### Changed

- Allow taking `None` in `Abi.fromStr`: returns `o32` in that case.
- Many cleanups and formats

## [1.0.0] - 2022-07-07

### Uncategorized

### Added

- New classes:
  - `RegistersTracker`: Intended to facilitate tracking the state of the general
    purpose registers.
- New enums:
  - `RegGprO32` and `RegGprN32`
- New configurations:
  - `misc_omit0XOnSmallImm`: If `True` then the leading `0x` of immediates in
    the [-9, 9] range is omitted. Defaults to `False`.
  - `misc_upperCaseImm`: If `True` then immediates are outputted in uppercase.
    Defaults to `True`.
- Many code cleanups

### Changed

- `Instruction` changes:
  - Constructor can accept `vram` and `category` parameters.
  - `isHiPair` renamed to `canBeHi`.
  - `isLoPair` renamed to `canBeLo`.
  - Added `doesLoad`, `doesStore`, `maybeIsMove`, `isPseudo` and
    `architectureVersion` to `InstrDescriptor`.
  - Python API: `Instruction#rs`, `Instruction#rt` and ``Instruction#rd` now
    return an enum gpr type and will raise an exception if the instruction does
    not reference the corresponding register.
  - New methods: `getOpcodeName`, `getProcessedImmediate`, `hasDelaySlot` and `isValid`.
  - Added `__reduce__` method to allow pickling the object.

## [0.1.0] - 2022-06-10

- First version

[unreleased]: https://github.com/Decompollaborate/rabbitizer/compare/master...develop
[1.9.5]: https://github.com/Decompollaborate/rabbitizer/compare/1.9.4...1.9.5
[1.9.4]: https://github.com/Decompollaborate/rabbitizer/compare/1.9.3...1.9.4
[1.9.3]: https://github.com/Decompollaborate/rabbitizer/compare/1.9.2...1.9.3
[1.9.2]: https://github.com/Decompollaborate/rabbitizer/compare/1.9.1...1.9.2
[1.9.1]: https://github.com/Decompollaborate/rabbitizer/compare/1.9.0...1.9.1
[1.9.0]: https://github.com/Decompollaborate/rabbitizer/compare/1.8.3...1.9.0
[1.8.3]: https://github.com/Decompollaborate/rabbitizer/compare/1.8.2...1.8.3
[1.8.2]: https://github.com/Decompollaborate/rabbitizer/compare/1.8.1...1.8.2
[1.8.1]: https://github.com/Decompollaborate/rabbitizer/compare/1.8.0...1.8.1
[1.8.0]: https://github.com/Decompollaborate/rabbitizer/compare/1.7.10...1.8.0
[1.7.10]: https://github.com/Decompollaborate/rabbitizer/compare/1.7.9...1.7.10
[1.7.9]: https://github.com/Decompollaborate/rabbitizer/compare/1.7.8...1.7.9
[1.7.8]: https://github.com/Decompollaborate/rabbitizer/compare/1.7.7...1.7.8
[1.7.7]: https://github.com/Decompollaborate/rabbitizer/compare/1.7.6...1.7.7
[1.7.6]: https://github.com/Decompollaborate/rabbitizer/compare/1.7.5...1.7.6
[1.7.5]: https://github.com/Decompollaborate/rabbitizer/compare/1.7.4...1.7.5
[1.7.4]: https://github.com/Decompollaborate/rabbitizer/compare/1.7.3...1.7.4
[1.7.3]: https://github.com/Decompollaborate/rabbitizer/compare/1.7.2...1.7.3
[1.7.2]: https://github.com/Decompollaborate/rabbitizer/compare/1.7.1...1.7.2
[1.7.1]: https://github.com/Decompollaborate/rabbitizer/compare/1.7.0...1.7.1
[1.7.0]: https://github.com/Decompollaborate/rabbitizer/compare/1.6.2...1.7.0
[1.6.2]: https://github.com/Decompollaborate/rabbitizer/compare/1.6.1...1.6.2
[1.6.1]: https://github.com/Decompollaborate/rabbitizer/compare/1.6.0...1.6.1
[1.6.0]: https://github.com/Decompollaborate/rabbitizer/compare/1.5.11...1.6.0
[1.5.11]: https://github.com/Decompollaborate/rabbitizer/compare/1.5.10...1.5.11
[1.5.10]: https://github.com/Decompollaborate/rabbitizer/compare/1.5.9...1.5.10
[1.5.9]: https://github.com/Decompollaborate/rabbitizer/compare/1.5.8...1.5.9
[1.5.8]: https://github.com/Decompollaborate/rabbitizer/compare/1.5.7...1.5.8
[1.5.7]: https://github.com/Decompollaborate/rabbitizer/compare/1.5.6...1.5.7
[1.5.6]: https://github.com/Decompollaborate/rabbitizer/compare/1.5.5...1.5.6
[1.5.5]: https://github.com/Decompollaborate/rabbitizer/compare/1.5.4...1.5.5
[1.5.4]: https://github.com/Decompollaborate/rabbitizer/compare/1.5.3...1.5.4
[1.5.3]: https://github.com/Decompollaborate/rabbitizer/compare/1.5.2...1.5.3
[1.5.2]: https://github.com/Decompollaborate/rabbitizer/compare/1.5.1...1.5.2
[1.5.1]: https://github.com/Decompollaborate/rabbitizer/compare/1.5.0...1.5.1
[1.5.0]: https://github.com/Decompollaborate/rabbitizer/compare/1.4.0...1.5.0
[1.4.0]: https://github.com/Decompollaborate/rabbitizer/compare/1.3.3...1.4.0
[1.3.3]: https://github.com/Decompollaborate/rabbitizer/compare/1.3.2...1.3.3
[1.3.2]: https://github.com/Decompollaborate/rabbitizer/compare/1.3.1...1.3.2
[1.3.1]: https://github.com/Decompollaborate/rabbitizer/compare/1.3.0...1.3.1
[1.3.0]: https://github.com/Decompollaborate/rabbitizer/compare/1.2.2...1.3.0
[1.2.2]: https://github.com/Decompollaborate/rabbitizer/compare/1.2.1...1.2.2
[1.2.1]: https://github.com/Decompollaborate/rabbitizer/compare/1.2.0...1.2.1
[1.2.0]: https://github.com/Decompollaborate/rabbitizer/compare/1.1.0...1.2.0
[1.1.0]: https://github.com/Decompollaborate/rabbitizer/compare/1.0.1...1.1.0
[1.0.1]: https://github.com/Decompollaborate/rabbitizer/compare/1.0.0...1.0.1
[1.0.0]: https://github.com/Decompollaborate/rabbitizer/compare/0.1.0...1.0.0
[0.1.0]: https://github.com/Decompollaborate/rabbitizer/releases/tag/0.1.0
