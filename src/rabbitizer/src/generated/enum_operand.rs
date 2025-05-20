/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::encoded_field_mask::EncodedFieldMask;
use crate::operands::{OperandDescriptor, OperandDisplay, IU16, OPERAND_COUNT};
use crate::registers::*;
use crate::vram::{Vram, VramOffset};
use core::fmt;
use core::num::NonZeroU16;
#[cfg(feature = "R4000ALLEGREX")]
use ordered_float::OrderedFloat;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum Operand {
    ALL_EMPTY,
    core_rs,
    core_rt,
    core_rd,
    core_sa,
    core_zero,
    core_cop0d,
    core_cop0cd,
    core_fs,
    core_ft,
    core_fd,
    core_cop1cs,
    core_cop2t,
    core_cop2d,
    core_cop2cd,
    core_op,
    core_hint,
    core_code,
    core_code_lower,
    core_copraw,
    core_label,
    core_immediate,
    core_branch_target_label,
    core_immediate_base,
    core_maybe_rd_rs,
    core_maybe_zero_rs,
    #[cfg(feature = "RSP")]
    rsp_cop0d,
    #[cfg(feature = "RSP")]
    rsp_cop2cd,
    #[cfg(feature = "RSP")]
    rsp_vs,
    #[cfg(feature = "RSP")]
    rsp_vd,
    #[cfg(feature = "RSP")]
    rsp_vt_elementhigh,
    #[cfg(feature = "RSP")]
    rsp_vt_elementlow,
    #[cfg(feature = "RSP")]
    rsp_vd_de,
    #[cfg(feature = "RSP")]
    rsp_vs_index,
    #[cfg(feature = "RSP")]
    rsp_offset_rs,
    #[cfg(feature = "R3000GTE")]
    r3000gte_sf,
    #[cfg(feature = "R3000GTE")]
    r3000gte_mx,
    #[cfg(feature = "R3000GTE")]
    r3000gte_v,
    #[cfg(feature = "R3000GTE")]
    r3000gte_cv,
    #[cfg(feature = "R3000GTE")]
    r3000gte_lm,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_s_vs,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_s_vt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_s_vd,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_s_vt_imm,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_s_vd_imm,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_p_vs,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_p_vt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_p_vd,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_t_vs,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_t_vt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_t_vd,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_q_vs,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_q_vt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_q_vd,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_q_vt_imm,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mp_vs,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mp_vt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mp_vd,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mp_vs_transpose,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mt_vs,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mt_vt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mt_vd,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mt_vs_transpose,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mq_vs,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mq_vt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mq_vd,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mq_vs_transpose,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_cop2cs,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_cop2cd,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_pos,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_size,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_size_plus_pos,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_imm3,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_offset14_base,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_offset14_base_maybe_wb,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vconstant,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_power_of_two,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vfpu_cc_bit,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_bn,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_int16,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_float16,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_p_vrot_code,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_t_vrot_code,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_q_vrot_code,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_wpx,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_wpy,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_wpz,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_wpw,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_rpx,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_rpy,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_rpz,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_rpw,
    #[cfg(feature = "R5900EE")]
    r5900ee_I,
    #[cfg(feature = "R5900EE")]
    r5900ee_Q,
    #[cfg(feature = "R5900EE")]
    r5900ee_R,
    #[cfg(feature = "R5900EE")]
    r5900ee_ACC,
    #[cfg(feature = "R5900EE")]
    r5900ee_immediate5,
    #[cfg(feature = "R5900EE")]
    r5900ee_immediate15,
    #[cfg(feature = "R5900EE")]
    r5900ee_vfs,
    #[cfg(feature = "R5900EE")]
    r5900ee_vft,
    #[cfg(feature = "R5900EE")]
    r5900ee_vfd,
    #[cfg(feature = "R5900EE")]
    r5900ee_vis,
    #[cfg(feature = "R5900EE")]
    r5900ee_vit,
    #[cfg(feature = "R5900EE")]
    r5900ee_vid,
    #[cfg(feature = "R5900EE")]
    r5900ee_ACCxyzw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vfsxyzw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vftxyzw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vfdxyzw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vftn,
    #[cfg(feature = "R5900EE")]
    r5900ee_vfsl,
    #[cfg(feature = "R5900EE")]
    r5900ee_vftm,
    #[cfg(feature = "R5900EE")]
    r5900ee_vis_predecr,
    #[cfg(feature = "R5900EE")]
    r5900ee_vit_predecr,
    #[cfg(feature = "R5900EE")]
    r5900ee_vis_postincr,
    #[cfg(feature = "R5900EE")]
    r5900ee_vit_postincr,
    #[cfg(feature = "R5900EE")]
    r5900ee_vis_parenthesis,
}
pub static OPERANDS: [OperandDescriptor; OPERAND_COUNT] = {
    let mut table = [OperandDescriptor::default(); OPERAND_COUNT];
    {
        table[Operand::ALL_EMPTY as usize] =
            OperandDescriptor::new(concat!("ALL", "_", "EMPTY"), EncodedFieldMask::empty())
                .check_panic_chain();
    }
    {
        table[Operand::core_rs as usize] =
            OperandDescriptor::new(concat!("core", "_", "rs"), EncodedFieldMask::rs)
                .check_panic_chain();
    }
    {
        table[Operand::core_rt as usize] =
            OperandDescriptor::new(concat!("core", "_", "rt"), EncodedFieldMask::rt)
                .check_panic_chain();
    }
    {
        table[Operand::core_rd as usize] =
            OperandDescriptor::new(concat!("core", "_", "rd"), EncodedFieldMask::rd)
                .check_panic_chain();
    }
    {
        table[Operand::core_sa as usize] =
            OperandDescriptor::new(concat!("core", "_", "sa"), EncodedFieldMask::sa)
                .check_panic_chain();
    }
    {
        table[Operand::core_zero as usize] =
            OperandDescriptor::new(concat!("core", "_", "zero"), EncodedFieldMask::empty())
                .check_panic_chain();
    }
    {
        table[Operand::core_cop0d as usize] =
            OperandDescriptor::new(concat!("core", "_", "cop0d"), EncodedFieldMask::cop0d)
                .check_panic_chain();
    }
    {
        table[Operand::core_cop0cd as usize] =
            OperandDescriptor::new(concat!("core", "_", "cop0cd"), EncodedFieldMask::cop0cd)
                .check_panic_chain();
    }
    {
        table[Operand::core_fs as usize] =
            OperandDescriptor::new(concat!("core", "_", "fs"), EncodedFieldMask::fs)
                .check_panic_chain();
    }
    {
        table[Operand::core_ft as usize] =
            OperandDescriptor::new(concat!("core", "_", "ft"), EncodedFieldMask::ft)
                .check_panic_chain();
    }
    {
        table[Operand::core_fd as usize] =
            OperandDescriptor::new(concat!("core", "_", "fd"), EncodedFieldMask::fd)
                .check_panic_chain();
    }
    {
        table[Operand::core_cop1cs as usize] =
            OperandDescriptor::new(concat!("core", "_", "cop1cs"), EncodedFieldMask::cop1cs)
                .check_panic_chain();
    }
    {
        table[Operand::core_cop2t as usize] =
            OperandDescriptor::new(concat!("core", "_", "cop2t"), EncodedFieldMask::cop2t)
                .check_panic_chain();
    }
    {
        table[Operand::core_cop2d as usize] =
            OperandDescriptor::new(concat!("core", "_", "cop2d"), EncodedFieldMask::cop2d)
                .check_panic_chain();
    }
    {
        table[Operand::core_cop2cd as usize] =
            OperandDescriptor::new(concat!("core", "_", "cop2cd"), EncodedFieldMask::cop2cd)
                .check_panic_chain();
    }
    {
        table[Operand::core_op as usize] =
            OperandDescriptor::new(concat!("core", "_", "op"), EncodedFieldMask::op)
                .check_panic_chain();
    }
    {
        table[Operand::core_hint as usize] =
            OperandDescriptor::new(concat!("core", "_", "hint"), EncodedFieldMask::hint)
                .check_panic_chain();
    }
    {
        table[Operand::core_code as usize] =
            OperandDescriptor::new(concat!("core", "_", "code"), EncodedFieldMask::code)
                .check_panic_chain();
    }
    {
        table[Operand::core_code_lower as usize] = OperandDescriptor::new(
            concat!("core", "_", "code_lower"),
            EncodedFieldMask::code_lower,
        )
        .check_panic_chain();
    }
    {
        table[Operand::core_copraw as usize] =
            OperandDescriptor::new(concat!("core", "_", "copraw"), EncodedFieldMask::copraw)
                .check_panic_chain();
    }
    {
        table[Operand::core_label as usize] =
            OperandDescriptor::new(concat!("core", "_", "label"), EncodedFieldMask::instr_index)
                .check_panic_chain();
    }
    {
        table[Operand::core_immediate as usize] = OperandDescriptor::new(
            concat!("core", "_", "immediate"),
            EncodedFieldMask::immediate,
        )
        .check_panic_chain();
    }
    {
        table[Operand::core_branch_target_label as usize] = OperandDescriptor::new(
            concat!("core", "_", "branch_target_label"),
            EncodedFieldMask::immediate,
        )
        .check_panic_chain();
    }
    {
        table[Operand::core_immediate_base as usize] = OperandDescriptor::new(
            concat!("core", "_", "immediate_base"),
            EncodedFieldMask::immediate.union(EncodedFieldMask::rs),
        )
        .check_panic_chain();
    }
    {
        table[Operand::core_maybe_rd_rs as usize] = OperandDescriptor::new(
            concat!("core", "_", "maybe_rd_rs"),
            EncodedFieldMask::rd.union(EncodedFieldMask::rs),
        )
        .check_panic_chain();
    }
    {
        table[Operand::core_maybe_zero_rs as usize] = OperandDescriptor::new(
            concat!("core", "_", "maybe_zero_rs"),
            EncodedFieldMask::empty().union(EncodedFieldMask::rs),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Operand::rsp_cop0d as usize] =
            OperandDescriptor::new(concat!("rsp", "_", "cop0d"), EncodedFieldMask::cop0d)
                .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Operand::rsp_cop2cd as usize] =
            OperandDescriptor::new(concat!("rsp", "_", "cop2cd"), EncodedFieldMask::cop2cd)
                .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Operand::rsp_vs as usize] =
            OperandDescriptor::new(concat!("rsp", "_", "vs"), EncodedFieldMask::rsp_vs)
                .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Operand::rsp_vd as usize] =
            OperandDescriptor::new(concat!("rsp", "_", "vd"), EncodedFieldMask::rsp_vd)
                .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Operand::rsp_vt_elementhigh as usize] = OperandDescriptor::new(
            concat!("rsp", "_", "vt_elementhigh"),
            EncodedFieldMask::rsp_vt.union(EncodedFieldMask::rsp_elementhigh),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Operand::rsp_vt_elementlow as usize] = OperandDescriptor::new(
            concat!("rsp", "_", "vt_elementlow"),
            EncodedFieldMask::rsp_vt.union(EncodedFieldMask::rsp_elementlow),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Operand::rsp_vd_de as usize] = OperandDescriptor::new(
            concat!("rsp", "_", "vd_de"),
            EncodedFieldMask::rsp_vd.union(EncodedFieldMask::rsp_de),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Operand::rsp_vs_index as usize] = OperandDescriptor::new(
            concat!("rsp", "_", "vs_index"),
            EncodedFieldMask::rsp_vs.union(EncodedFieldMask::rsp_index),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Operand::rsp_offset_rs as usize] = OperandDescriptor::new(
            concat!("rsp", "_", "offset_rs"),
            EncodedFieldMask::rsp_offset.union(EncodedFieldMask::rs),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Operand::r3000gte_sf as usize] = OperandDescriptor::new(
            concat!("r3000gte", "_", "sf"),
            EncodedFieldMask::r3000gte_sf,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Operand::r3000gte_mx as usize] = OperandDescriptor::new(
            concat!("r3000gte", "_", "mx"),
            EncodedFieldMask::r3000gte_mx,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Operand::r3000gte_v as usize] =
            OperandDescriptor::new(concat!("r3000gte", "_", "v"), EncodedFieldMask::r3000gte_v)
                .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Operand::r3000gte_cv as usize] = OperandDescriptor::new(
            concat!("r3000gte", "_", "cv"),
            EncodedFieldMask::r3000gte_cv,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Operand::r3000gte_lm as usize] = OperandDescriptor::new(
            concat!("r3000gte", "_", "lm"),
            EncodedFieldMask::r3000gte_lm,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_s_vs as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "s_vs"),
            EncodedFieldMask::r4000allegrex_vs,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_s_vt as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "s_vt"),
            EncodedFieldMask::r4000allegrex_vt,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_s_vd as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "s_vd"),
            EncodedFieldMask::r4000allegrex_vd,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_s_vt_imm as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "s_vt_imm"),
            EncodedFieldMask::r4000allegrex_vt_imm,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_s_vd_imm as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "s_vd_imm"),
            EncodedFieldMask::r4000allegrex_vd_imm,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_p_vs as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "p_vs"),
            EncodedFieldMask::r4000allegrex_vs,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_p_vt as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "p_vt"),
            EncodedFieldMask::r4000allegrex_vt,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_p_vd as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "p_vd"),
            EncodedFieldMask::r4000allegrex_vd,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_t_vs as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "t_vs"),
            EncodedFieldMask::r4000allegrex_vs,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_t_vt as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "t_vt"),
            EncodedFieldMask::r4000allegrex_vt,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_t_vd as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "t_vd"),
            EncodedFieldMask::r4000allegrex_vd,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_q_vs as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "q_vs"),
            EncodedFieldMask::r4000allegrex_vs,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_q_vt as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "q_vt"),
            EncodedFieldMask::r4000allegrex_vt,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_q_vd as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "q_vd"),
            EncodedFieldMask::r4000allegrex_vd,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_q_vt_imm as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "q_vt_imm"),
            EncodedFieldMask::r4000allegrex_vt_6_imm,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_mp_vs as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "mp_vs"),
            EncodedFieldMask::r4000allegrex_vs,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_mp_vt as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "mp_vt"),
            EncodedFieldMask::r4000allegrex_vt,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_mp_vd as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "mp_vd"),
            EncodedFieldMask::r4000allegrex_vd,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_mp_vs_transpose as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "mp_vs_transpose"),
            EncodedFieldMask::r4000allegrex_vs,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_mt_vs as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "mt_vs"),
            EncodedFieldMask::r4000allegrex_vs,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_mt_vt as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "mt_vt"),
            EncodedFieldMask::r4000allegrex_vt,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_mt_vd as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "mt_vd"),
            EncodedFieldMask::r4000allegrex_vd,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_mt_vs_transpose as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "mt_vs_transpose"),
            EncodedFieldMask::r4000allegrex_vs,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_mq_vs as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "mq_vs"),
            EncodedFieldMask::r4000allegrex_vs,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_mq_vt as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "mq_vt"),
            EncodedFieldMask::r4000allegrex_vt,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_mq_vd as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "mq_vd"),
            EncodedFieldMask::r4000allegrex_vd,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_mq_vs_transpose as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "mq_vs_transpose"),
            EncodedFieldMask::r4000allegrex_vs,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_cop2cs as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "cop2cs"),
            EncodedFieldMask::r4000allegrex_cop2cs,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_cop2cd as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "cop2cd"),
            EncodedFieldMask::r4000allegrex_cop2cd,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_pos as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "pos"),
            EncodedFieldMask::r4000allegrex_pos,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_size as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "size"),
            EncodedFieldMask::r4000allegrex_size,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_size_plus_pos as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "size_plus_pos"),
            EncodedFieldMask::r4000allegrex_size_plus_pos,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_imm3 as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "imm3"),
            EncodedFieldMask::r4000allegrex_imm3,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_offset14_base as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "offset14_base"),
            EncodedFieldMask::r4000allegrex_offset14.union(EncodedFieldMask::rs),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_offset14_base_maybe_wb as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "offset14_base_maybe_wb"),
            EncodedFieldMask::r4000allegrex_offset14
                .union(EncodedFieldMask::rs)
                .union(EncodedFieldMask::r4000allegrex_wb),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt as usize] =
            OperandDescriptor::new(
                concat!("r4000allegrex", "_", "vcmp_cond_s_maybe_vs_maybe_vt"),
                EncodedFieldMask::r4000allegrex_vcmp_cond
                    .union(EncodedFieldMask::r4000allegrex_vs)
                    .union(EncodedFieldMask::r4000allegrex_vt),
            )
            .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt as usize] =
            OperandDescriptor::new(
                concat!("r4000allegrex", "_", "vcmp_cond_p_maybe_vs_maybe_vt"),
                EncodedFieldMask::r4000allegrex_vcmp_cond
                    .union(EncodedFieldMask::r4000allegrex_vs)
                    .union(EncodedFieldMask::r4000allegrex_vt),
            )
            .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt as usize] =
            OperandDescriptor::new(
                concat!("r4000allegrex", "_", "vcmp_cond_t_maybe_vs_maybe_vt"),
                EncodedFieldMask::r4000allegrex_vcmp_cond
                    .union(EncodedFieldMask::r4000allegrex_vs)
                    .union(EncodedFieldMask::r4000allegrex_vt),
            )
            .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt as usize] =
            OperandDescriptor::new(
                concat!("r4000allegrex", "_", "vcmp_cond_q_maybe_vs_maybe_vt"),
                EncodedFieldMask::r4000allegrex_vcmp_cond
                    .union(EncodedFieldMask::r4000allegrex_vs)
                    .union(EncodedFieldMask::r4000allegrex_vt),
            )
            .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_vconstant as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "vconstant"),
            EncodedFieldMask::r4000allegrex_vconstant,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_power_of_two as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "power_of_two"),
            EncodedFieldMask::r4000allegrex_power_of_two,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_vfpu_cc_bit as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "vfpu_cc_bit"),
            EncodedFieldMask::r4000allegrex_vfpu_cc_bit,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_bn as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "bn"),
            EncodedFieldMask::r4000allegrex_bn,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_int16 as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "int16"),
            EncodedFieldMask::r4000allegrex_intfloat16,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_float16 as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "float16"),
            EncodedFieldMask::r4000allegrex_intfloat16,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_p_vrot_code as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "p_vrot_code"),
            EncodedFieldMask::r4000allegrex_vrot_code,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_t_vrot_code as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "t_vrot_code"),
            EncodedFieldMask::r4000allegrex_vrot_code,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_q_vrot_code as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "q_vrot_code"),
            EncodedFieldMask::r4000allegrex_vrot_code,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_wpx as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "wpx"),
            EncodedFieldMask::r4000allegrex_wpx,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_wpy as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "wpy"),
            EncodedFieldMask::r4000allegrex_wpy,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_wpz as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "wpz"),
            EncodedFieldMask::r4000allegrex_wpz,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_wpw as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "wpw"),
            EncodedFieldMask::r4000allegrex_wpw,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_rpx as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "rpx"),
            EncodedFieldMask::r4000allegrex_rpx,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_rpy as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "rpy"),
            EncodedFieldMask::r4000allegrex_rpy,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_rpz as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "rpz"),
            EncodedFieldMask::r4000allegrex_rpz,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Operand::r4000allegrex_rpw as usize] = OperandDescriptor::new(
            concat!("r4000allegrex", "_", "rpw"),
            EncodedFieldMask::r4000allegrex_rpw,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_I as usize] =
            OperandDescriptor::new(concat!("r5900ee", "_", "I"), EncodedFieldMask::empty())
                .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_Q as usize] =
            OperandDescriptor::new(concat!("r5900ee", "_", "Q"), EncodedFieldMask::empty())
                .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_R as usize] =
            OperandDescriptor::new(concat!("r5900ee", "_", "R"), EncodedFieldMask::empty())
                .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_ACC as usize] =
            OperandDescriptor::new(concat!("r5900ee", "_", "ACC"), EncodedFieldMask::empty())
                .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_immediate5 as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "immediate5"),
            EncodedFieldMask::r5900ee_immediate5,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_immediate15 as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "immediate15"),
            EncodedFieldMask::r5900ee_immediate15,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vfs as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vfs"),
            EncodedFieldMask::r5900ee_vfs,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vft as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vft"),
            EncodedFieldMask::r5900ee_vft,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vfd as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vfd"),
            EncodedFieldMask::r5900ee_vfd,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vis as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vis"),
            EncodedFieldMask::r5900ee_vis,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vit as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vit"),
            EncodedFieldMask::r5900ee_vit,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vid as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vid"),
            EncodedFieldMask::r5900ee_vid,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_ACCxyzw as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "ACCxyzw"),
            EncodedFieldMask::r5900ee_xyzw_xyzw,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vfsxyzw as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vfsxyzw"),
            EncodedFieldMask::r5900ee_vfs.union(EncodedFieldMask::r5900ee_xyzw_xyzw),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vftxyzw as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vftxyzw"),
            EncodedFieldMask::r5900ee_vft.union(EncodedFieldMask::r5900ee_xyzw_xyzw),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vfdxyzw as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vfdxyzw"),
            EncodedFieldMask::r5900ee_vfd.union(EncodedFieldMask::r5900ee_xyzw_xyzw),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vftn as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vftn"),
            EncodedFieldMask::r5900ee_vft.union(EncodedFieldMask::r5900ee_n),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vfsl as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vfsl"),
            EncodedFieldMask::r5900ee_vfs.union(EncodedFieldMask::r5900ee_l),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vftm as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vftm"),
            EncodedFieldMask::r5900ee_vft.union(EncodedFieldMask::r5900ee_m),
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vis_predecr as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vis_predecr"),
            EncodedFieldMask::r5900ee_vis,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vit_predecr as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vit_predecr"),
            EncodedFieldMask::r5900ee_vit,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vis_postincr as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vis_postincr"),
            EncodedFieldMask::r5900ee_vis,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vit_postincr as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vit_postincr"),
            EncodedFieldMask::r5900ee_vit,
        )
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Operand::r5900ee_vis_parenthesis as usize] = OperandDescriptor::new(
            concat!("r5900ee", "_", "vis_parenthesis"),
            EncodedFieldMask::r5900ee_vis,
        )
        .check_panic_chain();
    }
    let mut i = 0;
    while i < OPERAND_COUNT {
        table[i].check_panic();
        i += 1;
    }
    table
};
impl<T> OperandDisplay<'_, '_, T>
where
    T: fmt::Display,
{
    pub(crate) fn display_this_operand(
        &self,
        f: &mut fmt::Formatter<'_>, // '
    ) -> fmt::Result {
        match self.operand() {
            Operand::ALL_EMPTY => OperandDisplay::display_ALL_EMPTY(self, f),
            Operand::core_rs => OperandDisplay::display_core_rs(self, f),
            Operand::core_rt => OperandDisplay::display_core_rt(self, f),
            Operand::core_rd => OperandDisplay::display_core_rd(self, f),
            Operand::core_sa => OperandDisplay::display_core_sa(self, f),
            Operand::core_zero => OperandDisplay::display_core_zero(self, f),
            Operand::core_cop0d => OperandDisplay::display_core_cop0d(self, f),
            Operand::core_cop0cd => OperandDisplay::display_core_cop0cd(self, f),
            Operand::core_fs => OperandDisplay::display_core_fs(self, f),
            Operand::core_ft => OperandDisplay::display_core_ft(self, f),
            Operand::core_fd => OperandDisplay::display_core_fd(self, f),
            Operand::core_cop1cs => OperandDisplay::display_core_cop1cs(self, f),
            Operand::core_cop2t => OperandDisplay::display_core_cop2t(self, f),
            Operand::core_cop2d => OperandDisplay::display_core_cop2d(self, f),
            Operand::core_cop2cd => OperandDisplay::display_core_cop2cd(self, f),
            Operand::core_op => OperandDisplay::display_core_op(self, f),
            Operand::core_hint => OperandDisplay::display_core_hint(self, f),
            Operand::core_code => OperandDisplay::display_core_code(self, f),
            Operand::core_code_lower => OperandDisplay::display_core_code_lower(self, f),
            Operand::core_copraw => OperandDisplay::display_core_copraw(self, f),
            Operand::core_label => OperandDisplay::display_core_label(self, f),
            Operand::core_immediate => OperandDisplay::display_core_immediate(self, f),
            Operand::core_branch_target_label => {
                OperandDisplay::display_core_branch_target_label(self, f)
            }
            Operand::core_immediate_base => OperandDisplay::display_core_immediate_base(self, f),
            Operand::core_maybe_rd_rs => OperandDisplay::display_core_maybe_rd_rs(self, f),
            Operand::core_maybe_zero_rs => OperandDisplay::display_core_maybe_zero_rs(self, f),
            #[cfg(feature = "RSP")]
            Operand::rsp_cop0d => OperandDisplay::display_rsp_cop0d(self, f),
            #[cfg(feature = "RSP")]
            Operand::rsp_cop2cd => OperandDisplay::display_rsp_cop2cd(self, f),
            #[cfg(feature = "RSP")]
            Operand::rsp_vs => OperandDisplay::display_rsp_vs(self, f),
            #[cfg(feature = "RSP")]
            Operand::rsp_vd => OperandDisplay::display_rsp_vd(self, f),
            #[cfg(feature = "RSP")]
            Operand::rsp_vt_elementhigh => OperandDisplay::display_rsp_vt_elementhigh(self, f),
            #[cfg(feature = "RSP")]
            Operand::rsp_vt_elementlow => OperandDisplay::display_rsp_vt_elementlow(self, f),
            #[cfg(feature = "RSP")]
            Operand::rsp_vd_de => OperandDisplay::display_rsp_vd_de(self, f),
            #[cfg(feature = "RSP")]
            Operand::rsp_vs_index => OperandDisplay::display_rsp_vs_index(self, f),
            #[cfg(feature = "RSP")]
            Operand::rsp_offset_rs => OperandDisplay::display_rsp_offset_rs(self, f),
            #[cfg(feature = "R3000GTE")]
            Operand::r3000gte_sf => OperandDisplay::display_r3000gte_sf(self, f),
            #[cfg(feature = "R3000GTE")]
            Operand::r3000gte_mx => OperandDisplay::display_r3000gte_mx(self, f),
            #[cfg(feature = "R3000GTE")]
            Operand::r3000gte_v => OperandDisplay::display_r3000gte_v(self, f),
            #[cfg(feature = "R3000GTE")]
            Operand::r3000gte_cv => OperandDisplay::display_r3000gte_cv(self, f),
            #[cfg(feature = "R3000GTE")]
            Operand::r3000gte_lm => OperandDisplay::display_r3000gte_lm(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_s_vs => OperandDisplay::display_r4000allegrex_s_vs(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_s_vt => OperandDisplay::display_r4000allegrex_s_vt(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_s_vd => OperandDisplay::display_r4000allegrex_s_vd(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_s_vt_imm => {
                OperandDisplay::display_r4000allegrex_s_vt_imm(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_s_vd_imm => {
                OperandDisplay::display_r4000allegrex_s_vd_imm(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_p_vs => OperandDisplay::display_r4000allegrex_p_vs(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_p_vt => OperandDisplay::display_r4000allegrex_p_vt(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_p_vd => OperandDisplay::display_r4000allegrex_p_vd(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_t_vs => OperandDisplay::display_r4000allegrex_t_vs(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_t_vt => OperandDisplay::display_r4000allegrex_t_vt(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_t_vd => OperandDisplay::display_r4000allegrex_t_vd(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_q_vs => OperandDisplay::display_r4000allegrex_q_vs(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_q_vt => OperandDisplay::display_r4000allegrex_q_vt(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_q_vd => OperandDisplay::display_r4000allegrex_q_vd(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_q_vt_imm => {
                OperandDisplay::display_r4000allegrex_q_vt_imm(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mp_vs => OperandDisplay::display_r4000allegrex_mp_vs(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mp_vt => OperandDisplay::display_r4000allegrex_mp_vt(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mp_vd => OperandDisplay::display_r4000allegrex_mp_vd(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mp_vs_transpose => {
                OperandDisplay::display_r4000allegrex_mp_vs_transpose(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mt_vs => OperandDisplay::display_r4000allegrex_mt_vs(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mt_vt => OperandDisplay::display_r4000allegrex_mt_vt(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mt_vd => OperandDisplay::display_r4000allegrex_mt_vd(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mt_vs_transpose => {
                OperandDisplay::display_r4000allegrex_mt_vs_transpose(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mq_vs => OperandDisplay::display_r4000allegrex_mq_vs(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mq_vt => OperandDisplay::display_r4000allegrex_mq_vt(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mq_vd => OperandDisplay::display_r4000allegrex_mq_vd(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_mq_vs_transpose => {
                OperandDisplay::display_r4000allegrex_mq_vs_transpose(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_cop2cs => OperandDisplay::display_r4000allegrex_cop2cs(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_cop2cd => OperandDisplay::display_r4000allegrex_cop2cd(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_pos => OperandDisplay::display_r4000allegrex_pos(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_size => OperandDisplay::display_r4000allegrex_size(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_size_plus_pos => {
                OperandDisplay::display_r4000allegrex_size_plus_pos(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_imm3 => OperandDisplay::display_r4000allegrex_imm3(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_offset14_base => {
                OperandDisplay::display_r4000allegrex_offset14_base(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_offset14_base_maybe_wb => {
                OperandDisplay::display_r4000allegrex_offset14_base_maybe_wb(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt => {
                OperandDisplay::display_r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt => {
                OperandDisplay::display_r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt => {
                OperandDisplay::display_r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt => {
                OperandDisplay::display_r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_vconstant => {
                OperandDisplay::display_r4000allegrex_vconstant(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_power_of_two => {
                OperandDisplay::display_r4000allegrex_power_of_two(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_vfpu_cc_bit => {
                OperandDisplay::display_r4000allegrex_vfpu_cc_bit(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_bn => OperandDisplay::display_r4000allegrex_bn(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_int16 => OperandDisplay::display_r4000allegrex_int16(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_float16 => {
                OperandDisplay::display_r4000allegrex_float16(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_p_vrot_code => {
                OperandDisplay::display_r4000allegrex_p_vrot_code(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_t_vrot_code => {
                OperandDisplay::display_r4000allegrex_t_vrot_code(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_q_vrot_code => {
                OperandDisplay::display_r4000allegrex_q_vrot_code(self, f)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_wpx => OperandDisplay::display_r4000allegrex_wpx(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_wpy => OperandDisplay::display_r4000allegrex_wpy(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_wpz => OperandDisplay::display_r4000allegrex_wpz(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_wpw => OperandDisplay::display_r4000allegrex_wpw(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_rpx => OperandDisplay::display_r4000allegrex_rpx(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_rpy => OperandDisplay::display_r4000allegrex_rpy(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_rpz => OperandDisplay::display_r4000allegrex_rpz(self, f),
            #[cfg(feature = "R4000ALLEGREX")]
            Operand::r4000allegrex_rpw => OperandDisplay::display_r4000allegrex_rpw(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_I => OperandDisplay::display_r5900ee_I(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_Q => OperandDisplay::display_r5900ee_Q(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_R => OperandDisplay::display_r5900ee_R(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_ACC => OperandDisplay::display_r5900ee_ACC(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_immediate5 => OperandDisplay::display_r5900ee_immediate5(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_immediate15 => OperandDisplay::display_r5900ee_immediate15(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vfs => OperandDisplay::display_r5900ee_vfs(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vft => OperandDisplay::display_r5900ee_vft(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vfd => OperandDisplay::display_r5900ee_vfd(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vis => OperandDisplay::display_r5900ee_vis(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vit => OperandDisplay::display_r5900ee_vit(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vid => OperandDisplay::display_r5900ee_vid(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_ACCxyzw => OperandDisplay::display_r5900ee_ACCxyzw(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vfsxyzw => OperandDisplay::display_r5900ee_vfsxyzw(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vftxyzw => OperandDisplay::display_r5900ee_vftxyzw(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vfdxyzw => OperandDisplay::display_r5900ee_vfdxyzw(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vftn => OperandDisplay::display_r5900ee_vftn(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vfsl => OperandDisplay::display_r5900ee_vfsl(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vftm => OperandDisplay::display_r5900ee_vftm(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vis_predecr => OperandDisplay::display_r5900ee_vis_predecr(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vit_predecr => OperandDisplay::display_r5900ee_vit_predecr(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vis_postincr => OperandDisplay::display_r5900ee_vis_postincr(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vit_postincr => OperandDisplay::display_r5900ee_vit_postincr(self, f),
            #[cfg(feature = "R5900EE")]
            Operand::r5900ee_vis_parenthesis => {
                OperandDisplay::display_r5900ee_vis_parenthesis(self, f)
            }
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum ValuedOperand {
    ALL_EMPTY(),
    core_rs(Gpr),
    core_rt(Gpr),
    core_rd(Gpr),
    core_sa(u8),
    core_zero(),
    core_cop0d(Cop0),
    core_cop0cd(Cop0Control),
    core_fs(Cop1),
    core_ft(Cop1),
    core_fd(Cop1),
    core_cop1cs(Cop1Control),
    core_cop2t(Cop2),
    core_cop2d(Cop2),
    core_cop2cd(Cop2Control),
    core_op(u8),
    core_hint(u8),
    core_code(u16, Option<NonZeroU16>),
    core_code_lower(u16),
    core_copraw(u32),
    core_label(Vram),
    core_immediate(IU16),
    core_branch_target_label(VramOffset),
    core_immediate_base(IU16, Gpr),
    core_maybe_rd_rs(Option<Gpr>, Gpr),
    core_maybe_zero_rs((), Gpr),
    #[cfg(feature = "RSP")]
    rsp_cop0d(RspCop0),
    #[cfg(feature = "RSP")]
    rsp_cop2cd(RspCop2),
    #[cfg(feature = "RSP")]
    rsp_vs(RspVector),
    #[cfg(feature = "RSP")]
    rsp_vd(RspVector),
    #[cfg(feature = "RSP")]
    rsp_vt_elementhigh(RspVector, u8),
    #[cfg(feature = "RSP")]
    rsp_vt_elementlow(RspVector, u8),
    #[cfg(feature = "RSP")]
    rsp_vd_de(RspVector, u8),
    #[cfg(feature = "RSP")]
    rsp_vs_index(RspVector, u8),
    #[cfg(feature = "RSP")]
    rsp_offset_rs(u16, Gpr),
    #[cfg(feature = "R3000GTE")]
    r3000gte_sf(u8),
    #[cfg(feature = "R3000GTE")]
    r3000gte_mx(u8),
    #[cfg(feature = "R3000GTE")]
    r3000gte_v(u8),
    #[cfg(feature = "R3000GTE")]
    r3000gte_cv(u8),
    #[cfg(feature = "R3000GTE")]
    r3000gte_lm(u8),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_s_vs(R4000AllegrexS),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_s_vt(R4000AllegrexS),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_s_vd(R4000AllegrexS),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_s_vt_imm(R4000AllegrexS),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_s_vd_imm(R4000AllegrexS),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_p_vs(R4000AllegrexV2D),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_p_vt(R4000AllegrexV2D),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_p_vd(R4000AllegrexV2D),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_t_vs(R4000AllegrexV3D),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_t_vt(R4000AllegrexV3D),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_t_vd(R4000AllegrexV3D),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_q_vs(R4000AllegrexV4D),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_q_vt(R4000AllegrexV4D),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_q_vd(R4000AllegrexV4D),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_q_vt_imm(R4000AllegrexV4D),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mp_vs(R4000AllegrexM2x2),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mp_vt(R4000AllegrexM2x2),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mp_vd(R4000AllegrexM2x2),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mp_vs_transpose(R4000AllegrexM2x2),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mt_vs(R4000AllegrexM3x3),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mt_vt(R4000AllegrexM3x3),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mt_vd(R4000AllegrexM3x3),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mt_vs_transpose(R4000AllegrexM3x3),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mq_vs(R4000AllegrexM4x4),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mq_vt(R4000AllegrexM4x4),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mq_vd(R4000AllegrexM4x4),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mq_vs_transpose(R4000AllegrexM4x4),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_cop2cs(R4000AllegrexVfpuControl),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_cop2cd(R4000AllegrexVfpuControl),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_pos(u8),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_size(u8),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_size_plus_pos(i8),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_imm3(u8),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_offset14_base(u16, Gpr),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_offset14_base_maybe_wb(u16, Gpr, bool),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt(
        R4000AllegrexVCond,
        Option<R4000AllegrexS>,
        Option<R4000AllegrexS>,
    ),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt(
        R4000AllegrexVCond,
        Option<R4000AllegrexV2D>,
        Option<R4000AllegrexV2D>,
    ),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt(
        R4000AllegrexVCond,
        Option<R4000AllegrexV3D>,
        Option<R4000AllegrexV3D>,
    ),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt(
        R4000AllegrexVCond,
        Option<R4000AllegrexV4D>,
        Option<R4000AllegrexV4D>,
    ),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vconstant(R4000AllegrexVConstant),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_power_of_two(u8),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vfpu_cc_bit(u8),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_bn(u8),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_int16(i16),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_float16(OrderedFloat<f32>),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_p_vrot_code(u8),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_t_vrot_code(u8),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_q_vrot_code(u8),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_wpx(R4000AllegrexPrefixDst),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_wpy(R4000AllegrexPrefixDst),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_wpz(R4000AllegrexPrefixDst),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_wpw(R4000AllegrexPrefixDst),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_rpx(R4000AllegrexPrefixSrc),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_rpy(R4000AllegrexPrefixSrc),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_rpz(R4000AllegrexPrefixSrc),
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_rpw(R4000AllegrexPrefixSrc),
    #[cfg(feature = "R5900EE")]
    r5900ee_I(),
    #[cfg(feature = "R5900EE")]
    r5900ee_Q(),
    #[cfg(feature = "R5900EE")]
    r5900ee_R(),
    #[cfg(feature = "R5900EE")]
    r5900ee_ACC(),
    #[cfg(feature = "R5900EE")]
    r5900ee_immediate5(i8),
    #[cfg(feature = "R5900EE")]
    r5900ee_immediate15(u16),
    #[cfg(feature = "R5900EE")]
    r5900ee_vfs(R5900EEVF),
    #[cfg(feature = "R5900EE")]
    r5900ee_vft(R5900EEVF),
    #[cfg(feature = "R5900EE")]
    r5900ee_vfd(R5900EEVF),
    #[cfg(feature = "R5900EE")]
    r5900ee_vis(R5900EEVI),
    #[cfg(feature = "R5900EE")]
    r5900ee_vit(R5900EEVI),
    #[cfg(feature = "R5900EE")]
    r5900ee_vid(R5900EEVI),
    #[cfg(feature = "R5900EE")]
    r5900ee_ACCxyzw(bool, bool, bool, bool),
    #[cfg(feature = "R5900EE")]
    r5900ee_vfsxyzw(R5900EEVF, bool, bool, bool, bool),
    #[cfg(feature = "R5900EE")]
    r5900ee_vftxyzw(R5900EEVF, bool, bool, bool, bool),
    #[cfg(feature = "R5900EE")]
    r5900ee_vfdxyzw(R5900EEVF, bool, bool, bool, bool),
    #[cfg(feature = "R5900EE")]
    r5900ee_vftn(R5900EEVF, u8),
    #[cfg(feature = "R5900EE")]
    r5900ee_vfsl(R5900EEVF, u8),
    #[cfg(feature = "R5900EE")]
    r5900ee_vftm(R5900EEVF, u8),
    #[cfg(feature = "R5900EE")]
    r5900ee_vis_predecr((), R5900EEVI),
    #[cfg(feature = "R5900EE")]
    r5900ee_vit_predecr((), R5900EEVI),
    #[cfg(feature = "R5900EE")]
    r5900ee_vis_postincr(R5900EEVI, ()),
    #[cfg(feature = "R5900EE")]
    r5900ee_vit_postincr(R5900EEVI, ()),
    #[cfg(feature = "R5900EE")]
    r5900ee_vis_parenthesis(R5900EEVI),
}
impl Operand {
    #[must_use]
    pub const fn from_valued_operand(value: ValuedOperand) -> Self {
        match value {
            ValuedOperand::ALL_EMPTY(..) => Self::ALL_EMPTY,
            ValuedOperand::core_rs(..) => Self::core_rs,
            ValuedOperand::core_rt(..) => Self::core_rt,
            ValuedOperand::core_rd(..) => Self::core_rd,
            ValuedOperand::core_sa(..) => Self::core_sa,
            ValuedOperand::core_zero(..) => Self::core_zero,
            ValuedOperand::core_cop0d(..) => Self::core_cop0d,
            ValuedOperand::core_cop0cd(..) => Self::core_cop0cd,
            ValuedOperand::core_fs(..) => Self::core_fs,
            ValuedOperand::core_ft(..) => Self::core_ft,
            ValuedOperand::core_fd(..) => Self::core_fd,
            ValuedOperand::core_cop1cs(..) => Self::core_cop1cs,
            ValuedOperand::core_cop2t(..) => Self::core_cop2t,
            ValuedOperand::core_cop2d(..) => Self::core_cop2d,
            ValuedOperand::core_cop2cd(..) => Self::core_cop2cd,
            ValuedOperand::core_op(..) => Self::core_op,
            ValuedOperand::core_hint(..) => Self::core_hint,
            ValuedOperand::core_code(..) => Self::core_code,
            ValuedOperand::core_code_lower(..) => Self::core_code_lower,
            ValuedOperand::core_copraw(..) => Self::core_copraw,
            ValuedOperand::core_label(..) => Self::core_label,
            ValuedOperand::core_immediate(..) => Self::core_immediate,
            ValuedOperand::core_branch_target_label(..) => Self::core_branch_target_label,
            ValuedOperand::core_immediate_base(..) => Self::core_immediate_base,
            ValuedOperand::core_maybe_rd_rs(..) => Self::core_maybe_rd_rs,
            ValuedOperand::core_maybe_zero_rs(..) => Self::core_maybe_zero_rs,
            #[cfg(feature = "RSP")]
            ValuedOperand::rsp_cop0d(..) => Self::rsp_cop0d,
            #[cfg(feature = "RSP")]
            ValuedOperand::rsp_cop2cd(..) => Self::rsp_cop2cd,
            #[cfg(feature = "RSP")]
            ValuedOperand::rsp_vs(..) => Self::rsp_vs,
            #[cfg(feature = "RSP")]
            ValuedOperand::rsp_vd(..) => Self::rsp_vd,
            #[cfg(feature = "RSP")]
            ValuedOperand::rsp_vt_elementhigh(..) => Self::rsp_vt_elementhigh,
            #[cfg(feature = "RSP")]
            ValuedOperand::rsp_vt_elementlow(..) => Self::rsp_vt_elementlow,
            #[cfg(feature = "RSP")]
            ValuedOperand::rsp_vd_de(..) => Self::rsp_vd_de,
            #[cfg(feature = "RSP")]
            ValuedOperand::rsp_vs_index(..) => Self::rsp_vs_index,
            #[cfg(feature = "RSP")]
            ValuedOperand::rsp_offset_rs(..) => Self::rsp_offset_rs,
            #[cfg(feature = "R3000GTE")]
            ValuedOperand::r3000gte_sf(..) => Self::r3000gte_sf,
            #[cfg(feature = "R3000GTE")]
            ValuedOperand::r3000gte_mx(..) => Self::r3000gte_mx,
            #[cfg(feature = "R3000GTE")]
            ValuedOperand::r3000gte_v(..) => Self::r3000gte_v,
            #[cfg(feature = "R3000GTE")]
            ValuedOperand::r3000gte_cv(..) => Self::r3000gte_cv,
            #[cfg(feature = "R3000GTE")]
            ValuedOperand::r3000gte_lm(..) => Self::r3000gte_lm,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_s_vs(..) => Self::r4000allegrex_s_vs,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_s_vt(..) => Self::r4000allegrex_s_vt,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_s_vd(..) => Self::r4000allegrex_s_vd,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_s_vt_imm(..) => Self::r4000allegrex_s_vt_imm,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_s_vd_imm(..) => Self::r4000allegrex_s_vd_imm,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_p_vs(..) => Self::r4000allegrex_p_vs,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_p_vt(..) => Self::r4000allegrex_p_vt,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_p_vd(..) => Self::r4000allegrex_p_vd,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_t_vs(..) => Self::r4000allegrex_t_vs,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_t_vt(..) => Self::r4000allegrex_t_vt,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_t_vd(..) => Self::r4000allegrex_t_vd,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_q_vs(..) => Self::r4000allegrex_q_vs,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_q_vt(..) => Self::r4000allegrex_q_vt,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_q_vd(..) => Self::r4000allegrex_q_vd,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_q_vt_imm(..) => Self::r4000allegrex_q_vt_imm,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_mp_vs(..) => Self::r4000allegrex_mp_vs,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_mp_vt(..) => Self::r4000allegrex_mp_vt,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_mp_vd(..) => Self::r4000allegrex_mp_vd,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_mp_vs_transpose(..) => Self::r4000allegrex_mp_vs_transpose,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_mt_vs(..) => Self::r4000allegrex_mt_vs,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_mt_vt(..) => Self::r4000allegrex_mt_vt,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_mt_vd(..) => Self::r4000allegrex_mt_vd,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_mt_vs_transpose(..) => Self::r4000allegrex_mt_vs_transpose,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_mq_vs(..) => Self::r4000allegrex_mq_vs,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_mq_vt(..) => Self::r4000allegrex_mq_vt,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_mq_vd(..) => Self::r4000allegrex_mq_vd,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_mq_vs_transpose(..) => Self::r4000allegrex_mq_vs_transpose,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_cop2cs(..) => Self::r4000allegrex_cop2cs,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_cop2cd(..) => Self::r4000allegrex_cop2cd,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_pos(..) => Self::r4000allegrex_pos,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_size(..) => Self::r4000allegrex_size,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_size_plus_pos(..) => Self::r4000allegrex_size_plus_pos,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_imm3(..) => Self::r4000allegrex_imm3,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_offset14_base(..) => Self::r4000allegrex_offset14_base,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_offset14_base_maybe_wb(..) => {
                Self::r4000allegrex_offset14_base_maybe_wb
            }
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt(..) => {
                Self::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt
            }
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt(..) => {
                Self::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt
            }
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt(..) => {
                Self::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt
            }
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt(..) => {
                Self::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt
            }
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_vconstant(..) => Self::r4000allegrex_vconstant,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_power_of_two(..) => Self::r4000allegrex_power_of_two,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_vfpu_cc_bit(..) => Self::r4000allegrex_vfpu_cc_bit,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_bn(..) => Self::r4000allegrex_bn,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_int16(..) => Self::r4000allegrex_int16,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_float16(..) => Self::r4000allegrex_float16,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_p_vrot_code(..) => Self::r4000allegrex_p_vrot_code,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_t_vrot_code(..) => Self::r4000allegrex_t_vrot_code,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_q_vrot_code(..) => Self::r4000allegrex_q_vrot_code,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_wpx(..) => Self::r4000allegrex_wpx,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_wpy(..) => Self::r4000allegrex_wpy,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_wpz(..) => Self::r4000allegrex_wpz,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_wpw(..) => Self::r4000allegrex_wpw,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_rpx(..) => Self::r4000allegrex_rpx,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_rpy(..) => Self::r4000allegrex_rpy,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_rpz(..) => Self::r4000allegrex_rpz,
            #[cfg(feature = "R4000ALLEGREX")]
            ValuedOperand::r4000allegrex_rpw(..) => Self::r4000allegrex_rpw,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_I(..) => Self::r5900ee_I,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_Q(..) => Self::r5900ee_Q,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_R(..) => Self::r5900ee_R,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_ACC(..) => Self::r5900ee_ACC,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_immediate5(..) => Self::r5900ee_immediate5,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_immediate15(..) => Self::r5900ee_immediate15,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vfs(..) => Self::r5900ee_vfs,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vft(..) => Self::r5900ee_vft,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vfd(..) => Self::r5900ee_vfd,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vis(..) => Self::r5900ee_vis,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vit(..) => Self::r5900ee_vit,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vid(..) => Self::r5900ee_vid,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_ACCxyzw(..) => Self::r5900ee_ACCxyzw,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vfsxyzw(..) => Self::r5900ee_vfsxyzw,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vftxyzw(..) => Self::r5900ee_vftxyzw,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vfdxyzw(..) => Self::r5900ee_vfdxyzw,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vftn(..) => Self::r5900ee_vftn,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vfsl(..) => Self::r5900ee_vfsl,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vftm(..) => Self::r5900ee_vftm,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vis_predecr(..) => Self::r5900ee_vis_predecr,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vit_predecr(..) => Self::r5900ee_vit_predecr,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vis_postincr(..) => Self::r5900ee_vis_postincr,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vit_postincr(..) => Self::r5900ee_vit_postincr,
            #[cfg(feature = "R5900EE")]
            ValuedOperand::r5900ee_vis_parenthesis(..) => Self::r5900ee_vis_parenthesis,
        }
    }
}
