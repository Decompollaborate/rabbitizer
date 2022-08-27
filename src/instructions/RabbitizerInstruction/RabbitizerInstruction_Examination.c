/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"

#include <assert.h>

#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerInstructionRsp.h"
#include "instructions/RabbitizerInstructionR5900.h"
#include "instructions/RabbitizerRegister.h"

bool RabbitizerInstruction_isImplemented(const RabbitizerInstruction *self) {
    switch (self->uniqueId) {
        case RABBITIZER_INSTR_ID_cpu_INVALID:
        case RABBITIZER_INSTR_ID_rsp_INVALID:
        case RABBITIZER_INSTR_ID_r5900_INVALID:
            return false;

        default:
            return true;
    }
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
    return self->word == 0;
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

bool RabbitizerInstruction_hasDelaySlot(const RabbitizerInstruction *self) {
    return RabbitizerInstrDescriptor_isBranch(self->descriptor) || RabbitizerInstrDescriptor_isJump(self->descriptor);
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

bool RabbitizerInstruction_hasOperandAlias(const RabbitizerInstruction *self, RabbitizerOperandType operand) {
    switch (operand) {
        case RABBITIZER_OPERAND_TYPE_rs:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_IMM_base)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_RSP_rs)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_RSP_offset_rs)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_RSP_IMM_base)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_IMM:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_IMM_base)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_RSP_IMM_base)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_rt:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_RSP_rt)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_rd:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_RSP_rd)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_sa:
        case RABBITIZER_OPERAND_TYPE_zero:
        // case RABBITIZER_OPERAND_TYPE_function:
        case RABBITIZER_OPERAND_TYPE_cop0d:
        case RABBITIZER_OPERAND_TYPE_fs:
        case RABBITIZER_OPERAND_TYPE_ft:
        case RABBITIZER_OPERAND_TYPE_fd:
        case RABBITIZER_OPERAND_TYPE_cop1cs:
        case RABBITIZER_OPERAND_TYPE_cop2t:
        case RABBITIZER_OPERAND_TYPE_op:
        case RABBITIZER_OPERAND_TYPE_code:
        case RABBITIZER_OPERAND_TYPE_copraw:
        case RABBITIZER_OPERAND_TYPE_LABEL:
            break;

        case RABBITIZER_OPERAND_TYPE_IMM_base:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_rs)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_IMM)) {
                return true;
            }
            break;

        /* rsp */
        case RABBITIZER_OPERAND_TYPE_RSP_rs:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_rs)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_RSP_offset_rs)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_RSP_rt:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_rt)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_RSP_rd:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_rd)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_RSP_cop0d:
        case RABBITIZER_OPERAND_TYPE_RSP_cop2t:
        case RABBITIZER_OPERAND_TYPE_RSP_cop2cd:
            break;

            // case RABBITIZER_OPERAND_TYPE_RSP_elementhigh:
            // case RABBITIZER_OPERAND_TYPE_RSP_elementlow:
            // case RABBITIZER_OPERAND_TYPE_RSP_index:
            // case RABBITIZER_OPERAND_TYPE_RSP_offset:

        case RABBITIZER_OPERAND_TYPE_RSP_vs:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_RSP_vs_index)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_RSP_vt:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_RSP_vd:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_RSP_vd_de)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_RSP_vt)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_RSP_vt)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_RSP_vd_de:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_RSP_vd)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_RSP_vs_index:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_RSP_vs)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_RSP_offset_rs:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_RSP_rs)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_RSP_IMM_base:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_RSP_rs)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_rs)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_IMM)) {
                return true;
            }
            break;
            /* rsp */

            /* r5900 */
        case RABBITIZER_OPERAND_TYPE_R5900_I:
        case RABBITIZER_OPERAND_TYPE_R5900_Q:
        case RABBITIZER_OPERAND_TYPE_R5900_R:
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_ACC:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_ACCxyzw)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_ACCxyzw:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_ACC)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vfs:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfsxyzw)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfsn)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfsl)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfsm)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vft:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vftxyzw)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vftn)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vftl)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vftm)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vfd:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfdxyzw)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfdn)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfdl)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfdm)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vfsxyzw:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfs)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfsn)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vftxyzw:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vft)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vftn)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vfdxyzw:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfd)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfdn)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vfsn:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfs)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfsxyzw)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vftn:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vft)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vftxyzw)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vfdn:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfd)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vfdxyzw)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vfsl:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_R5900_vfs)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vftl:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_R5900_vft)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vfdl:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_R5900_vfd)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vfsm:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_R5900_vfs)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vftm:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_R5900_vft)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vfdm:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_R5900_vfd)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vis:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vis_predecr)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vis_postincr)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vit:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vit_predecr)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vit_postincr)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vid:
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vid_predecr)) {
                return true;
            }
            if (RabbitizerInstruction_hasOperand(self, RABBITIZER_OPERAND_TYPE_R5900_vid_postincr)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vis_predecr:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_R5900_vis)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vit_predecr:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_R5900_vit)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vid_predecr:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_R5900_vid)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vis_postincr:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_R5900_vis)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vit_postincr:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_R5900_vit)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_vid_postincr:
            if (RabbitizerInstruction_hasOperandAlias(self, RABBITIZER_OPERAND_TYPE_R5900_vid)) {
                return true;
            }
            break;

        case RABBITIZER_OPERAND_TYPE_R5900_imm5:
            break;
            /* r5900 */

        case RABBITIZER_OPERAND_TYPE_INVALID:
        case RABBITIZER_OPERAND_TYPE_MAX:
            assert(operand != RABBITIZER_OPERAND_TYPE_INVALID && operand != RABBITIZER_OPERAND_TYPE_MAX);
            break;
    }

    return RabbitizerInstruction_hasOperand(self, operand);
}

