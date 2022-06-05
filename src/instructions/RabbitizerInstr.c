/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstr.h"

#include "common/Utils.h"
#include "instructions/RabbitizerRegister.h"


void RabbitizerInstr_Init(RabbitizerInstr *self, uint32_t word) {
    self->opcode = (word >> 26) & 0x3F;
    self->rs = (word >> 21) & 0x1F;
    self->rt = (word >> 16) & 0x1F;
    self->rd = (word >> 11) & 0x1F;
    self->sa = (word >>  6) & 0x1F;
    self->function = (word >> 0) & 0x3F;

    self->uniqueId = RABBITIZER_INSTR_CPU_ID_INVALID;
    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];

    self->vram = 0;
    self->_handwrittenCategory = false;
    self->inHandwrittenFunction = false;
}

void RabbitizerInstr_Destroy(RabbitizerInstr *self) {
}


uint8_t RabbitizerInstr_GetFs(const RabbitizerInstr *self) {
    return self->rd;
}
uint8_t RabbitizerInstr_GetFt(const RabbitizerInstr *self) {
    return self->rt;
}
uint8_t RabbitizerInstr_GetFd(const RabbitizerInstr *self) {
    return self->sa;
}

uint8_t RabbitizerInstr_GetFmt(const RabbitizerInstr *self) {
    return self->rs;
}

uint8_t RabbitizerInstr_GetTf(const RabbitizerInstr *self) {
    return self->rt & 0x1;
}
uint8_t RabbitizerInstr_GetNd(const RabbitizerInstr *self) {
    return (self->rt >> 1) & 0x1;
}
uint8_t RabbitizerInstr_GetFc(const RabbitizerInstr *self) {
    return (self->function >> 4) & 0x3;
}
uint8_t RabbitizerInstr_GetCond(const RabbitizerInstr *self) {
    return self->function & 0xF;
}

uint32_t RabbitizerInstr_GetRaw(const RabbitizerInstr *self) {
    return (self->opcode << 26) | (self->rs << 21) | (self->rt << 16) | (self->rd << 11) | (self->sa << 6) | (self->function);
}

uint32_t RabbitizerInstr_GetImmediate(const RabbitizerInstr *self) {
    return (self->rd << 11) | (self->sa << 6) | (self->function);
}
uint32_t RabbitizerInstr_GetInstrIndex(const RabbitizerInstr *self) {
    return (self->rs << 21) | (self->rt << 16) | (self->rd << 11) | (self->sa << 6) | (self->function);
}

uint32_t RabbitizerInstr_GetInstrIndexAsVram(const RabbitizerInstr *self) {
    uint32_t vram = RabbitizerInstr_GetInstrIndex(self) << 2;

    if (self->vram == 0) {
        vram |= 0x80000000;
    } else {
        // Jumps are PC-region branches. The upper bits are filled with the address in the delay slot
        vram |= (self->vram+4) & 0xFF000000;
    }
    return vram;
}

int32_t RabbitizerInstr_GetBranchOffset(const RabbitizerInstr *self) {
    int32_t diff = RabbitizerUtils_From2Complement(RabbitizerInstr_GetImmediate(self), 16);

    return diff*4 + 4;
}


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

    if (/* InstructionConfig.TREAT_J_AS_UNCONDITIONAL_BRANCH && */ self->uniqueId == RABBITIZER_INSTR_CPU_ID_j) {
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
