# rab-disasmdis

![GitHub](https://img.shields.io/github/license/Decompollaborate/rabbitizer)
![crate.io](https://img.shields.io/crates/dv/rab-disasmdis)
![Crates.io MSRV](https://img.shields.io/crates/msrv/rab-disasmdis)
![GitHub License](https://img.shields.io/github/license/Decompollaborate/rabbitizer)
![GitHub contributors](https://img.shields.io/github/contributors/Decompollaborate/rabbitizer?logo=purple)

Disassemble MIPS instructions directly in your terminal, using the power of
rabbitizer and Rust.

## Basic usage

`rab-disasmdis` can disassemble the hexadecimal representation of MIPS
instructions by passing them as parameters.

Passing a big stream of bytes:

```bash
$ rab-disasmdis 000470803C028000004E102103E000088C420090
sll         $t6, $a0, 2
lui         $v0, 0x8000
addu        $v0, $v0, $t6
jr          $ra
lw          $v0, 0x90($v0)
```

Words with `0x` prefix:

```bash
$ rab-disasmdis 0x000470800x3C0280000x004E10210x03E000080x8C420090
sll         $t6, $a0, 2
lui         $v0, 0x8000
addu        $v0, $v0, $t6
jr          $ra
lw          $v0, 0x90($v0)
```

Passing individual bytes:

```bash
$ rab-disasmdis 0x00 0x04 0x70 0x80 0x3C 0x02 0x80 0x00 0x00 0x4E 0x10 0x21 0x03 0xE0 0x00 0x08 0x8C 0x42 0x00 0x90
sll         $t6, $a0, 2
lui         $v0, 0x8000
addu        $v0, $v0, $t6
jr          $ra
lw          $v0, 0x90($v0)
```

Omitting the `0x` prefix:

```bash
$ rab-disasmdis 00 04 70 80 3C 02 80 00 00 4E 10 21 03 E0 00 08 8C 42 00 90
sll         $t6, $a0, 2
lui         $v0, 0x8000
addu        $v0, $v0, $t6
jr          $ra
lw          $v0, 0x90($v0)
```

Little endian support (`-EL` or `--endian little`):

```bash
$ rab-disasmdis -EL 80 70 04 00 00 80 02 3C 21 10 4E 00 08 00 E0 03 90 00 42 8C
sll         $t6, $a0, 2
lui         $v0, 0x8000
addu        $v0, $v0, $t6
jr          $ra
lw          $v0, 0x90($v0)
```

## Installing

### Cargo

The recommended way for installing `rab-disasmdis` is by using Rust's `Cargo`.

Since `rabbitizer` is still on alpha, you'll need to specify a version.

```bash
cargo install rab-disasmdis@2.0.0-alpha.3
```

### Prebuilt binaries

Prebuilt binaries for the latest version of the CLI are available at the
[releases page on Github](https://github.com/Decompollaborate/rabbitizer/releases).
<!--
TODO change this link to
https://github.com/Decompollaborate/rabbitizer/releases/latest
when we do the first stable release
-->

## Versioning and changelog

This library follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
We try to always keep backwards compatibility, so no breaking changes should
happen until a major release (i.e. jumping from 1.X.X to 2.0.0).

<!--
TODO change this link when we do the first stable release
-->
To see what changed on each release check either the
[CHANGELOG.md](https://github.com/Decompollaborate/rabbitizer/blob/%F0%9F%A6%80/CHANGELOG.md)
file or check the [releases page on Github](https://github.com/Decompollaborate/rabbitizer/releases).
You can also use [this link](https://github.com/Decompollaborate/rabbitizer/releases/latest)
to check the latest release.
