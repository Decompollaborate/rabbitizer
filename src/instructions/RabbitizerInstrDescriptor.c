/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstrDescriptor.h"

#include <assert.h>

#include "instructions/RabbitizerInstruction.h"

#define RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, ...) [RABBITIZER_INSTR_ID_##prefix##_##name] = { __VA_ARGS__ },

#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, caseBits, name, altname, ...) RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, __VA_ARGS__)

const RabbitizerInstrDescriptor RabbitizerInstrDescriptor_Descriptors[] = {
#include "instructions/instr_id/RabbitizerInstrId_cpu.inc"
#include "instructions/instr_id/RabbitizerInstrId_rsp.inc"
#include "instructions/instr_id/RabbitizerInstrId_r5900.inc"
};

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME


bool RabbitizerInstrDescriptor_hasSpecificOperand(const RabbitizerInstrDescriptor *self, RabbitizerOperandType operand) {
    size_t i;

    for (i = 0; i < ARRAY_COUNT(self->operands) && self->operands[i] != RAB_OPERAND_ALL_INVALID; i++) {
        if (self->operands[i] == operand) {
            return true;
        }
    }

    return false;
}

bool RabbitizerInstrDescriptor_hasOperandAlias(const RabbitizerInstrDescriptor *self, RabbitizerOperandType operand) {
    switch (operand) {
        case RAB_OPERAND_cpu_rs:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_cpu_immediate_base)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_rsp_rs)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_rsp_offset_rs)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_rsp_immediate_base)) {
                return true;
            }
            break;

        case RAB_OPERAND_cpu_immediate:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_cpu_immediate_base)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_cpu_branch_target_label)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_rsp_immediate_base)) {
                return true;
            }
            break;

        case RAB_OPERAND_cpu_rt:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_rsp_rt)) {
                return true;
            }
            break;

        case RAB_OPERAND_cpu_rd:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_rsp_rd)) {
                return true;
            }
            break;

        case RAB_OPERAND_cpu_sa:
        case RAB_OPERAND_cpu_zero:
        // case RAB_OPERAND_cpu_function:
        case RAB_OPERAND_cpu_cop0d:
        case RAB_OPERAND_cpu_fs:
        case RAB_OPERAND_cpu_ft:
        case RAB_OPERAND_cpu_fd:
        case RAB_OPERAND_cpu_cop1cs:
        case RAB_OPERAND_cpu_cop2t:
        case RAB_OPERAND_cpu_op:
        case RAB_OPERAND_cpu_code:
        case RAB_OPERAND_cpu_code_lower:
        case RAB_OPERAND_cpu_copraw:
        case RAB_OPERAND_cpu_label:
            break;

        case RAB_OPERAND_cpu_branch_target_label:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_cpu_immediate)) {
                return true;
            }
            break;

        case RAB_OPERAND_cpu_immediate_base:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_cpu_rs)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_cpu_immediate)) {
                return true;
            }
            break;

        /* rsp */
        case RAB_OPERAND_rsp_rs:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_cpu_rs)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_rsp_offset_rs)) {
                return true;
            }
            break;

        case RAB_OPERAND_rsp_rt:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_cpu_rt)) {
                return true;
            }
            break;

        case RAB_OPERAND_rsp_rd:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_cpu_rd)) {
                return true;
            }
            break;

        case RAB_OPERAND_rsp_cop0d:
        case RAB_OPERAND_rsp_cop2t:
        case RAB_OPERAND_rsp_cop2cd:
            break;

            // case RAB_OPERAND_rsp_elementhigh:
            // case RAB_OPERAND_rsp_elementlow:
            // case RAB_OPERAND_rsp_index:
            // case RAB_OPERAND_rsp_offset:

        case RAB_OPERAND_rsp_vs:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_rsp_vs_index)) {
                return true;
            }
            break;

        case RAB_OPERAND_rsp_vt:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_rsp_vt_elementhigh)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_rsp_vt_elementlow)) {
                return true;
            }
            break;

        case RAB_OPERAND_rsp_vd:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_rsp_vd_de)) {
                return true;
            }
            break;

        case RAB_OPERAND_rsp_vt_elementhigh:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_rsp_vt)) {
                return true;
            }
            break;

        case RAB_OPERAND_rsp_vt_elementlow:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_rsp_vt)) {
                return true;
            }
            break;

        case RAB_OPERAND_rsp_vd_de:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_rsp_vd)) {
                return true;
            }
            break;

        case RAB_OPERAND_rsp_vs_index:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_rsp_vs)) {
                return true;
            }
            break;

        case RAB_OPERAND_rsp_offset_rs:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_rsp_rs)) {
                return true;
            }
            break;

        case RAB_OPERAND_rsp_immediate_base:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_rsp_rs)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_cpu_rs)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_cpu_immediate)) {
                return true;
            }
            break;
            /* rsp */

            /* r5900 */
        case RAB_OPERAND_r5900_I:
        case RAB_OPERAND_r5900_Q:
        case RAB_OPERAND_r5900_R:
            break;

        case RAB_OPERAND_r5900_ACC:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_ACCxyzw)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_ACCxyzw:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_ACC)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vfs:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfsxyzw)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfsn)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfsl)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfsm)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vft:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vftxyzw)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vftn)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vftl)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vftm)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vfd:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfdxyzw)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfdn)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfdl)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfdm)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vfsxyzw:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfs)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfsn)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vftxyzw:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vft)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vftn)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vfdxyzw:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfd)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfdn)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vfsn:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfs)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfsxyzw)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vftn:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vft)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vftxyzw)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vfdn:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfd)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vfdxyzw)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vfsl:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_r5900_vfs)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vftl:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_r5900_vft)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vfdl:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_r5900_vfd)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vfsm:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_r5900_vfs)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vftm:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_r5900_vft)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vfdm:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_r5900_vfd)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vis:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vis_predecr)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vis_postincr)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vit:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vit_predecr)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vit_postincr)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vid:
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vid_predecr)) {
                return true;
            }
            if (RabbitizerInstrDescriptor_hasSpecificOperand(self, RAB_OPERAND_r5900_vid_postincr)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vis_predecr:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_r5900_vis)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vit_predecr:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_r5900_vit)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vid_predecr:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_r5900_vid)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vis_postincr:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_r5900_vis)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vit_postincr:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_r5900_vit)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_vid_postincr:
            if (RabbitizerInstrDescriptor_hasOperandAlias(self, RAB_OPERAND_r5900_vid)) {
                return true;
            }
            break;

        case RAB_OPERAND_r5900_immediate5:
            break;
            /* r5900 */

        case RAB_OPERAND_ALL_INVALID:
        case RAB_OPERAND_ALL_MAX:
            assert(operand != RAB_OPERAND_ALL_INVALID && operand != RAB_OPERAND_ALL_MAX);
            break;
    }

    return RabbitizerInstrDescriptor_hasSpecificOperand(self, operand);
}


