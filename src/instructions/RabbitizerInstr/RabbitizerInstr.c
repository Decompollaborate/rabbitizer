/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstr.h"

#include <assert.h>

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


void RabbitizerInstr_blankOut(RabbitizerInstr *self) {
    for (size_t i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_REGISTER_TYPE_INVALID; i++) {
        switch (self->descriptor->operands[i]) {
            case RABBITIZER_REGISTER_TYPE_rs:
                self->rs = 0;
                break;

            case RABBITIZER_REGISTER_TYPE_rt:
            case RABBITIZER_REGISTER_TYPE_ft:
            case RABBITIZER_REGISTER_TYPE_cop2t:
            case RABBITIZER_REGISTER_TYPE_op:
                self->rt = 0;
                break;

            case RABBITIZER_REGISTER_TYPE_rd:
            case RABBITIZER_REGISTER_TYPE_cop0d:
            case RABBITIZER_REGISTER_TYPE_fs:
            case RABBITIZER_REGISTER_TYPE_cop1cs:
                self->rd = 0;
                break;

            case RABBITIZER_REGISTER_TYPE_sa:
            case RABBITIZER_REGISTER_TYPE_fd:
                self->sa = 0;
                break;

            case RABBITIZER_REGISTER_TYPE_LABEL:
                // rs rt rd sa function
                self->function = 0;
                FALLTHROUGH;
            case RABBITIZER_REGISTER_TYPE_code:
                // rs rt rd sa
                self->rs = 0;
                self->rt = 0;
                self->rd = 0;
                self->sa = 0;
                break;

            case RABBITIZER_REGISTER_TYPE_IMM_base:
                // rs rd sa function
                self->rs = 0;
                FALLTHROUGH;
            case RABBITIZER_REGISTER_TYPE_IMM:
                // rd sa function
                self->rd = 0;
                self->sa = 0;
                self->function = 0;
                break;

            case RABBITIZER_REGISTER_TYPE_INVALID:
            case RABBITIZER_REGISTER_TYPE_MAX:
                assert(self->descriptor->operands[i] != RABBITIZER_REGISTER_TYPE_INVALID && self->descriptor->operands[i] != RABBITIZER_REGISTER_TYPE_MAX);
                break;
        }
    }
}
