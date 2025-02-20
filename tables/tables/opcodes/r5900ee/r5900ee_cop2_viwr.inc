/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
     31--------26-25-24--21---------------------10--------------2-1--0
     |    COP2   |1|  fhi  |                   |0 1 1 1 1 1 1 1 1|flo|
     ------6------1----4--------------------------------9----------2--
     |---00--|---01--|---10--|---11--| lo
0000 | ---   | ---   | ---   | ---   |
0001 | ---   | ---   |VILWR.W|VISWR.W|
0010 | ---   | ---   |VILWR.Z|VISWR.Z|
0011 | ---   | ---   | ---   | ---   |
0100 | ---   | ---   |VILWR.Y|VISWR.Y|
0101 | ---   | ---   | ---   | ---   |
0110 | ---   | ---   | ---   | ---   |
0111 | ---   | ---   | ---   | ---   |
1000 | ---   | ---   |VILWR.X|VISWR.X|
1001 | ---   | ---   | ---   | ---   |
1010 | ---   | ---   | ---   | ---   |
1011 | ---   | ---   | ---   | ---   |
1100 | ---   | ---   | ---   | ---   |
1101 | ---   | ---   | ---   | ---   |
1110 | ---   | ---   | ---   | ---   |
1111 | ---   | ---   | ---   | ---   |
 hi  |-------|-------|-------|-------|
*/

    RAB_DEF_OPCODE_ALTNAME(
        r5900ee, 0x0200002, vilwr_w, vilwr.w, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD)
    ) // Integer load word register
    RAB_DEF_OPCODE_ALTNAME(
        r5900ee, 0x0400002, vilwr_z, vilwr.z, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD)
    ) // Integer load word register
    RAB_DEF_OPCODE_ALTNAME(
        r5900ee, 0x0800002, vilwr_y, vilwr.y, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD)
    ) // Integer load word register
    RAB_DEF_OPCODE_ALTNAME(
        r5900ee, 0x1000002, vilwr_x, vilwr.x, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD)
    ) // Integer load word register

    RAB_DEF_OPCODE_ALTNAME(
        r5900ee, 0x0200003, viswr_w, viswr.w, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD)
    ) // Integer store word register
    RAB_DEF_OPCODE_ALTNAME(
        r5900ee, 0x0400003, viswr_z, viswr.z, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD)
    ) // Integer store word register
    RAB_DEF_OPCODE_ALTNAME(
        r5900ee, 0x0800003, viswr_y, viswr.y, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD)
    ) // Integer store word register
    RAB_DEF_OPCODE_ALTNAME(
        r5900ee, 0x1000003, viswr_x, viswr.x, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD)
    ) // Integer store word register
