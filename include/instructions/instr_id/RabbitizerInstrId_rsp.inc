/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // RSP instructions
    RABBITIZER_DEF_INSTR_ID(rsp, , INVALID,               .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})

    // OP vd, vs, vt[elementhigh]
    RABBITIZER_DEF_INSTR_ID(rsp, , vmulf,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmulu,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vrndp,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmulq,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmudl,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmudm,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmudn,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmudh,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmacf,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmacu,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vrndn,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmacq,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmadl,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmadm,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmadn,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmadh,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vadd,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vsub,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vabs,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vaddc,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vsubc,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vsar,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vand,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vnand,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vor,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vnor,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vxor,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vnxor,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vlt,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , veq,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vne,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vge,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vcl,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vch,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vcr,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmrg,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})

    // OP vd[de], vt[elementhigh]
    RABBITIZER_DEF_INSTR_ID(rsp, , vrcp,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_de, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vrcpl,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_de, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}) // operands may be wrong
    RABBITIZER_DEF_INSTR_ID(rsp, , vrcph,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_de, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vmov,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_de, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vrsq,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_de, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vrsql,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_de, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})
    RABBITIZER_DEF_INSTR_ID(rsp, , vrsqh,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_de, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})

    // OP
    RABBITIZER_DEF_INSTR_ID(rsp, , vnop,                  .operands={0})

    // OP cop2t, vd[index]
    RABBITIZER_DEF_INSTR_ID(rsp, , mfc2,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_cop2t, RABBITIZER_OPERAND_TYPE_RSP_vs_index}, .modifiesRt=true)
    RABBITIZER_DEF_INSTR_ID(rsp, , mtc2,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_cop2t, RABBITIZER_OPERAND_TYPE_RSP_vs_index})

    // OP rt, cop2cd
    RABBITIZER_DEF_INSTR_ID(rsp, , cfc2,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_RSP_cop2cd}, .modifiesRt=true)
    RABBITIZER_DEF_INSTR_ID(rsp, , ctc2,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_RSP_cop2cd})

    // OP vt[elementlow], offset(vs)
    RABBITIZER_DEF_INSTR_ID(rsp, , sbv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , ssv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , slv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , sdv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , sqv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , srv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , spv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , suv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , swv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , shv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , sfv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , stv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , lbv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , lsv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , llv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , ldv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , lqv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , lrv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , lpv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , luv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , lhv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , lfv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
    RABBITIZER_DEF_INSTR_ID(rsp, , ltv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})


// CPU instructions but with rsp registers

#include "instructions/instr_id/rsp/rsp_normal.inc"
#include "instructions/instr_id/rsp/rsp_special.inc"
#include "instructions/instr_id/rsp/rsp_regimm.inc"
#include "instructions/instr_id/rsp/rsp_cop0.inc"
