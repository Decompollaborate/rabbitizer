/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"

#include <assert.h>

#include "common/Utils.h"
#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"


void RabbitizerInstruction_init(RabbitizerInstruction *self, uint32_t word) {
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
    self->category = RABBITIZER_INSTRCAT_CPU;
}

void RabbitizerInstruction_destroy(RabbitizerInstruction *self) {
    (void)self;
}


/* Register getters */

uint8_t RabbitizerInstruction_getFs(const RabbitizerInstruction *self) {
    return self->rd;
}
uint8_t RabbitizerInstruction_getFt(const RabbitizerInstruction *self) {
    return self->rt;
}
uint8_t RabbitizerInstruction_getFd(const RabbitizerInstruction *self) {
    return self->sa;
}

/* Register getters */


/* Coprocessor stuffs */

uint8_t RabbitizerInstruction_getFmt(const RabbitizerInstruction *self) {
    return self->rs;
}

uint8_t RabbitizerInstruction_getTf(const RabbitizerInstruction *self) {
    return self->rt & 0x1;
}
uint8_t RabbitizerInstruction_getNd(const RabbitizerInstruction *self) {
    return (self->rt >> 1) & 0x1;
}
uint8_t RabbitizerInstruction_getFc(const RabbitizerInstruction *self) {
    return (self->function >> 4) & 0x3;
}
uint8_t RabbitizerInstruction_getCond(const RabbitizerInstruction *self) {
    return self->function & 0xF;
}

/* Coprocessor stuffs */


/* General getters */

uint32_t RabbitizerInstruction_getRaw(const RabbitizerInstruction *self) {
    return (self->opcode << 26) | (self->rs << 21) | (self->rt << 16) | (self->rd << 11) | (self->sa << 6) | (self->function);
}

uint32_t RabbitizerInstruction_getImmediate(const RabbitizerInstruction *self) {
    //return (self->rd << 11) | (self->sa << 6) | (self->function);
    return RAB_INSTR_GET_IMMEDIATE(self);
}

uint32_t RabbitizerInstruction_getInstrIndex(const RabbitizerInstruction *self) {
    return (self->rs << 21) | (self->rt << 16) | (self->rd << 11) | (self->sa << 6) | (self->function);
}

uint32_t RabbitizerInstruction_getInstrIndexAsVram(const RabbitizerInstruction *self) {
    uint32_t vram = RabbitizerInstruction_getInstrIndex(self) << 2;

    if (self->vram == 0) {
        vram |= 0x80000000;
    } else {
        // Jumps are PC-region branches. The upper bits are filled with the address in the delay slot
        vram |= (self->vram+4) & 0xFF000000;
    }
    return vram;
}

int32_t RabbitizerInstruction_getBranchOffset(const RabbitizerInstruction *self) {
    int32_t diff = RabbitizerUtils_From2Complement(RabbitizerInstruction_getImmediate(self), 16);

    return diff*4 + 4;
}

int32_t RabbitizerInstruction_getGenericBranchOffset(const RabbitizerInstruction *self, uint32_t currentVram) {
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_j) {
        return RabbitizerInstruction_getInstrIndexAsVram(self) - currentVram;
    }
    return RabbitizerInstruction_getBranchOffset(self);
}

/* General getters */


void RabbitizerInstruction_blankOut(RabbitizerInstruction *self) {
    for (size_t i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID; i++) {
        switch (self->descriptor->operands[i]) {
            case RABBITIZER_OPERAND_TYPE_rs:
            case RABBITIZER_OPERAND_TYPE_RSP_rs:
            case RABBITIZER_OPERAND_TYPE_RSP_vs:
                self->rs = 0;
                break;

            case RABBITIZER_OPERAND_TYPE_rt:
            case RABBITIZER_OPERAND_TYPE_ft:
            case RABBITIZER_OPERAND_TYPE_cop2t:
            case RABBITIZER_OPERAND_TYPE_op:
            case RABBITIZER_OPERAND_TYPE_RSP_rt:
            case RABBITIZER_OPERAND_TYPE_RSP_vt:
                self->rt = 0;
                break;

            case RABBITIZER_OPERAND_TYPE_rd:
            case RABBITIZER_OPERAND_TYPE_cop0d:
            case RABBITIZER_OPERAND_TYPE_fs:
            case RABBITIZER_OPERAND_TYPE_cop1cs:
            case RABBITIZER_OPERAND_TYPE_RSP_rd:
            case RABBITIZER_OPERAND_TYPE_RSP_vd:
            case RABBITIZER_OPERAND_TYPE_RSP_cop0d:
                self->rd = 0;
                break;

            case RABBITIZER_OPERAND_TYPE_sa:
            case RABBITIZER_OPERAND_TYPE_fd:
                self->sa = 0;
                break;

            case RABBITIZER_OPERAND_TYPE_LABEL:
                // rs rt rd sa function
                self->function = 0;
                FALLTHROUGH;
            case RABBITIZER_OPERAND_TYPE_code:
                // rs rt rd sa
                self->rs = 0;
                self->rt = 0;
                self->rd = 0;
                self->sa = 0;
                break;

            case RABBITIZER_OPERAND_TYPE_IMM_base:
                // rs rd sa function
                self->rs = 0;
                FALLTHROUGH;
            case RABBITIZER_OPERAND_TYPE_IMM:
                // rd sa function
                self->rd = 0;
                self->sa = 0;
                self->function = 0;
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh:
                self->rt = 0;
                self->rs &= ~0xF;
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow:
                self->rt = 0;
                self->sa &= ~0x1E;
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vd_vs:
                self->rd = 0;
                self->rs = 0;
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vd_index:
                self->rd = 0;
                self->sa &= ~0x1;
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_offset_rs:
                self->function = 0;
                self->sa &= ~0x1;
                self->rs = 0;
                break;

            case RABBITIZER_OPERAND_TYPE_INVALID:
            case RABBITIZER_OPERAND_TYPE_MAX:
                assert(self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_MAX);
                break;
        }
    }
}