bool RabbitizerInstrDescriptor_isUnknownType(const RabbitizerInstrDescriptor *self) {
    return self->instrType == RABBITIZER_INSTR_TYPE_UNKNOWN;
}
bool RabbitizerInstrDescriptor_isJType(const RabbitizerInstrDescriptor *self) {
    return self->instrType == RABBITIZER_INSTR_TYPE_J;
}
bool RabbitizerInstrDescriptor_isIType(const RabbitizerInstrDescriptor *self) {
    return self->instrType == RABBITIZER_INSTR_TYPE_I;
}
bool RabbitizerInstrDescriptor_isRType(const RabbitizerInstrDescriptor *self) {
    return self->instrType == RABBITIZER_INSTR_TYPE_R;
}
bool RabbitizerInstrDescriptor_isRegimmType(const RabbitizerInstrDescriptor *self) {
    return self->instrType == RABBITIZER_INSTR_TYPE_REGIMM;
}

RabbitizerInstrSuffix RabbitizerInstrDescriptor_instrSuffix(const RabbitizerInstrDescriptor *self) {
    return self->instrSuffix;
}

bool RabbitizerInstrDescriptor_isBranch(const RabbitizerInstrDescriptor *self) {
    return self->isBranch;
}
bool RabbitizerInstrDescriptor_isBranchLikely(const RabbitizerInstrDescriptor *self) {
    return self->isBranchLikely;
}
bool RabbitizerInstrDescriptor_isJump(const RabbitizerInstrDescriptor *self) {
    return self->isJump;
}
bool RabbitizerInstrDescriptor_isJumpWithAddress(const RabbitizerInstrDescriptor *self) {
    return self->isJumpWithAddress;
}
bool RabbitizerInstrDescriptor_isTrap(const RabbitizerInstrDescriptor *self) {
    return self->isTrap;
}

