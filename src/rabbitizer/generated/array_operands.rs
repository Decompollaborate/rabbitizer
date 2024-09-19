/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{operand, EncodedFieldMask, Operand, OperandDescriptor};
pub static OPERANDS: [OperandDescriptor; operand::OPERAND_COUNT] = {
    let mut table = [OperandDescriptor::default(); operand::OPERAND_COUNT];
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
