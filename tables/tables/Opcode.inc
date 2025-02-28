/* SPDX-FileCopyrightText: © 2023-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
The macros for defining an opcode use the following syntax:

- RAB_DEF_OPCODE_MIPS1(prefix, case_bits, name, ...)
  - Define a MIPS I opcode.
- RAB_DEF_OPCODE_MIPS1_ALTNAME(prefix, case_bits, name, altname, ...)
  - Define an opcode that doesn't use the `name` argument as the mnemonic.
    Useful for instructions with dots on their mnemonics.
- RAB_DEF_OPCODE_MIPS1_PSEUDO(prefix, name, ...)
  - Defines a pseudo instruction.

- RAB_DEF_OPCODE(prefix, case_bits, name, isa_version_min, ...)
  - Define a non-extension opcode.
- RAB_DEF_OPCODE_ALTNAME(prefix, case_bits, name, altname, isa_version_min, ...)
  - Define an opcode that doesn't use the `name` argument as the mnemonic.
    Useful for instructions with dots on their mnemonics.
- RAB_DEF_OPCODE_PSEUDO(prefix, name, isa_version_min, ...)
  - Defines a pseudo instruction.

- RAB_DEF_OPCODE_EXTENSION(prefix, case_bits, name, isa_exten, ...)
  - Define an opcode of an extension ISA.
- RAB_DEF_OPCODE_EXTENSION_ALTNAME(prefix, case_bits, name, altname, isa_exten, ...)
  - Define an opcode of an extension ISA.
- RAB_DEF_OPCODE_EXTENSION_GATED(prefix, case_bits, name, isa_exten, gate, ...)
  - Define an opcode that should be conditionally decoded depending on
    compile-time features and runtime decoding flags.
- RAB_DEF_OPCODE_EXTENSION_REMOVED(case_bits, prefix, name, isa_exten)
  - The opcode was removed by the given extension.

- RAB_DEF_OPCODE_REDIRECT(case_bits, prefix, kind)
  - Indicates a redirection is needed to decode the word.
  - This table will redirect to table `kind` from extension `prefix`.

Arguments:
- prefix: The name of the extension group.
- case_bits: A value used to match against a corresponding masked field to
  decode the instruction.
- name: Actual name of the instruction. Should be a valid token for programing
  languages. Will be used as the mnemonic of the disassembled instruction unless
  the macro contains an `altname` argument.
- altname: Used as mnemonic for the instruction in case the `name` argument was
  modified to be a valid token.
- kind: The table name to redirect to when decoding the instruction.
- isa_version_min: The ISA version that introduced this instruction. Should be
  EXTENSION if `isa_exten` is not NONE, or non-EXTENSION if `isa_exten` is NONE.
- isa_exten: The extension instruction set that this instruction belongs to.
- gate: The decoding flags needed to recognize this instruction.
- ...: Extra attributes for the opcode.

*/


    RAB_DEF_OPCODE_MIPS1(ALL, , INVALID, is_invalid: true)

    #include "opcodes/opcodes_core.inc"
    #include "opcodes/opcodes_rsp.inc"
    #include "opcodes/opcodes_r3000gte.inc"
    #include "opcodes/opcodes_r4000allegrex.inc"
    #include "opcodes/opcodes_r5900ee.inc"
