/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"

#include <assert.h>

#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"
#include "instructions/RabbitizerInstructionRsp.h"
#include "instructions/RabbitizerInstructionR3000GTE.h"
#include "instructions/RabbitizerInstructionR4000Allegrex.h"
#include "instructions/RabbitizerInstructionR5900.h"

void RabbitizerInstruction_init(RabbitizerInstruction *self, uint32_t word, uint32_t vram) {
    self->word = word;
    self->_mandatorybits = 0;

    self->uniqueId = RABBITIZER_INSTR_ID_cpu_INVALID;
    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    self->instrIdType = RAB_INSTR_ID_TYPE_ALL_INVALID;

    self->vram = vram;
    self->_handwrittenCategory = false;
    self->inHandwrittenFunction = false;
    self->category = RABBITIZER_INSTRCAT_CPU;

    self->flags = 0;
    RAB_INSTR_FLAGS_SET_r5900DisasmAsData(self, RAB_TRINARY_VAL_NONE);
}

void RabbitizerInstruction_destroy(UNUSED RabbitizerInstruction *self) {
}

/* General getters */

/**
 * Get the word value encoding the instruction.
 *
 * The returned value may not be the same as the one to instance this Instruction
 * if a method that modifies the word has been used, like `RabbitizerInstruction_blankOut`.
 */
uint32_t RabbitizerInstruction_getRaw(const RabbitizerInstruction *self) {
    return self->word;
}

/**
 * Get the (possibly signed) immediate value for this instruction.

 * This only makes sense for an instruction with an immediate,
 * which can be checked with
 * `RabbitizerInstruction_hasOperandAlias(&instr, RAB_OPERAND_cpu_immediate)`
 */
int32_t RabbitizerInstruction_getProcessedImmediate(const RabbitizerInstruction *self) {
    if (RabbitizerInstrDescriptor_isUnsigned(self->descriptor)) {
        return RAB_INSTR_GET_immediate(self);
    }
    return RabbitizerUtils_From2Complement(RAB_INSTR_GET_immediate(self), 16);
}

/**
 * Get the target vram address this instruction jumps to.
 * This method is intended only for direct jump instructions.

 * This only makes sense if the instruction is a direct jump,
 * which can be checked with `RabbitizerInstrDescriptor_isJumpWithAddress`.
 */
uint32_t RabbitizerInstruction_getInstrIndexAsVram(const RabbitizerInstruction *self) {
    uint32_t vram = RAB_INSTR_GET_instr_index(self) << 2;

    if (self->vram == 0) {
        vram |= 0x80000000;
    } else {
        // Jumps are PC-region branches. The upper bits are filled with the address in the delay slot
        vram |= (self->vram + 4) & 0xF0000000;
    }
    return vram;
}

/**
 * Returns the offset (in bytes) that the branch instruction would branch,
 * relative to the instruction itself. This method is intended only for branch
 * instructions.
 *
 * The returned value can be negative, meaning the branch instructions does
 * a backwards branch.
 *
 * This only makes sense for an instruction is a branch,
 * which can be checked with `RabbitizerInstrDescriptor_isBranch`.
 *
 * To get the branch offset of either a branch instruction or a jump instruction
 * use `RabbitizerInstruction_getBranchOffsetGeneric` instead.
 */
int32_t RabbitizerInstruction_getBranchOffset(const RabbitizerInstruction *self) {
    int32_t diff = RabbitizerUtils_From2Complement(RAB_INSTR_GET_immediate(self), 16);

    return diff * 4 + 4;
}

//! @deprecated
int32_t RabbitizerInstruction_getGenericBranchOffset(const RabbitizerInstruction *self, uint32_t currentVram) {
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_j) {
        return RabbitizerInstruction_getInstrIndexAsVram(self) - currentVram;
    }
    return RabbitizerInstruction_getBranchOffset(self);
}

