/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstr.h"

#include "common/Utils.h"
#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"


void RabbitizerInstr_init(RabbitizerInstr *self, uint32_t word) {
    self->opcode = (word >> 26) & 0x3F;
    self->rs = (word >> 21) & 0x1F;
    self->rt = (word >> 16) & 0x1F;
    self->rd = (word >> 11) & 0x1F;
    self->sa = (word >>  6) & 0x1F;
    self->function = (word >> 0) & 0x3F;

    self->uniqueId = RABBITIZER_INSTR_ID_cpu_INVALID;
    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];

    self->vram = 0;
    self->_handwrittenCategory = false;
    self->inHandwrittenFunction = false;
}

void RabbitizerInstr_destroy(RabbitizerInstr *self) {
    (void)self;
}


/* Register getters */

uint8_t RabbitizerInstr_getFs(const RabbitizerInstr *self) {
    return self->rd;
}
uint8_t RabbitizerInstr_getFt(const RabbitizerInstr *self) {
    return self->rt;
}
uint8_t RabbitizerInstr_getFd(const RabbitizerInstr *self) {
    return self->sa;
}

/* Register getters */


/* Coprocessor stuffs */

uint8_t RabbitizerInstr_getFmt(const RabbitizerInstr *self) {
    return self->rs;
}

uint8_t RabbitizerInstr_getTf(const RabbitizerInstr *self) {
    return self->rt & 0x1;
}
uint8_t RabbitizerInstr_getNd(const RabbitizerInstr *self) {
    return (self->rt >> 1) & 0x1;
}
uint8_t RabbitizerInstr_getFc(const RabbitizerInstr *self) {
    return (self->function >> 4) & 0x3;
}
uint8_t RabbitizerInstr_getCond(const RabbitizerInstr *self) {
    return self->function & 0xF;
}

/* Coprocessor stuffs */


/* General getters */

uint32_t RabbitizerInstr_getRaw(const RabbitizerInstr *self) {
    return (self->opcode << 26) | (self->rs << 21) | (self->rt << 16) | (self->rd << 11) | (self->sa << 6) | (self->function);
}

uint32_t RabbitizerInstr_getImmediate(const RabbitizerInstr *self) {
    return (self->rd << 11) | (self->sa << 6) | (self->function);
}

uint32_t RabbitizerInstr_getInstrIndex(const RabbitizerInstr *self) {
    return (self->rs << 21) | (self->rt << 16) | (self->rd << 11) | (self->sa << 6) | (self->function);
}

uint32_t RabbitizerInstr_getInstrIndexAsVram(const RabbitizerInstr *self) {
    uint32_t vram = RabbitizerInstr_getInstrIndex(self) << 2;

    if (self->vram == 0) {
        vram |= 0x80000000;
    } else {
        // Jumps are PC-region branches. The upper bits are filled with the address in the delay slot
        vram |= (self->vram+4) & 0xFF000000;
    }
    return vram;
}

int32_t RabbitizerInstr_getBranchOffset(const RabbitizerInstr *self) {
    int32_t diff = RabbitizerUtils_From2Complement(RabbitizerInstr_getImmediate(self), 16);

    return diff*4 + 4;
}

/* General getters */
