/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstr.h"

#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"


/* Instruction examination */

bool RabbitizerInstr_IsImplemented(const RabbitizerInstr *self) {
    if (self->uniqueId == RABBITIZER_INSTR_CPU_ID_INVALID) {
        return false;
    }
    if (self->uniqueId == RABBITIZER_INSTR_RSP_ID_INVALID) {
        return false;
    }
    return true;
}

bool RabbitizerInstr_IsLikelyHandwritten(const RabbitizerInstr *self) {
    if (self->_handwrittenCategory) {
        return true;
    }

    if (RabbitizerInstrDescriptor_IsIType(self->descriptor) && !RabbitizerInstrDescriptor_IsFloat(self->descriptor)) {
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

bool RabbitizerInstr_IsNop(const RabbitizerInstr *self) {
    return self->opcode == 0 &&
    self->rs == 0 &&
    self->rt == 0 &&
    self->rd == 0 &&
    self->sa == 0 &&
    self->function == 0;
}

bool RabbitizerInstr_IsUnconditionalBranch(const RabbitizerInstr *self) {
    if (self->uniqueId == RABBITIZER_INSTR_CPU_ID_b) {
        return true;
    }
    if (self->uniqueId == RABBITIZER_INSTR_CPU_ID_beq && self->rt == 0 && self->rs == 0) {
        return true;
    }
    if (RabbitizerConfig_Cfg.toolchainTweaks.treatJAsUnconditionalBranch && self->uniqueId == RABBITIZER_INSTR_CPU_ID_j) {
        return true;
    }
    return false;
}

bool RabbitizerInstr_IsJrRa(const RabbitizerInstr *self) {
    if (self->uniqueId == RABBITIZER_INSTR_CPU_ID_jr) {
        // TODO: abi stuffs
        return self->rs == RABBITIZER_REG_GPR_O32_ra;
    }
    return false;
}

bool RabbitizerInstr_IsJrNotRa(const RabbitizerInstr *self) {
    if (self->uniqueId == RABBITIZER_INSTR_CPU_ID_jr) {
        // TODO: abi stuffs
        return self->rs != RABBITIZER_REG_GPR_O32_ra;
    }
    return false;
}


const char *RabbitizerInstr_MaprInstrToType(const RabbitizerInstr *self) {
    if (RabbitizerInstrDescriptor_IsDouble(self->descriptor)) {
        return "f64";
    }
    if (RabbitizerInstrDescriptor_IsFloat(self->descriptor)) {
        return "f32";
    }
    if (self->uniqueId == RABBITIZER_INSTR_CPU_ID_lwu) {
        return "u32";
    }
    if (self->uniqueId == RABBITIZER_INSTR_CPU_ID_lh || self->uniqueId == RABBITIZER_INSTR_CPU_ID_sh) {
        return "s16";
    }
    if (self->uniqueId == RABBITIZER_INSTR_CPU_ID_lhu) {
        return "u16";
    }
    if (self->uniqueId == RABBITIZER_INSTR_CPU_ID_lb || self->uniqueId == RABBITIZER_INSTR_CPU_ID_sb) {
        return "s8";
    }
    if (self->uniqueId == RABBITIZER_INSTR_CPU_ID_lbu) {
        return "u8";
    }
    if (self->uniqueId == RABBITIZER_INSTR_CPU_ID_ld || self->uniqueId == RABBITIZER_INSTR_CPU_ID_sd) {
        return "s64";
    }
    return NULL;
}

/* Instruction examination */
