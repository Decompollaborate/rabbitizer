/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"

#include "common/RabbitizerConfig.h"
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
        if (self->rs == RABBITIZER_REG_GPR_O32_k0 || self->rs == RABBITIZER_REG_GPR_O32_k1) {
            return true;
        }
        if (self->rt == RABBITIZER_REG_GPR_O32_k0 || self->rt == RABBITIZER_REG_GPR_O32_k1) {
            return true;
        }
    }

    if (RabbitizerInstrDescriptor_notEmitedByCompilers(self->descriptor)) {
        return true;
    }

    return false;
}

bool RabbitizerInstruction_isNop(const RabbitizerInstruction *self) {
    return self->opcode == 0 &&
    self->rs == 0 &&
    self->rt == 0 &&
    self->rd == 0 &&
    self->sa == 0 &&
    self->function == 0;
}

bool RabbitizerInstruction_isUnconditionalBranch(const RabbitizerInstruction *self) {
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_b) {
        return true;
    }
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_beq && self->rt == 0 && self->rs == 0) {
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
        return self->rs == RABBITIZER_REG_GPR_O32_ra;
    }
    return false;
}

bool RabbitizerInstruction_isJrNotRa(const RabbitizerInstruction *self) {
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_jr) {
        // TODO: abi stuffs
        return self->rs != RABBITIZER_REG_GPR_O32_ra;
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
