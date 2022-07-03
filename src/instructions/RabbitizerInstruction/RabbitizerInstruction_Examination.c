/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"

#include <assert.h>

#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerInstructionRsp.h"
#include "instructions/RabbitizerRegister.h"


bool RabbitizerInstruction_isImplemented(const RabbitizerInstruction *self) {
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_INVALID) {
        return false;
    }
    if (self->uniqueId == RABBITIZER_INSTR_ID_rsp_INVALID) {
        return false;
    }
    return true;
}

bool RabbitizerInstruction_isLikelyHandwritten(const RabbitizerInstruction *self) {
    if (self->_handwrittenCategory) {
        return true;
    }

    if (RabbitizerInstrDescriptor_isIType(self->descriptor) && !RabbitizerInstrDescriptor_isFloat(self->descriptor)) {
        if (RAB_INSTR_GET_rs(self) == RABBITIZER_REG_GPR_O32_k0 || RAB_INSTR_GET_rs(self) == RABBITIZER_REG_GPR_O32_k1) {
            return true;
        }
        if (RAB_INSTR_GET_rt(self) == RABBITIZER_REG_GPR_O32_k0 || RAB_INSTR_GET_rt(self) == RABBITIZER_REG_GPR_O32_k1) {
            return true;
        }
    }

    if (RabbitizerInstrDescriptor_notEmitedByCompilers(self->descriptor)) {
        return true;
    }

    return false;
}

bool RabbitizerInstruction_isNop(const RabbitizerInstruction *self) {
    return RAB_INSTR_GET_opcode(self) == 0 &&
    RAB_INSTR_GET_rs(self) == 0 &&
    RAB_INSTR_GET_rt(self) == 0 &&
    RAB_INSTR_GET_rd(self) == 0 &&
    RAB_INSTR_GET_sa(self) == 0 &&
    RAB_INSTR_GET_function(self) == 0;
}

bool RabbitizerInstruction_isUnconditionalBranch(const RabbitizerInstruction *self) {
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_b) {
        return true;
    }
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_beq && RAB_INSTR_GET_rt(self) == 0 && RAB_INSTR_GET_rs(self) == 0) {
        return true;
    }
    if (RabbitizerConfig_Cfg.toolchainTweaks.treatJAsUnconditionalBranch && self->uniqueId == RABBITIZER_INSTR_ID_cpu_j) {
        return true;
    }
    return false;
}

bool RabbitizerInstruction_isJrRa(const RabbitizerInstruction *self) {
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_jr) {
        // TODO: abi stuffs
        return RAB_INSTR_GET_rs(self) == RABBITIZER_REG_GPR_O32_ra;
    }
    return false;
}

bool RabbitizerInstruction_isJrNotRa(const RabbitizerInstruction *self) {
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_jr) {
        // TODO: abi stuffs
        return RAB_INSTR_GET_rs(self) != RABBITIZER_REG_GPR_O32_ra;
    }
    return false;
}

const char *RabbitizerInstruction_mapInstrToType(const RabbitizerInstruction *self) {
    if (RabbitizerInstrDescriptor_isDouble(self->descriptor)) {
        return "f64";
    }
    if (RabbitizerInstrDescriptor_isFloat(self->descriptor)) {
        return "f32";
    }
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_lwu) {
        return "u32";
    }
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_lh || self->uniqueId == RABBITIZER_INSTR_ID_cpu_sh) {
        return "s16";
    }
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_lhu) {
        return "u16";
    }
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_lb || self->uniqueId == RABBITIZER_INSTR_ID_cpu_sb) {
        return "s8";
    }
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_lbu) {
        return "u8";
    }
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_ld || self->uniqueId == RABBITIZER_INSTR_ID_cpu_sd) {
        return "s64";
    }
    return NULL;
}

