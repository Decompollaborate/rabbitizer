/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionR3000_GTE.h"
#include "common/RabbitizerConfig.h"

#define RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, ...)    \
    case (caseBits):                                            \
        self->uniqueId = RABBITIZER_INSTR_ID_##prefix##_##name; \
        break;
#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, caseBits, name, altname, ...) \
    RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, __VA_ARGS__)


void RabbitizerInstructionR3000_GTE_processUniqueId_Normal(RabbitizerInstruction *self) {
    RabbitizerInstruction_processUniqueId_Normal(self);
}

void RabbitizerInstructionR3000_GTE_processUniqueId_Special(RabbitizerInstruction *self) {
    RabbitizerInstruction_processUniqueId_Special(self);
}

void RabbitizerInstructionR3000_GTE_processUniqueId_Regimm(RabbitizerInstruction *self) {
    RabbitizerInstruction_processUniqueId_Regimm(self);
}

void RabbitizerInstructionR3000_GTE_processUniqueId_Coprocessor0(RabbitizerInstruction *self) {
    RabbitizerInstruction_processUniqueId_Coprocessor0(self);
}

void RabbitizerInstructionR3000_GTE_processUniqueId_Coprocessor1(RabbitizerInstruction *self) {
    RabbitizerInstruction_processUniqueId_Coprocessor1(self);
}

void RabbitizerInstructionR3000_GTE_processUniqueId_Coprocessor2(RabbitizerInstruction *self) {
    uint8_t fmt = RAB_INSTR_GET_fmt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);

    switch (fmt) {
#include "instructions/instr_id/r3000_gte/r3000_gte_cop2.inc"

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

void RabbitizerInstructionR3000_GTE_processUniqueId(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);

    switch (opcode) {
        default:
            RabbitizerInstructionR3000_GTE_processUniqueId_Normal(self);
            break;
        case 0x00:
            RabbitizerInstructionR3000_GTE_processUniqueId_Special(self);
            break;
        case 0x01:
            RabbitizerInstructionR3000_GTE_processUniqueId_Regimm(self);
            break;
        case 0x10:
            RabbitizerInstructionR3000_GTE_processUniqueId_Coprocessor0(self);
            break;
        case 0x11:
            RabbitizerInstructionR3000_GTE_processUniqueId_Coprocessor1(self);
            break;
        case 0x12:
            RabbitizerInstructionR3000_GTE_processUniqueId_Coprocessor2(self);
            break;
    }
}
