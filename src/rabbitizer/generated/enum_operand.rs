/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::EncodedFieldMask;
use crate::{display_operand::DisplayOperandCallback, DisplayOperand};
use crate::{operand::OPERAND_COUNT, OperandDescriptor, ValuedOperand};
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum Operand {
    ALL_EMPTY,
    cpu_rs,
    cpu_rt,
    cpu_rd,
    cpu_sa,
    cpu_zero,
    cpu_cop0d,
    cpu_fs,
    cpu_ft,
    cpu_fd,
    cpu_cop1cs,
    cpu_cop2t,
    cpu_cop2cd,
    cpu_op,
    cpu_hint,
    cpu_code,
    cpu_code_lower,
    cpu_copraw,
    cpu_label,
    cpu_immediate,
    cpu_branch_target_label,
    cpu_immediate_base,
    cpu_maybe_rd_rs,
    rsp_rs,
    rsp_rt,
    rsp_rd,
    rsp_cop0d,
    rsp_cop2t,
    rsp_cop2cd,
    rsp_vs,
    rsp_vt,
    rsp_vd,
    rsp_hint,
    rsp_vt_elementhigh,
    rsp_vt_elementlow,
    rsp_vd_de,
    rsp_vs_index,
    rsp_offset_rs,
    rsp_immediate_base,
    rsp_maybe_rd_rs,
    r3000gte_sf,
    r3000gte_mx,
    r3000gte_v,
    r3000gte_cv,
    r3000gte_lm,
    r4000allegrex_s_vs,
    r4000allegrex_s_vt,
    r4000allegrex_s_vd,
    r4000allegrex_s_vt_imm,
    r4000allegrex_s_vd_imm,
    r4000allegrex_p_vs,
    r4000allegrex_p_vt,
    r4000allegrex_p_vd,
    r4000allegrex_t_vs,
    r4000allegrex_t_vt,
    r4000allegrex_t_vd,
    r4000allegrex_q_vs,
    r4000allegrex_q_vt,
    r4000allegrex_q_vd,
    r4000allegrex_q_vt_imm,
    r4000allegrex_mp_vs,
    r4000allegrex_mp_vt,
    r4000allegrex_mp_vd,
    r4000allegrex_mp_vs_transpose,
    r4000allegrex_mt_vs,
    r4000allegrex_mt_vt,
    r4000allegrex_mt_vd,
    r4000allegrex_mt_vs_transpose,
    r4000allegrex_mq_vs,
    r4000allegrex_mq_vt,
    r4000allegrex_mq_vd,
    r4000allegrex_mq_vs_transpose,
    r4000allegrex_cop2cs,
    r4000allegrex_cop2cd,
    r4000allegrex_pos,
    r4000allegrex_size,
    r4000allegrex_size_plus_pos,
    r4000allegrex_imm3,
    r4000allegrex_offset14_base,
    r4000allegrex_offset14_base_maybe_wb,
    r4000allegrex_vcmp_cond,
    r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt,
    r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt,
    r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt,
    r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt,
    r4000allegrex_vconstant,
    r4000allegrex_power_of_two,
    r4000allegrex_vfpu_cc_bit,
    r4000allegrex_bn,
    r4000allegrex_int16,
    r4000allegrex_float16,
    r4000allegrex_p_vrot_code,
    r4000allegrex_t_vrot_code,
    r4000allegrex_q_vrot_code,
    r4000allegrex_rpx,
    r4000allegrex_rpy,
    r4000allegrex_rpz,
    r4000allegrex_rpw,
    r4000allegrex_wpx,
    r4000allegrex_wpy,
    r4000allegrex_wpz,
    r4000allegrex_wpw,
    r5900_I,
    r5900_Q,
    r5900_R,
    r5900_ACC,
    r5900_ACCxyzw,
    r5900_vfs,
    r5900_vft,
    r5900_vfd,
    r5900_vfsxyzw,
    r5900_vftxyzw,
    r5900_vfdxyzw,
    r5900_vfsn,
    r5900_vftn,
    r5900_vfdn,
    r5900_vfsl,
    r5900_vftl,
    r5900_vfdl,
    r5900_vfsm,
    r5900_vftm,
    r5900_vfdm,
    r5900_vis,
    r5900_vit,
    r5900_vid,
    r5900_vis_predecr,
    r5900_vit_predecr,
    r5900_vid_predecr,
    r5900_vis_postincr,
    r5900_vit_postincr,
    r5900_vid_postincr,
    r5900_vis_parenthesis,
    r5900_immediate5,
    r5900_immediate15,
}
pub static OPERANDS: [OperandDescriptor; OPERAND_COUNT] = {
    let mut table = [OperandDescriptor::default(); OPERAND_COUNT];
    table[Operand::ALL_EMPTY as usize] =
        OperandDescriptor::new(concat!("ALL", "_", "EMPTY"), EncodedFieldMask::empty());
    table[Operand::cpu_rs as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "rs"), EncodedFieldMask::rs);
    table[Operand::cpu_rt as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "rt"), EncodedFieldMask::rt);
    table[Operand::cpu_rd as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "rd"), EncodedFieldMask::rd);
    table[Operand::cpu_sa as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "sa"), EncodedFieldMask::sa);
    table[Operand::cpu_zero as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "zero"), EncodedFieldMask::empty());
    table[Operand::cpu_cop0d as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "cop0d"), EncodedFieldMask::cop0d);
    table[Operand::cpu_fs as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "fs"), EncodedFieldMask::fs);
    table[Operand::cpu_ft as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "ft"), EncodedFieldMask::ft);
    table[Operand::cpu_fd as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "fd"), EncodedFieldMask::fd);
    table[Operand::cpu_cop1cs as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "cop1cs"), EncodedFieldMask::cop1cs);
    table[Operand::cpu_cop2t as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "cop2t"), EncodedFieldMask::cop2t);
    table[Operand::cpu_cop2cd as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "cop2cd"), EncodedFieldMask::cop2cd);
    table[Operand::cpu_op as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "op"), EncodedFieldMask::op);
    table[Operand::cpu_hint as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "hint"), EncodedFieldMask::hint);
    table[Operand::cpu_code as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "code"), EncodedFieldMask::code);
    table[Operand::cpu_code_lower as usize] = OperandDescriptor::new(
        concat!("cpu", "_", "code_lower"),
        EncodedFieldMask::code_lower,
    );
    table[Operand::cpu_copraw as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "copraw"), EncodedFieldMask::copraw);
    table[Operand::cpu_label as usize] =
        OperandDescriptor::new(concat!("cpu", "_", "label"), EncodedFieldMask::instr_index);
    table[Operand::cpu_immediate as usize] = OperandDescriptor::new(
        concat!("cpu", "_", "immediate"),
        EncodedFieldMask::immediate,
    );
    table[Operand::cpu_branch_target_label as usize] = OperandDescriptor::new(
        concat!("cpu", "_", "branch_target_label"),
        EncodedFieldMask::immediate,
    );
    table[Operand::cpu_immediate_base as usize] = OperandDescriptor::new(
        concat!("cpu", "_", "immediate_base"),
        EncodedFieldMask::immediate.union(EncodedFieldMask::rs),
    );
    table[Operand::cpu_maybe_rd_rs as usize] = OperandDescriptor::new(
        concat!("cpu", "_", "maybe_rd_rs"),
        EncodedFieldMask::rd.union(EncodedFieldMask::rs),
    );
    table[Operand::rsp_rs as usize] =
        OperandDescriptor::new(concat!("rsp", "_", "rs"), EncodedFieldMask::rs);
    table[Operand::rsp_rt as usize] =
        OperandDescriptor::new(concat!("rsp", "_", "rt"), EncodedFieldMask::rt);
    table[Operand::rsp_rd as usize] =
        OperandDescriptor::new(concat!("rsp", "_", "rd"), EncodedFieldMask::rd);
    table[Operand::rsp_cop0d as usize] =
        OperandDescriptor::new(concat!("rsp", "_", "cop0d"), EncodedFieldMask::cop0d);
    table[Operand::rsp_cop2t as usize] =
        OperandDescriptor::new(concat!("rsp", "_", "cop2t"), EncodedFieldMask::cop2t);
    table[Operand::rsp_cop2cd as usize] =
        OperandDescriptor::new(concat!("rsp", "_", "cop2cd"), EncodedFieldMask::cop2cd);
    table[Operand::rsp_vs as usize] =
        OperandDescriptor::new(concat!("rsp", "_", "vs"), EncodedFieldMask::rsp_vs);
    table[Operand::rsp_vt as usize] =
        OperandDescriptor::new(concat!("rsp", "_", "vt"), EncodedFieldMask::rsp_vt);
    table[Operand::rsp_vd as usize] =
        OperandDescriptor::new(concat!("rsp", "_", "vd"), EncodedFieldMask::rsp_vd);
    table[Operand::rsp_hint as usize] =
        OperandDescriptor::new(concat!("rsp", "_", "hint"), EncodedFieldMask::hint);
    table[Operand::rsp_vt_elementhigh as usize] = OperandDescriptor::new(
        concat!("rsp", "_", "vt_elementhigh"),
        EncodedFieldMask::rsp_vt.union(EncodedFieldMask::rsp_elementhigh),
    );
    table[Operand::rsp_vt_elementlow as usize] = OperandDescriptor::new(
        concat!("rsp", "_", "vt_elementlow"),
        EncodedFieldMask::rsp_vt.union(EncodedFieldMask::rsp_elementlow),
    );
    table[Operand::rsp_vd_de as usize] = OperandDescriptor::new(
        concat!("rsp", "_", "vd_de"),
        EncodedFieldMask::rsp_vd.union(EncodedFieldMask::rsp_de),
    );
    table[Operand::rsp_vs_index as usize] = OperandDescriptor::new(
        concat!("rsp", "_", "vs_index"),
        EncodedFieldMask::rsp_vs.union(EncodedFieldMask::rsp_index),
    );
    table[Operand::rsp_offset_rs as usize] = OperandDescriptor::new(
        concat!("rsp", "_", "offset_rs"),
        EncodedFieldMask::rsp_offset.union(EncodedFieldMask::rs),
    );
    table[Operand::rsp_immediate_base as usize] = OperandDescriptor::new(
        concat!("rsp", "_", "immediate_base"),
        EncodedFieldMask::immediate.union(EncodedFieldMask::rs),
    );
    table[Operand::rsp_maybe_rd_rs as usize] = OperandDescriptor::new(
        concat!("rsp", "_", "maybe_rd_rs"),
        EncodedFieldMask::rd.union(EncodedFieldMask::rs),
    );
    table[Operand::r3000gte_sf as usize] = OperandDescriptor::new(
        concat!("r3000gte", "_", "sf"),
        EncodedFieldMask::r3000gte_sf,
    );
    table[Operand::r3000gte_mx as usize] = OperandDescriptor::new(
        concat!("r3000gte", "_", "mx"),
        EncodedFieldMask::r3000gte_mx,
    );
    table[Operand::r3000gte_v as usize] =
        OperandDescriptor::new(concat!("r3000gte", "_", "v"), EncodedFieldMask::r3000gte_v);
    table[Operand::r3000gte_cv as usize] = OperandDescriptor::new(
        concat!("r3000gte", "_", "cv"),
        EncodedFieldMask::r3000gte_cv,
    );
    table[Operand::r3000gte_lm as usize] = OperandDescriptor::new(
        concat!("r3000gte", "_", "lm"),
        EncodedFieldMask::r3000gte_lm,
    );
    table[Operand::r4000allegrex_s_vs as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "s_vs"),
        EncodedFieldMask::r4000allegrex_vs,
    );
    table[Operand::r4000allegrex_s_vt as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "s_vt"),
        EncodedFieldMask::r4000allegrex_vt,
    );
    table[Operand::r4000allegrex_s_vd as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "s_vd"),
        EncodedFieldMask::r4000allegrex_vd,
    );
    table[Operand::r4000allegrex_s_vt_imm as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "s_vt_imm"),
        EncodedFieldMask::r4000allegrex_vt_imm,
    );
    table[Operand::r4000allegrex_s_vd_imm as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "s_vd_imm"),
        EncodedFieldMask::r4000allegrex_vd_imm,
    );
    table[Operand::r4000allegrex_p_vs as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "p_vs"),
        EncodedFieldMask::r4000allegrex_vs,
    );
    table[Operand::r4000allegrex_p_vt as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "p_vt"),
        EncodedFieldMask::r4000allegrex_vt,
    );
    table[Operand::r4000allegrex_p_vd as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "p_vd"),
        EncodedFieldMask::r4000allegrex_vd,
    );
    table[Operand::r4000allegrex_t_vs as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "t_vs"),
        EncodedFieldMask::r4000allegrex_vs,
    );
    table[Operand::r4000allegrex_t_vt as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "t_vt"),
        EncodedFieldMask::r4000allegrex_vt,
    );
    table[Operand::r4000allegrex_t_vd as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "t_vd"),
        EncodedFieldMask::r4000allegrex_vd,
    );
    table[Operand::r4000allegrex_q_vs as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "q_vs"),
        EncodedFieldMask::r4000allegrex_vs,
    );
    table[Operand::r4000allegrex_q_vt as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "q_vt"),
        EncodedFieldMask::r4000allegrex_vt,
    );
    table[Operand::r4000allegrex_q_vd as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "q_vd"),
        EncodedFieldMask::r4000allegrex_vd,
    );
    table[Operand::r4000allegrex_q_vt_imm as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "q_vt_imm"),
        EncodedFieldMask::r4000allegrex_vt_imm,
    );
    table[Operand::r4000allegrex_mp_vs as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "mp_vs"),
        EncodedFieldMask::r4000allegrex_vs,
    );
    table[Operand::r4000allegrex_mp_vt as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "mp_vt"),
        EncodedFieldMask::r4000allegrex_vt,
    );
    table[Operand::r4000allegrex_mp_vd as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "mp_vd"),
        EncodedFieldMask::r4000allegrex_vd,
    );
    table[Operand::r4000allegrex_mp_vs_transpose as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "mp_vs_transpose"),
        EncodedFieldMask::r4000allegrex_vs,
    );
    table[Operand::r4000allegrex_mt_vs as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "mt_vs"),
        EncodedFieldMask::r4000allegrex_vs,
    );
    table[Operand::r4000allegrex_mt_vt as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "mt_vt"),
        EncodedFieldMask::r4000allegrex_vt,
    );
    table[Operand::r4000allegrex_mt_vd as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "mt_vd"),
        EncodedFieldMask::r4000allegrex_vd,
    );
    table[Operand::r4000allegrex_mt_vs_transpose as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "mt_vs_transpose"),
        EncodedFieldMask::r4000allegrex_vs,
    );
    table[Operand::r4000allegrex_mq_vs as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "mq_vs"),
        EncodedFieldMask::r4000allegrex_vs,
    );
    table[Operand::r4000allegrex_mq_vt as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "mq_vt"),
        EncodedFieldMask::r4000allegrex_vt,
    );
    table[Operand::r4000allegrex_mq_vd as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "mq_vd"),
        EncodedFieldMask::r4000allegrex_vd,
    );
    table[Operand::r4000allegrex_mq_vs_transpose as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "mq_vs_transpose"),
        EncodedFieldMask::r4000allegrex_vs,
    );
    table[Operand::r4000allegrex_cop2cs as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "cop2cs"),
        EncodedFieldMask::r4000allegrex_cop2cs,
    );
    table[Operand::r4000allegrex_cop2cd as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "cop2cd"),
        EncodedFieldMask::r4000allegrex_cop2cd,
    );
    table[Operand::r4000allegrex_pos as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "pos"),
        EncodedFieldMask::r4000allegrex_pos,
    );
    table[Operand::r4000allegrex_size as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "size"),
        EncodedFieldMask::r4000allegrex_size,
    );
    table[Operand::r4000allegrex_size_plus_pos as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "size_plus_pos"),
        EncodedFieldMask::r4000allegrex_size_plus_pos,
    );
    table[Operand::r4000allegrex_imm3 as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "imm3"),
        EncodedFieldMask::r4000allegrex_imm3,
    );
    table[Operand::r4000allegrex_offset14_base as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "offset14_base"),
        EncodedFieldMask::r4000allegrex_offset14.union(EncodedFieldMask::rs),
    );
    table[Operand::r4000allegrex_offset14_base_maybe_wb as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "offset14_base_maybe_wb"),
        EncodedFieldMask::r4000allegrex_offset14
            .union(EncodedFieldMask::rs)
            .union(EncodedFieldMask::r4000allegrex_wb),
    );
    table[Operand::r4000allegrex_vcmp_cond as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "vcmp_cond"),
        EncodedFieldMask::r4000allegrex_vcmp_cond,
    );
    table[Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "vcmp_cond_s_maybe_vs_maybe_vt"),
        EncodedFieldMask::r4000allegrex_vcmp_cond
            .union(EncodedFieldMask::r4000allegrex_vs)
            .union(EncodedFieldMask::r4000allegrex_vt),
    );
    table[Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "vcmp_cond_p_maybe_vs_maybe_vt"),
        EncodedFieldMask::r4000allegrex_vcmp_cond
            .union(EncodedFieldMask::r4000allegrex_vs)
            .union(EncodedFieldMask::r4000allegrex_vt),
    );
    table[Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "vcmp_cond_t_maybe_vs_maybe_vt"),
        EncodedFieldMask::r4000allegrex_vcmp_cond
            .union(EncodedFieldMask::r4000allegrex_vs)
            .union(EncodedFieldMask::r4000allegrex_vt),
    );
    table[Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "vcmp_cond_q_maybe_vs_maybe_vt"),
        EncodedFieldMask::r4000allegrex_vcmp_cond
            .union(EncodedFieldMask::r4000allegrex_vs)
            .union(EncodedFieldMask::r4000allegrex_vt),
    );
    table[Operand::r4000allegrex_vconstant as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "vconstant"),
        EncodedFieldMask::r4000allegrex_vconstant,
    );
    table[Operand::r4000allegrex_power_of_two as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "power_of_two"),
        EncodedFieldMask::r4000allegrex_power_of_two,
    );
    table[Operand::r4000allegrex_vfpu_cc_bit as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "vfpu_cc_bit"),
        EncodedFieldMask::r4000allegrex_vfpu_cc_bit,
    );
    table[Operand::r4000allegrex_bn as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "bn"),
        EncodedFieldMask::r4000allegrex_bn,
    );
    table[Operand::r4000allegrex_int16 as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "int16"),
        EncodedFieldMask::r4000allegrex_intfloat16,
    );
    table[Operand::r4000allegrex_float16 as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "float16"),
        EncodedFieldMask::r4000allegrex_intfloat16,
    );
    table[Operand::r4000allegrex_p_vrot_code as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "p_vrot_code"),
        EncodedFieldMask::r4000allegrex_vrot_code,
    );
    table[Operand::r4000allegrex_t_vrot_code as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "t_vrot_code"),
        EncodedFieldMask::r4000allegrex_vrot_code,
    );
    table[Operand::r4000allegrex_q_vrot_code as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "q_vrot_code"),
        EncodedFieldMask::r4000allegrex_vrot_code,
    );
    table[Operand::r4000allegrex_rpx as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "rpx"),
        EncodedFieldMask::r4000allegrex_rpx,
    );
    table[Operand::r4000allegrex_rpy as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "rpy"),
        EncodedFieldMask::r4000allegrex_rpy,
    );
    table[Operand::r4000allegrex_rpz as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "rpz"),
        EncodedFieldMask::r4000allegrex_rpz,
    );
    table[Operand::r4000allegrex_rpw as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "rpw"),
        EncodedFieldMask::r4000allegrex_rpw,
    );
    table[Operand::r4000allegrex_wpx as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "wpx"),
        EncodedFieldMask::r4000allegrex_wpx,
    );
    table[Operand::r4000allegrex_wpy as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "wpy"),
        EncodedFieldMask::r4000allegrex_wpy,
    );
    table[Operand::r4000allegrex_wpz as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "wpz"),
        EncodedFieldMask::r4000allegrex_wpz,
    );
    table[Operand::r4000allegrex_wpw as usize] = OperandDescriptor::new(
        concat!("r4000allegrex", "_", "wpw"),
        EncodedFieldMask::r4000allegrex_wpw,
    );
    table[Operand::r5900_I as usize] =
        OperandDescriptor::new(concat!("r5900", "_", "I"), EncodedFieldMask::empty());
    table[Operand::r5900_Q as usize] =
        OperandDescriptor::new(concat!("r5900", "_", "Q"), EncodedFieldMask::empty());
    table[Operand::r5900_R as usize] =
        OperandDescriptor::new(concat!("r5900", "_", "R"), EncodedFieldMask::empty());
    table[Operand::r5900_ACC as usize] =
        OperandDescriptor::new(concat!("r5900", "_", "ACC"), EncodedFieldMask::empty());
    table[Operand::r5900_ACCxyzw as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "ACCxyzw"),
        EncodedFieldMask::r5900_xyzw_xyzw,
    );
    table[Operand::r5900_vfs as usize] =
        OperandDescriptor::new(concat!("r5900", "_", "vfs"), EncodedFieldMask::r5900_vfs);
    table[Operand::r5900_vft as usize] =
        OperandDescriptor::new(concat!("r5900", "_", "vft"), EncodedFieldMask::r5900_vft);
    table[Operand::r5900_vfd as usize] =
        OperandDescriptor::new(concat!("r5900", "_", "vfd"), EncodedFieldMask::r5900_vfd);
    table[Operand::r5900_vfsxyzw as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vfsxyzw"),
        EncodedFieldMask::r5900_vfs.union(EncodedFieldMask::r5900_xyzw_xyzw),
    );
    table[Operand::r5900_vftxyzw as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vftxyzw"),
        EncodedFieldMask::r5900_vft.union(EncodedFieldMask::r5900_xyzw_xyzw),
    );
    table[Operand::r5900_vfdxyzw as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vfdxyzw"),
        EncodedFieldMask::r5900_vfd.union(EncodedFieldMask::r5900_xyzw_xyzw),
    );
    table[Operand::r5900_vfsn as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vfsn"),
        EncodedFieldMask::r5900_vfs.union(EncodedFieldMask::r5900_n),
    );
    table[Operand::r5900_vftn as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vftn"),
        EncodedFieldMask::r5900_vft.union(EncodedFieldMask::r5900_n),
    );
    table[Operand::r5900_vfdn as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vfdn"),
        EncodedFieldMask::r5900_vfd.union(EncodedFieldMask::r5900_n),
    );
    table[Operand::r5900_vfsl as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vfsl"),
        EncodedFieldMask::r5900_vfs.union(EncodedFieldMask::r5900_l),
    );
    table[Operand::r5900_vftl as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vftl"),
        EncodedFieldMask::r5900_vft.union(EncodedFieldMask::r5900_l),
    );
    table[Operand::r5900_vfdl as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vfdl"),
        EncodedFieldMask::r5900_vfd.union(EncodedFieldMask::r5900_l),
    );
    table[Operand::r5900_vfsm as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vfsm"),
        EncodedFieldMask::r5900_vfs.union(EncodedFieldMask::r5900_m),
    );
    table[Operand::r5900_vftm as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vftm"),
        EncodedFieldMask::r5900_vft.union(EncodedFieldMask::r5900_m),
    );
    table[Operand::r5900_vfdm as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vfdm"),
        EncodedFieldMask::r5900_vfd.union(EncodedFieldMask::r5900_m),
    );
    table[Operand::r5900_vis as usize] =
        OperandDescriptor::new(concat!("r5900", "_", "vis"), EncodedFieldMask::r5900_vis);
    table[Operand::r5900_vit as usize] =
        OperandDescriptor::new(concat!("r5900", "_", "vit"), EncodedFieldMask::r5900_vit);
    table[Operand::r5900_vid as usize] =
        OperandDescriptor::new(concat!("r5900", "_", "vid"), EncodedFieldMask::r5900_vid);
    table[Operand::r5900_vis_predecr as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vis_predecr"),
        EncodedFieldMask::r5900_vis,
    );
    table[Operand::r5900_vit_predecr as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vit_predecr"),
        EncodedFieldMask::r5900_vit,
    );
    table[Operand::r5900_vid_predecr as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vid_predecr"),
        EncodedFieldMask::r5900_vid,
    );
    table[Operand::r5900_vis_postincr as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vis_postincr"),
        EncodedFieldMask::r5900_vis,
    );
    table[Operand::r5900_vit_postincr as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vit_postincr"),
        EncodedFieldMask::r5900_vit,
    );
    table[Operand::r5900_vid_postincr as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vid_postincr"),
        EncodedFieldMask::r5900_vid,
    );
    table[Operand::r5900_vis_parenthesis as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "vis_parenthesis"),
        EncodedFieldMask::r5900_vis,
    );
    table[Operand::r5900_immediate5 as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "immediate5"),
        EncodedFieldMask::r5900_imm5,
    );
    table[Operand::r5900_immediate15 as usize] = OperandDescriptor::new(
        concat!("r5900", "_", "immediate15"),
        EncodedFieldMask::r5900_imm15,
    );
    table
};
pub(crate) static DISPLAY_OPERAND_CALLBACKS: [DisplayOperandCallback; OPERAND_COUNT] = {
    let mut table: [DisplayOperandCallback; OPERAND_COUNT] =
        [DisplayOperand::display_ALL_EMPTY; OPERAND_COUNT];
    table[Operand::ALL_EMPTY as usize] = DisplayOperand::display_ALL_EMPTY;
    table[Operand::cpu_rs as usize] = DisplayOperand::display_cpu_rs;
    table[Operand::cpu_rt as usize] = DisplayOperand::display_cpu_rt;
    table[Operand::cpu_rd as usize] = DisplayOperand::display_cpu_rd;
    table[Operand::cpu_sa as usize] = DisplayOperand::display_cpu_sa;
    table[Operand::cpu_zero as usize] = DisplayOperand::display_cpu_zero;
    table[Operand::cpu_cop0d as usize] = DisplayOperand::display_cpu_cop0d;
    table[Operand::cpu_fs as usize] = DisplayOperand::display_cpu_fs;
    table[Operand::cpu_ft as usize] = DisplayOperand::display_cpu_ft;
    table[Operand::cpu_fd as usize] = DisplayOperand::display_cpu_fd;
    table[Operand::cpu_cop1cs as usize] = DisplayOperand::display_cpu_cop1cs;
    table[Operand::cpu_cop2t as usize] = DisplayOperand::display_cpu_cop2t;
    table[Operand::cpu_cop2cd as usize] = DisplayOperand::display_cpu_cop2cd;
    table[Operand::cpu_op as usize] = DisplayOperand::display_cpu_op;
    table[Operand::cpu_hint as usize] = DisplayOperand::display_cpu_hint;
    table[Operand::cpu_code as usize] = DisplayOperand::display_cpu_code;
    table[Operand::cpu_code_lower as usize] = DisplayOperand::display_cpu_code_lower;
    table[Operand::cpu_copraw as usize] = DisplayOperand::display_cpu_copraw;
    table[Operand::cpu_label as usize] = DisplayOperand::display_cpu_label;
    table[Operand::cpu_immediate as usize] = DisplayOperand::display_cpu_immediate;
    table[Operand::cpu_branch_target_label as usize] =
        DisplayOperand::display_cpu_branch_target_label;
    table[Operand::cpu_immediate_base as usize] = DisplayOperand::display_cpu_immediate_base;
    table[Operand::cpu_maybe_rd_rs as usize] = DisplayOperand::display_cpu_maybe_rd_rs;
    table[Operand::rsp_rs as usize] = DisplayOperand::display_rsp_rs;
    table[Operand::rsp_rt as usize] = DisplayOperand::display_rsp_rt;
    table[Operand::rsp_rd as usize] = DisplayOperand::display_rsp_rd;
    table[Operand::rsp_cop0d as usize] = DisplayOperand::display_rsp_cop0d;
    table[Operand::rsp_cop2t as usize] = DisplayOperand::display_rsp_cop2t;
    table[Operand::rsp_cop2cd as usize] = DisplayOperand::display_rsp_cop2cd;
    table[Operand::rsp_vs as usize] = DisplayOperand::display_rsp_vs;
    table[Operand::rsp_vt as usize] = DisplayOperand::display_rsp_vt;
    table[Operand::rsp_vd as usize] = DisplayOperand::display_rsp_vd;
    table[Operand::rsp_hint as usize] = DisplayOperand::display_rsp_hint;
    table[Operand::rsp_vt_elementhigh as usize] = DisplayOperand::display_rsp_vt_elementhigh;
    table[Operand::rsp_vt_elementlow as usize] = DisplayOperand::display_rsp_vt_elementlow;
    table[Operand::rsp_vd_de as usize] = DisplayOperand::display_rsp_vd_de;
    table[Operand::rsp_vs_index as usize] = DisplayOperand::display_rsp_vs_index;
    table[Operand::rsp_offset_rs as usize] = DisplayOperand::display_rsp_offset_rs;
    table[Operand::rsp_immediate_base as usize] = DisplayOperand::display_rsp_immediate_base;
    table[Operand::rsp_maybe_rd_rs as usize] = DisplayOperand::display_rsp_maybe_rd_rs;
    table[Operand::r3000gte_sf as usize] = DisplayOperand::display_r3000gte_sf;
    table[Operand::r3000gte_mx as usize] = DisplayOperand::display_r3000gte_mx;
    table[Operand::r3000gte_v as usize] = DisplayOperand::display_r3000gte_v;
    table[Operand::r3000gte_cv as usize] = DisplayOperand::display_r3000gte_cv;
    table[Operand::r3000gte_lm as usize] = DisplayOperand::display_r3000gte_lm;
    table[Operand::r4000allegrex_s_vs as usize] = DisplayOperand::display_r4000allegrex_s_vs;
    table[Operand::r4000allegrex_s_vt as usize] = DisplayOperand::display_r4000allegrex_s_vt;
    table[Operand::r4000allegrex_s_vd as usize] = DisplayOperand::display_r4000allegrex_s_vd;
    table[Operand::r4000allegrex_s_vt_imm as usize] =
        DisplayOperand::display_r4000allegrex_s_vt_imm;
    table[Operand::r4000allegrex_s_vd_imm as usize] =
        DisplayOperand::display_r4000allegrex_s_vd_imm;
    table[Operand::r4000allegrex_p_vs as usize] = DisplayOperand::display_r4000allegrex_p_vs;
    table[Operand::r4000allegrex_p_vt as usize] = DisplayOperand::display_r4000allegrex_p_vt;
    table[Operand::r4000allegrex_p_vd as usize] = DisplayOperand::display_r4000allegrex_p_vd;
    table[Operand::r4000allegrex_t_vs as usize] = DisplayOperand::display_r4000allegrex_t_vs;
    table[Operand::r4000allegrex_t_vt as usize] = DisplayOperand::display_r4000allegrex_t_vt;
    table[Operand::r4000allegrex_t_vd as usize] = DisplayOperand::display_r4000allegrex_t_vd;
    table[Operand::r4000allegrex_q_vs as usize] = DisplayOperand::display_r4000allegrex_q_vs;
    table[Operand::r4000allegrex_q_vt as usize] = DisplayOperand::display_r4000allegrex_q_vt;
    table[Operand::r4000allegrex_q_vd as usize] = DisplayOperand::display_r4000allegrex_q_vd;
    table[Operand::r4000allegrex_q_vt_imm as usize] =
        DisplayOperand::display_r4000allegrex_q_vt_imm;
    table[Operand::r4000allegrex_mp_vs as usize] = DisplayOperand::display_r4000allegrex_mp_vs;
    table[Operand::r4000allegrex_mp_vt as usize] = DisplayOperand::display_r4000allegrex_mp_vt;
    table[Operand::r4000allegrex_mp_vd as usize] = DisplayOperand::display_r4000allegrex_mp_vd;
    table[Operand::r4000allegrex_mp_vs_transpose as usize] =
        DisplayOperand::display_r4000allegrex_mp_vs_transpose;
    table[Operand::r4000allegrex_mt_vs as usize] = DisplayOperand::display_r4000allegrex_mt_vs;
    table[Operand::r4000allegrex_mt_vt as usize] = DisplayOperand::display_r4000allegrex_mt_vt;
    table[Operand::r4000allegrex_mt_vd as usize] = DisplayOperand::display_r4000allegrex_mt_vd;
    table[Operand::r4000allegrex_mt_vs_transpose as usize] =
        DisplayOperand::display_r4000allegrex_mt_vs_transpose;
    table[Operand::r4000allegrex_mq_vs as usize] = DisplayOperand::display_r4000allegrex_mq_vs;
    table[Operand::r4000allegrex_mq_vt as usize] = DisplayOperand::display_r4000allegrex_mq_vt;
    table[Operand::r4000allegrex_mq_vd as usize] = DisplayOperand::display_r4000allegrex_mq_vd;
    table[Operand::r4000allegrex_mq_vs_transpose as usize] =
        DisplayOperand::display_r4000allegrex_mq_vs_transpose;
    table[Operand::r4000allegrex_cop2cs as usize] = DisplayOperand::display_r4000allegrex_cop2cs;
    table[Operand::r4000allegrex_cop2cd as usize] = DisplayOperand::display_r4000allegrex_cop2cd;
    table[Operand::r4000allegrex_pos as usize] = DisplayOperand::display_r4000allegrex_pos;
    table[Operand::r4000allegrex_size as usize] = DisplayOperand::display_r4000allegrex_size;
    table[Operand::r4000allegrex_size_plus_pos as usize] =
        DisplayOperand::display_r4000allegrex_size_plus_pos;
    table[Operand::r4000allegrex_imm3 as usize] = DisplayOperand::display_r4000allegrex_imm3;
    table[Operand::r4000allegrex_offset14_base as usize] =
        DisplayOperand::display_r4000allegrex_offset14_base;
    table[Operand::r4000allegrex_offset14_base_maybe_wb as usize] =
        DisplayOperand::display_r4000allegrex_offset14_base_maybe_wb;
    table[Operand::r4000allegrex_vcmp_cond as usize] =
        DisplayOperand::display_r4000allegrex_vcmp_cond;
    table[Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt as usize] =
        DisplayOperand::display_r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt;
    table[Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt as usize] =
        DisplayOperand::display_r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt;
    table[Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt as usize] =
        DisplayOperand::display_r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt;
    table[Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt as usize] =
        DisplayOperand::display_r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt;
    table[Operand::r4000allegrex_vconstant as usize] =
        DisplayOperand::display_r4000allegrex_vconstant;
    table[Operand::r4000allegrex_power_of_two as usize] =
        DisplayOperand::display_r4000allegrex_power_of_two;
    table[Operand::r4000allegrex_vfpu_cc_bit as usize] =
        DisplayOperand::display_r4000allegrex_vfpu_cc_bit;
    table[Operand::r4000allegrex_bn as usize] = DisplayOperand::display_r4000allegrex_bn;
    table[Operand::r4000allegrex_int16 as usize] = DisplayOperand::display_r4000allegrex_int16;
    table[Operand::r4000allegrex_float16 as usize] = DisplayOperand::display_r4000allegrex_float16;
    table[Operand::r4000allegrex_p_vrot_code as usize] =
        DisplayOperand::display_r4000allegrex_p_vrot_code;
    table[Operand::r4000allegrex_t_vrot_code as usize] =
        DisplayOperand::display_r4000allegrex_t_vrot_code;
    table[Operand::r4000allegrex_q_vrot_code as usize] =
        DisplayOperand::display_r4000allegrex_q_vrot_code;
    table[Operand::r4000allegrex_rpx as usize] = DisplayOperand::display_r4000allegrex_rpx;
    table[Operand::r4000allegrex_rpy as usize] = DisplayOperand::display_r4000allegrex_rpy;
    table[Operand::r4000allegrex_rpz as usize] = DisplayOperand::display_r4000allegrex_rpz;
    table[Operand::r4000allegrex_rpw as usize] = DisplayOperand::display_r4000allegrex_rpw;
    table[Operand::r4000allegrex_wpx as usize] = DisplayOperand::display_r4000allegrex_wpx;
    table[Operand::r4000allegrex_wpy as usize] = DisplayOperand::display_r4000allegrex_wpy;
    table[Operand::r4000allegrex_wpz as usize] = DisplayOperand::display_r4000allegrex_wpz;
    table[Operand::r4000allegrex_wpw as usize] = DisplayOperand::display_r4000allegrex_wpw;
    table[Operand::r5900_I as usize] = DisplayOperand::display_r5900_I;
    table[Operand::r5900_Q as usize] = DisplayOperand::display_r5900_Q;
    table[Operand::r5900_R as usize] = DisplayOperand::display_r5900_R;
    table[Operand::r5900_ACC as usize] = DisplayOperand::display_r5900_ACC;
    table[Operand::r5900_ACCxyzw as usize] = DisplayOperand::display_r5900_ACCxyzw;
    table[Operand::r5900_vfs as usize] = DisplayOperand::display_r5900_vfs;
    table[Operand::r5900_vft as usize] = DisplayOperand::display_r5900_vft;
    table[Operand::r5900_vfd as usize] = DisplayOperand::display_r5900_vfd;
    table[Operand::r5900_vfsxyzw as usize] = DisplayOperand::display_r5900_vfsxyzw;
    table[Operand::r5900_vftxyzw as usize] = DisplayOperand::display_r5900_vftxyzw;
    table[Operand::r5900_vfdxyzw as usize] = DisplayOperand::display_r5900_vfdxyzw;
    table[Operand::r5900_vfsn as usize] = DisplayOperand::display_r5900_vfsn;
    table[Operand::r5900_vftn as usize] = DisplayOperand::display_r5900_vftn;
    table[Operand::r5900_vfdn as usize] = DisplayOperand::display_r5900_vfdn;
    table[Operand::r5900_vfsl as usize] = DisplayOperand::display_r5900_vfsl;
    table[Operand::r5900_vftl as usize] = DisplayOperand::display_r5900_vftl;
    table[Operand::r5900_vfdl as usize] = DisplayOperand::display_r5900_vfdl;
    table[Operand::r5900_vfsm as usize] = DisplayOperand::display_r5900_vfsm;
    table[Operand::r5900_vftm as usize] = DisplayOperand::display_r5900_vftm;
    table[Operand::r5900_vfdm as usize] = DisplayOperand::display_r5900_vfdm;
    table[Operand::r5900_vis as usize] = DisplayOperand::display_r5900_vis;
    table[Operand::r5900_vit as usize] = DisplayOperand::display_r5900_vit;
    table[Operand::r5900_vid as usize] = DisplayOperand::display_r5900_vid;
    table[Operand::r5900_vis_predecr as usize] = DisplayOperand::display_r5900_vis_predecr;
    table[Operand::r5900_vit_predecr as usize] = DisplayOperand::display_r5900_vit_predecr;
    table[Operand::r5900_vid_predecr as usize] = DisplayOperand::display_r5900_vid_predecr;
    table[Operand::r5900_vis_postincr as usize] = DisplayOperand::display_r5900_vis_postincr;
    table[Operand::r5900_vit_postincr as usize] = DisplayOperand::display_r5900_vit_postincr;
    table[Operand::r5900_vid_postincr as usize] = DisplayOperand::display_r5900_vid_postincr;
    table[Operand::r5900_vis_parenthesis as usize] = DisplayOperand::display_r5900_vis_parenthesis;
    table[Operand::r5900_immediate5 as usize] = DisplayOperand::display_r5900_immediate5;
    table[Operand::r5900_immediate15 as usize] = DisplayOperand::display_r5900_immediate15;
    table
};
impl Operand {
    #[must_use]
    pub const fn from_valued_operand(value: ValuedOperand) -> Self {
        match value {
            ValuedOperand::ALL_EMPTY(..) => Self::ALL_EMPTY,
            ValuedOperand::cpu_rs(..) => Self::cpu_rs,
            ValuedOperand::cpu_rt(..) => Self::cpu_rt,
            ValuedOperand::cpu_rd(..) => Self::cpu_rd,
            ValuedOperand::cpu_sa(..) => Self::cpu_sa,
            ValuedOperand::cpu_zero(..) => Self::cpu_zero,
            ValuedOperand::cpu_cop0d(..) => Self::cpu_cop0d,
            ValuedOperand::cpu_fs(..) => Self::cpu_fs,
            ValuedOperand::cpu_ft(..) => Self::cpu_ft,
            ValuedOperand::cpu_fd(..) => Self::cpu_fd,
            ValuedOperand::cpu_cop1cs(..) => Self::cpu_cop1cs,
            ValuedOperand::cpu_cop2t(..) => Self::cpu_cop2t,
            ValuedOperand::cpu_cop2cd(..) => Self::cpu_cop2cd,
            ValuedOperand::cpu_op(..) => Self::cpu_op,
            ValuedOperand::cpu_hint(..) => Self::cpu_hint,
            ValuedOperand::cpu_code(..) => Self::cpu_code,
            ValuedOperand::cpu_code_lower(..) => Self::cpu_code_lower,
            ValuedOperand::cpu_copraw(..) => Self::cpu_copraw,
            ValuedOperand::cpu_label(..) => Self::cpu_label,
            ValuedOperand::cpu_immediate(..) => Self::cpu_immediate,
            ValuedOperand::cpu_branch_target_label(..) => Self::cpu_branch_target_label,
            ValuedOperand::cpu_immediate_base(..) => Self::cpu_immediate_base,
            ValuedOperand::cpu_maybe_rd_rs(..) => Self::cpu_maybe_rd_rs,
            ValuedOperand::rsp_rs(..) => Self::rsp_rs,
            ValuedOperand::rsp_rt(..) => Self::rsp_rt,
            ValuedOperand::rsp_rd(..) => Self::rsp_rd,
            ValuedOperand::rsp_cop0d(..) => Self::rsp_cop0d,
            ValuedOperand::rsp_cop2t(..) => Self::rsp_cop2t,
            ValuedOperand::rsp_cop2cd(..) => Self::rsp_cop2cd,
            ValuedOperand::rsp_vs(..) => Self::rsp_vs,
            ValuedOperand::rsp_vt(..) => Self::rsp_vt,
            ValuedOperand::rsp_vd(..) => Self::rsp_vd,
            ValuedOperand::rsp_hint(..) => Self::rsp_hint,
            ValuedOperand::rsp_vt_elementhigh(..) => Self::rsp_vt_elementhigh,
            ValuedOperand::rsp_vt_elementlow(..) => Self::rsp_vt_elementlow,
            ValuedOperand::rsp_vd_de(..) => Self::rsp_vd_de,
            ValuedOperand::rsp_vs_index(..) => Self::rsp_vs_index,
            ValuedOperand::rsp_offset_rs(..) => Self::rsp_offset_rs,
            ValuedOperand::rsp_immediate_base(..) => Self::rsp_immediate_base,
            ValuedOperand::rsp_maybe_rd_rs(..) => Self::rsp_maybe_rd_rs,
            ValuedOperand::r3000gte_sf(..) => Self::r3000gte_sf,
            ValuedOperand::r3000gte_mx(..) => Self::r3000gte_mx,
            ValuedOperand::r3000gte_v(..) => Self::r3000gte_v,
            ValuedOperand::r3000gte_cv(..) => Self::r3000gte_cv,
            ValuedOperand::r3000gte_lm(..) => Self::r3000gte_lm,
            ValuedOperand::r4000allegrex_s_vs(..) => Self::r4000allegrex_s_vs,
            ValuedOperand::r4000allegrex_s_vt(..) => Self::r4000allegrex_s_vt,
            ValuedOperand::r4000allegrex_s_vd(..) => Self::r4000allegrex_s_vd,
            ValuedOperand::r4000allegrex_s_vt_imm(..) => Self::r4000allegrex_s_vt_imm,
            ValuedOperand::r4000allegrex_s_vd_imm(..) => Self::r4000allegrex_s_vd_imm,
            ValuedOperand::r4000allegrex_p_vs(..) => Self::r4000allegrex_p_vs,
            ValuedOperand::r4000allegrex_p_vt(..) => Self::r4000allegrex_p_vt,
            ValuedOperand::r4000allegrex_p_vd(..) => Self::r4000allegrex_p_vd,
            ValuedOperand::r4000allegrex_t_vs(..) => Self::r4000allegrex_t_vs,
            ValuedOperand::r4000allegrex_t_vt(..) => Self::r4000allegrex_t_vt,
            ValuedOperand::r4000allegrex_t_vd(..) => Self::r4000allegrex_t_vd,
            ValuedOperand::r4000allegrex_q_vs(..) => Self::r4000allegrex_q_vs,
            ValuedOperand::r4000allegrex_q_vt(..) => Self::r4000allegrex_q_vt,
            ValuedOperand::r4000allegrex_q_vd(..) => Self::r4000allegrex_q_vd,
            ValuedOperand::r4000allegrex_q_vt_imm(..) => Self::r4000allegrex_q_vt_imm,
            ValuedOperand::r4000allegrex_mp_vs(..) => Self::r4000allegrex_mp_vs,
            ValuedOperand::r4000allegrex_mp_vt(..) => Self::r4000allegrex_mp_vt,
            ValuedOperand::r4000allegrex_mp_vd(..) => Self::r4000allegrex_mp_vd,
            ValuedOperand::r4000allegrex_mp_vs_transpose(..) => Self::r4000allegrex_mp_vs_transpose,
            ValuedOperand::r4000allegrex_mt_vs(..) => Self::r4000allegrex_mt_vs,
            ValuedOperand::r4000allegrex_mt_vt(..) => Self::r4000allegrex_mt_vt,
            ValuedOperand::r4000allegrex_mt_vd(..) => Self::r4000allegrex_mt_vd,
            ValuedOperand::r4000allegrex_mt_vs_transpose(..) => Self::r4000allegrex_mt_vs_transpose,
            ValuedOperand::r4000allegrex_mq_vs(..) => Self::r4000allegrex_mq_vs,
            ValuedOperand::r4000allegrex_mq_vt(..) => Self::r4000allegrex_mq_vt,
            ValuedOperand::r4000allegrex_mq_vd(..) => Self::r4000allegrex_mq_vd,
            ValuedOperand::r4000allegrex_mq_vs_transpose(..) => Self::r4000allegrex_mq_vs_transpose,
            ValuedOperand::r4000allegrex_cop2cs(..) => Self::r4000allegrex_cop2cs,
            ValuedOperand::r4000allegrex_cop2cd(..) => Self::r4000allegrex_cop2cd,
            ValuedOperand::r4000allegrex_pos(..) => Self::r4000allegrex_pos,
            ValuedOperand::r4000allegrex_size(..) => Self::r4000allegrex_size,
            ValuedOperand::r4000allegrex_size_plus_pos(..) => Self::r4000allegrex_size_plus_pos,
            ValuedOperand::r4000allegrex_imm3(..) => Self::r4000allegrex_imm3,
            ValuedOperand::r4000allegrex_offset14_base(..) => Self::r4000allegrex_offset14_base,
            ValuedOperand::r4000allegrex_offset14_base_maybe_wb(..) => {
                Self::r4000allegrex_offset14_base_maybe_wb
            }
            ValuedOperand::r4000allegrex_vcmp_cond(..) => Self::r4000allegrex_vcmp_cond,
            ValuedOperand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt(..) => {
                Self::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt
            }
            ValuedOperand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt(..) => {
                Self::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt
            }
            ValuedOperand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt(..) => {
                Self::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt
            }
            ValuedOperand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt(..) => {
                Self::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt
            }
            ValuedOperand::r4000allegrex_vconstant(..) => Self::r4000allegrex_vconstant,
            ValuedOperand::r4000allegrex_power_of_two(..) => Self::r4000allegrex_power_of_two,
            ValuedOperand::r4000allegrex_vfpu_cc_bit(..) => Self::r4000allegrex_vfpu_cc_bit,
            ValuedOperand::r4000allegrex_bn(..) => Self::r4000allegrex_bn,
            ValuedOperand::r4000allegrex_int16(..) => Self::r4000allegrex_int16,
            ValuedOperand::r4000allegrex_float16(..) => Self::r4000allegrex_float16,
            ValuedOperand::r4000allegrex_p_vrot_code(..) => Self::r4000allegrex_p_vrot_code,
            ValuedOperand::r4000allegrex_t_vrot_code(..) => Self::r4000allegrex_t_vrot_code,
            ValuedOperand::r4000allegrex_q_vrot_code(..) => Self::r4000allegrex_q_vrot_code,
            ValuedOperand::r4000allegrex_rpx(..) => Self::r4000allegrex_rpx,
            ValuedOperand::r4000allegrex_rpy(..) => Self::r4000allegrex_rpy,
            ValuedOperand::r4000allegrex_rpz(..) => Self::r4000allegrex_rpz,
            ValuedOperand::r4000allegrex_rpw(..) => Self::r4000allegrex_rpw,
            ValuedOperand::r4000allegrex_wpx(..) => Self::r4000allegrex_wpx,
            ValuedOperand::r4000allegrex_wpy(..) => Self::r4000allegrex_wpy,
            ValuedOperand::r4000allegrex_wpz(..) => Self::r4000allegrex_wpz,
            ValuedOperand::r4000allegrex_wpw(..) => Self::r4000allegrex_wpw,
            ValuedOperand::r5900_I(..) => Self::r5900_I,
            ValuedOperand::r5900_Q(..) => Self::r5900_Q,
            ValuedOperand::r5900_R(..) => Self::r5900_R,
            ValuedOperand::r5900_ACC(..) => Self::r5900_ACC,
            ValuedOperand::r5900_ACCxyzw(..) => Self::r5900_ACCxyzw,
            ValuedOperand::r5900_vfs(..) => Self::r5900_vfs,
            ValuedOperand::r5900_vft(..) => Self::r5900_vft,
            ValuedOperand::r5900_vfd(..) => Self::r5900_vfd,
            ValuedOperand::r5900_vfsxyzw(..) => Self::r5900_vfsxyzw,
            ValuedOperand::r5900_vftxyzw(..) => Self::r5900_vftxyzw,
            ValuedOperand::r5900_vfdxyzw(..) => Self::r5900_vfdxyzw,
            ValuedOperand::r5900_vfsn(..) => Self::r5900_vfsn,
            ValuedOperand::r5900_vftn(..) => Self::r5900_vftn,
            ValuedOperand::r5900_vfdn(..) => Self::r5900_vfdn,
            ValuedOperand::r5900_vfsl(..) => Self::r5900_vfsl,
            ValuedOperand::r5900_vftl(..) => Self::r5900_vftl,
            ValuedOperand::r5900_vfdl(..) => Self::r5900_vfdl,
            ValuedOperand::r5900_vfsm(..) => Self::r5900_vfsm,
            ValuedOperand::r5900_vftm(..) => Self::r5900_vftm,
            ValuedOperand::r5900_vfdm(..) => Self::r5900_vfdm,
            ValuedOperand::r5900_vis(..) => Self::r5900_vis,
            ValuedOperand::r5900_vit(..) => Self::r5900_vit,
            ValuedOperand::r5900_vid(..) => Self::r5900_vid,
            ValuedOperand::r5900_vis_predecr(..) => Self::r5900_vis_predecr,
            ValuedOperand::r5900_vit_predecr(..) => Self::r5900_vit_predecr,
            ValuedOperand::r5900_vid_predecr(..) => Self::r5900_vid_predecr,
            ValuedOperand::r5900_vis_postincr(..) => Self::r5900_vis_postincr,
            ValuedOperand::r5900_vit_postincr(..) => Self::r5900_vit_postincr,
            ValuedOperand::r5900_vid_postincr(..) => Self::r5900_vid_postincr,
            ValuedOperand::r5900_vis_parenthesis(..) => Self::r5900_vis_parenthesis,
            ValuedOperand::r5900_immediate5(..) => Self::r5900_immediate5,
            ValuedOperand::r5900_immediate15(..) => Self::r5900_immediate15,
        }
    }
}