/**
 * Returns the offset (in bytes) that the instruction would branch,
 * relative to the instruction itself. This method is intended for both branch
 * and jump instructions.
 *
 * The returned value can be either positive or negative.
 *
 * This only makes sense for an instruction is a branch or a direct jump,
 * which can be checked with `RabbitizerInstruction_getBranchOffsetGeneric`
 * or `RabbitizerInstrDescriptor_isJumpWithAddress`.
 */
int32_t RabbitizerInstruction_getBranchOffsetGeneric(const RabbitizerInstruction *self) {
    if (RabbitizerInstruction_hasOperandAlias(self, RAB_OPERAND_cpu_label)) {
        return RabbitizerInstruction_getInstrIndexAsVram(self) - self->vram;
    }
    return RabbitizerInstruction_getBranchOffset(self);
}

/**
 * Get the target vram address this instruction jumps to.
 * This method is intended only for branch or direct jump instructions.
 *
 * This only makes sense for an instruction is a branch or a direct jump,
 * which can be checked with `RabbitizerInstruction_getBranchOffsetGeneric`
 * or `RabbitizerInstrDescriptor_isJumpWithAddress`.
 */
uint32_t RabbitizerInstruction_getBranchVramGeneric(const RabbitizerInstruction *self) {
    if (RabbitizerInstruction_hasOperandAlias(self, RAB_OPERAND_cpu_label)) {
        return RabbitizerInstruction_getInstrIndexAsVram(self);
    }
    return RabbitizerInstruction_getBranchOffset(self) + self->vram;
}

/**
 * Returns the general purpose register (GPR) which this instruction modifies,
 * or a negative value if the instruction does not modify the state of any GPR
 */
int8_t RabbitizerInstruction_getDestinationGpr(const RabbitizerInstruction *self) {
    if (RabbitizerInstrDescriptor_modifiesRd(self->descriptor)) {
        return RAB_INSTR_GET_rd(self);
    }
    if (RabbitizerInstrDescriptor_modifiesRt(self->descriptor)) {
        return RAB_INSTR_GET_rt(self);
    }
    return -1;
}

/**
 * Returns `true` if the GPR which is modified by this register is $zero, `false` otherwise.
 * Returns `false` if this instruction does not modify a GPR.
 */
bool RabbitizerInstruction_outputsToGprZero(const RabbitizerInstruction *self) {
    return RabbitizerInstruction_getDestinationGpr(self) == 0;
}

/* General getters */

