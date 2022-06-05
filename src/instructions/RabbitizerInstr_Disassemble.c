/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstr.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>

#include <common/Utils.h>
#include "instructions/RabbitizerRegister.h"


typedef size_t (*OperandCallback)(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength);


size_t RabbitizerRegisterType_ProcessRs(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    const char *reg;
    size_t regLen;

    reg = RabbitizerRegister_GetNameGpr(self->rs);
    regLen = strlen(reg);

    memcpy(dst, reg, regLen);
    return regLen;
}

size_t RabbitizerRegisterType_ProcessRt(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    const char *reg;
    size_t regLen;

    reg = RabbitizerRegister_GetNameGpr(self->rt);
    regLen = strlen(reg);

    memcpy(dst, reg, regLen);
    return regLen;
}

size_t RabbitizerRegisterType_ProcessRd(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    const char *reg;
    size_t regLen;

    reg = RabbitizerRegister_GetNameGpr(self->rd);
    regLen = strlen(reg);

    memcpy(dst, reg, regLen);
    return regLen;
}

size_t RabbitizerRegisterType_ProcessCop0d(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    const char *reg;
    size_t regLen;

    reg = RabbitizerRegister_GetNameCop0(self->rd);
    regLen = strlen(reg);

    memcpy(dst, reg, regLen);
    return regLen;
}

size_t RabbitizerRegisterType_ProcessFs(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    const char *reg;
    size_t regLen;

    reg = RabbitizerRegister_GetNameCop1(RabbitizerInstr_GetFs(self));
    regLen = strlen(reg);

    memcpy(dst, reg, regLen);
    return regLen;
}

size_t RabbitizerRegisterType_ProcessFt(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    const char *reg;
    size_t regLen;

    reg = RabbitizerRegister_GetNameCop1(RabbitizerInstr_GetFt(self));
    regLen = strlen(reg);

    memcpy(dst, reg, regLen);
    return regLen;
}

size_t RabbitizerRegisterType_ProcessFd(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    const char *reg;
    size_t regLen;

    reg = RabbitizerRegister_GetNameCop1(RabbitizerInstr_GetFd(self));
    regLen = strlen(reg);

    memcpy(dst, reg, regLen);
    return regLen;
}

size_t RabbitizerRegisterType_ProcessCop2t(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    const char *reg;
    size_t regLen;

    reg = RabbitizerRegister_GetNameCop2(self->rt);
    regLen = strlen(reg);

    memcpy(dst, reg, regLen);
    return regLen;
}

size_t RabbitizerRegisterType_ProcessSa(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    int len;

    len = sprintf(dst, "%i", self->sa);
    assert(len > 0);
    return len;
}

size_t RabbitizerRegisterType_ProcessOp(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    int len;

    len = sprintf(dst, "0x%02X", self->rt);
    assert(len > 0);
    return len;
}

size_t RabbitizerRegisterType_ProcessCode(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    int len;
    int code = (self->rs << 5) | (self->rt);
    int lower = (self->rd << 5) | (self->sa);

    len = sprintf(dst, "%i", code);
    assert(len > 0);
    dst += len;
    totalSize += len;

    if (lower) {
        len = sprintf(dst, ", %i", lower);
        assert(len > 0);
        dst += len;
        totalSize += len;
    }

    return totalSize;
}

size_t RabbitizerRegisterType_ProcessLabel(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    size_t tempSize;
    int len;

    if (immOverride != NULL) {
        memcpy(dst, immOverride, immOverrideLength);
        return immOverrideLength;
    }

    tempSize = strlen("func_");
    memcpy(dst, "func_", tempSize);
    dst += tempSize;
    totalSize += tempSize;

    len = sprintf(dst, "%06X", RabbitizerInstr_GetInstrIndexAsVram(self));
    assert(len > 0);
    dst += len;
    totalSize += len;

    return totalSize;
}

size_t RabbitizerRegisterType_ProcessImmediate(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    int len;
    uint32_t imm;

    if (immOverride != NULL) {
        memcpy(dst, immOverride, immOverrideLength);
        return immOverrideLength;
    }

    imm = RabbitizerInstr_GetImmediate(self);

    if (!self->descriptor->isUnsigned) {
        int32_t number = RabbitizerUtils_From2Complement(imm, 16);
        if (number < 0) {
            len = sprintf(dst, "-0x%X", -number);
            assert(len > 0);
            return len;
        }
    }

    len = sprintf(dst, "0x%X", imm);
    assert(len > 0);
    return len;
}

