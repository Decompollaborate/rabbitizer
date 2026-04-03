/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
The macros for defining an instruction suffix use the following syntax:

- RAB_DEF_INSTRSUFFIX(prefix, name, isa_exten, ...)

Arguments:
- prefix: The name of the extension group.
- name: Name of the suffix kind.
- isa_exten: The extension instruction set that this instruction belongs to.
- ...: Extra attributes for the opcode.

*/

    RAB_DEF_INSTRSUFFIX(r5900ee, xyzw, R5900EE, )
