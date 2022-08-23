/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionR5900.h"


#define RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, ...)    \
    case (caseBits):                                            \
        self->uniqueId = RABBITIZER_INSTR_ID_##prefix##_##name; \
        break;
#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, caseBits, name, altname, ...) RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, __VA_ARGS__)

void RabbitizerInstructionR5900_processUniqueId_Normal(RabbitizerInstruction *self) {
    RabbitizerInstruction_processUniqueId_Normal(self);
}

void RabbitizerInstructionR5900_processUniqueId_Special(RabbitizerInstruction *self) {
    RabbitizerInstruction_processUniqueId_Special(self);
}

void RabbitizerInstructionR5900_processUniqueId_Regimm(RabbitizerInstruction *self) {
    RabbitizerInstruction_processUniqueId_Regimm(self);
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor0(RabbitizerInstruction *self) {
    RabbitizerInstruction_processUniqueId_Coprocessor0(self);
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor1(RabbitizerInstruction *self) {
    RabbitizerInstruction_processUniqueId_Coprocessor1(self);
}

void RabbitizerInstructionR5900_processUniqueId_Coprocessor2(RabbitizerInstruction *self) {
    RabbitizerInstruction_processUniqueId_Coprocessor2(self);
}

void RabbitizerInstructionR5900_processUniqueId_MMI_0(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);

    switch (function) {
#include "instructions/instr_id/r5900/r5900_mmi_0.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI_1(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);

    switch (function) {
#include "instructions/instr_id/r5900/r5900_mmi_1.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI_2(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);

    switch (function) {
#include "instructions/instr_id/r5900/r5900_mmi_2.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI_3(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);

    switch (function) {
#include "instructions/instr_id/r5900/r5900_mmi_3.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);

    switch (function) {
#include "instructions/instr_id/r5900/r5900_mmi.inc"

        case 0x08:
            RabbitizerInstructionR5900_processUniqueId_MMI_0(self);
            break;
        case 0x09:
            RabbitizerInstructionR5900_processUniqueId_MMI_1(self);
            break;
        case 0x28:
            RabbitizerInstructionR5900_processUniqueId_MMI_2(self);
            break;
        case 0x29:
            RabbitizerInstructionR5900_processUniqueId_MMI_3(self);
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME


void RabbitizerInstructionR5900_processUniqueId(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);

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
