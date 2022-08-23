/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionR5900.h"


#define SET_UNIQUE_ID_CASE(self, caseBits, mnemonic) \
        case (caseBits): \
            (self)->uniqueId = RABBITIZER_INSTR_ID_r5900_##mnemonic; \
            break

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

void RabbitizerInstructionR5900_processUniqueId_MMI0(UNUSED RabbitizerInstruction *self) {
    // TODO
}

void RabbitizerInstructionR5900_processUniqueId_MMI1(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);

    switch (function) {
/*
    |---00--|---01--|---10--|---11--| lo
000 |  ---  | PABSW | PCEQW | PMINW |
001 |PADSBH | PABSH | PCEQH | PMINH |
010 |  ---  |  ---  | PCEQB |  ---  |
011 |  ---  |  ---  |  ---  |  ---  |
100 |PADDUW |PSUBUW |PEXTUW |  ---  |
101 |PADDUH |PSUBUH |PEXTUH |  ---  |
110 |PADDUB |PSUBUB |PEXTUB | QFSRV |
111 |  ---  |  ---  |  ---  |  ---  |
 hi |-------|-------|-------|-------|
*/
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

void RabbitizerInstructionR5900_processUniqueId_MMI2(UNUSED RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_R5900_GET_mmi_function(self);

    self->_mandatorybits = RAB_INSTR_R5900_PACK_mmi_function(self->_mandatorybits, function);

    switch (function) {
/*
    |---00--|---01--|---10--|---11--| lo
000 |PMADDW |  ---  |PSLLVW |PSRLVW |
001 |PMSUBW |  ---  |  ---  |  ---  |
010 |PMFHI  |PMFLO  |PINTH  |  ---  |
011 |PMULTW |PDIVW  |PCPYLD |  ---  |
100 |PMADDH |PHMADH | PAND  |  PXOR |
101 |PMSUBH |PHMSBH |  ---  |  ---  |
110 | ---   |  ---  | PEXEH | PREVH |
111 |PMULTH |PDIVBW | PEXEW |PROT3W |
 hi |-------|-------|-------|-------|
*/
        SET_UNIQUE_ID_CASE(self, 0x00, pmaddw);
        SET_UNIQUE_ID_CASE(self, 0x02, psllvw);
        SET_UNIQUE_ID_CASE(self, 0x03, psrlvw);

        SET_UNIQUE_ID_CASE(self, 0x04, pmsubw);

        SET_UNIQUE_ID_CASE(self, 0x08, pmfhi);
        SET_UNIQUE_ID_CASE(self, 0x09, pmflo);
        SET_UNIQUE_ID_CASE(self, 0x0A, pinth);

        SET_UNIQUE_ID_CASE(self, 0x0C, pmultw);
        SET_UNIQUE_ID_CASE(self, 0x0D, pdivw);
        SET_UNIQUE_ID_CASE(self, 0x0E, pcpyld);

        SET_UNIQUE_ID_CASE(self, 0x10, pmaddh);
        SET_UNIQUE_ID_CASE(self, 0x11, phmadh);
        SET_UNIQUE_ID_CASE(self, 0x12, pand);
        SET_UNIQUE_ID_CASE(self, 0x13, pxor);

        SET_UNIQUE_ID_CASE(self, 0x14, pmsubh);
        SET_UNIQUE_ID_CASE(self, 0x15, phmsbh);

        SET_UNIQUE_ID_CASE(self, 0x1A, pexeh);
        SET_UNIQUE_ID_CASE(self, 0x1B, prevh);

        SET_UNIQUE_ID_CASE(self, 0x1C, pmulth);
        SET_UNIQUE_ID_CASE(self, 0x1D, pdivbw);
        SET_UNIQUE_ID_CASE(self, 0x1E, pexew);
        SET_UNIQUE_ID_CASE(self, 0x1F, prot3w);
    }
}

void RabbitizerInstructionR5900_processUniqueId_MMI3(UNUSED RabbitizerInstruction *self) {
    // TODO
}

void RabbitizerInstructionR5900_processUniqueId_MMI(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);

    switch (function) {
        case 0x04:
            RabbitizerInstructionR5900_processUniqueId_MMI0(self);
            break;
        case 0x05:
            RabbitizerInstructionR5900_processUniqueId_MMI1(self);
            break;
        case 0x28:
            RabbitizerInstructionR5900_processUniqueId_MMI2(self);
            break;
        case 0x29:
            RabbitizerInstructionR5900_processUniqueId_MMI3(self);
            break;

        // TODO
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}


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
