/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionR4000ALLEGREX.h"
#include "common/RabbitizerConfig.h"
#include "stdio.h"

#define RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, ...)    \
    case (caseBits):                                            \
        self->uniqueId = RABBITIZER_INSTR_ID_##prefix##_##name; \
        break;
#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, caseBits, name, altname, ...) \
    RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, __VA_ARGS__)

void RabbitizerInstructionR4000ALLEGREX_processUniqueId_Normal(RabbitizerInstruction *self) {
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_NORMAL;

    RabbitizerInstruction_processUniqueId_Normal(self);
}

void RabbitizerInstructionR4000ALLEGREX_processUniqueId_Special(RabbitizerInstruction *self) {
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_SPECIAL;

    RabbitizerInstruction_processUniqueId_Special(self);
}

void RabbitizerInstructionR4000ALLEGREX_processUniqueId_Regimm(RabbitizerInstruction *self) {
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_REGIMM;

    RabbitizerInstruction_processUniqueId_Regimm(self);
}

void RabbitizerInstructionR4000ALLEGREX_processUniqueId_Coprocessor0(RabbitizerInstruction *self) {
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP0;

    RabbitizerInstruction_processUniqueId_Coprocessor0(self);
}

void RabbitizerInstructionR4000ALLEGREX_processUniqueId_Coprocessor1(RabbitizerInstruction *self) {
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP1;

    RabbitizerInstruction_processUniqueId_Coprocessor1(self);
}

void RabbitizerInstructionR4000ALLEGREX_processUniqueId_Coprocessor2(RabbitizerInstruction *self) {
    uint8_t fmt = RAB_INSTR_GET_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_COP2;

    switch (fmt) {
        default:
            RabbitizerInstruction_processUniqueId_Coprocessor2(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME

void RabbitizerInstructionR4000ALLEGREX_processUniqueId(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);
    self->instrIdType = RAB_INSTR_ID_TYPE_R4000ALLEGREX_INVALID;

    switch (opcode) {
        default:
            RabbitizerInstructionR4000ALLEGREX_processUniqueId_Normal(self);
            break;
        case 0x00:
            RabbitizerInstructionR4000ALLEGREX_processUniqueId_Special(self);
            break;
        case 0x01:
            RabbitizerInstructionR4000ALLEGREX_processUniqueId_Regimm(self);
            break;
        case 0x10:
            RabbitizerInstructionR4000ALLEGREX_processUniqueId_Coprocessor0(self);
            break;
        case 0x11:
            RabbitizerInstructionR4000ALLEGREX_processUniqueId_Coprocessor1(self);
            break;
        case 0x12:
            RabbitizerInstructionR4000ALLEGREX_processUniqueId_Coprocessor2(self);
            break;
    }
}
