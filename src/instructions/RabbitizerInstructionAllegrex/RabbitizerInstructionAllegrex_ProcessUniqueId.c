/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionAllegrex.h"
#include "common/RabbitizerConfig.h"

#define RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, ...)    \
    case (caseBits):                                            \
        self->uniqueId = RABBITIZER_INSTR_ID_##prefix##_##name; \
        break;
#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, caseBits, name, altname, ...) \
    RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, __VA_ARGS__)

void RabbitizerInstructionAllegrex_processUniqueId_Normal(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_NORMAL;

    switch (opcode) {
#include "tables/instr_id/allegrex/allegrex_normal.inc"

        default:
           RabbitizerInstruction_processUniqueId_Normal(self);
           fetchDescriptor = false;
           break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionAllegrex_processUniqueId_Special(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_SPECIAL;

    switch (function) {
#include "tables/instr_id/allegrex/allegrex_special.inc"

        case 0x02:
           RabbitizerInstructionAllegrex_processUniqueId_Special_Rs(self);
           fetchDescriptor = false;
           break;

        case 0x06:
           RabbitizerInstructionAllegrex_processUniqueId_Special_Sa(self);
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

void RabbitizerInstructionAllegrex_processUniqueId_Special_Rs(RabbitizerInstruction *self) {
    uint32_t rs = RAB_INSTR_GET_rs(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_rs(self->_mandatorybits, rs);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_SPECIAL_RS;

    switch (rs) {
#include "tables/instr_id/allegrex/allegrex_special_rs.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionAllegrex_processUniqueId_Special_Sa(RabbitizerInstruction *self) {
    uint32_t sa = RAB_INSTR_GET_sa(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_sa(self->_mandatorybits, sa);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_SPECIAL_SA;

    switch (sa) {
#include "tables/instr_id/allegrex/allegrex_special_sa.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionAllegrex_processUniqueId_Regimm(RabbitizerInstruction *self) {
    uint32_t rt = RAB_INSTR_GET_rt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_rt(self->_mandatorybits, rt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_REGIMM;

    switch (rt) {
#include "tables/instr_id/allegrex/allegrex_regimm.inc"

        default:
           RabbitizerInstruction_processUniqueId_Regimm(self);
           fetchDescriptor = false;
           break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME

void RabbitizerInstructionAllegrex_processUniqueId(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_INVALID;

    switch (opcode) {
        default:
            RabbitizerInstructionAllegrex_processUniqueId_Normal(self);
            break;
        case 0x00:
            RabbitizerInstructionAllegrex_processUniqueId_Special(self);
            break;
        case 0x01:
            RabbitizerInstructionAllegrex_processUniqueId_Regimm(self);
            break;
    }
}
