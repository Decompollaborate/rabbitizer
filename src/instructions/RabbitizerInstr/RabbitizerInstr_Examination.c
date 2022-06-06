/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstr.h"

#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"


bool RabbitizerInstr_isImplemented(const RabbitizerInstr *self) {
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_INVALID) {
        return false;
    }
    if (self->uniqueId == RABBITIZER_INSTR_ID_rsp_INVALID) {
        return false;
    }
    return true;
}

bool RabbitizerInstr_isLikelyHandwritten(const RabbitizerInstr *self) {
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

    for (size_t i = 0; RabbitizerInstrId_NotEmitedByCompilers[i] != 0; i++) {
        if (self->uniqueId == RabbitizerInstrId_NotEmitedByCompilers[i]) {
            return true;
        }
    }

    return false;
}

bool RabbitizerInstr_isNop(const RabbitizerInstr *self) {
    return self->opcode == 0 &&
    self->rs == 0 &&
    self->rt == 0 &&
    self->rd == 0 &&
    self->sa == 0 &&
    self->function == 0;
}

bool RabbitizerInstr_isUnconditionalBranch(const RabbitizerInstr *self) {
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

bool RabbitizerInstr_isJrRa(const RabbitizerInstr *self) {
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_jr) {
        // TODO: abi stuffs
        return self->rs == RABBITIZER_REG_GPR_O32_ra;
    }
    return false;
}

bool RabbitizerInstr_isJrNotRa(const RabbitizerInstr *self) {
    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_jr) {
        // TODO: abi stuffs
        return self->rs != RABBITIZER_REG_GPR_O32_ra;
    }
    return false;
}

const char *RabbitizerInstr_mapInstrToType(const RabbitizerInstr *self) {
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

bool RabbitizerInstr_sameOpcode(const RabbitizerInstr *self, const RabbitizerInstr *other) {
    if (!RabbitizerInstr_isImplemented(self) || !RabbitizerInstr_isImplemented(other)) {
        return false;
    }
    return self->uniqueId == other->uniqueId;
}

bool RabbitizerInstr_sameOpcodeButDifferentArguments(const RabbitizerInstr *self, const RabbitizerInstr *other) {
    if (!RabbitizerInstr_sameOpcode(self, other)) {
        return false;
    }
    return RabbitizerInstr_getRaw(self) != RabbitizerInstr_getRaw(other);
}
