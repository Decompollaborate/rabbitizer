/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstr.h"
#include "instructions/RabbitizerInstrRsp.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>

#include "common/Utils.h"
#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"


#define RABUTILS_BUFFER_WRITE_CHAR(buffer, totalSize, character) \
    do { \
        *(buffer) = (character); \
        (buffer)++; \
        (totalSize)++; \
    } while(0)

#define RABUTILS_BUFFER_SPRINTF(buffer, totalSize, format, ...) \
    do { \
        int len = sprintf(buffer, format, __VA_ARGS__);\
        assert(len > 0); \
        (buffer) += len; \
        (totalSize) += len; \
    } while(0)

#define RABUTILS_BUFFER_CPY(buffer, totalSize, string) \
    do { \
        size_t tempSize = strlen(string);\
        memcpy(buffer, string, tempSize); \
        (buffer) += tempSize; \
        (totalSize) += tempSize; \
    } while(0)

#define RABUTILS_BUFFER_ADVANCE(buffer, totalSize, expression) \
    do { \
        size_t tempSize = expression;\
        (buffer) += tempSize; \
        (totalSize) += tempSize; \
    } while(0)


typedef size_t (*OperandCallback)(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength);


size_t RabbitizerRegisterType_processRs(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameGpr(self->rs);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerRegisterType_processRt(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameGpr(self->rt);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerRegisterType_processRd(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameGpr(self->rd);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerRegisterType_processCop0d(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop0(self->rd);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerRegisterType_processFs(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop1(RabbitizerInstr_getFs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerRegisterType_processFt(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop1(RabbitizerInstr_getFt(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerRegisterType_processFd(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop1(RabbitizerInstr_getFd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerRegisterType_processCop1Cs(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop1Control(RabbitizerInstr_getFs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerRegisterType_processCop2t(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop2(self->rt);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerRegisterType_processSa(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", self->sa);
    return totalSize;
}

size_t RabbitizerRegisterType_processOp(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%02X", self->rt);
    return totalSize;
}

size_t RabbitizerRegisterType_processCode(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    int code = (self->rs << 5) | (self->rt);
    int lower = (self->rd << 5) | (self->sa);

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", code);
    if (lower) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, ", %i", lower);
    }

    return totalSize;
}

size_t RabbitizerRegisterType_processLabel(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    if (immOverride != NULL) {
        memcpy(dst, immOverride, immOverrideLength);
        return immOverrideLength;
    }

    RABUTILS_BUFFER_CPY(dst, totalSize, "func_");
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%06X", RabbitizerInstr_getInstrIndexAsVram(self));
    return totalSize;
}

size_t RabbitizerRegisterType_processImmediate(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t imm;

    if (immOverride != NULL) {
        memcpy(dst, immOverride, immOverrideLength);
        return immOverrideLength;
    }

    imm = RabbitizerInstr_getImmediate(self);
    if (!self->descriptor->isUnsigned) {
        int32_t number = RabbitizerUtils_From2Complement(imm, 16);

        if (number < 0) {
            RABUTILS_BUFFER_SPRINTF(dst, totalSize, "-0x%X", -number);
            return totalSize;
        }
    }

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%X", imm);
    return totalSize;
}

size_t RabbitizerRegisterType_processImmediateBase(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerRegisterType_processImmediate(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerRegisterType_processRs(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');

    return totalSize;
}

/*
RabbitizerRegisterTypeRsp_processRs
RabbitizerRegisterTypeRsp_processRt
RabbitizerRegisterTypeRsp_processRd
RabbitizerRegisterTypeRsp_processCop0d
RabbitizerRegisterTypeRsp_processVs
RabbitizerRegisterTypeRsp_processVt
RabbitizerRegisterTypeRsp_processVd
*/

size_t RabbitizerRegisterTypeRsp_processVtElement(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg;
    uint8_t element;

    reg = RabbitizerRegister_getNameRspVector(self->rt);
    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '[');

    element = RabbitizerInstrRsp_processVectorElement(self, RAB_INSTR_RSP_GET_ELEMENT_LOW(self));
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", element);

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ']');

    return totalSize;
}

size_t RabbitizerRegisterTypeRsp_processOffsetVs(const RabbitizerInstr *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg;

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%X", RabbitizerInstrRsp_GetOffsetVector(self));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');

    reg = RabbitizerRegister_getNameRspGpr(self->rs);
    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');

    return totalSize;
}

const OperandCallback instrOpercandCallbacks[] = {
    [RABBITIZER_REGISTER_TYPE_rs]               = RabbitizerRegisterType_processRs,
    [RABBITIZER_REGISTER_TYPE_rt]               = RabbitizerRegisterType_processRt,
    [RABBITIZER_REGISTER_TYPE_rd]               = RabbitizerRegisterType_processRd,
    [RABBITIZER_REGISTER_TYPE_cop0d]            = RabbitizerRegisterType_processCop0d,
    [RABBITIZER_REGISTER_TYPE_fs]               = RabbitizerRegisterType_processFs,
    [RABBITIZER_REGISTER_TYPE_ft]               = RabbitizerRegisterType_processFt,
    [RABBITIZER_REGISTER_TYPE_fd]               = RabbitizerRegisterType_processFd,
    [RABBITIZER_REGISTER_TYPE_cop1cs]           = RabbitizerRegisterType_processCop1Cs,
    [RABBITIZER_REGISTER_TYPE_cop2t]            = RabbitizerRegisterType_processCop2t,
    [RABBITIZER_REGISTER_TYPE_sa]               = RabbitizerRegisterType_processSa,
    [RABBITIZER_REGISTER_TYPE_op]               = RabbitizerRegisterType_processOp,
    [RABBITIZER_REGISTER_TYPE_code]             = RabbitizerRegisterType_processCode,
    [RABBITIZER_REGISTER_TYPE_LABEL]            = RabbitizerRegisterType_processLabel,
    [RABBITIZER_REGISTER_TYPE_IMM]              = RabbitizerRegisterType_processImmediate,
    [RABBITIZER_REGISTER_TYPE_IMM_base]         = RabbitizerRegisterType_processImmediateBase,

    // [RABBITIZER_REGISTER_TYPE_RSP_rs]           = RabbitizerRegisterTypeRsp_processRs,
    // [RABBITIZER_REGISTER_TYPE_RSP_rt]           = RabbitizerRegisterTypeRsp_processRt,
    // [RABBITIZER_REGISTER_TYPE_RSP_rd]           = RabbitizerRegisterTypeRsp_processRd,
    // [RABBITIZER_REGISTER_TYPE_RSP_cop0d]        = RabbitizerRegisterTypeRsp_processCop0d,
    // [RABBITIZER_REGISTER_TYPE_RSP_vs]           = RabbitizerRegisterTypeRsp_processVs,
    // [RABBITIZER_REGISTER_TYPE_RSP_vt]           = RabbitizerRegisterTypeRsp_processVt,
    // [RABBITIZER_REGISTER_TYPE_RSP_vd]           = RabbitizerRegisterTypeRsp_processVd,
    [RABBITIZER_REGISTER_TYPE_RSP_vt_element]   = RabbitizerRegisterTypeRsp_processVtElement,
    [RABBITIZER_REGISTER_TYPE_RSP_offset_rs]    = RabbitizerRegisterTypeRsp_processOffsetVs,
};


size_t RabbitizerInstr_getSizeForBufferInstrDisasm(const RabbitizerInstr *self, size_t immOverrideLength, int extraLJust) {
    size_t totalSize = 0;
    size_t opcodeNameLength;

    opcodeNameLength = strlen(RabbitizerInstrId_getOpcodeName(self->uniqueId));

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


size_t RabbitizerInstr_disassembleInstruction(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust) {
    size_t totalSize = 0;
    const char *opcodeName = RabbitizerInstrId_getOpcodeName(self->uniqueId);

    RABUTILS_BUFFER_CPY(dst, totalSize, opcodeName);

    if (self->descriptor->operands[0] == RABBITIZER_REGISTER_TYPE_INVALID) {
        // There are no operands
        *dst = '\0';
        return totalSize;
    }

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerUtils_CharFill(dst, RabbitizerConfig_Cfg.misc.opcodeLJust + extraLJust - totalSize, ' '));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ' ');

    for (size_t i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_REGISTER_TYPE_INVALID; i++) {
        RabbitizerRegisterType operand;
        OperandCallback callback;

        if (i != 0) {
            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ',');
            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ' ');
        }

        operand = self->descriptor->operands[i];
        assert(operand > RABBITIZER_REGISTER_TYPE_INVALID);
        assert(operand < RABBITIZER_REGISTER_TYPE_MAX);

        callback = instrOpercandCallbacks[operand];
        assert(callback != NULL);

        RABUTILS_BUFFER_ADVANCE(dst, totalSize, callback(self, dst, immOverride, immOverrideLength));
    }

    *dst = '\0';
    return totalSize;
}


size_t RabbitizerInstr_getSizeForBufferDataDisasm(UNUSED const RabbitizerInstr *self, int extraLJust) {
    size_t totalSize = 0;

    totalSize += strlen(".word");
    totalSize += RabbitizerConfig_Cfg.misc.opcodeLJust + extraLJust;
    totalSize += 11;
    return totalSize;
}


size_t RabbitizerInstr_disassembleAsData(const RabbitizerInstr *self, char *dst, int extraLJust) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_CPY(dst, totalSize, ".word");

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerUtils_CharFill(dst, RabbitizerConfig_Cfg.misc.opcodeLJust + extraLJust - totalSize, ' '));

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, " 0x%08X", RabbitizerInstr_getRaw(self));
    return totalSize;
}


bool RabbitizerInstr_mustDisasmAsData(const RabbitizerInstr *self) {
    if (RabbitizerConfig_Cfg.toolchainTweaks.sn64DivFix) {
        if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_break) {
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
                if (self->rd != 0 && self->uniqueId != RABBITIZER_INSTR_ID_cpu_jalr) {
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



size_t RabbitizerInstr_getSizeForBuffer(const RabbitizerInstr *self, size_t immOverrideLength, int extraLJust) {
    if (!RabbitizerInstr_isImplemented(self) || RabbitizerInstr_mustDisasmAsData(self)) {
        size_t totalSize = RabbitizerInstr_getSizeForBufferDataDisasm(self, extraLJust);

        if (RabbitizerConfig_Cfg.misc.unknownInstrComment) {
            totalSize += 40;
            totalSize += 3;
            totalSize += RabbitizerInstr_getSizeForBufferInstrDisasm(self, immOverrideLength, extraLJust);
        }
        return totalSize;
    }

    return RabbitizerInstr_getSizeForBufferInstrDisasm(self,immOverrideLength, extraLJust);
}


size_t RabbitizerInstr_disassemble(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust) {
    assert(dst != NULL);

    if (!RabbitizerInstr_isImplemented(self) || RabbitizerInstr_mustDisasmAsData(self)) {
        size_t totalSize = 0;

        RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerInstr_disassembleAsData(self, dst, extraLJust));

        if (RabbitizerConfig_Cfg.misc.unknownInstrComment) {
            RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerUtils_CharFill(dst, 40-totalSize, ' '));

            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ' ');
            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '#');
            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ' ');

            RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerInstr_disassembleInstruction(self, dst, immOverride, immOverrideLength, extraLJust));
        }

        return totalSize;
    }

    return RabbitizerInstr_disassembleInstruction(self, dst, immOverride, immOverrideLength, extraLJust);
}
