/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionR4000Allegrex.h"
#include "common/RabbitizerConfig.h"

#define RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, ...)    \
    case (caseBits):                                            \
        self->uniqueId = RABBITIZER_INSTR_ID_##prefix##_##name; \
        break;
#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, caseBits, name, altname, ...) \
    RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, __VA_ARGS__)

void RabbitizerInstructionR4000Allegrex_processUniqueId_Normal(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_NORMAL;

    switch (opcode) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_normal.inc"

        default:
            RabbitizerInstruction_processUniqueId_Normal(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Special(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_SPECIAL;

    switch (function) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_special.inc"

        case 0x02:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Special_Rs(self);
            fetchDescriptor = false;
            break;

        case 0x06:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Special_Sa(self);
            fetchDescriptor = false;
            break;

        default:
            RabbitizerInstruction_processUniqueId_Special(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Special_Rs(RabbitizerInstruction *self) {
    uint32_t rs = RAB_INSTR_GET_rs(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_rs(self->_mandatorybits, rs);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_SPECIAL_RS;

    switch (rs) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_special_rs.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Special_Sa(RabbitizerInstruction *self) {
    uint32_t sa = RAB_INSTR_GET_sa(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_sa(self->_mandatorybits, sa);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_SPECIAL_SA;

    switch (sa) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_special_sa.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Regimm(RabbitizerInstruction *self) {
    uint32_t rt = RAB_INSTR_GET_rt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_rt(self->_mandatorybits, rt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_REGIMM;

    switch (rt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_regimm.inc"

        default:
            RabbitizerInstruction_processUniqueId_Regimm(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Special2(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_SPECIAL2;

    switch (function) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_special2.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Special3(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_SPECIAL3;

    switch (function) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_special3.inc"

        case 0x20:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Special3_Bshfl(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Special3_Bshfl(RabbitizerInstruction *self) {
    uint32_t sa = RAB_INSTR_GET_sa(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_sa(self->_mandatorybits, sa);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_SPECIAL3_BSHFL;

    switch (sa) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_special3_bshfl.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor0(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP0;
    self->_handwrittenCategory = true;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_cop0.inc"

        case 0x08:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor0_BC0(self);
            fetchDescriptor = false;
            break;

        case 0x10:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor0_Tlb(self);
            fetchDescriptor = false;
            break;

        default:
            RabbitizerInstruction_processUniqueId_Coprocessor0(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor0_BC0(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_bc_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_bc_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP0_BC0;
    self->_handwrittenCategory = true;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_cop0_bc0.inc"

        default:
            RabbitizerInstruction_processUniqueId_Coprocessor0_BC0(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor0_Tlb(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP0_BC0;
    self->_handwrittenCategory = true;

    switch (function) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_cop0_tlb.inc"

        default:
            RabbitizerInstruction_processUniqueId_Coprocessor0_Tlb(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor1(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP1;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_cop1.inc"

        case 0x08:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor1_BC1(self);
            fetchDescriptor = false;
            break;

        case 0x10:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor1_FpuS(self);
            fetchDescriptor = false;
            break;

        case 0x14:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor1_FpuW(self);
            fetchDescriptor = false;
            break;

        case 0x11:
        case 0x15:
            // Allegrex doesn't have D and L?
            break;

        default:
            RabbitizerInstruction_processUniqueId_Coprocessor1(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor1_BC1(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_bc_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_bc_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP1_BC1;
    self->_handwrittenCategory = true;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_cop1_bc1.inc"

        default:
            RabbitizerInstruction_processUniqueId_Coprocessor1_BC1(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor1_FpuS(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP1_FPUS;

    switch (function) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_cop1_fpu_s.inc"

        case 0x08:
        case 0x09:
        case 0x0A:
        case 0x0B:
        case 0x21:
        case 0x25:
            break;

        default:
            RabbitizerInstruction_processUniqueId_Coprocessor1_FpuS(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor1_FpuW(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP1_FPUW;

    switch (function) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_cop1_fpu_w.inc"

        case 0x21:
            break;

        default:
            RabbitizerInstruction_processUniqueId_Coprocessor1_FpuW(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP2;
    // self->_handwrittenCategory = true;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_cop2.inc"

        case 0x08:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_BC2(self);
            fetchDescriptor = false;
            break;

        case 0x3:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_MFHC2(self);
            fetchDescriptor = false;
            break;

        case 0x7:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_MTHC2(self);
            fetchDescriptor = false;
            break;

        default:
            RabbitizerInstruction_processUniqueId_Coprocessor2(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_BC2(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_bc2_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_bc2_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP2_BC2;
    // self->_handwrittenCategory = true;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_cop2_bc2.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_MFHC2(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_mxhc2(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_mxhc2(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP2_MFHC2;
    // self->_handwrittenCategory = true;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_cop2_mfhc2.inc"

        case 0x1:
            fetchDescriptor = false;
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_MFHC2_p(self);
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_MFHC2_p(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_mfhc2_p_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_mfhc2_p_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP2_MFHC2_P;
    // self->_handwrittenCategory = true;

    switch (fmt) {
        case 0x7:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_MFHC2_p_s(self);
            fetchDescriptor = false;
            break;

#include "tables/instr_id/r4000allegrex/r4000allegrex_cop2_mfhc2_p.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_MFHC2_p_s(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_mfhc2_p_s_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_mfhc2_p_s_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP2_MFHC2_P_S;
    // self->_handwrittenCategory = true;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_cop2_mfhc2_p_s.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_MTHC2(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_mxhc2(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_mxhc2(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP2_MTHC2;
    // self->_handwrittenCategory = true;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_cop2_mthc2.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu0(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU0;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu0.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu1(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU1;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu1.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu3(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU3;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu3.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4.inc"

        case 0x0:
        case 0x1:
        case 0x2:
        case 0x3:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0(self);
            fetchDescriptor = false;
            break;

        case 0x8:
        case 0x9:
        case 0xA:
        case 0xB:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt2(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT0;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt0.inc"

        case 0x0:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt0(self);
            fetchDescriptor = false;
            break;

        case 0x2:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt2(self);
            fetchDescriptor = false;
            break;

        case 0x3:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt3(self);
            fetchDescriptor = false;
            break;

        case 0x4:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Rnd(self);
            fetchDescriptor = false;
            break;

        case 0x6:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_CvtFlt(self);
            fetchDescriptor = false;
            break;

        case 0x7:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_CvtInt(self);
            fetchDescriptor = false;
            break;

        case 0x8:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt8(self);
            fetchDescriptor = false;
            break;

        case 0x9:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt9(self);
            fetchDescriptor = false;
            break;

        case 0xA:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Control(self);
            fetchDescriptor = false;
            break;

        case 0xB:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Color(self);
            fetchDescriptor = false;
            break;

        case 0xC:
        case 0xD:
        case 0xE:
        case 0xF:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Cst(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt0(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT0_FMT0;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt0_fmt0.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt2(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT0_FMT2;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt0_fmt2.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt3(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT0_FMT3;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt0_fmt3.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Rnd(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT0_RND;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt0_rnd.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_CvtFlt(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT0_CVTFLT;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt0_cvtflt.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_CvtInt(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT0_CVTINT;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt0_cvtint.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt8(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT0_FMT8;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt0_fmt8.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt9(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT0_FMT9;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt0_fmt9.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Control(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT0_CONTROL;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt0_control.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Color(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT0_COLOR;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt0_color.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Cst(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_tp(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_tp(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT0_CST;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt0_cst.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt2(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt2_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt2_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT2;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt2.inc"

        case 0x14:
        case 0x15:
        case 0x16:
        case 0x17:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt2_CndMove(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt2_CndMove(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt2_cndmove_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt2_cndmove_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU4_FMT2_CNDMOVE;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu4_fmt2_cndmove.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu5(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu5_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu5_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU5;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu5.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu6(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu6_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu6_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU6;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu6.inc"

        case 0x1C:
        case 0x1D:
        case 0x1E:
        case 0x1F:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu6_Fmt7(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu6_Fmt7(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu6_fmt7_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu6_fmt7_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU6_FMT7;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu6_fmt7.inc"

        case 0x0:
        case 0x1:
        case 0x2:
        case 0x3:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu6_Fmt7_Fmt0(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu6_Fmt7_Fmt0(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu6_fmt7_fmt0_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu6_fmt7_fmt0_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU6_FMT7_FMT0;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu6_fmt7_fmt0.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu7(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_R4000ALLEGREX_GET_vfpu7_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_vfpu7_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_VFPU7;

    switch (fmt) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_vfpu7.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR4000Allegrex_processUniqueId_Quadlr(RabbitizerInstruction *self) {
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_wb(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_R4000ALLEGREX_PACK_wb(self->_mandatorybits, temp);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_QUADLR;
    self->_handwrittenCategory = true;

    switch (temp) {
#include "tables/instr_id/r4000allegrex/r4000allegrex_quadlr.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME

void RabbitizerInstructionR4000Allegrex_processUniqueId(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_INVALID;

    switch (opcode) {
        default:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Normal(self);
            break;
        case 0x00:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Special(self);
            break;
        case 0x01:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Regimm(self);
            break;

        case 0x1C:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Special2(self);
            break;
        case 0x1F:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Special3(self);
            break;

        case 0x10:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor0(self);
            break;
        case 0x11:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor1(self);
            break;
        case 0x12:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2(self);
            break;

        case 0x18:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu0(self);
            break;
        case 0x19:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu1(self);
            break;
        case 0x1B:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu3(self);
            break;
        case 0x34:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4(self);
            break;
        case 0x37:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu5(self);
            break;
        case 0x3C:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu6(self);
            break;

        case 0x3D:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Quadlr(self);
            break;

        case 0x3F:
            RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu7(self);
            break;
    }
}