bool RabbitizerInstruction_sameOpcode(const RabbitizerInstruction *self, const RabbitizerInstruction *other) {
    if (!RabbitizerInstruction_isImplemented(self) || !RabbitizerInstruction_isImplemented(other)) {
        return false;
    }
    return self->uniqueId == other->uniqueId;
}

bool RabbitizerInstruction_sameOpcodeButDifferentArguments(const RabbitizerInstruction *self, const RabbitizerInstruction *other) {
    if (!RabbitizerInstruction_sameOpcode(self, other)) {
        return false;
    }
    return RabbitizerInstruction_getRaw(self) != RabbitizerInstruction_getRaw(other);
}


bool RabbitizerInstruction_hasOperand(const RabbitizerInstruction *self, RabbitizerOperandType operand) {
    size_t i;

    for (i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID; i++) {
        if (self->descriptor->operands[i] == operand) {
            return true;
        }
    }

    return false;
}

bool RabbitizerInstruction_isValid(const RabbitizerInstruction *self) {
    size_t i;
    uint32_t validbits;

    validbits = self->_mandatorybits;

    for (i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID; i++) {

        switch (self->descriptor->operands[i]) {
            case RABBITIZER_OPERAND_TYPE_rs:
                validbits = RAB_INSTR_PACK_rs(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_rt:
                validbits = RAB_INSTR_PACK_rt(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_rd:
                validbits = RAB_INSTR_PACK_rd(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_sa:
                validbits = RAB_INSTR_PACK_sa(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_cop0d:
                validbits = RAB_INSTR_PACK_cop0d(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_fs:
                validbits = RAB_INSTR_PACK_fs(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_ft:
                validbits = RAB_INSTR_PACK_ft(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_fd:
                validbits = RAB_INSTR_PACK_fd(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_cop1cs:
                validbits = RAB_INSTR_PACK_cop1cs(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_cop2t:
                validbits = RAB_INSTR_PACK_cop2t(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_op:
                validbits = RAB_INSTR_PACK_op(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_code:
                validbits = RAB_INSTR_PACK_code(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_LABEL:
                validbits = RAB_INSTR_PACK_instr_index(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_IMM:
                validbits = RAB_INSTR_PACK_immediate(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_IMM_base:
                validbits = RAB_INSTR_PACK_immediate(validbits, ~0);
                validbits = RAB_INSTR_PACK_rs(validbits, ~0);
                break;


            case RABBITIZER_OPERAND_TYPE_RSP_rs:
                validbits = RAB_INSTR_PACK_rs(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_rt:
                validbits = RAB_INSTR_PACK_rt(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_rd:
                validbits = RAB_INSTR_PACK_rd(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_cop0d:
                validbits = RAB_INSTR_PACK_cop0d(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vs:
                validbits = RAB_INSTR_RSP_PACK_vs(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vt:
                validbits = RAB_INSTR_RSP_PACK_vt(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vd:
                validbits = RAB_INSTR_RSP_PACK_vd(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh:
                validbits = RAB_INSTR_RSP_PACK_vt(validbits, ~0);
                validbits = RAB_INSTR_RSP_PACK_elementhigh(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow:
                validbits = RAB_INSTR_RSP_PACK_vt(validbits, ~0);
                validbits = RAB_INSTR_RSP_PACK_elementlow(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vd_vs:
                validbits = RAB_INSTR_RSP_PACK_vd(validbits, ~0);
                validbits = RAB_INSTR_RSP_PACK_vs(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vd_index:
                validbits = RAB_INSTR_RSP_PACK_vd(validbits, ~0);
                validbits = RAB_INSTR_RSP_PACK_index(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_offset_rs:
                validbits = RAB_INSTR_RSP_PACK_offset(validbits, ~0);
                validbits = RAB_INSTR_PACK_rs(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_INVALID:
            case RABBITIZER_OPERAND_TYPE_MAX:
                assert(self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_MAX);
                break;
        }
    }

    return ((~validbits) & self->word) == 0;
}
