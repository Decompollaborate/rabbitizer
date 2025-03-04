/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, I, EncodedFieldMask::empty(), ())
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, Q, EncodedFieldMask::empty(), ())
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, R, EncodedFieldMask::empty(), ())
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, ACC, EncodedFieldMask::empty(), ())

    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, immediate5, EncodedFieldMask::r5900ee_immediate5, (i8))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, immediate15, EncodedFieldMask::r5900ee_immediate15, (u16))

    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vfs, EncodedFieldMask::r5900ee_vfs, (R5900EEVF))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vft, EncodedFieldMask::r5900ee_vft, (R5900EEVF))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vfd, EncodedFieldMask::r5900ee_vfd, (R5900EEVF))

    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vis, EncodedFieldMask::r5900ee_vis, (R5900EEVI))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vit, EncodedFieldMask::r5900ee_vit, (R5900EEVI))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vid, EncodedFieldMask::r5900ee_vid, (R5900EEVI))

    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, ACCxyzw, EncodedFieldMask::r5900ee_xyzw_xyzw, (bool, bool, bool, bool))

    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vfsxyzw, EncodedFieldMask::r5900ee_vfs.union(EncodedFieldMask::r5900ee_xyzw_xyzw), (R5900EEVF, bool, bool, bool, bool))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vftxyzw, EncodedFieldMask::r5900ee_vft.union(EncodedFieldMask::r5900ee_xyzw_xyzw), (R5900EEVF, bool, bool, bool, bool))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vfdxyzw, EncodedFieldMask::r5900ee_vfd.union(EncodedFieldMask::r5900ee_xyzw_xyzw), (R5900EEVF, bool, bool, bool, bool))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vftn, EncodedFieldMask::r5900ee_vft.union(EncodedFieldMask::r5900ee_n), (R5900EEVF, u8))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vfsl, EncodedFieldMask::r5900ee_vfs.union(EncodedFieldMask::r5900ee_l), (R5900EEVF, u8))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vftm, EncodedFieldMask::r5900ee_vft.union(EncodedFieldMask::r5900ee_m), (R5900EEVF, u8))

    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vis_predecr, EncodedFieldMask::r5900ee_vis, ((), R5900EEVI))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vit_predecr, EncodedFieldMask::r5900ee_vit, ((), R5900EEVI))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vis_postincr, EncodedFieldMask::r5900ee_vis, (R5900EEVI, ()))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vit_postincr, EncodedFieldMask::r5900ee_vit, (R5900EEVI, ()))
    RAB_DEF_OPERAND_EXT(R5900EE, r5900ee, vis_parenthesis, EncodedFieldMask::r5900ee_vis, (R5900EEVI))
