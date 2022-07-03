/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"

#include <assert.h>

#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"
#include "instructions/RabbitizerInstructionRsp.h"


void RabbitizerInstruction_init(RabbitizerInstruction *self, uint32_t word) {
    self->word = word;

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
    return RAB_INSTR_GET_fs(self);
}
uint8_t RabbitizerInstruction_getFt(const RabbitizerInstruction *self) {
    return RAB_INSTR_GET_ft(self);
}
uint8_t RabbitizerInstruction_getFd(const RabbitizerInstruction *self) {
    return RAB_INSTR_GET_fd(self);
}

/* Register getters */


/* Coprocessor stuffs */

uint8_t RabbitizerInstruction_getFmt(const RabbitizerInstruction *self) {
    return RAB_INSTR_GET_fmt(self);
}

uint8_t RabbitizerInstruction_getTf(const RabbitizerInstruction *self) {
    return RAB_INSTR_GET_rt(self) & 0x1;
}
uint8_t RabbitizerInstruction_getNd(const RabbitizerInstruction *self) {
    return (RAB_INSTR_GET_rt(self) >> 1) & 0x1;
}
uint8_t RabbitizerInstruction_getFc(const RabbitizerInstruction *self) {
    return RAB_INSTR_GET_fc(self);
}
uint8_t RabbitizerInstruction_getCond(const RabbitizerInstruction *self) {
    return RAB_INSTR_GET_cond(self);
}

/* Coprocessor stuffs */


/* General getters */

uint32_t RabbitizerInstruction_getRaw(const RabbitizerInstruction *self) {
    return self->word;
}

uint32_t RabbitizerInstruction_getImmediate(const RabbitizerInstruction *self) {
    return RAB_INSTR_GET_immediate(self);
}

uint32_t RabbitizerInstruction_getInstrIndex(const RabbitizerInstruction *self) {
    return RAB_INSTR_GET_instr_index(self);
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
                RAB_INSTR_SET_rs(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_rt:
            case RABBITIZER_OPERAND_TYPE_cop2t:
            case RABBITIZER_OPERAND_TYPE_op:
            case RABBITIZER_OPERAND_TYPE_RSP_rt:
            case RABBITIZER_OPERAND_TYPE_RSP_vt:
                RAB_INSTR_SET_rt(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_rd:
            case RABBITIZER_OPERAND_TYPE_cop0d:
            case RABBITIZER_OPERAND_TYPE_cop1cs:
            case RABBITIZER_OPERAND_TYPE_RSP_rd:
            case RABBITIZER_OPERAND_TYPE_RSP_vd:
            case RABBITIZER_OPERAND_TYPE_RSP_cop0d:
                RAB_INSTR_SET_rd(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_sa:
                RAB_INSTR_SET_sa(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_fs:
                RAB_INSTR_SET_fs(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_ft:
                RAB_INSTR_SET_ft(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_fd:
                RAB_INSTR_SET_fd(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_LABEL:
                RAB_INSTR_SET_instr_index(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_code:
                RAB_INSTR_SET_code(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_IMM_base:
                // rs imm
                RAB_INSTR_SET_rs(self, 0);
                FALLTHROUGH;
            case RABBITIZER_OPERAND_TYPE_IMM:
                RAB_INSTR_SET_immediate(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh:
                RAB_INSTR_RSP_SET_vt(self, 0);
                RAB_INSTR_RSP_SET_elementhigh(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow:
                RAB_INSTR_RSP_SET_vt(self, 0);
                RAB_INSTR_RSP_SET_elementlow(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vd_vs:
                RAB_INSTR_RSP_SET_vd(self, 0);
                RAB_INSTR_RSP_SET_vs(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vd_index:
                RAB_INSTR_RSP_SET_vd(self, 0);
                RAB_INSTR_RSP_SET_index(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_offset_rs:
                RAB_INSTR_RSP_SET_offset(self, 0);
                RAB_INSTR_SET_rs(self, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_INVALID:
            case RABBITIZER_OPERAND_TYPE_MAX:
                assert(self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_MAX);
                break;
        }
    }
}
