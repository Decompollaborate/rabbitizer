/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstr.h"


void RabbitizerInstr_Init(RabbitizerInstr *self, uint32_t word) {
    self->opcode = (word >> 26) & 0x3F;
    self->rs = (word >> 21) & 0x1F;
    self->rt = (word >> 16) & 0x1F;
    self->rd = (word >> 11) & 0x1F;
    self->sa = (word >>  6) & 0x1F;
    self->function = (word >> 0) & 0x3F;

    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_INVALID;
    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId.cpuId];

    self->extraLjustWidthOpcode = 0;
    self->vram = 0;
    self->_handwrittenCategory = false;
    self->inHandwrittenFunction = false;
}

void RabbitizerInstr_Destroy(RabbitizerInstr* self) {
}



uint32_t RabbitizerInstr_GetInstrIndex(RabbitizerInstr *self) {
    return (self->rs << 21) | (self->rt << 16) | (self->rd << 11) | (self->sa << 6) | (self->function);
}

uint32_t RabbitizerInstr_GetInstrIndexAsVram(RabbitizerInstr *self) {
    uint32_t vram = RabbitizerInstr_GetInstrIndex(self) << 2;

    if (self->vram == 0) {
        vram |= 0x80000000;
    } else {
        // Jumps are PC-region branches. The upper bits are filled with the address in the delay slot
        vram |= (self->vram+4) & 0xFF000000;
    }
    return vram;
}
