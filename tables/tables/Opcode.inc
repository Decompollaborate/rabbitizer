/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
The macros for defining an opcode use the following syntax:

- RAB_DEF_OPCODE(prefix, case_bits, name, isa_version_min, isa_exten, ...)
  - The normal way to define an opcode.
- RAB_DEF_OPCODE_ALTNAME(prefix, case_bits, name, altname, isa_version_min, isa_exten, ...)
  - Define an opcode that doesn't use the `name` argument as the mnemonic.
    Useful for instructions with dots on their mnemonics.
- RAB_DEF_OPCODE_PSEUDO(prefix, name, isa_version_min, isa_exten, ...)
  - Defines a pseudo instruction.

Arguments:
- prefix: The name of the extension group.
- case_bits: A value used to match against a corresponding masked field to
  decode the instruction.
- name: Actual name of the instruction. Should be a valid token for programing
  languages. Will be used as the mnemonic of the disassembled instruction unless
  the macro contains an `altname` argument.
- altname: Used as mnemonic for the instruction in case the `name` argument was
  modified to be a valid token.
- isa_version_min: The ISA version that introduced this instruction. Should be
  EXTENSION if `isa_exten` is not NONE, or non-EXTENSION if `isa_exten` is NONE.
- isa_exten: The extension instruction set that this instruction belongs to.
- ...: Extra attributes for the opcode.

*/


    RAB_DEF_OPCODE(
        ALL, , INVALID, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )

    #include "opcodes/opcodes_core.inc"
    #include "opcodes/opcodes_rsp.inc"
    #include "opcodes/opcodes_r3000gte.inc"
    #include "opcodes/opcodes_r4000allegrex.inc"
    #include "opcodes/opcodes_r5900.inc"