void RabbitizerInstruction_blankOut(RabbitizerInstruction *self) {
    size_t i;

    for (i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RAB_OPERAND_ALL_INVALID;
         i++) {
        switch (self->descriptor->operands[i]) {
            case RAB_OPERAND_cpu_rs:
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                break;

            case RAB_OPERAND_cpu_rt:
                self->word = RAB_INSTR_PACK_rt(self->word, 0);
                break;

            case RAB_OPERAND_cpu_rd:
                self->word = RAB_INSTR_PACK_rd(self->word, 0);
                break;

            case RAB_OPERAND_cpu_sa:
                self->word = RAB_INSTR_PACK_sa(self->word, 0);
                break;

            case RAB_OPERAND_cpu_zero:
                break;

            case RAB_OPERAND_cpu_cop0d:
                self->word = RAB_INSTR_PACK_cop0d(self->word, 0);
                break;

            case RAB_OPERAND_cpu_fs:
                self->word = RAB_INSTR_PACK_fs(self->word, 0);
                break;

            case RAB_OPERAND_cpu_ft:
                self->word = RAB_INSTR_PACK_ft(self->word, 0);
                break;

            case RAB_OPERAND_cpu_fd:
                self->word = RAB_INSTR_PACK_fd(self->word, 0);
                break;

            case RAB_OPERAND_cpu_cop1cs:
                self->word = RAB_INSTR_PACK_cop1cs(self->word, 0);
                break;

            case RAB_OPERAND_cpu_cop2t:
                self->word = RAB_INSTR_PACK_cop2t(self->word, 0);
                break;

            case RAB_OPERAND_cpu_cop2cd:
                self->word = RAB_INSTR_PACK_cop2cd(self->word, 0);
                break;

            case RAB_OPERAND_cpu_op:
                self->word = RAB_INSTR_PACK_op(self->word, 0);
                break;

            case RAB_OPERAND_cpu_hint:
                self->word = RAB_INSTR_PACK_hint(self->word, 0);
                break;

            case RAB_OPERAND_cpu_code:
                self->word = RAB_INSTR_PACK_code(self->word, 0);
                break;

            case RAB_OPERAND_cpu_code_lower:
                self->word = RAB_INSTR_PACK_code_lower(self->word, 0);
                break;

            case RAB_OPERAND_cpu_copraw:
                self->word = RAB_INSTR_PACK_copraw(self->word, 0);
                break;

            case RAB_OPERAND_cpu_label:
                self->word = RAB_INSTR_PACK_instr_index(self->word, 0);
                break;

            case RAB_OPERAND_cpu_immediate:
                self->word = RAB_INSTR_PACK_immediate(self->word, 0);
                break;

            case RAB_OPERAND_cpu_branch_target_label:
                self->word = RAB_INSTR_PACK_immediate(self->word, 0);
                break;

            case RAB_OPERAND_cpu_immediate_base:
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                self->word = RAB_INSTR_PACK_immediate(self->word, 0);
                break;

            case RAB_OPERAND_cpu_maybe_rd_rs:
                self->word = RAB_INSTR_PACK_rd(self->word, 0);
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                break;

            /* rsp */
            case RAB_OPERAND_rsp_rs:
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                break;

            case RAB_OPERAND_rsp_rt:
                self->word = RAB_INSTR_PACK_rt(self->word, 0);
                break;

            case RAB_OPERAND_rsp_rd:
                self->word = RAB_INSTR_PACK_rd(self->word, 0);
                break;

            case RAB_OPERAND_rsp_cop0d:
                self->word = RAB_INSTR_PACK_cop0d(self->word, 0);
                break;

            case RAB_OPERAND_rsp_cop2t:
                self->word = RAB_INSTR_RSP_PACK_cop2t(self->word, 0);
                break;

            case RAB_OPERAND_rsp_cop2cd:
                self->word = RAB_INSTR_RSP_PACK_cop2cd(self->word, 0);
                break;

            case RAB_OPERAND_rsp_hint:
                self->word = RAB_INSTR_RSP_PACK_hint(self->word, 0);
                break;

            case RAB_OPERAND_rsp_vs:
                self->word = RAB_INSTR_RSP_PACK_vs(self->word, 0);
                break;

            case RAB_OPERAND_rsp_vt:
                self->word = RAB_INSTR_RSP_PACK_vt(self->word, 0);
                break;

            case RAB_OPERAND_rsp_vd:
                self->word = RAB_INSTR_RSP_PACK_vd(self->word, 0);
                break;

            case RAB_OPERAND_rsp_vt_elementhigh:
                self->word = RAB_INSTR_RSP_PACK_vt(self->word, 0);
                self->word = RAB_INSTR_RSP_PACK_elementhigh(self->word, 0);
                break;

            case RAB_OPERAND_rsp_vt_elementlow:
                self->word = RAB_INSTR_RSP_PACK_vt(self->word, 0);
                self->word = RAB_INSTR_RSP_PACK_elementlow(self->word, 0);
                break;

            case RAB_OPERAND_rsp_vd_de:
                self->word = RAB_INSTR_RSP_PACK_vd(self->word, 0);
                self->word = RAB_INSTR_RSP_PACK_de(self->word, 0);
                break;

            case RAB_OPERAND_rsp_vs_index:
                self->word = RAB_INSTR_RSP_PACK_vs(self->word, 0);
                self->word = RAB_INSTR_RSP_PACK_index(self->word, 0);
                break;

            case RAB_OPERAND_rsp_offset_rs:
                self->word = RAB_INSTR_RSP_PACK_offset(self->word, 0);
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                break;

            case RAB_OPERAND_rsp_immediate_base:
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                self->word = RAB_INSTR_PACK_immediate(self->word, 0);
                break;

            case RAB_OPERAND_rsp_maybe_rd_rs:
                self->word = RAB_INSTR_PACK_rd(self->word, 0);
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                break;
            /* rsp */

            /* r3000gte */
            case RAB_OPERAND_r3000gte_sf:
                self->word = RAB_INSTR_R3000GTE_PACK_sf(self->word, 0);
                break;

            case RAB_OPERAND_r3000gte_mx:
                self->word = RAB_INSTR_R3000GTE_PACK_mx(self->word, 0);
                break;

            case RAB_OPERAND_r3000gte_v:
                self->word = RAB_INSTR_R3000GTE_PACK_v(self->word, 0);
                break;

            case RAB_OPERAND_r3000gte_cv:
                self->word = RAB_INSTR_R3000GTE_PACK_cv(self->word, 0);
                break;

            case RAB_OPERAND_r3000gte_lm:
                self->word = RAB_INSTR_R3000GTE_PACK_lm(self->word, 0);
                break;
            /* r3000gte */

            /* r4000allegrex */
            case RAB_OPERAND_r4000allegrex_s_vs:
            case RAB_OPERAND_r4000allegrex_p_vs:
            case RAB_OPERAND_r4000allegrex_t_vs:
            case RAB_OPERAND_r4000allegrex_q_vs:
            case RAB_OPERAND_r4000allegrex_mp_vs:
            case RAB_OPERAND_r4000allegrex_mt_vs:
            case RAB_OPERAND_r4000allegrex_mq_vs:
            case RAB_OPERAND_r4000allegrex_mp_vs_transpose:
            case RAB_OPERAND_r4000allegrex_mt_vs_transpose:
            case RAB_OPERAND_r4000allegrex_mq_vs_transpose:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_vs(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_s_vt:
            case RAB_OPERAND_r4000allegrex_p_vt:
            case RAB_OPERAND_r4000allegrex_t_vt:
            case RAB_OPERAND_r4000allegrex_q_vt:
            case RAB_OPERAND_r4000allegrex_mp_vt:
            case RAB_OPERAND_r4000allegrex_mt_vt:
            case RAB_OPERAND_r4000allegrex_mq_vt:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_vt(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_s_vd:
            case RAB_OPERAND_r4000allegrex_p_vd:
            case RAB_OPERAND_r4000allegrex_t_vd:
            case RAB_OPERAND_r4000allegrex_q_vd:
            case RAB_OPERAND_r4000allegrex_mp_vd:
            case RAB_OPERAND_r4000allegrex_mt_vd:
            case RAB_OPERAND_r4000allegrex_mq_vd:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_vd(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_s_vt_imm:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_vt_imm(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_s_vd_imm:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_vd_imm(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_q_vt_imm:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_vt_6_imm(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_cop2cd:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_cop2cd(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_cop2cs:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_cop2cs(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_pos:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_pos(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_size:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_size(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_size_plus_pos:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_size_plus_pos(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_imm3:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_imm3(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_offset14_base_maybe_wb:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_wb(self->word, 0);
                FALLTHROUGH;
            case RAB_OPERAND_r4000allegrex_offset14_base:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_offset14(self->word, 0);
                self->word = RAB_INSTR_PACK_rs(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt:
            case RAB_OPERAND_r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt:
            case RAB_OPERAND_r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt:
            case RAB_OPERAND_r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_vs(self->word, 0);
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_vt(self->word, 0);
                FALLTHROUGH;
            case RAB_OPERAND_r4000allegrex_vcmp_cond:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_vcmp_cond(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_vconstant:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_vconstant(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_power_of_two:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_power_of_two(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_vfpu_cc_bit:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_vfpu_cc_bit(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_bn:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_bn(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_int16:
            case RAB_OPERAND_r4000allegrex_float16:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_intfloat16(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_p_vrot_code:
            case RAB_OPERAND_r4000allegrex_t_vrot_code:
            case RAB_OPERAND_r4000allegrex_q_vrot_code:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_vrot_code(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_rpx:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_rpx(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_rpy:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_rpy(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_rpz:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_rpz(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_rpw:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_rpw(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_wpx:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_wpx(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_wpy:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_wpy(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_wpz:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_wpz(self->word, 0);
                break;

            case RAB_OPERAND_r4000allegrex_wpw:
                self->word = RAB_INSTR_R4000ALLEGREX_PACK_wpw(self->word, 0);
                break;
            /* r4000allegrex */

            /* r5900 */
            case RAB_OPERAND_r5900_I:
            case RAB_OPERAND_r5900_Q:
            case RAB_OPERAND_r5900_R:
            case RAB_OPERAND_r5900_ACC:
                // Not real registers encoded on the instruction itself
                break;

            case RAB_OPERAND_r5900_ACCxyzw:
                self->word = RAB_INSTR_R5900_PACK_xyzw_x(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_y(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_z(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_w(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vfs:
                self->word = RAB_INSTR_R5900_PACK_vfs(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vft:
                self->word = RAB_INSTR_R5900_PACK_vft(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vfd:
                self->word = RAB_INSTR_R5900_PACK_vfd(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vfsxyzw:
                self->word = RAB_INSTR_R5900_PACK_vfs(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_x(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_y(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_z(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_w(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vftxyzw:
                self->word = RAB_INSTR_R5900_PACK_vft(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_x(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_y(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_z(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_w(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vfdxyzw:
                self->word = RAB_INSTR_R5900_PACK_vfd(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_x(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_y(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_z(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_xyzw_w(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vfsn:
                self->word = RAB_INSTR_R5900_PACK_vfs(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_n(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vftn:
                self->word = RAB_INSTR_R5900_PACK_vft(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_n(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vfdn:
                self->word = RAB_INSTR_R5900_PACK_vfd(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_n(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vfsl:
                self->word = RAB_INSTR_R5900_PACK_vfs(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_l(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vftl:
                self->word = RAB_INSTR_R5900_PACK_vft(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_l(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vfdl:
                self->word = RAB_INSTR_R5900_PACK_vfd(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_l(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vfsm:
                self->word = RAB_INSTR_R5900_PACK_vfs(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_m(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vftm:
                self->word = RAB_INSTR_R5900_PACK_vft(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_m(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vfdm:
                self->word = RAB_INSTR_R5900_PACK_vfd(self->word, 0);
                self->word = RAB_INSTR_R5900_PACK_m(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vis:
                self->word = RAB_INSTR_R5900_PACK_vis(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vit:
                self->word = RAB_INSTR_R5900_PACK_vit(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vid:
                self->word = RAB_INSTR_R5900_PACK_vid(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vis_predecr:
                self->word = RAB_INSTR_R5900_PACK_vis(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vit_predecr:
                self->word = RAB_INSTR_R5900_PACK_vit(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vid_predecr:
                self->word = RAB_INSTR_R5900_PACK_vid(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vis_postincr:
                self->word = RAB_INSTR_R5900_PACK_vis(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vit_postincr:
                self->word = RAB_INSTR_R5900_PACK_vit(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vid_postincr:
                self->word = RAB_INSTR_R5900_PACK_vid(self->word, 0);
                break;

            case RAB_OPERAND_r5900_vis_parenthesis:
                self->word = RAB_INSTR_R5900_PACK_vis(self->word, 0);
                break;

            case RAB_OPERAND_r5900_immediate5:
                self->word = RAB_INSTR_R5900_PACK_imm5(self->word, 0);
                break;

            case RAB_OPERAND_r5900_immediate15:
                self->word = RAB_INSTR_R5900_PACK_imm15(self->word, 0);
                break;
                /* r5900 */

            case RAB_OPERAND_ALL_INVALID:
            case RAB_OPERAND_ALL_MAX:
                assert(self->descriptor->operands[i] != RAB_OPERAND_ALL_INVALID &&
                       self->descriptor->operands[i] != RAB_OPERAND_ALL_MAX);
                break;
        }
    }
}