uint32_t RabbitizerInstruction_getValidBits(const RabbitizerInstruction *self) {
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

            case RABBITIZER_OPERAND_TYPE_zero:
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

            case RABBITIZER_OPERAND_TYPE_copraw:
                validbits = RAB_INSTR_PACK_copraw(validbits, ~0);
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

            /* rsp */
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

            case RABBITIZER_OPERAND_TYPE_RSP_cop2t:
                validbits = RAB_INSTR_RSP_PACK_cop2t(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_cop2cd:
                validbits = RAB_INSTR_RSP_PACK_cop2cd(validbits, ~0);
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

            case RABBITIZER_OPERAND_TYPE_RSP_vd_de:
                validbits = RAB_INSTR_RSP_PACK_vd(validbits, ~0);
                validbits = RAB_INSTR_RSP_PACK_de(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_vs_index:
                validbits = RAB_INSTR_RSP_PACK_vs(validbits, ~0);
                validbits = RAB_INSTR_RSP_PACK_index(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_offset_rs:
                validbits = RAB_INSTR_RSP_PACK_offset(validbits, ~0);
                validbits = RAB_INSTR_PACK_rs(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_RSP_IMM_base:
                validbits = RAB_INSTR_PACK_immediate(validbits, ~0);
                validbits = RAB_INSTR_PACK_rs(validbits, ~0);
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
                validbits = RAB_INSTR_R5900_PACK_xyzw_x(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_y(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_z(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_w(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfs:
                validbits = RAB_INSTR_R5900_PACK_vfs(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vft:
                validbits = RAB_INSTR_R5900_PACK_vft(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfd:
                validbits = RAB_INSTR_R5900_PACK_vfd(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfsxyzw:
                validbits = RAB_INSTR_R5900_PACK_vfs(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_x(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_y(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_z(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_w(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vftxyzw:
                validbits = RAB_INSTR_R5900_PACK_vft(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_x(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_y(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_z(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_w(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfdxyzw:
                validbits = RAB_INSTR_R5900_PACK_vfd(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_x(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_y(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_z(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_xyzw_w(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfsn:
                validbits = RAB_INSTR_R5900_PACK_vfs(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_n(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vftn:
                validbits = RAB_INSTR_R5900_PACK_vft(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_n(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfdn:
                validbits = RAB_INSTR_R5900_PACK_vfd(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_n(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfsl:
                validbits = RAB_INSTR_R5900_PACK_vfs(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_l(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vftl:
                validbits = RAB_INSTR_R5900_PACK_vft(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_l(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfdl:
                validbits = RAB_INSTR_R5900_PACK_vfd(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_l(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfsm:
                validbits = RAB_INSTR_R5900_PACK_vfs(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_m(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vftm:
                validbits = RAB_INSTR_R5900_PACK_vft(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_m(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vfdm:
                validbits = RAB_INSTR_R5900_PACK_vfd(validbits, ~0);
                validbits = RAB_INSTR_R5900_PACK_m(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vis:
                validbits = RAB_INSTR_R5900_PACK_vis(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vit:
                validbits = RAB_INSTR_R5900_PACK_vit(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vid:
                validbits = RAB_INSTR_R5900_PACK_vid(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vis_predecr:
                validbits = RAB_INSTR_R5900_PACK_vis(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vit_predecr:
                validbits = RAB_INSTR_R5900_PACK_vit(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vid_predecr:
                validbits = RAB_INSTR_R5900_PACK_vid(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vis_postincr:
                validbits = RAB_INSTR_R5900_PACK_vis(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vit_postincr:
                validbits = RAB_INSTR_R5900_PACK_vit(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_vid_postincr:
                validbits = RAB_INSTR_R5900_PACK_vid(validbits, ~0);
                break;

            case RABBITIZER_OPERAND_TYPE_R5900_imm5:
                validbits = RAB_INSTR_R5900_PACK_imm5(validbits, ~0);
                break;
                /* r5900 */

            case RABBITIZER_OPERAND_TYPE_INVALID:
            case RABBITIZER_OPERAND_TYPE_MAX:
                assert(self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_MAX);
                break;
        }
    }

    return validbits;
}

bool RabbitizerInstruction_isValid(const RabbitizerInstruction *self) {
    uint32_t validbits;

    validbits = RabbitizerInstruction_getValidBits(self);

    return ((~validbits) & self->word) == 0;
}