size_t RabbitizerRegisterType_ProcessImmediateBase(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    size_t tempSize;

    tempSize = RabbitizerRegisterType_ProcessImmediate(self, dst, immOverride, immOverrideLength);
    dst += tempSize;
    totalSize += tempSize;

    *dst = '(';
    dst++;
    totalSize++;

    tempSize = RabbitizerRegisterType_ProcessRs(self, dst, immOverride, immOverrideLength);
    dst += tempSize;
    totalSize += tempSize;

    *dst = ')';
    dst++;
    totalSize++;

    return totalSize;
}

const OperandCallback instrOpercandCallbacks[] = {
    [RABBITIZER_REGISTER_TYPE_rs] = RabbitizerRegisterType_ProcessRs,
    [RABBITIZER_REGISTER_TYPE_rt] = RabbitizerRegisterType_ProcessRt,
    [RABBITIZER_REGISTER_TYPE_rd] = RabbitizerRegisterType_ProcessRd,
    [RABBITIZER_REGISTER_TYPE_cop0d] = RabbitizerRegisterType_ProcessCop0d,
    [RABBITIZER_REGISTER_TYPE_fs] = RabbitizerRegisterType_ProcessFs,
    [RABBITIZER_REGISTER_TYPE_ft] = RabbitizerRegisterType_ProcessFt,
    [RABBITIZER_REGISTER_TYPE_fd] = RabbitizerRegisterType_ProcessFd,
    [RABBITIZER_REGISTER_TYPE_cop2t] = RabbitizerRegisterType_ProcessCop2t,
    [RABBITIZER_REGISTER_TYPE_sa] = RabbitizerRegisterType_ProcessSa,
    [RABBITIZER_REGISTER_TYPE_op] = RabbitizerRegisterType_ProcessOp,
    [RABBITIZER_REGISTER_TYPE_code] = RabbitizerRegisterType_ProcessCode,
    [RABBITIZER_REGISTER_TYPE_LABEL] = RabbitizerRegisterType_ProcessLabel,
    [RABBITIZER_REGISTER_TYPE_IMM] = RabbitizerRegisterType_ProcessImmediate,
    [RABBITIZER_REGISTER_TYPE_IMM_base] = RabbitizerRegisterType_ProcessImmediateBase,
};


size_t RabbitizerInstr_GetSizeForBufferInstrDisasm(const RabbitizerInstr *self, size_t immOverrideLength, int extraLJust) {
    size_t totalSize = 0;
    size_t opcodeNameLength;

    opcodeNameLength = strlen(RabbitizerInstr_GetOpcodeName(self));

    totalSize += opcodeNameLength;

    if (self->descriptor->operands[0] == RABBITIZER_REGISTER_TYPE_INVALID) {
        // There are no operands
        return totalSize;
    }

    totalSize += extraLJust;
    totalSize++;

    for (size_t i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_REGISTER_TYPE_INVALID; i++) {
        if (i != 0) {
            totalSize += 2;
        }

        // A bit arbitrary, but no operand should be longer than 25 characters
        totalSize += 25;
        totalSize += immOverrideLength;
    }

    return totalSize;
}


size_t RabbitizerInstr_DisassembleInstruction(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust) {
    size_t totalSize = 0;
    const char *opcodeName;
    size_t opcodeNameLength;
    size_t len;

    opcodeName = RabbitizerInstr_GetOpcodeName(self);
    opcodeNameLength = strlen(opcodeName);

    memcpy(dst, opcodeName, opcodeNameLength);
    dst += opcodeNameLength;
    totalSize += opcodeNameLength;

    if (self->descriptor->operands[0] == RABBITIZER_REGISTER_TYPE_INVALID) {
        // There are no operands
        *dst = '\0';
        return totalSize;
    }

    len = RabbitizerUtils_CharFill(dst, /*InstructionConfig.OPCODE_LJUST +*/ extraLJust - totalSize, ' ');
    dst += len;
    totalSize += len;

    *dst = ' ';
    dst++;
    totalSize++;

    for (size_t i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_REGISTER_TYPE_INVALID; i++) {
        RabbitizerRegisterType operand;
        OperandCallback callback;
        size_t writtenBytes;

        if (i != 0) {
            *dst = ',';
            dst++;
            totalSize++;
            *dst = ' ';
            dst++;
            totalSize++;
        }

        operand = self->descriptor->operands[i];
        assert(operand > RABBITIZER_REGISTER_TYPE_INVALID);
        assert(operand < RABBITIZER_REGISTER_TYPE_MAX);

        callback = instrOpercandCallbacks[operand];
        assert(callback != NULL);

        writtenBytes = callback(self, dst, immOverride, immOverrideLength);
        dst += writtenBytes;
        totalSize += writtenBytes;
    }

    *dst = '\0';
    return totalSize;
}


