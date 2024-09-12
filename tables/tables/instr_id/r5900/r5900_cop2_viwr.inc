/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
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

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, (0x01 << 2) | 0x02, vilwr_w, vilwr.w,
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true
    ) // Integer load word register
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, (0x02 << 2) | 0x02, vilwr_z, vilwr.z,
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true
    ) // Integer load word register
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, (0x04 << 2) | 0x02, vilwr_y, vilwr.y,
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true
    ) // Integer load word register
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, (0x08 << 2) | 0x02, vilwr_x, vilwr.x,
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true
    ) // Integer load word register

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, (0x01 << 2) | 0x03, viswr_w, viswr.w,
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true
    ) // Integer store word register
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, (0x02 << 2) | 0x03, viswr_z, viswr.z,
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true
    ) // Integer store word register
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, (0x04 << 2) | 0x03, viswr_y, viswr.y,
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true
    ) // Integer store word register
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, (0x08 << 2) | 0x03, viswr_x, viswr.x,
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true
    ) // Integer store word register