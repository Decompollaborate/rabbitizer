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

void RabbitizerInstructionR5900_processUniqueId_MMI_0(UNUSED RabbitizerInstruction *self) {
    // TODO
}

void RabbitizerInstructionR5900_processUniqueId_MMI_1(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);

    switch (function) {
        case 0x01:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_pabsw;
            break;
        case 0x02:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_pceqw;
            break;
        case 0x03:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_pminw;
            break;

        case 0x04:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_padsbh;
            break;
        case 0x05:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_pabsh;
            break;
        case 0x06:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_pceqh;
            break;
        case 0x07:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_pminh;
            break;

        case 0x0A:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_pceqb;
            break;

        case 0x10:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_padduw;
            break;
        case 0x11:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_psubuw;
            break;
        case 0x12:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_pextuw;
            break;

        case 0x14:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_padduh;
            break;
        case 0x15:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_psubuh;
            break;
        case 0x16:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_pextuh;
            break;

        case 0x18:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_paddub;
            break;
        case 0x19:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_psubub;
            break;
        case 0x1A:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_pextub;
            break;
        case 0x1B:
            self->uniqueId = RABBITIZER_INSTR_ID_r5900_qfsrv;
            break;
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI_2(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);

    switch (function) {
#include "instructions/instr_id/r5900/r5900_mmi_2.inc"
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI_3(UNUSED RabbitizerInstruction *self) {
    // TODO
}

void RabbitizerInstructionR5900_processUniqueId_MMI(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);

    switch (function) {
        case 0x04:
            RabbitizerInstructionR5900_processUniqueId_MMI_0(self);
            break;
        case 0x05:
            RabbitizerInstructionR5900_processUniqueId_MMI_1(self);
            break;
        case 0x28:
            RabbitizerInstructionR5900_processUniqueId_MMI_2(self);
            break;
        case 0x29:
            RabbitizerInstructionR5900_processUniqueId_MMI_3(self);
            break;

        // TODO
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