size_t RabbitizerInstr_GetSizeForBufferDataDisasm(const RabbitizerInstr *self, int extraLJust) {
    size_t totalSize = 0;

    totalSize += strlen(".word");
    totalSize += /*InstructionConfig.OPCODE_LJUST +*/ extraLJust;
    totalSize += 11;

    return totalSize;
}


size_t RabbitizerInstr_DisassembleAsData(const RabbitizerInstr *self, char *dst, int extraLJust) {
    size_t totalSize = 0;
    size_t tempSize;
    size_t len;

    tempSize = strlen(".word");
    memcpy(dst, ".word", tempSize);
    dst += tempSize;
    totalSize += tempSize;

    len = RabbitizerUtils_CharFill(dst, /*InstructionConfig.OPCODE_LJUST +*/ extraLJust - totalSize, ' ');
    dst += len;
    totalSize += len;

    len = sprintf(dst, " 0x%08X", RabbitizerInstr_GetRaw(self));
    assert(len > 0);
    dst += len;
    totalSize += len;

    return totalSize;
}


bool RabbitizerInstr_MustDisasmAsData(const RabbitizerInstr *self) {
    if (/*InstructionConfig.SN64_DIV_FIX*/ false) {
        if (self->uniqueId == RABBITIZER_INSTR_CPU_ID_break) {
            return true;
        }
    }

    if (self->descriptor->instrType == RABBITIZER_INSTR_TYPE_R) {
        bool hasCode = false;
        bool hasRs = false;
        bool hasRt = false;
        bool hasRd = false;
        bool hasSa = false;

        for (size_t i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_REGISTER_TYPE_INVALID; i++) {
            RabbitizerRegisterType operand = self->descriptor->operands[i];

            if (operand == RABBITIZER_REGISTER_TYPE_code) {
                hasCode = true;
            }
            if (operand == RABBITIZER_REGISTER_TYPE_rs) {
                hasRs = true;
            }
            if (operand == RABBITIZER_REGISTER_TYPE_rt) {
                hasRt = true;
            }
            if (operand == RABBITIZER_REGISTER_TYPE_rd) {
                hasRd = true;
            }
            if (operand == RABBITIZER_REGISTER_TYPE_sa) {
                hasSa = true;
            }
        }

        if (!hasCode) {
            if (!hasRs) {
                if (self->rs != 0) {
                    return true;
                }
            }
            if (!hasRt) {
                if (self->rt != 0) {
                    return true;
                }
            }
            if (!hasRd) {
                if (self->rd != 0 && self->uniqueId != RABBITIZER_INSTR_CPU_ID_jalr) {
                    return true;
                }
            }
            if (!hasSa) {
                if (self->sa != 0) {
                    return true;
                }
            }
        }
    }
    return false;
}



size_t RabbitizerInstr_GetSizeForBuffer(const RabbitizerInstr *self, size_t immOverrideLength, int extraLJust) {
    if (!RabbitizerInstr_IsImplemented(self) || RabbitizerInstr_MustDisasmAsData(self)) {
        size_t totalSize = 0;

        totalSize += RabbitizerInstr_GetSizeForBufferDataDisasm(self, extraLJust);

        /* if (InstructionConfig.UNKNOWN_INSTR_COMMENT) */ {
            totalSize += 40;
            totalSize += 3;
            totalSize += RabbitizerInstr_GetSizeForBufferInstrDisasm(self, immOverrideLength, extraLJust);
        }

        return totalSize;
    }

    return RabbitizerInstr_GetSizeForBufferInstrDisasm(self,immOverrideLength, extraLJust);
}


size_t RabbitizerInstr_Disassemble(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust) {
    assert(dst != NULL);

    if (!RabbitizerInstr_IsImplemented(self) || RabbitizerInstr_MustDisasmAsData(self)) {
        size_t totalSize = 0;
        size_t tempSize;

        tempSize = RabbitizerInstr_DisassembleAsData(self, dst, extraLJust);
        dst += tempSize;
        totalSize += tempSize;

        /* if (InstructionConfig.UNKNOWN_INSTR_COMMENT) */ {
            tempSize = RabbitizerUtils_CharFill(dst, 40-totalSize, ' ');
            dst += tempSize;
            totalSize += tempSize;

            *dst = ' ';
            dst++;
            totalSize++;
            *dst = '#';
            dst++;
            totalSize++;
            *dst = ' ';
            dst++;
            totalSize++;

            tempSize = RabbitizerInstr_DisassembleInstruction(self, dst, immOverride, immOverrideLength, extraLJust);
            dst += tempSize;
            totalSize += tempSize;
        }

        return totalSize;
    }

    return RabbitizerInstr_DisassembleInstruction(self, dst, immOverride, immOverrideLength, extraLJust);
}
