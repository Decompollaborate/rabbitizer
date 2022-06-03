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
    self->descriptor = &rabbitizer_InstructionDescriptors[self->uniqueId.cpuId];

    self->extraLjustWidthOpcode = 0;
    self->vram = 0;
    self->_handwrittenCategory = false;
    self->inHandwrittenFunction = false;
}

void RabbitizerInstr_Destroy(RabbitizerInstr* self) {
}
