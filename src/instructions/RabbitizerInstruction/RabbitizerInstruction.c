/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"

#include <assert.h>

#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"
#include "instructions/RabbitizerInstructionRsp.h"
#include "instructions/RabbitizerInstructionR5900.h"

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

            case RABBITIZER_OPERAND_TYPE_zero:
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

            case RABBITIZER_OPERAND_TYPE_copraw:
                self->word = RAB_INSTR_PACK_copraw(self->word, 0);
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

            /* rsp */
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

            case RABBITIZER_OPERAND_TYPE_RSP_cop2t:
                self->word = RAB_INSTR_RSP_PACK_cop2t(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_cop2cd:
                self->word = RAB_INSTR_RSP_PACK_cop2cd(self->word, 0);
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

            case RABBITIZER_OPERAND_TYPE_RSP_vd_de:
                self->word = RAB_INSTR_RSP_PACK_vd(self->word, 0);
                self->word = RAB_INSTR_RSP_PACK_de(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vs_index:
                self->word = RAB_INSTR_RSP_PACK_vs(self->word, 0);
                self->word = RAB_INSTR_RSP_PACK_index(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_offset_rs:
                self->word = RAB_INSTR_RSP_PACK_offset(self->word, 0);
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_IMM_base:
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                self->word = RAB_INSTR_PACK_immediate(self->word, 0);
                break;
            /* rsp */

            /* r5900 */
            case RABBITIZER_OPERAND_TYPE_R5900_I:
            case RABBITIZER_OPERAND_TYPE_R5900_Q:
            case RABBITIZER_OPERAND_TYPE_R5900_R:
            case RABBITIZER_OPERAND_TYPE_R5900_ACC:
                // Not real registers encoded on the instruction itself
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_ACCxyzw:
                self->word = RAB_INSTR_R5900_PACK_xyzw_x(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_y(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_z(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_w(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfs:
                self->word = RAB_INSTR_R5900_PACK_vfs(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vft:
                self->word = RAB_INSTR_R5900_PACK_vft(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfd:
                self->word = RAB_INSTR_R5900_PACK_vfd(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfsxyzw:
                self->word = RAB_INSTR_R5900_PACK_vfs(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_x(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_y(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_z(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_w(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vftxyzw:
                self->word = RAB_INSTR_R5900_PACK_vft(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_x(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_y(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_z(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_w(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfdxyzw:
                self->word = RAB_INSTR_R5900_PACK_vfd(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_x(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_y(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_z(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_w(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfsn:
                self->word = RAB_INSTR_R5900_PACK_vfs(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_n(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vftn:
                self->word = RAB_INSTR_R5900_PACK_vft(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_n(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfdn:
                self->word = RAB_INSTR_R5900_PACK_vfd(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_n(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfsl:
                self->word = RAB_INSTR_R5900_PACK_vfs(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_l(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vftl:
                self->word = RAB_INSTR_R5900_PACK_vft(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_l(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfdl:
                self->word = RAB_INSTR_R5900_PACK_vfd(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_l(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfsm:
                self->word = RAB_INSTR_R5900_PACK_vfs(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_m(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vftm:
                self->word = RAB_INSTR_R5900_PACK_vft(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_m(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfdm:
                self->word = RAB_INSTR_R5900_PACK_vfd(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_m(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vis:
                self->word = RAB_INSTR_R5900_PACK_vis(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vit:
                self->word = RAB_INSTR_R5900_PACK_vit(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vid:
                self->word = RAB_INSTR_R5900_PACK_vid(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vis_predecr:
                self->word = RAB_INSTR_R5900_PACK_vis(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vit_predecr:
                self->word = RAB_INSTR_R5900_PACK_vit(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vid_predecr:
                self->word = RAB_INSTR_R5900_PACK_vid(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vis_postincr:
                self->word = RAB_INSTR_R5900_PACK_vis(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vit_postincr:
                self->word = RAB_INSTR_R5900_PACK_vit(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vid_postincr:
                self->word = RAB_INSTR_R5900_PACK_vid(self->word, 0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_imm5:
                self->word = RAB_INSTR_R5900_PACK_imm5(self->word, 0);
                break;
                /* r5900 */

            case RABBITIZER_OPERAND_TYPE_INVALID:
            case RABBITIZER_OPERAND_TYPE_MAX:
                assert(self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_MAX);
                break;
        }
    }
}