bool RabbitizerInstrDescriptor_isFloat(const RabbitizerInstrDescriptor *self) {
    return self->isFloat;
}
bool RabbitizerInstrDescriptor_isDouble(const RabbitizerInstrDescriptor *self) {
    return self->isDouble;
}

bool RabbitizerInstrDescriptor_isUnsigned(const RabbitizerInstrDescriptor *self) {
    return self->isUnsigned;
}

bool RabbitizerInstrDescriptor_modifiesRt(const RabbitizerInstrDescriptor *self) {
    return self->modifiesRt;
}
bool RabbitizerInstrDescriptor_modifiesRd(const RabbitizerInstrDescriptor *self) {
    return self->modifiesRd;
}

bool RabbitizerInstrDescriptor_readsRs(const RabbitizerInstrDescriptor *self) {
    return self->readsRs;
}
bool RabbitizerInstrDescriptor_readsRt(const RabbitizerInstrDescriptor *self) {
    return self->readsRt;
}
bool RabbitizerInstrDescriptor_readsRd(const RabbitizerInstrDescriptor *self) {
    return self->readsRd;
}

bool RabbitizerInstrDescriptor_readsHI(const RabbitizerInstrDescriptor *self) {
    return self->readsHI;
}
bool RabbitizerInstrDescriptor_readsLO(const RabbitizerInstrDescriptor *self) {
    return self->readsLO;
}
bool RabbitizerInstrDescriptor_modifiesHI(const RabbitizerInstrDescriptor *self) {
    return self->modifiesHI;
}
bool RabbitizerInstrDescriptor_modifiesLO(const RabbitizerInstrDescriptor *self) {
    return self->modifiesLO;
}

bool RabbitizerInstrDescriptor_notEmitedByCompilers(const RabbitizerInstrDescriptor *self) {
    return self->notEmitedByCompilers;
}

bool RabbitizerInstrDescriptor_canBeHi(const RabbitizerInstrDescriptor *self) {
    return self->canBeHi;
}
bool RabbitizerInstrDescriptor_canBeLo(const RabbitizerInstrDescriptor *self) {
    return self->canBeLo;
}
bool RabbitizerInstrDescriptor_doesLink(const RabbitizerInstrDescriptor *self) {
    return self->doesLink;
}
bool RabbitizerInstrDescriptor_doesDereference(const RabbitizerInstrDescriptor *self) {
    return self->doesDereference;
}
bool RabbitizerInstrDescriptor_doesLoad(const RabbitizerInstrDescriptor *self) {
    return self->doesLoad;
}
bool RabbitizerInstrDescriptor_doesStore(const RabbitizerInstrDescriptor *self) {
    return self->doesStore;
}
bool RabbitizerInstrDescriptor_maybeIsMove(const RabbitizerInstrDescriptor *self) {
    return self->maybeIsMove;
}

bool RabbitizerInstrDescriptor_isPseudo(const RabbitizerInstrDescriptor *self) {
    return self->isPseudo;
}

RabbitizerAccessType RabbitizerInstrDescriptor_getAccessType(const RabbitizerInstrDescriptor *self) {
    return self->accessType;
}

bool RabbitizerInstrDescriptor_doesUnsignedMemoryAccess(const RabbitizerInstrDescriptor *self) {
    return self->doesUnsignedMemoryAccess;
}
