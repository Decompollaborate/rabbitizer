/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"

#include <assert.h>

#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"
#include "instructions/RabbitizerInstructionRsp.h"

void RabbitizerInstruction_init(RabbitizerInstruction *self, uint32_t word, uint32_t vram) {
    self->word = word;
    self->_mandatorybits = 0;

    self->uniqueId = RABBITIZER_INSTR_ID_cpu_INVALID;
    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];

    self->vram = vram;
    self->_handwrittenCategory = false;
    self->inHandwrittenFunction = false;
    self->category = RABBITIZER_INSTRCAT_CPU;
}

void RabbitizerInstruction_destroy(UNUSED RabbitizerInstruction *self) {
}

/* General getters */

uint32_t RabbitizerInstruction_getRaw(const RabbitizerInstruction *self) {
    return self->word;
}

uint32_t RabbitizerInstruction_getImmediate(const RabbitizerInstruction *self) {
    return RAB_INSTR_GET_immediate(self);
}
int32_t RabbitizerInstruction_getProcessedImmediate(const RabbitizerInstruction *self) {
    if (RabbitizerInstrDescriptor_isUnsigned(self->descriptor)) {
        return RAB_INSTR_GET_immediate(self);
    }
    return RabbitizerUtils_From2Complement(RAB_INSTR_GET_immediate(self), 16);
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
        vram |= (self->vram + 4) & 0xFF000000;
    }
    return vram;
}

int32_t RabbitizerInstruction_getBranchOffset(const RabbitizerInstruction *self) {
    int32_t diff = RabbitizerUtils_From2Complement(RabbitizerInstruction_getImmediate(self), 16);

    return diff * 4 + 4;
}

int32_t RabbitizerInstruction_getGenericBranchOffset(const RabbitizerInstruction *self, uint32_t currentVram) {
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_j) {
        return RabbitizerInstruction_getInstrIndexAsVram(self) - currentVram;
    }
    return RabbitizerInstruction_getBranchOffset(self);
}

/* General getters */

void RabbitizerInstruction_blankOut(RabbitizerInstruction *self) {
    size_t i;

    for (i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID; i++) {
        switch (self->descriptor->operands[i]) {
            case RABBITIZER_OPERAND_TYPE_rs:
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_rt:
                self->word = RAB_INSTR_PACK_rt(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_rd:
                self->word = RAB_INSTR_PACK_rd(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_sa:
                self->word = RAB_INSTR_PACK_sa(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_cop0d:
                self->word = RAB_INSTR_PACK_cop0d(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_fs:
                self->word = RAB_INSTR_PACK_fs(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_ft:
                self->word = RAB_INSTR_PACK_ft(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_fd:
                self->word = RAB_INSTR_PACK_fd(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_cop1cs:
                self->word = RAB_INSTR_PACK_cop1cs(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_cop2t:
                self->word = RAB_INSTR_PACK_cop2t(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_op:
                self->word = RAB_INSTR_PACK_op(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_code:
                self->word = RAB_INSTR_PACK_code(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_LABEL:
                self->word = RAB_INSTR_PACK_instr_index(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_IMM_base:
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                self->word = RAB_INSTR_PACK_immediate(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_IMM:
                self->word = RAB_INSTR_PACK_immediate(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_rs:
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_rt:
                self->word = RAB_INSTR_PACK_rt(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_rd:
                self->word = RAB_INSTR_PACK_rd(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_cop0d:
                self->word = RAB_INSTR_PACK_cop0d(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vs:
                self->word = RAB_INSTR_RSP_PACK_vs(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vt:
                self->word = RAB_INSTR_RSP_PACK_vt(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vd:
                self->word = RAB_INSTR_RSP_PACK_vd(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh:
                self->word = RAB_INSTR_RSP_PACK_vt(self->word, 0);
                self->word = RAB_INSTR_RSP_PACK_elementhigh(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow:
                self->word = RAB_INSTR_RSP_PACK_vt(self->word, 0);
                self->word = RAB_INSTR_RSP_PACK_elementlow(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vd_vs:
                self->word = RAB_INSTR_RSP_PACK_vd(self->word, 0);
                self->word = RAB_INSTR_RSP_PACK_vs(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vd_index:
                self->word = RAB_INSTR_RSP_PACK_vd(self->word, 0);
                self->word = RAB_INSTR_RSP_PACK_index(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_offset_rs:
                self->word = RAB_INSTR_RSP_PACK_offset(self->word, 0);
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_INVALID:
            case RABBITIZER_OPERAND_TYPE_MAX:
                assert(self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_MAX);
                break;
        }
    }
}
