/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionR5900.h"
#include "common/RabbitizerConfig.h"

#define RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, ...)    \
    case (caseBits):                                            \
        self->uniqueId = RABBITIZER_INSTR_ID_##prefix##_##name; \
        break;
#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, caseBits, name, altname, ...) \
    RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, __VA_ARGS__)

void RabbitizerInstructionR5900_processUniqueId_Normal(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_NORMAL;

    switch (opcode) {
#include "tables/instr_id/r5900/r5900_normal.inc"

        default:
            RabbitizerInstruction_processUniqueId_Normal(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR5900_processUniqueId_Special(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);
    bool fetchDescriptor = true;
    uint32_t stype;

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_SPECIAL;

    switch (function) {
#include "tables/instr_id/r5900/r5900_special.inc"

        default:
            RabbitizerInstruction_processUniqueId_Special(self);
            fetchDescriptor = false;
            break;
    }

    switch (self->uniqueId) {
        case RABBITIZER_INSTR_ID_cpu_sync:
            stype = RAB_INSTR_GET_stype(self);
            self->_mandatorybits = RAB_INSTR_PACK_stype(self->_mandatorybits, stype);
            if ((stype & 0x10) == 0x10) {
                self->uniqueId = RABBITIZER_INSTR_ID_r5900_sync_p;
                fetchDescriptor = true;
            }
            break;

        default:
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR5900_processUniqueId_Regimm(RabbitizerInstruction *self) {
    uint32_t rt = RAB_INSTR_GET_rt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_rt(self->_mandatorybits, rt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_REGIMM;

    switch (rt) {
#include "tables/instr_id/r5900/r5900_regimm.inc"

        default:
            RabbitizerInstruction_processUniqueId_Regimm(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor0_Tlb(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_COP0_TLB;

    switch (function) {
#include "tables/instr_id/r5900/r5900_cop0_tlb.inc"

        default:
            RabbitizerInstruction_processUniqueId_Coprocessor0_Tlb(self);
            break;
    }
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor0(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_fmt(self);

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_COP0;
    self->_handwrittenCategory = true;

    switch (fmt) {
#include "tables/instr_id/cpu/cpu_cop0.inc"

        case 0x08:
            RabbitizerInstruction_processUniqueId_Coprocessor0_BC0(self);
            break;

        case 0x10:
            RabbitizerInstructionR5900_processUniqueId_Coprocessor0_Tlb(self);
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor1_FpuS(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_COP1_FPUS;

    switch (function) {
#include "tables/instr_id/r5900/r5900_cop1_fpu_s.inc"

        default:
            RabbitizerInstruction_processUniqueId_Coprocessor1_FpuS(self);
            break;
    }
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor1(RabbitizerInstruction *self) {
    uint8_t fmt = RAB_INSTR_GET_fmt(self);

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_COP1;

    switch (fmt) {
#include "tables/instr_id/cpu/cpu_cop1.inc"

        case 0x08: // fmt = BC
            RabbitizerInstruction_processUniqueId_Coprocessor1_BC1(self);
            break;

        case 0x10:
            RabbitizerInstructionR5900_processUniqueId_Coprocessor1_FpuS(self);
            break;

        case 0x14:
            RabbitizerInstruction_processUniqueId_Coprocessor1_FpuW(self);
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstruction_processUniqueId_Coprocessor2_BC2(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_bc_fmt(self);

    self->_mandatorybits = RAB_INSTR_PACK_bc_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_COP2_BC2;

    switch (fmt) {
#include "tables/instr_id/r5900/r5900_cop2_bc2.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor2_viwr(RabbitizerInstruction *self) {
    uint32_t fhiflo = RAB_INSTR_R5900_GET_viwr_fhilo(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_viwr_fhilo(self->_mandatorybits, fhiflo);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_COP2_VIWR;

    switch (fhiflo) {
#include "tables/instr_id/r5900/r5900_cop2_viwr.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor2_Special2(RabbitizerInstruction *self) {
    uint32_t fhiflo = RAB_INSTR_R5900_GET_fhi_flo(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_fhi_flo(self->_mandatorybits, fhiflo);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_COP2_SPECIAL2;

    switch (fhiflo) {
#include "tables/instr_id/r5900/r5900_cop2_special2.inc"

        case 0x3E:
        case 0x3F:
            RabbitizerInstructionR5900_processUniqueId_Coprocessor2_viwr(self);
            break;
    }
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor2_Special1(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_COP2_SPECIAL1;

    switch (function) {
#include "tables/instr_id/r5900/r5900_cop2_special1.inc"

        case 0x3C:
        case 0x3D:
        case 0x3E:
        case 0x3F:
            RabbitizerInstructionR5900_processUniqueId_Coprocessor2_Special2(self);
            break;
    }
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor2_ni(RabbitizerInstruction *self) {
    uint8_t fmt = RAB_INSTR_R5900_GET_cop2_nohighbit_fmt(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_cop2_nohighbit_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_COP2_NOHIGHBIT;

    switch (fmt) {
#include "tables/instr_id/r5900/r5900_cop2_ni.inc"

        case 0x08:
            RabbitizerInstruction_processUniqueId_Coprocessor2_BC2(self);
            break;
    }
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor2_i(RabbitizerInstruction *self) {
    uint8_t fmt = RAB_INSTR_R5900_GET_cop2_nohighbit_fmt(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_cop2_nohighbit_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_COP2_NOHIGHBIT;

    switch (fmt) {
#include "tables/instr_id/r5900/r5900_cop2_i.inc"

        case 0x08:
            RabbitizerInstruction_processUniqueId_Coprocessor2_BC2(self);
            break;
    }
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor2(RabbitizerInstruction *self) {
    uint8_t fmt = RAB_INSTR_R5900_GET_cop2_highlowbit(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_cop2_highlowbit(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_COP2;

    switch (fmt) {
        case 0x00:
            RabbitizerInstructionR5900_processUniqueId_Coprocessor2_ni(self);
            break;

        case 0x01:
            RabbitizerInstructionR5900_processUniqueId_Coprocessor2_i(self);
            break;

        case 0x02:
        case 0x03:
            RabbitizerInstructionR5900_processUniqueId_Coprocessor2_Special1(self);
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstructionR5900_processUniqueId_MMI_0(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_MMI_0;

    switch (function) {
#include "tables/instr_id/r5900/r5900_mmi_0.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI_1(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_MMI_1;

    switch (function) {
#include "tables/instr_id/r5900/r5900_mmi_1.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI_2(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_MMI_2;

    switch (function) {
#include "tables/instr_id/r5900/r5900_mmi_2.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI_3(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_MMI_3;

    switch (function) {
#include "tables/instr_id/r5900/r5900_mmi_3.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI_PMFHL(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_MMI_PMFHL;

    switch (function) {
#include "tables/instr_id/r5900/r5900_mmi_pmfhl.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI_PMTHL(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_MMI_PMTHL;

    switch (function) {
#include "tables/instr_id/r5900/r5900_mmi_pmthl.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_MMI;

    switch (function) {
#include "tables/instr_id/r5900/r5900_mmi.inc"

        case 0x08:
            RabbitizerInstructionR5900_processUniqueId_MMI_0(self);
            break;
        case 0x09:
            RabbitizerInstructionR5900_processUniqueId_MMI_2(self);
            break;
        case 0x28:
            RabbitizerInstructionR5900_processUniqueId_MMI_1(self);
            break;
        case 0x29:
            RabbitizerInstructionR5900_processUniqueId_MMI_3(self);
            break;

        case 0x30:
            RabbitizerInstructionR5900_processUniqueId_MMI_PMFHL(self);
            break;
        case 0x31:
            RabbitizerInstructionR5900_processUniqueId_MMI_PMTHL(self);
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME

void RabbitizerInstructionR5900_processUniqueId(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);
    self->instrIdType = RAB_INSTR_ID_TYPE_R5900_INVALID;

    switch (opcode) {
        default:
            RabbitizerInstructionR5900_processUniqueId_Normal(self);
            break;
        case 0x00:
            RabbitizerInstructionR5900_processUniqueId_Special(self);
            break;
        case 0x01:
            RabbitizerInstructionR5900_processUniqueId_Regimm(self);
            break;
        case 0x10:
            RabbitizerInstructionR5900_processUniqueId_Coprocessor0(self);
            break;
        case 0x11:
            RabbitizerInstructionR5900_processUniqueId_Coprocessor1(self);
            break;
        case 0x12:
            RabbitizerInstructionR5900_processUniqueId_Coprocessor2(self);
            break;
        case 0x1C:
            RabbitizerInstructionR5900_processUniqueId_MMI(self);
            break;
    }
}
